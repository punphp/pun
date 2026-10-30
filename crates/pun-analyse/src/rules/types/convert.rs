//! Mago's inferred types as PHPStan's types (`pun_phpstan::types::Type`), so rules compare and describe them the way
//! PHPStan's rules do (`Type::describe(VerbosityLevel)`).

use std::collections::HashMap;
use std::hash::BuildHasher;

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
use mago_syntax::cst::{Access, ArrayElement, Expression, TokenSeparatedSequence};
use pun_phpstan::types::{
    Accessory, ConstArray, NeverKind, Param, ShapeBuilder, Signature, Template, Tri, Type, intersect, mixed, union,
};

use super::super::{Checker, arrays, declared, doc, infer};

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

#[derive(Clone, Copy)]
enum ConversionScope {
    Current,
    Selected,
}

impl<'a, 'arena> Checker<'a, 'arena> {
    /// `$scope->getType($expr)`: Mago's type of an expression the rules asked about, as PHPStan's.
    pub(in super::super) fn phpstan_type(&self, e: &mago_syntax::cst::Expression) -> Option<Type> {
        Some(self.phpstan_type_from(e, self.type_of(e)?))
    }

    /// Convert an expression and all its dependencies in the supplied scope, without ordinary-scope recovery.
    pub(in super::super) fn phpstan_type_in(
        &self,
        e: &Expression,
        types: &HashMap<Span, TUnion, impl BuildHasher>,
    ) -> Option<Type> {
        Some(self.phpstan_type_from_in(e, types.get(&e.span())?, types))
    }

    /// Selectors are stored as spans rather than Expression nodes, but have the same scope-owned metadata.
    pub(in super::super) fn phpstan_type_at(&self, at: Span) -> Option<Type> {
        self.phpstan_type_at_in(at, &self.types)
    }

    pub(in super::super) fn phpstan_type_at_in(
        &self,
        at: Span,
        types: &HashMap<Span, TUnion, impl BuildHasher>,
    ) -> Option<Type> {
        let actual = types.get(&at)?;
        Some(declared::with_classes(self, || {
            let source = self.source_type(at, &self.origins, types);
            self.restore_inferred(at, self.to_phpstan(actual), source.as_ref())
        }))
    }

    /// Convert an observed scope value using the expression's own source metadata.
    pub(in super::super) fn phpstan_type_from(&self, e: &mago_syntax::cst::Expression, t: &TUnion) -> Type {
        self.phpstan_type_from_with(e, t, &self.types, ConversionScope::Current)
    }

    /// Restore source metadata using only the selected scope's dependency values, not Checker.types.
    pub(in super::super) fn phpstan_type_from_in(
        &self,
        e: &Expression,
        t: &TUnion,
        types: &HashMap<Span, TUnion, impl BuildHasher>,
    ) -> Type {
        // The native observation map is a separate authority, not a PHPDoc-aware selected scope.
        // Compare map identity, never equal values: an unrelated selected map keeps its own policy.
        if std::ptr::eq(
            std::ptr::from_ref(types).cast::<()>(),
            std::ptr::from_ref(&self.native_types).cast::<()>(),
        ) {
            return self.native_type_from(e.span(), t);
        }
        self.phpstan_type_from_with(e, t, types, ConversionScope::Selected)
    }

    fn native_type_from(&self, at: Span, actual: &TUnion) -> Type {
        let actual = self.to_phpstan(actual);
        self.native_source_order(at)
            .map_or(actual.clone(), |source| restore_metadata(actual, &source, false))
    }

    /// `getNativeType` promotes native observations, not parameter or annotation PHPDoc.
    /// Only literal, copy and offset edges provide an order, with every dependency observed natively.
    fn native_source_order(&self, at: Span) -> Option<Type> {
        self.native_types.get(&at)?;
        match self.origins.get(&at)? {
            infer::Origin::Copy(from) => self.native_source_order(*from),
            infer::Origin::Array(items) => {
                let mut builder = ShapeBuilder::new();
                for (key, value) in items {
                    let key = match key {
                        Some(key) => Some(array_key(self.to_phpstan(self.native_types.get(key)?))?),
                        None => None,
                    };
                    let actual = self.native_types.get(value)?;
                    builder.set(key, self.native_type_from(*value, actual), false);
                }
                Some(builder.get())
            }
            infer::Origin::Offset(array, key) => {
                let Type::ConstArray(source) = self.native_source_order(*array)? else {
                    return None;
                };
                let key = array_key(self.to_phpstan(self.native_types.get(key)?))?;
                source.keys.iter().position(|k| *k == key).map(|i| source.values[i].clone())
            }
            // Declarations, calls and forgotten mutations are not native order evidence.
            _ => None,
        }
    }

