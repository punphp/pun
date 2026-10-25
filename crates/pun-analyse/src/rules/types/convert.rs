//! Mago's inferred types as PHPStan's types (`pun_phpstan::types::Type`), so rules compare and describe them the way
//! PHPStan's rules do (`Type::describe(VerbosityLevel)`).

use mago_codex::misc::GenericParent;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::generic::TGenericParameter;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::float::TFloat;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::atomic::scalar::string::{TStringCasing, TStringLiteral};
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::{ArrayElement, Expression, TokenSeparatedSequence};
use pun_phpstan::types::{
    Accessory, ConstArray, NeverKind, Param, ShapeBuilder, Signature, Template, Tri, Type, intersect, mixed, union,
};

use super::super::{Checker, declared, doc, infer};

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

impl<'a, 'arena> Checker<'a, 'arena> {
    /// `$scope->getType($expr)`: Mago's type of an expression the rules asked about, as PHPStan's.
    pub(in super::super) fn phpstan_type(&self, e: &mago_syntax::cst::Expression) -> Option<Type> {
        Some(self.phpstan_type_from(e, self.type_of(e)?))
    }

    /// Convert an observed scope value using the expression's own source metadata.
    pub(in super::super) fn phpstan_type_from(&self, e: &mago_syntax::cst::Expression, t: &TUnion) -> Type {
        use mago_syntax::cst::Expression as E;
        // Mago types `new` of an unknown class `never` (NonExistentClass), PHPStan as an object of that class
        if let E::Instantiation(new) = e
            && let E::Identifier(id) = new.class
            && t.is_never()
            && self.codebase.get_class_like(self.names.get(id)).is_none()
        {
            return Type::Object(text(self.names.get(id)), vec![]);
        }
        declared::with_classes(self, || {
            let source = self.source_type(e.span(), &self.origins);
            // Mago's keyed arrays are BTreeMaps: only the literal still knows PHP's insertion order. Unknown
            // keys/unpacks keep the inferred fallback, rather than inventing an order for a nonliteral shape.
            let ty = self.literal_array(e).unwrap_or_else(|| {
                source.as_ref().and_then(|s| self.source_array(t, s))
                    .unwrap_or_else(|| self.to_phpstan(t))
            });
            let ty = source.as_ref().and_then(|s| self.source_template(e.span(), &ty, s))
                .unwrap_or(ty);
            let ty = self.refine_method_guards(e, ty);
            // MutatingScope::enterFunction/enterClassMethod converts PHPDoc parameter/return values to arguments.
            // An inline @var annotation does not: its source metadata can restore that distinct template strategy.
            let ty = inferred_argument(&ty);
            source.map_or(ty.clone(), |source| {
                restore_metadata(ty, &source, !self.source_is_mutated(e.span(), &self.origins))
            })
        })
    }

    /// Resolve an array-bearing source edge's template metadata. The actual keyed array now carries its
    /// own constant/generic source kind; a declaration must not turn a refined generic array into a shape.
    fn source_array(&self, actual: &TUnion, source: &Type) -> Option<Type> {
        match source {
            Type::ConstArray(_) => match actual.types.as_ref() {
                [TAtomic::Array(a)] => Some(self.array(a)),
                _ => None,
            },
            Type::Template(t) => {
                if let [TAtomic::GenericParameter(g)] = actual.types.as_ref() {
                    return self
                        .source_array(&g.constraint, &t.bound)
                        .map(|bound| self.generic_parameter(g, bound));
                }
                self.source_array(actual, &t.bound)
            }
            Type::Union(parts) => actual
                .types
                .iter()
                .map(|a| {
                    let actual = TUnion::from_atomic(a.clone());
                    parts.iter().find_map(|s| self.source_array(&actual, s))
                })
                .collect::<Option<Vec<_>>>()
                .map(union),
            Type::Intersection(parts) => parts.iter().find_map(|s| self.source_array(actual, s)),
            _ => None,
        }
    }

    /// Mago rejects a parenthesized intersection template bound, then reads its uses as a class name. Recover
    /// only a parameter/annotation's own array template, resolved in that declaration's exact namespace scope.
    fn source_template(&self, at: Span, actual: &Type, source: &Type) -> Option<Type> {
        // source_type() has already validated every deferred call on this edge.
        let mut origin = self.origins.get(&at)?;
        let key = loop {
            match origin {
                infer::Origin::Copy(from) | infer::Origin::Unset(from, _) => origin = self.origins.get(from)?,
                infer::Origin::AfterMethodCall(_, previous) => origin = previous,
                infer::Origin::Parameter(key, _) | infer::Origin::Annotation(_, _, Some(key)) => {
                    break key;
                }
                _ => return None,
            }
        };
        let (Type::Object(name, args), Type::Template(template)) = (actual, source) else {
            return None;
        };
        let function = self.codebase.function_likes.get(key)?;
        let scope = declared::value_scope(self, function, self.codebase.get_class_like(key.0.as_bytes()));
        let expected = scope.names.resolve(mago_names::kind::NameKind::Default, template.name.as_bytes()).0;
        (args.is_empty() && template.bound.is_array().yes()
            && name.as_bytes().eq_ignore_ascii_case(&expected)
            && matches!(scope.templates.get(&template.name), Some(Type::Template(t)) if t.scope == template.scope))
        .then(|| source.clone())
    }