    fn phpstan_type_from_with(
        &self,
        e: &Expression,
        t: &TUnion,
        types: &HashMap<Span, TUnion, impl BuildHasher>,
        scope: ConversionScope,
    ) -> Type {
        use mago_syntax::cst::Expression as E;
        declared::with_classes(self, || {
            let source = self.source_type(e.span(), &self.origins, types).or_else(|| {
                match e.unparenthesized() {
                    E::Access(Access::ClassConstant(_)) => {
                        match scope {
                            ConversionScope::Current => arrays::key_type(self, e, self.names, self.class_scope()),
                            ConversionScope::Selected => arrays::key_type_from(
                                self, e, self.names, self.class_scope(), types,
                                &|e, t| self.phpstan_type_from_in(e, t, types),
                            ),
                        }
                    }
                    _ => None,
                }
            });
            // Mago's keyed arrays are BTreeMaps: only the literal still knows PHP's insertion order. Unknown
            // keys/unpacks keep the inferred fallback, rather than inventing an order for a nonliteral shape.
            let ty = self.literal_array(e, types, scope).unwrap_or_else(|| {
                source.as_ref().and_then(|s| self.source_array(t, s))
                    .unwrap_or_else(|| self.to_phpstan(t))
            });
            let ty = source.as_ref().and_then(|s| self.source_template(e.span(), &ty, s))
                .unwrap_or(ty);
            let ty = match scope {
                ConversionScope::Current => self.refine_method_guards(e, ty),
                ConversionScope::Selected => self.refine_method_guards_in(e, ty, types),
            };
            self.restore_inferred(e.span(), ty, source.as_ref())
        })
    }