    /// Only recover metadata from the expression's source edges. In particular, a structurally equal but
    /// unrelated declaration is never evidence for an array's order or an implicit mixed's explicitness.
    fn source_type(
        &self,
        span: Span,
        origins: &std::collections::HashMap<Span, infer::Origin>,
    ) -> Option<Type> {
        self.origin_type(origins.get(&span)?, origins)
    }

    /// Mutable ordering scaffolds do not carry declarations' list-certainty metadata.
    fn source_is_mutated(&self, span: Span, origins: &infer::Origins) -> bool {
        let mut origin = origins.get(&span);
        while let Some(infer::Origin::AfterMethodCall(_, previous)) = origin {
            origin = Some(previous);
        }
        match origin {
            Some(infer::Origin::Unset(..)) => true,
            Some(infer::Origin::Copy(from) | infer::Origin::Offset(from, _)) => self.source_is_mutated(*from, origins),
            Some(infer::Origin::Array(items)) => items.iter().any(|(_, value)| self.source_is_mutated(*value, origins)),
            Some(infer::Origin::Call(call)) if call.receiver.is_none() && call.class.is_none()
                && matches!(call.name.as_str(), "array_slice" | "array_unique") => {
                call.args.iter().find(|(name, _)| name.as_deref().is_none_or(|name| name == "array"))
                    .is_some_and(|(_, span)| self.source_is_mutated(*span, origins))
            }
            _ => false,
        }
    }

    fn origin_type(
        &self,
        origin: &infer::Origin,
        origins: &std::collections::HashMap<Span, infer::Origin>,
    ) -> Option<Type> {
        let function = |key: &(mago_word::Word, mago_word::Word)| {
            Some((
                self.codebase.function_likes.get(key)?.as_ref(),
                self.codebase.get_class_like(key.0.as_bytes()),
            ))
        };
        match origin {
            infer::Origin::Parameter(key, i) => {
                let (f, k) = function(key)?;
                declared::codex_params(self, f, k)
                    .into_iter()
                    .nth(*i)
                    .map(|t| inferred_argument(&t))
            }
            infer::Origin::Annotation(at, name, key) => {
                let (raw, _) = declared::doc_at(self, *at)?;
                let scope = key.and_then(|key| function(&key)).map_or_else(
                    || declared::scope_at(self, *at, self.class_scope()),
                    |(f, k)| declared::value_scope(self, f, k),
                );
                doc::Doc::parse(&raw)
                    .var_types(&scope)
                    .into_iter()
                    .find(|(n, _)| n.is_empty() || n == name)
                    .map(|(_, t)| t)
            }
            infer::Origin::Copy(from) => self.source_type(*from, origins),
            infer::Origin::Unset(from, offsets) => {
                let source = self.source_type(*from, origins)?;
                let keys = offsets.iter().map(|span| {
                    self.types.get(span).and_then(|t| array_key(self.to_phpstan(t)))
                }).collect::<Vec<_>>();
                Some(unset_order(source, &keys))
            }
            infer::Origin::Array(items) => {
                let mut builder = ShapeBuilder::new();
                for (key, value) in items {
                    let key = match key {
                        Some(key) => Some(array_key(self.to_phpstan(self.types.get(key)?))?),
                        None => None,
                    };
                    if key == Some(Type::ConstInt(i64::MAX)) { return None; }
                    let actual = self.to_phpstan(self.types.get(value)?);
                    let value = self.source_type(*value, origins)
                        .map_or(actual.clone(), |source| restore(actual, &source));
                    builder.set(key, value, false);
                }
                Some(builder.get())
            }
            infer::Origin::Offset(array, key) => {
                let Type::ConstArray(source) = self.source_type(*array, origins)? else {
                    return None;
                };
                let key = array_key(self.to_phpstan(self.types.get(key)?))?;
                source.keys.iter().position(|k| *k == key).map(|i| source.values[i].clone())
            }
            infer::Origin::AfterMethodCall(call, previous) => {
                let actual = self.to_phpstan(self.types.get(&call.receiver)?);
                let classes = actual.object_class_names();
                // An annotation or magic fallback is not evidence for the invoked PHP signature. Every
                // native alternative/variant must be known and reference-free, including unpacked calls.
                let safe = !classes.is_empty()
                    && classes.iter().all(|name| {
                        self.codebase
                            .get_class_like(name.as_bytes())
                            .and_then(|class| {
                                super::super::members::method(self, class, call.name.as_bytes())
                            })
                            .is_some_and(|method| {
                                let variants = method.variants();
                                variants
                                    .all
                                    .iter()
                                    .chain(variants.named.iter().flatten())
                                    .all(|v| {
                                        v.by_ref
                                            .iter()
                                            .all(|b| *b == pun_phpstan::symbols::ByRef::No)
                                    })
                            })
                    });
                safe.then(|| self.origin_type(previous, origins)).flatten()
            }
            infer::Origin::Call(call) | infer::Origin::Callback(call, ..) => {
                let (name, args) = (&call.name, &call.args);
                // These extensions preserve their input key/value types. The provider alone decides keys,
                // cardinality and listness; this edge restores only information its Mago types cannot store.
                if call.receiver.is_none()
                    && call.class.is_none()
                    && matches!(origin, infer::Origin::Call(_))
                    && matches!(name.as_str(), "array_slice" | "array_unique")
                {
                    let (_, from) = args
                        .iter()
                        .enumerate()
                        .find(|(i, (n, _))| {
                            n.as_deref() == Some("array") || (*i == 0 && n.is_none())
                        })?
                        .1;
                    let source = self.source_type(*from, origins)?;
                    return Some(if matches!(source, Type::Mixed { .. }) {
                        Type::array(source.clone(), source)
                    } else {
                        source
                    });
                }
                let (f, declaring, receiver_map) =
                    if call.receiver.is_some() || call.class.is_some() {
                        let (mut receiver, class) = if let Some(span) = call.receiver {
                            let actual = self.to_phpstan(self.types.get(&span)?);
                            let mut receiver = self
                                .source_type(span, origins)
                                .map_or(actual.clone(), |source| restore(actual, &source));
                            while let Type::Template(template) = receiver {
                                receiver = template.bound;
                            }
                            let name = match &receiver {
                                Type::Object(name, _)
                                | Type::Static(name, _)
                                | Type::This(name) => name,
                                _ => return None,
                            };
                            let class = self.codebase.get_class_like(name.as_bytes())?;
                            (receiver, class)
                        } else {
                            let written = call.class?;
                            let name = written.as_bytes();
                            let context = match name {
                                b"self" | b"static" | b"parent" => self.class_scope()?,
                                _ => self.codebase.get_class_like(name)?,
                            };
                            let class = if name == b"parent" {
                                self.codebase
                                    .get_class_like(context.direct_parent_class?.as_bytes())?
                            } else {
                                context
                            };
                            let args = if matches!(name, b"self" | b"static" | b"parent") {
                                declared::class_templates(self, context)
                                    .into_iter()
                                    .map(|t| Type::Template(Box::new(t)))
                                    .collect()
                            } else {
                                Vec::new()
                            };
                            (Type::Object(declared::original_name(context), args), class)
                        };
                        if matches!(receiver, Type::This(_)) {
                            receiver = Type::Object(
                                declared::original_name(class),
                                declared::class_templates(self, class)
                                    .into_iter()
                                    .map(|t| Type::Template(Box::new(t)))
                                    .collect(),
                            );
                        }
                        let super::super::members::Method::User { declaring, f } =
                            super::super::members::method(self, class, name.as_bytes())?
                        else {
                            return None;
                        };
                        let pattern = Type::Object(
                            declared::original_name(declaring),
                            declared::class_templates(self, declaring)
                                .into_iter()
                                .map(|t| Type::Template(Box::new(t)))
                                .collect(),
                        );
                        let map = pun_phpstan::types::infer_template_types(&pattern, &receiver);
                        (f, Some(declaring), map)
                    } else {
                        (
                            self.codebase.get_function(name.as_bytes())?,
                            None,
                            pun_phpstan::types::TemplateMap::default(),
                        )
                    };
                let infer::Origin::Callback(_, argument, parameter) = origin else {
                    return declared::codex_return(self, f, declaring).map(|t| {
                        inferred_argument(&pun_phpstan::types::resolve_template_types(
                            &t,
                            &receiver_map,
                        ))
                    });
                };
                let params: Vec<_> = declared::codex_params(self, f, declaring)
                    .iter()
                    .map(|t| pun_phpstan::types::resolve_template_types(t, &receiver_map))
                    .collect();
                let positions: Vec<_> = args
                    .iter()
                    .enumerate()
                    .map(|(i, (name, _))| match name {
                        Some(n) => f
                            .parameters
                            .iter()
                            .position(|p| &p.name.0.as_bytes()[1..] == n.as_bytes()),
                        None => Some(i),
                    })
                    .collect();
                let callable = params.get(positions[*argument]?)?;
                let mut map = pun_phpstan::types::TemplateMap::default();
                for (i, (_, span)) in args.iter().enumerate().filter(|(i, _)| i != argument) {
                    let Some(p) = positions[i].and_then(|i| params.get(i)) else {
                        continue;
                    };
                    let actual = self.types.get(span)?;
                    let actual = self.to_phpstan(actual);
                    let actual = self
                        .source_type(*span, origins)
                        .map_or(actual.clone(), |source| restore(actual, &source));
                    map = map.union(declared::with_classes(self, || {
                        pun_phpstan::types::infer_template_types(p, &actual)
                    }));
                }
                let Type::Callable(Some(signature)) =
                    pun_phpstan::types::resolve_template_types(callable, &map)
                else {
                    return None;
                };
                signature.params.get(*parameter).map(|p| p.ty.clone())
            }
        }
    }