    fn restore_inferred(&self, at: Span, actual: Type, source: Option<&Type>) -> Type {
        // MutatingScope::enterFunction/enterClassMethod converts parameter/return values to arguments.
        // An inline @var annotation does not: its source restores that distinct template strategy.
        let ty = inferred_argument(&actual);
        source.map_or(ty.clone(), |source| {
            restore_metadata(ty, source, !self.source_is_mutated(at, &self.origins))
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
        let (Type::Object(name, args, false), Type::Template(template)) = (actual, source) else {
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
        types: &HashMap<Span, TUnion, impl BuildHasher>,
    ) -> Option<Type> {
        let source = self.origin_type(origins.get(&span)?, origins, types);
        // Reconciliation may erase benevolence, but the original observed source value still owns it.
        // Recover only through these source edges, never by matching an unrelated equal declaration.
        match types.get(&span).filter(|t| super::super::return_types::is_benevolent(t)) {
            Some(observed) => {
                let actual = self.to_phpstan(observed);
                Some(source.map_or(actual.clone(), |source| restore(actual, &source)))
            }
            None => source,
        }
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
        types: &HashMap<Span, TUnion, impl BuildHasher>,
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
            infer::Origin::Copy(from) => self.source_type(*from, origins, types),
            infer::Origin::Unset(from, offsets) => {
                let source = self.source_type(*from, origins, types)?;
                let keys = offsets.iter().map(|span| {
                    types.get(span).and_then(|t| array_key(self.to_phpstan(t)))
                }).collect::<Vec<_>>();
                Some(unset_order(source, &keys))
            }
            infer::Origin::Array(items) => {
                let mut builder = ShapeBuilder::new();
                for (key, value) in items {
                    let key = match key {
                        Some(key) => Some(array_key(self.to_phpstan(types.get(key)?))?),
                        None => None,
                    };
                    if key == Some(Type::ConstInt(i64::MAX)) { return None; }
                    let actual = self.to_phpstan(types.get(value)?);
                    let value = self.source_type(*value, origins, types)
                        .map_or(actual.clone(), |source| restore(actual, &source));
                    builder.set(key, value, false);
                }
                Some(builder.get())
            }
            infer::Origin::Offset(array, key) => {
                let Type::ConstArray(source) = self.source_type(*array, origins, types)? else {
                    return None;
                };
                let key = array_key(self.to_phpstan(types.get(key)?))?;
                source.keys.iter().position(|k| *k == key).map(|i| source.values[i].clone())
            }
            infer::Origin::AfterMethodCall(call, previous) => {
                let actual = self.to_phpstan(types.get(&call.receiver)?);
                let classes = actual.object_class_names();
                // An annotation or magic fallback is not evidence for the invoked PHP signature. Every
                // native alternative/variant must be known and reference-free, including unpacked calls.
                // With no arguments no local can be passed by reference, even for an unknown receiver.
                let safe = !call.has_arguments || !classes.is_empty()
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
                safe.then(|| self.origin_type(previous, origins, types)).flatten()
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
                    let source = self.source_type(*from, origins, types)?;
                    return Some(if matches!(source, Type::Mixed { .. }) {
                        Type::array(source.clone(), source)
                    } else {
                        source
                    });
                }
                let (f, declaring, receiver_map) =
                    if call.receiver.is_some() || call.class.is_some() {
                        let (mut receiver, class) = if let Some(span) = call.receiver {
                            let actual = self.to_phpstan(types.get(&span)?);
                            let mut receiver = self
                                .source_type(span, origins, types)
                                .map_or(actual.clone(), |source| restore(actual, &source));
                            while let Type::Template(template) = receiver {
                                receiver = template.bound;
                            }
                            let name = match &receiver {
                                Type::Object(name, _, _)
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
                            (Type::Object(declared::original_name(context), args, false), class)
                        };
                        if matches!(receiver, Type::This(_)) {
                            receiver = Type::Object(
                                declared::original_name(class),
                                declared::class_templates(self, class)
                                    .into_iter()
                                    .map(|t| Type::Template(Box::new(t)))
                                    .collect(),
                                false,
                            );
                        }
                        let super::super::members::Method::User { declaring, f, .. } =
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
                            false,
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
                    let actual = types.get(span)?;
                    let actual = self.to_phpstan(actual);
                    let actual = self
                        .source_type(*span, origins, types)
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
    fn literal_array(
        &self,
        e: &Expression,
        types: &HashMap<Span, TUnion, impl BuildHasher>,
        scope: ConversionScope,
    ) -> Option<Type> {
        let elements = literal_elements(e)?;
        if elements.len() > 256 { return None; }
        let convert = |e: &Expression| {
            types.get(&e.span()).map(|t| self.phpstan_type_from_with(e, t, types, scope))
        };
        let mut builder = ShapeBuilder::new();
        for element in elements.iter() {
            let value = convert(element.get_value()?)?;
            match element {
                ArrayElement::KeyValue(kv) => {
                    let key = array_key(convert(kv.key)?)?;
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
        if let Some(kind) = t.never_kind() {
            return Type::Never(match kind {
                mago_codex::ttype::union::NeverKind::Implicit => NeverKind::Implicit,
                mago_codex::ttype::union::NeverKind::Explicit => NeverKind::Explicit,
                mago_codex::ttype::union::NeverKind::NonAccepting => NeverKind::NonAccepting,
            });
        }
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
        let mut signature = Signature {
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
        };
        if let Some(id) = s.get_source()
            && let Some(f) = self.codebase.get_function_like(&id)
        {
            let class = match id {
                mago_codex::identifier::function_like::FunctionLikeIdentifier::Method(class, _) => self.codebase.get_class_like(class.as_bytes()),
                _ => None,
            };
            let scope = declared::value_scope(self, f, class);
            signature.templates = f.template_types.keys().filter_map(|name| match scope.templates.get(&text(name.as_bytes())) {
                Some(Type::Template(t)) => Some((**t).clone()),
                _ => None,
            }).collect();
            // Callable expansion erases declaration metadata too. Restore only matching source identities and
            // explicitness, so an instantiated receiver or a narrower inferred value is never widened.
            for (param, source) in signature.params.iter_mut().zip(declared::codex_params(self, f, class)) {
                param.ty = restore(param.ty.clone(), &source);
            }
            signature.ret = restore(signature.ret, &declared::codex_return_type(self, f, class, &text(f.original_name.as_bytes())));
        }
        Box::new(signature)
    }

    /// `InitializerExprTypeResolver::createFirstClassCallable()`: every positional source variant is a closure,
    /// not the single acceptor selected for an invocation with no arguments.
    fn builtin_closures(&self, s: &mago_codex::ttype::atomic::callable::TCallableSignature) -> Option<Type> {
        use mago_codex::identifier::function_like::FunctionLikeIdentifier;
        use pun_phpstan::{resolve::resolve_str, symbols::ByRef};

        // get_source deliberately excludes bound partial applications: their declaration no longer describes
        // the remaining parameters. Methods and user declarations retain the existing inferred conversion.
        let FunctionLikeIdentifier::Function(name) = s.get_source()? else { return None; };
        let source = self.codebase.get_function(name.as_bytes())?;
        if !source.flags.is_built_in() { return None; }
        let function = self.builtins.functions.get(&text(name.as_bytes()).to_ascii_lowercase())?;
        let signature = self.signature(s);
        Some(union(function.variants.iter().map(|variant| {
            let mut signature = signature.clone();
            signature.params = variant.params.iter().map(|p| Param {
                name: p.name.clone(),
                ty: resolve_str(&p.ty, None),
                optional: p.optional,
                by_ref: p.by_ref != ByRef::No,
                variadic: p.variadic,
            }).collect();
            // This is the effective channel. The independently exported native/PHPDoc returns are not its value.
            signature.ret = resolve_str(&variant.ret, None);
            signature.templates.clear();
            Type::Closure(signature)
        }).collect()))
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
            TAtomic::Mixed(m) => m.effective_subtracted_type().map_or_else(
                || Type::Mixed { explicit: m.is_explicit() },
                |excluded| Type::mixed_subtracted(m.is_explicit(), self.to_phpstan(&excluded)),
            ),
            TAtomic::Resource(_) => Type::Resource,
            TAtomic::Object(o) => self.object(o),
            TAtomic::Array(a) => match a.callable() {
                Some(callable) => intersect(vec![self.array(a), self.atom(callable)]),
                None => self.array(a),
            },
            TAtomic::Iterable(i) => {
                let it = Type::Iterable(
                    Box::new(self.to_phpstan(&i.key_type)),
                    Box::new(self.to_phpstan(&i.value_type)),
                );
                self.with_intersections(it, i.intersection_types.as_deref())
            }
            TAtomic::Callable(TCallable::Signature(s)) => {
                if s.is_closure {
                    self.builtin_closures(s).unwrap_or_else(|| Type::Closure(self.signature(s)))
                } else if unspecified_callable(&self.signature(s)) {
                    Type::Callable(None)
                } else {
                    Type::Callable(Some(self.signature(s)))
                }
            }
            // Mago's alias of a function-like (a closure assigned to a variable, `Closure::getCurrent()`): a ClosureType
            // of its signature
            TAtomic::Callable(TCallable::Alias(id)) => {
                mago_codex::ttype::expander::get_signature_of_function_like_identifier(id, self.codebase)
                    .map_or(Type::Callable(None), |s| self.builtin_closures(&s)
                        .unwrap_or_else(|| Type::Closure(self.signature(&s))))
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
                false,
            ),
            TAtomic::GenericParameter(g) => {
                self.generic_parameter(g, explicit_mixed(self.to_phpstan(&g.constraint)))
            }
            _ => mixed(),
        }
    }

    fn generic_parameter(&self, g: &TGenericParameter, bound: Type) -> Type {
        let scope = self.template_scope(&g.defining_entity);
        let name = text(g.parameter_name.as_bytes());
        // Mago drops defaults from inferred generics. Resolve the exact defining entity, not a same-named
        // template in the current caller's scope; this also works inside unions and nested containers.
        let source = match &g.defining_entity {
            GenericParent::FunctionLike(key) => self.codebase.function_likes.get(key)
                .map(|f| declared::value_scope(self, f, self.codebase.get_class_like(key.0.as_bytes()))),
            GenericParent::ClassLike(n) => self.codebase.get_class_like(n.as_bytes())
                .map(|k| declared::class_scope(self, k.span, k)),
        };
        let default = source.and_then(|s| s.templates.into_values().find_map(|t| match t {
            Type::Template(t) if t.name == name && t.scope == scope => t.default,
            _ => None,
        }));
        let t = Type::Template(Box::new(Template {
            name,
            scope,
            bound,
            default,
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
                    return Type::ConstString(v.as_bytes().to_vec());
                }
                let mut accessories = Vec::new();
                if let Some(decimal) = s.decimal_integer {
                    accessories.push(Accessory::DecimalIntString { inverse: !decimal });
                }
                if s.is_numeric && s.decimal_integer != Some(true) {
                    accessories.push(Accessory::NumericString);
                }
                if s.is_truthy {
                    accessories.push(Accessory::NonFalsyString);
                } else if s.has_non_empty_accessory() {
                    accessories.push(Accessory::NonEmptyString);
                }
                if matches!(s.literal, Some(TStringLiteral::Unspecified)) {
                    accessories.push(Accessory::LiteralString);
                }
                match s.casing {
                    TStringCasing::Lowercase => accessories.push(Accessory::LowercaseString),
                    TStringCasing::Uppercase => accessories.push(Accessory::UppercaseString),
                    TStringCasing::Both => {
                        accessories.push(Accessory::LowercaseString);
                        accessories.push(Accessory::UppercaseString);
                    }
                    TStringCasing::Unspecified => {}
                }
                if s.is_callable {
                    let mut members = vec![Type::String, Type::Callable(None)];
                    members.extend(accessories.into_iter().filter_map(|accessory| {
                        matches!(accessory, Accessory::LowercaseString | Accessory::UppercaseString | Accessory::DecimalIntString { .. })
                            .then_some(Type::Accessory(accessory))
                    }));
                    return Type::Intersection(members);
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
                TClassLikeString::LiteralAny => intersect(vec![Type::ClassString(None), Type::Accessory(Accessory::LiteralString)]),
                TClassLikeString::Literal { value } => Type::ConstString(self.reflected_name(value.as_bytes()).into_bytes()),
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
            TObject::NonexistentParent => Type::NonexistentParent,
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
                    .map(|p| {
                        let argument = self.to_phpstan(p);
                        // Constructor arguments have already undergone template inference in Mago.
                        if n.is_exact {
                            pun_phpstan::types::remove_final_by_keyword_overrides(&argument)
                        } else {
                            argument
                        }
                    })
                    .collect();
                let t = if n.is_this {
                    Type::This(name)
                } else if n.is_static {
                    Type::Static(name, args)
                } else {
                    // ClassReflection::asFinal() does not override abstract/non-class/native-final types.
                    // NewHandler's own generic constructor resolves the class templates by traversing a
                    // GenericObjectType; recreate() drops its outer reflection override. A dummy or
                    // inherited constructor instead returns a freshly exact GenericObjectType.
                    let exact = n.is_exact && self.codebase.get_class_like(n.name.as_bytes()).is_some_and(|class| {
                        class.kind.is_class() && !class.flags.is_abstract() && !class.flags.is_final()
                            && (args.is_empty() || self.codebase.get_method(n.name.as_bytes(), b"__construct").is_none())
                    });
                    Type::Object(name, args, exact)
                };
                let t = n.subtracted_type.as_ref().map_or_else(
                    || t.clone(),
                    |excluded| Type::SubtractedObject {
                        object: Box::new(t.clone()),
                        excluded: Box::new(self.to_phpstan(excluded)),
                    },
                );
                self.with_intersections(t, n.intersection_types.as_deref().map(Vec::as_slice))
            }
            TObject::Enum(e) => match &e.case {
                Some(case) => Type::EnumCase(self.reflected_name(e.name.as_bytes()), text(case.as_bytes())),
                None => Type::Object(self.reflected_name(e.name.as_bytes()), vec![], false),
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
            ArrayKey::String(s) => Type::ConstString(s.as_bytes().to_vec()),
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
                let mut parts = vec![Type::list(item)];
                if l.non_empty || l.known_elements.as_ref().is_some_and(|k| k.values().any(|(o, _)| !o)) {
                    parts.push(Type::Accessory(Accessory::NonEmptyArray));
                }
                if l.callable.is_some() {
                    parts.extend(
                        l.known_elements
                            .iter()
                            .flat_map(|elements| elements.iter())
                            .filter_map(|(key, (optional, value))| {
                                (!optional).then(|| {
                                    Type::Accessory(Accessory::HasOffsetValue(
                                        Box::new(Type::ConstInt(*key as i64)),
                                        Box::new(self.to_phpstan(value)),
                                    ))
                                })
                            }),
                    );
                }
                intersect(parts)
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
                || ((matches!(actual, Type::Callable(None))
                    || matches!(actual, Type::Callable(Some(a)) if unspecified_callable(a)))
                    && matches!(s, Type::Callable(Some(_))))
        })
    };
    match (actual, source) {
        (Type::Template(mut actual), Type::Template(source))
            if actual.name == source.name && actual.scope == source.scope =>
        {
            actual.argument = source.argument;
            actual.default = source.default.clone();
            // A matching template identity proves which bound was erased. A concrete narrowed bound still
            // wins; only Mago's unconstrained fallback can regain the source's callable-array intersection.
            actual.bound = if matches!(actual.bound, Type::Mixed { .. }) {
                source.bound.clone()
            } else {
                restore_metadata(actual.bound, &source.bound, restore_list)
            };
            Type::Template(actual)
        }
        (actual, Type::Template(source)) => restore_metadata(
            pun_phpstan::types::remove_final_by_keyword_overrides(&actual), &source.bound, restore_list,
        ),
        (Type::Union(actual) | Type::Benevolent(actual), Type::Benevolent(source)) => {
            pun_phpstan::types::to_benevolent(restore_metadata(
                Type::Union(actual), &Type::Union(source.clone()), restore_list,
            ))
        }
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
        (Type::Mixed { explicit: inferred }, Type::Mixed { explicit }) => Type::Mixed {
            // Recover erased declaration metadata without erasing an assertion's source-owned identity.
            explicit: inferred || *explicit,
        },
        (Type::SubtractedMixed { explicit: inferred, excluded }, Type::Mixed { explicit }) => Type::SubtractedMixed {
            explicit: inferred || *explicit,
            excluded,
        },
        // Mago's PHPDoc parser reads an unnamed trailing `&` as an intersection and rejects the tag. The
        // surviving native callable is exactly TCallableSignature::mixed(false), not an inferred signature.
        (Type::Callable(None), source @ Type::Callable(Some(_))) => source.clone(),
        (Type::Callable(Some(actual)), source) if unspecified_callable(&actual) => {
            if matches!(source, Type::Callable(Some(_))) {
                source.clone()
            } else {
                Type::Callable(Some(actual))
            }
        }
        (Type::Object(name, actual, _), Type::Object(source_name, source, exact))
            if name.eq_ignore_ascii_case(source_name) && actual.len() == source.len() =>
        {
            Type::Object(
                name,
                actual
                    .into_iter()
                    .zip(source)
                    .map(|(a, s)| restore(a, s))
                    .collect(),
                *exact,
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
        (Type::Mixed { .. }, source @ (Type::Intersection(_) | Type::ConstArray(_)))
            if source.is_array().yes() && source.is_callable().yes() =>
        {
            // Mago can reject a callable-array PHPDoc intersection outright. Its source-owned parameter
            // or annotation still supplies that erased bound, even if PHPStan normalizes it to a shape;
            // a concrete inferred value always wins.
            source.clone()
        }
        (actual @ Type::ConstArray(_), source @ Type::Intersection(parts))
            if source.is_array().yes() && source.is_callable().yes() =>
        {
            // IntersectionType retains the callable conjunct for a constant array too. Restore the
            // surviving shape's source order first; never replace narrowed values or removed keys.
            let actual = parts.iter().fold(actual, |actual, source| restore_metadata(actual, source, restore_list));
            intersect(std::iter::once(actual)
                .chain(parts.iter().filter(|part| matches!(part, Type::Callable(_))).cloned())
                .collect())
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

/// A template's bound: an unbounded template's is the explicit `mixed` of `@template T`.
fn explicit_mixed(t: Type) -> Type {
    if matches!(t, Type::Mixed { .. }) { Type::Mixed { explicit: true } } else { t }
}

/// Value-level templates, recursively through class strings, shapes, bounds and other inferred containers.
fn inferred_argument(t: &Type) -> Type {
    inferred_argument_in(t, &[])
}

/// TemplateTypeHelper::toArgument(): templates owned by a callable remain parameters of that callable.
fn inferred_argument_in(t: &Type, owned: &[Template]) -> Type {
    match t {
        Type::Template(t) => Type::Template(Box::new(Template {
            bound: inferred_argument_in(&t.bound, owned),
            default: t.default.as_ref().map(|t| inferred_argument_in(t, owned)),
            argument: t.scope != "anonymous function" && !owned.iter().any(|o| o.name == t.name && o.scope == t.scope),
            ..(**t).clone()
        })),
        Type::Callable(Some(s)) | Type::Closure(s) => {
            let owned: Vec<_> = owned.iter().chain(&s.templates).cloned().collect();
            t.traverse(&|t| inferred_argument_in(t, &owned))
        }
        t => t.traverse(&|t| inferred_argument_in(t, owned)),
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
        Type::ConstString(s) => match std::str::from_utf8(&s).ok().and_then(|s| s.parse::<i64>().ok()) {
            Some(i) if i.to_string().as_bytes() == s => Type::ConstInt(i),
            _ => Type::ConstString(s),
        },
        Type::ConstBool(b) => Type::ConstInt(i64::from(b)),
        Type::Null => Type::ConstString(Vec::new()),
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

#[cfg(test)]
mod never_identity_tests {
    use std::collections::HashMap;
    use std::path::Path;

    use mago_codex::ttype::{get_explicit_never, get_never, get_non_accepting_never};
    use mago_word::{empty_word, word};
    use serde_json::Value;

    #[test]
    fn source_owned_bottom_metadata_round_trips_through_public_dump_types() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/never-identity");
        let file = root.join("conversion.php");
        let oracle: Value = serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap()).unwrap();
        let project = crate::Project::new(vec![file.clone()], Vec::new());
        let (mut codebase, sources) = crate::codebase::build_with_sources(&project).unwrap();
        let mut expected = Vec::new();
        // Model the producer/consumer boundary directly: accepting explicit never has no declaration
        // syntax. Its expression producers and call-evaluation facts are independently maintained.
        for (name, kind, ty) in [
            ("implicitValue", "implicit", get_never()),
            ("explicitValue", "explicit", get_explicit_never()),
            ("nonacceptingValue", "nonaccepting", get_non_accepting_never()),
        ] {
            let metadata = codebase.function_likes.get_mut(&(empty_word(), word(name.to_ascii_lowercase()))).unwrap();
            metadata.return_type_metadata.as_mut().unwrap().type_union = ty;
            expected.push(format!("Dumped type: {}", oracle["types"][kind]["description"].as_str().unwrap()));
        }
        let checks = crate::rules::Checks::of(&project, &codebase);
        let errors = crate::rules::check_file(&file, &codebase, &checks, &HashMap::new(), &sources);
        let actual: Vec<_> = errors.into_iter().map(|error| {
            assert_eq!(error.identifier.as_deref(), Some("phpstan.dumpType"));
            error.message
        }).collect();
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod callable_metadata_tests {
    use super::*;

    #[test]
    fn erased_callable_metadata_retains_signatures_not_replaced_values() {
        let concrete = Type::Callable(Some(Box::new(Signature {
            params: vec![Param { name: String::new(), ty: Type::Int, optional: false, by_ref: false, variadic: false }],
            ret: Type::String,
            templates: vec![],
        })));
        let source = Type::Union(vec![concrete.clone(), Type::Null]);
        let erased = Type::Callable(Some(Box::new(Signature {
            params: vec![Param { name: String::new(), ty: mixed(), optional: true, by_ref: false, variadic: true }],
            ret: mixed(),
            templates: vec![],
        })));
        assert_eq!(restore(erased, &source), concrete);
        assert_eq!(restore(Type::Callable(None), &source), concrete);
        assert_eq!(restore(Type::Int, &source), Type::Int);
        assert_eq!(restore(Type::ConstBool(false), &source), Type::ConstBool(false));
    }
}