    /// InitializerExprTypeResolver::getArrayType(), bounded to required constant keys and literal unpacks.
    fn literal_array(&self, e: &Expression) -> Option<Type> {
        let elements = literal_elements(e)?;
        if elements.len() > 256 { return None; }
        let mut builder = ShapeBuilder::new();
        for element in elements.iter() {
            let value = self.phpstan_type(element.get_value()?)?;
            match element {
                ArrayElement::KeyValue(kv) => {
                    let key = array_key(self.phpstan_type(kv.key)?)?;
                    // There is no next auto index beyond PHP_INT_MAX. Keep Mago's result for that edge.
                    if key == Type::ConstInt(i64::MAX) { return None; }
                    builder.set(Some(key), value, false);
                }
                ArrayElement::Variadic(v) => {
                    literal_elements(v.value)?;
                    let Type::ConstArray(a) = value else { return None; };
                    if !a.optional.is_empty() || a.unsealed.is_some() { return None; }
                    for (key, value) in a.keys.iter().zip(&a.values) {
                        // Unpacking renumbers integer keys, but replaces string keys in their original slots.
                        let key = matches!(key, Type::ConstString(_)).then(|| key.clone());
                        builder.set(key, value.clone(), false);
                    }
                }
                _ => builder.set(None, value, false),
            }
        }
        Some(builder.get())
    }

    /// A Mago type as PHPStan's.
    pub(in super::super) fn to_phpstan(&self, t: &TUnion) -> Type {
        // Combinators need declared variance before they decide whether generic alternatives are redundant.
        declared::with_classes(self, || {
            let types = union(t.types.iter().map(|a| self.atom(a)).collect());
            if super::super::return_types::is_benevolent(t) {
                return pun_phpstan::types::to_benevolent(types);
            }
            types
        })
    }

    fn signature(&self, s: &mago_codex::ttype::atomic::callable::TCallableSignature) -> Box<Signature> {
        Box::new(Signature {
            params: s
                .parameters
                .iter()
                .map(|p| Param {
                    name: p.get_name().map(|n| String::from_utf8_lossy(&n.0.as_bytes()[1..]).into_owned()).unwrap_or_default(),
                    ty: p.get_type_signature().map_or_else(mixed, |t| self.to_phpstan(t)),
                    optional: p.has_default(),
                    by_ref: p.is_by_reference(),
                    variadic: p.is_variadic(),
                })
                .collect(),
            ret: s.return_type.as_deref().map_or_else(mixed, |t| self.to_phpstan(t)),
            templates: vec![],
        })
    }

    /// A class name as its reflection names it (the declaration's case, an anonymous class's display name).
    pub(super) fn reflected_name(&self, name: &[u8]) -> String {
        match self.codebase.get_class_like(name) {
            Some(k) if k.name.as_bytes().starts_with(b"{anonymous-class:") => self.display(k),
            Some(k) => text(k.original_name.as_bytes()),
            None => text(name),
        }
    }

    fn atom(&self, a: &TAtomic) -> Type {
        match a {
            TAtomic::Never => Type::Never(NeverKind::Implicit),
            TAtomic::Null => Type::Null,
            TAtomic::Void => Type::Void,
            TAtomic::Scalar(s) => self.scalar(s),
            TAtomic::Mixed(m) if m.is_error() => Type::Error,
            TAtomic::Mixed(m) if m.get_subtracted_type().is_some() => {
                Type::mixed_subtracted(false, self.to_phpstan(m.get_subtracted_type().unwrap()))
            }
            TAtomic::Mixed(_) => mixed(),
            TAtomic::Resource(_) => Type::Resource,
            TAtomic::Object(o) => self.object(o),
            TAtomic::Array(a) => self.array(a),
            TAtomic::Iterable(i) => {
                let it = Type::Iterable(
                    Box::new(self.to_phpstan(&i.key_type)),
                    Box::new(self.to_phpstan(&i.value_type)),
                );
                self.with_intersections(it, i.intersection_types.as_deref())
            }
            TAtomic::Callable(TCallable::Signature(s)) => {
                if s.is_closure {
                    Type::Closure(self.signature(s))
                } else {
                    Type::Callable(Some(self.signature(s)))
                }
            }
            // Mago's alias of a function-like (a closure assigned to a variable, `Closure::getCurrent()`): a ClosureType
            // of its signature
            TAtomic::Callable(TCallable::Alias(id)) => {
                mago_codex::ttype::expander::get_signature_of_function_like_identifier(id, self.codebase)
                    .map_or(Type::Callable(None), |s| Type::Closure(self.signature(&s)))
            }
            TAtomic::Reference(TReference::Symbol {
                name, parameters, ..
            }) => Type::Object(
                self.reflected_name(name.as_bytes()),
                parameters
                    .iter()
                    .flatten()
                    .map(|p| self.to_phpstan(p))
                    .collect(),
            ),
            TAtomic::GenericParameter(g) => {
                self.generic_parameter(g, explicit_mixed(self.to_phpstan(&g.constraint)))
            }
            _ => mixed(),
        }
    }

    fn generic_parameter(&self, g: &TGenericParameter, bound: Type) -> Type {
        let t = Type::Template(Box::new(Template {
            name: text(g.parameter_name.as_bytes()),
            scope: self.template_scope(&g.defining_entity),
            bound,
            default: None,
            argument: false,
        }));
        self.with_intersections(t, g.intersection_types.as_deref())
    }

    fn with_intersections(&self, t: Type, more: Option<&[TAtomic]>) -> Type {
        match more {
            Some(more) if !more.is_empty() => {
                intersect(std::iter::once(t).chain(more.iter().map(|m| self.atom(m))).collect())
            }
            _ => t,
        }
    }

    /// `TemplateTypeScope::describe()`.
    fn template_scope(&self, parent: &GenericParent) -> String {
        match parent {
            GenericParent::ClassLike(name) => format!("class {}", self.reflected_name(name.as_bytes())),
            // (Mago names a function's templates by an empty class and the function's lowercase name)
            GenericParent::FunctionLike((class, function)) if class.is_empty() => {
                let name = self.codebase.get_function(function.as_bytes()).map_or(function.as_bytes(), |f| f.original_name.as_bytes());
                format!("function {}()", text(name))
            }
            GenericParent::FunctionLike((class, method)) => {
                let name = self
                    .codebase
                    .get_method(class.as_bytes(), method.as_bytes())
                    .map_or(method.as_bytes(), |m| m.original_name.as_bytes());
                format!("method {}::{}()", self.reflected_name(class.as_bytes()), text(name))
            }
        }
    }

    fn scalar(&self, s: &TScalar) -> Type {
        match s {
            TScalar::Bool(b) => b.value.map_or(Type::Bool, Type::ConstBool),
            TScalar::Integer(i) => match i {
                TInteger::Literal(v) => Type::ConstInt(*v),
                TInteger::From(v) => Type::int_range(Some(*v), None),
                TInteger::To(v) => Type::int_range(None, Some(*v)),
                TInteger::Range(a, b) => Type::int_range(Some(*a), Some(*b)),
                _ => Type::Int,
            },
            TScalar::Float(TFloat::Literal(f)) => Type::ConstFloat(f.0),
            TScalar::Float(_) => Type::Float,
            TScalar::String(s) => {
                if let Some(TStringLiteral::Value(v)) = &s.literal {
                    return Type::ConstString(text(v.as_bytes()));
                }
                let mut accessories = Vec::new();
                if s.is_numeric {
                    accessories.push(Accessory::NumericString);
                }
                if s.is_truthy {
                    accessories.push(Accessory::NonFalsyString);
                } else if s.is_non_empty {
                    accessories.push(Accessory::NonEmptyString);
                }
                if matches!(s.literal, Some(TStringLiteral::Unspecified)) {
                    accessories.push(Accessory::LiteralString);
                }
                match s.casing {
                    TStringCasing::Lowercase => accessories.push(Accessory::LowercaseString),
                    TStringCasing::Uppercase => accessories.push(Accessory::UppercaseString),
                    TStringCasing::Unspecified => {}
                }
                if s.is_callable {
                    return Type::Intersection(vec![Type::String, Type::Callable(None)]);
                }
                if accessories.is_empty() {
                    Type::String
                } else {
                    intersect(
                        std::iter::once(Type::String)
                            .chain(accessories.into_iter().map(Type::Accessory))
                            .collect(),
                    )
                }
            }
            TScalar::ClassLikeString(c) => match c {
                TClassLikeString::Literal { value } => Type::ConstString(self.reflected_name(value.as_bytes())),
                TClassLikeString::OfType { constraint, .. } => {
                    Type::ClassString(Some(Box::new(self.atom(constraint))))
                }
                TClassLikeString::Generic {
                    parameter_name,
                    defining_entity,
                    constraint,
                    ..
                } => Type::ClassString(Some(Box::new(Type::Template(Box::new(Template {
                    name: text(parameter_name.as_bytes()),
                    scope: self.template_scope(defining_entity),
                    bound: explicit_mixed(self.atom(constraint)),
                    default: None,
                    argument: false,
                }))))),
                TClassLikeString::Any { .. } => Type::ClassString(None),
            },
            TScalar::ArrayKey => Type::Benevolent(vec![Type::Int, Type::String]),
            TScalar::Numeric => Type::Union(vec![
                Type::Int,
                Type::Float,
                Type::Intersection(vec![Type::String, Type::Accessory(Accessory::NumericString)]),
            ]),
            TScalar::Generic => Type::Union(vec![Type::Int, Type::Float, Type::String, Type::Bool]),
        }
    }

    fn object(&self, o: &TObject) -> Type {
        match o {
            TObject::Any => Type::ObjectWithoutClass,
            TObject::Named(n) => {
                let mut name = self.reflected_name(n.name.as_bytes());
                // a trait's `$this` / `static` is the class the trait is analysed in
                if (n.is_this || n.is_static)
                    && self
                        .codebase
                        .get_class_like(n.name.as_bytes())
                        .is_some_and(|k| k.kind == mago_codex::symbol::SymbolKind::Trait)
                    && let Some(k) = self.class_scope()
                {
                    name = self.reflected_name(k.name.as_bytes());
                }
                let args: Vec<Type> = n
                    .type_parameters
                    .iter()
                    .flat_map(|p| p.iter())
                    .map(|p| self.to_phpstan(p))
                    .collect();
                let t = if n.is_this {
                    Type::This(name)
                } else if n.is_static {
                    Type::Static(name, args)
                } else {
                    Type::Object(name, args)
                };
                self.with_intersections(t, n.intersection_types.as_deref().map(Vec::as_slice))
            }
            TObject::Enum(e) => match &e.case {
                Some(case) => Type::EnumCase(self.reflected_name(e.name.as_bytes()), text(case.as_bytes())),
                None => Type::Object(self.reflected_name(e.name.as_bytes()), vec![]),
            },
            TObject::WithProperties(p) => Type::ObjectShape(
                p.known_properties
                    .iter()
                    .map(|(name, (optional, t))| (text(name.as_bytes()), self.to_phpstan(t), *optional))
                    .collect(),
            ),
            _ => Type::ObjectWithoutClass,
        }
    }

    fn key(&self, k: &ArrayKey) -> Type {
        match k {
            ArrayKey::Integer(i) => Type::ConstInt(*i),
            ArrayKey::String(s) => Type::ConstString(text(s.as_bytes())),
            k => super::super::enums::constant_value(self, &k.to_atomic()).unwrap_or(Type::Error),
        }
    }

    fn array(&self, a: &TArray) -> Type {
        match a {
            TArray::List(l) => {
                if l.element_type.is_never() && l.known_elements.is_some() {
                    let mut shape = ConstArray::empty();
                    for (i, (optional, v)) in l.known_elements.iter().flat_map(|k| k.iter()) {
                        if *optional {
                            shape.optional.push(shape.keys.len());
                        }
                        shape.keys.push(Type::ConstInt(*i as i64));
                        shape.values.push(self.to_phpstan(v));
                    }
                    shape.is_list = Tri::Yes;
                    return finish_shape(shape);
                }
                let mut item = self.to_phpstan(&l.element_type);
                if let Some(known) = &l.known_elements {
                    item = union(
                        std::iter::once(item)
                            .chain(known.values().map(|(_, v)| self.to_phpstan(v)))
                            .collect(),
                    );
                }
                let list = Type::list(item);
                if l.non_empty || l.known_elements.as_ref().is_some_and(|k| k.values().any(|(o, _)| !o)) {
                    intersect(vec![list, Type::Accessory(Accessory::NonEmptyArray)])
                } else {
                    list
                }
            }
            TArray::Keyed(k) => {
                match &k.parameters {
                    Some((key, value)) if !key.is_never() && !k.is_constant_shape() =>
                    {
                        let mut keys = vec![self.to_phpstan(key)];
                        let mut values = vec![self.to_phpstan(value)];
                        for (k, (_, v)) in k.known_items.iter().flat_map(|k| k.iter()) {
                            keys.push(self.key(k));
                            values.push(self.to_phpstan(v));
                        }
                        let mut parts = vec![Type::array(union(keys), union(values))];
                        if k.non_empty
                            || k.known_items.as_ref().is_some_and(|i| i.values().any(|(o, _)| !o))
                        {
                            parts.push(Type::Accessory(Accessory::NonEmptyArray));
                        }
                        parts.extend(k.known_items.iter().flat_map(|k| k.iter()).filter_map(|(key, (optional, value))| {
                            (!optional).then(|| Type::Accessory(Accessory::HasOffsetValue(Box::new(self.key(key)), Box::new(self.to_phpstan(value)))))
                        }));
                        intersect(parts)
                    }
                    parameters => {
                        let mut shape = ConstArray::empty();
                        for (key, (optional, v)) in k.known_items.iter().flat_map(|k| k.iter()) {
                            if *optional {
                                shape.optional.push(shape.keys.len());
                            }
                            shape.keys.push(self.key(key));
                            shape.values.push(self.to_phpstan(v));
                        }
                        if let Some((key, value)) = parameters
                            && !key.is_never()
                        {
                            let key = self.to_phpstan(key);
                            if shape.keys.is_empty() {
                                shape.is_list = if key.is_integer().yes() { Tri::Maybe } else { Tri::No };
                            }
                            shape.unsealed = Some((key, self.to_phpstan(value)));
                        }
                        if shape.keys.is_empty() && shape.unsealed.is_some() {
                            // Recreate an existing ConstantArrayType, not a new PHPDoc shape. The
                            // builder intentionally normalizes newly declared empty shapes to ArrayType.
                            Type::ConstArray(Box::new(shape))
                        } else {
                            finish_shape(shape)
                        }
                    }
                }
            }
        }
    }
}

/// MutatingScope::unsetExpression writes each parent back with ConstantArrayTypeBuilder's slot order.
/// This is a source-order scaffold only: removed values stay removed, and new values come from inference.
fn unset_order(source: Type, keys: &[Option<Type>]) -> Type {
    match source {
        Type::ConstArray(mut shape) => {
            let (key, path) = keys.split_first().expect("an offset unset has a key");
            // No single order is known beyond a weak/unknown key. A known parent before it can still
            // be appended, even when the terminal key is unknown.
            let Some(key) = key else { return Type::ConstArray(shape); };
            let position = shape.keys.iter().position(|k| k == key);
            if path.is_empty() {
                if let Some(i) = position {
                    shape.keys.remove(i);
                    shape.values.remove(i);
                    shape.optional = shape.optional.into_iter().filter_map(|p| {
                        (p != i).then(|| if p > i { p - 1 } else { p })
                    }).collect();
                }
            } else {
                let i = position.unwrap_or_else(|| {
                    shape.keys.push(key.clone());
                    shape.values.push(shape.unsealed.as_ref().map_or_else(
                        || Type::ConstArray(Box::new(ConstArray::empty())), |(_, value)| value.clone(),
                    ));
                    shape.keys.len() - 1
                });
                shape.values[i] = unset_order(shape.values[i].clone(), path);
            }
            Type::ConstArray(shape)
        }
        Type::Union(parts) => Type::Union(parts.into_iter().map(|t| unset_order(t, keys)).collect()),
        source => source,
    }
}

/// Recover erased source metadata, never a declaration's broader value type over a concrete inferred type.
fn restore(actual: Type, source: &Type) -> Type {
    restore_metadata(actual, source, true)
}

fn restore_metadata(actual: Type, source: &Type, restore_list: bool) -> Type {
    let matching = |actual: &Type, source: &[Type]| {
        source.iter().position(|s| {
            pun_phpstan::types::is_super_type_of(s, actual).yes()
                || (matches!(actual, Type::Callable(Some(a)) if unspecified_callable(a))
                    && declared_callable(s).is_some())
        })
    };
    match (actual, source) {
        (Type::Template(mut actual), Type::Template(source))
            if actual.name == source.name && actual.scope == source.scope =>
        {
            actual.argument = source.argument;
            // A matching template identity proves which bound was erased. A concrete narrowed bound still
            // wins; only Mago's unconstrained fallback can regain the source's callable-array intersection.
            actual.bound = if matches!(actual.bound, Type::Mixed { .. }) {
                source.bound.clone()
            } else {
                restore_metadata(actual.bound, &source.bound, restore_list)
            };
            Type::Template(actual)
        }
        (actual, Type::Template(source)) => restore_metadata(actual, &source.bound, restore_list),
        (Type::Union(mut actual), Type::Union(source)) => {
            // Reflection's first method comes from the first written alternative, not Mago's canonical
            // atomic order. Reorder only surviving inferred alternatives; never reintroduce a removed one.
            actual.sort_by_key(|a| matching(a, source).unwrap_or(usize::MAX));
            Type::Union(
                actual
                    .into_iter()
                    .map(|a| {
                        matching(&a, source).map_or_else(|| a.clone(), |i| restore_metadata(a.clone(), &source[i], restore_list))
                    })
                    .collect(),
            )
        }
        (actual, Type::Union(source)) => {
            // A narrowed single alternative still owns its metadata, not its eliminated siblings' types.
            matching(&actual, source)
                .map_or_else(|| actual.clone(), |i| restore_metadata(actual.clone(), &source[i], restore_list))
        }
        (Type::Mixed { .. }, Type::Mixed { explicit }) => Type::Mixed {
            explicit: *explicit,
        },
        // Mago's PHPDoc parser reads an unnamed trailing `&` as an intersection and rejects the tag. The
        // surviving native callable is exactly TCallableSignature::mixed(false), not an inferred signature.
        (Type::Callable(Some(actual)), source) if unspecified_callable(&actual) => {
            declared_callable(source).unwrap_or(Type::Callable(Some(actual)))
        }
        (Type::Union(parts), source)
            if parts.iter().any(|t| matches!(t, Type::Callable(Some(s)) if unspecified_callable(s))) =>
        {
            // Restore only callable members: a narrowed callable must not regain the declaration's int/null.
            union(parts.into_iter().map(|t| restore(t, source)).collect())
        }
        (Type::Object(name, actual), Type::Object(source_name, source))
            if name.eq_ignore_ascii_case(source_name) && actual.len() == source.len() =>
        {
            Type::Object(
                name,
                actual
                    .into_iter()
                    .zip(source)
                    .map(|(a, s)| restore(a, s))
                    .collect(),
            )
        }
        (Type::Union(parts), source @ Type::ConstArray(_)) => {
            union(parts.into_iter().map(|t| restore_metadata(t, source, restore_list)).collect())
        }
        (Type::ConstArray(mut actual), Type::ConstArray(source))
            if actual.keys.iter().all(|k| source.keys.contains(k)) =>
        {
            // ConstantArrayType::unsetOffset() keeps the surviving keys in their original slots. Mago's
            // BTreeMap cannot retain that order, including inside weak writes' union alternatives.
            let positions: Vec<_> = source
                .keys
                .iter()
                .enumerate()
                .filter_map(|(i, k)| actual.keys.iter().position(|a| a == k).map(|p| (i, p)))
                .collect();
            actual.values = positions
                .iter()
                .map(|(i, p)| restore_metadata(actual.values[*p].clone(), &source.values[*i], restore_list))
                .collect();
            actual.optional = positions
                .iter()
                .enumerate()
                .filter_map(|(i, (_, p))| actual.optional.contains(p).then_some(i))
                .collect();
            actual.keys = positions.iter().map(|(i, _)| source.keys[*i].clone()).collect();
            if restore_list && actual.keys.len() == source.keys.len() && actual.optional == source.optional {
                actual.is_list = source.is_list;
            } else if !restore_list {
                // Reordering can disprove Mago's canonical map's listness, but never regain certainty
                // lost by an actual unset. The mutable scaffold's original list flag is not evidence.
                actual.is_list = actual.is_list.and(finish_shape((*actual).clone()).is_list());
            }
            actual.unsealed = match (&actual.unsealed, &source.unsealed) {
                (Some((k, v)), Some((sk, sv))) => Some((restore(k.clone(), sk), restore(v.clone(), sv))),
                (None, Some((Type::Never(NeverKind::Explicit | NeverKind::NonAccepting), _))) => {
                    // ConstantArrayType's constructor treats every explicit never subclass as a sealed key
                    // and normalizes its value to explicit never. The shared model stores that base kind.
                    Some((Type::Never(NeverKind::Explicit), Type::Never(NeverKind::Explicit)))
                }
                _ => actual.unsealed,
            };
            Type::ConstArray(actual)
        }
        (Type::Array(k, v), Type::Array(sk, sv) | Type::Iterable(sk, sv)) => {
            Type::array(restore(*k, sk), restore(*v, sv))
        }
        (Type::Iterable(k, v), Type::Array(sk, sv) | Type::Iterable(sk, sv)) => {
            Type::Iterable(Box::new(restore(*k, sk)), Box::new(restore(*v, sv)))
        }
        (Type::Intersection(parts), source) => {
            intersect(parts.into_iter().map(|t| restore_metadata(t, source, restore_list)).collect())
        }
        (actual @ Type::Array(..), source @ Type::Intersection(_))
            if source.is_array().yes() && source.is_callable().yes() =>
        {
            // Unlike a union's unrelated callable member, this source guarantees both properties. An
            // intersection retains actual key/value narrowing instead of replacing it with the declaration.
            intersect(vec![actual, source.clone()])
        }
        (actual, Type::Intersection(parts)) => parts.iter().fold(actual, |actual, source| restore_metadata(actual, source, restore_list)),
        (actual, _) => actual,
    }
}

/// The native/common callable fallback, not a concrete closure or a signature with parameter/return information.
fn unspecified_callable(s: &Signature) -> bool {
    matches!(s.params.as_slice(), [p] if p.name.is_empty() && !p.by_ref && p.variadic && p.optional && p.ty == mixed())
        && s.ret == mixed()
        && s.templates.is_empty()
}

/// Only signatures on this value's source edge, excluding unrelated non-callable members of a union.
fn declared_callable(source: &Type) -> Option<Type> {
    match source {
        Type::Callable(Some(_)) => Some(source.clone()),
        Type::Union(parts) => {
            let found: Vec<_> = parts.iter().filter_map(declared_callable).collect();
            (!found.is_empty()).then(|| union(found))
        }
        _ => None,
    }
}

/// A template's bound: an unbounded template's is the explicit `mixed` of `@template T`.
fn explicit_mixed(t: Type) -> Type {
    if matches!(t, Type::Mixed { .. }) { Type::Mixed { explicit: true } } else { t }
}

/// Value-level templates, recursively through class strings, shapes, bounds and other inferred containers.
fn inferred_argument(t: &Type) -> Type {
    match t {
        Type::Template(t) => Type::Template(Box::new(Template {
            bound: inferred_argument(&t.bound),
            default: t.default.as_ref().map(inferred_argument),
            argument: t.scope != "anonymous function",
            ..(**t).clone()
        })),
        t => t.traverse(&inferred_argument),
    }
}

fn literal_elements<'e, 'arena>(mut e: &'e Expression<'arena>) -> Option<&'e TokenSeparatedSequence<'arena, ArrayElement<'arena>>> {
    while let Expression::Parenthesized(p) = e { e = p.expression; }
    match e {
        Expression::Array(a) => Some(&a.elements),
        Expression::LegacyArray(a) => Some(&a.elements),
        _ => None,
    }
}

/// Constant scalar `toArrayKey()`: canonical decimal strings are ints; other strings keep their bytes.
fn array_key(t: Type) -> Option<Type> {
    Some(match t {
        Type::ConstInt(_) => t,
        Type::ConstString(s) => match s.parse::<i64>() {
            Ok(i) if i.to_string() == s => Type::ConstInt(i),
            _ => Type::ConstString(s),
        },
        Type::ConstBool(b) => Type::ConstInt(i64::from(b)),
        Type::Null => Type::ConstString(String::new()),
        Type::ConstFloat(f) if f >= i64::MIN as f64 && f < -(i64::MIN as f64) => Type::ConstInt(f.trunc() as i64),
        _ => return None,
    })
}

/// A shape through PHPStan's ConstantArrayTypeBuilder (`array{}` for no items).
fn finish_shape(shape: ConstArray) -> Type {
    let mut builder = pun_phpstan::types::ShapeBuilder::new();
    for (i, (k, v)) in shape.keys.iter().zip(&shape.values).enumerate() {
        builder.set(Some(k.clone()), v.clone(), shape.optional.contains(&i));
    }
    if let Some((k, v)) = shape.unsealed {
        builder.make_unsealed(k, v);
    }
    builder.get()
}
