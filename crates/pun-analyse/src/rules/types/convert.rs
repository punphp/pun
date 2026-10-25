//! Mago's inferred types as PHPStan's types (`pun_phpstan::types::Type`), so rules compare and describe them the way
//! PHPStan's rules do (`Type::describe(VerbosityLevel)`).

use mago_codex::misc::GenericParent;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::callable::TCallable;
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
        use mago_syntax::cst::Expression as E;
        let t = self.type_of(e)?;
        // Mago types `new` of an unknown class `never` (NonExistentClass), PHPStan as an object of that class
        if let E::Instantiation(new) = e
            && let E::Identifier(id) = new.class
            && t.is_never()
            && self.codebase.get_class_like(self.names.get(id)).is_none()
        {
            return Some(Type::Object(text(self.names.get(id)), vec![]));
        }
        // Mago's keyed arrays are BTreeMaps: only the literal still knows PHP's insertion order. Unknown
        // keys/unpacks keep the inferred fallback, rather than inventing an order for a nonliteral shape.
        let ty = self.literal_array(e).unwrap_or_else(|| self.to_phpstan(t));
        // MutatingScope::enterFunction/enterClassMethod converts PHPDoc parameter/return values to arguments.
        // An inline @var annotation does not: its source metadata can restore that distinct template strategy.
        let ty = inferred_argument(&ty);
        let ty = self
            .source_type(e.span(), &self.origins)
            .map_or(ty.clone(), |source| restore(ty, &source));
        Some(ty)
    }

    /// Only recover metadata from the expression's source edges. In particular, a structurally equal but
    /// unrelated declaration is never evidence for an array's order or an implicit mixed's explicitness.
    fn source_type(
        &self,
        span: Span,
        origins: &std::collections::HashMap<Span, infer::Origin>,
    ) -> Option<Type> {
        let function = |key: &(mago_word::Word, mago_word::Word)| {
            Some((
                self.codebase.function_likes.get(key)?.as_ref(),
                self.codebase.get_class_like(key.0.as_bytes()),
            ))
        };
        match origins.get(&span)? {
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
            infer::Origin::Call(call) => {
                let (name, args) = (&call.name, &call.args);
                // These extensions preserve their input key/value types. The provider alone decides keys,
                // cardinality and listness; this edge restores only information its Mago types cannot store.
                if matches!(name.as_str(), "array_slice" | "array_unique") {
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
                declared::codex_return(self, self.codebase.get_function(name.as_bytes())?, None)
                    .map(|t| inferred_argument(&t))
            }
            infer::Origin::Callback(call, argument, parameter) => {
                let (name, args) = (&call.name, &call.args);
                let f = self.codebase.get_function(name.as_bytes())?;
                let params = declared::codex_params(self, f, None);
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
        let types = union(t.types.iter().map(|a| self.atom(a)).collect());
        if super::super::return_types::is_benevolent(t) {
            return pun_phpstan::types::to_benevolent(types);
        }
        types
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
                let t = Type::Template(Box::new(Template {
                    name: text(g.parameter_name.as_bytes()),
                    scope: self.template_scope(&g.defining_entity),
                    bound: explicit_mixed(self.to_phpstan(&g.constraint)),
                    default: None,
                    argument: false,
                }));
                self.with_intersections(t, g.intersection_types.as_deref())
            }
            _ => mixed(),
        }
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
                if l.element_type.is_never() {
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
                    None => {
                        let mut shape = ConstArray::empty();
                        for (key, (optional, v)) in k.known_items.iter().flat_map(|k| k.iter()) {
                            if *optional {
                                shape.optional.push(shape.keys.len());
                            }
                            shape.keys.push(self.key(key));
                            shape.values.push(self.to_phpstan(v));
                        }
                        finish_shape(shape)
                    }
                    Some((key, value)) => {
                        let mut keys = vec![self.to_phpstan(key)];
                        let mut values = vec![self.to_phpstan(value)];
                        for (k, (_, v)) in k.known_items.iter().flat_map(|k| k.iter()) {
                            keys.push(self.key(k));
                            values.push(self.to_phpstan(v));
                        }
                        let array = Type::array(union(keys), union(values));
                        if k.non_empty
                            || k.known_items.as_ref().is_some_and(|i| i.values().any(|(o, _)| !o))
                        {
                            intersect(vec![array, Type::Accessory(Accessory::NonEmptyArray)])
                        } else {
                            array
                        }
                    }
                }
            }
        }
    }
}

/// Overlay only the two erased facts, never a declaration's broader value type over a narrowed inferred type.
fn restore(actual: Type, source: &Type) -> Type {
    match (actual, source) {
        (Type::Template(mut actual), Type::Template(source))
            if actual.name == source.name && actual.scope == source.scope =>
        {
            actual.argument = source.argument;
            Type::Template(actual)
        }
        (Type::Mixed { .. }, Type::Mixed { explicit }) => Type::Mixed {
            explicit: *explicit,
        },
        (Type::ConstArray(mut actual), Type::ConstArray(source))
            if actual.keys.len() == source.keys.len()
                && source.keys.iter().all(|k| actual.keys.contains(k)) =>
        {
            let positions: Vec<_> = source
                .keys
                .iter()
                .map(|k| actual.keys.iter().position(|a| a == k).unwrap())
                .collect();
            actual.values = positions
                .iter()
                .enumerate()
                .map(|(i, p)| restore(actual.values[*p].clone(), &source.values[i]))
                .collect();
            actual.optional = positions
                .iter()
                .enumerate()
                .filter_map(|(i, p)| actual.optional.contains(p).then_some(i))
                .collect();
            actual.keys = source.keys.clone();
            actual.is_list = source.is_list;
            Type::ConstArray(actual)
        }
        (Type::Array(k, v), Type::Array(sk, sv) | Type::Iterable(sk, sv)) => {
            Type::array(restore(*k, sk), restore(*v, sv))
        }
        (Type::Iterable(k, v), Type::Array(sk, sv) | Type::Iterable(sk, sv)) => {
            Type::Iterable(Box::new(restore(*k, sk)), Box::new(restore(*v, sv)))
        }
        (Type::Intersection(parts), source) => {
            intersect(parts.into_iter().map(|t| restore(t, source)).collect())
        }
        (actual, Type::Intersection(parts)) => parts.iter().fold(actual, restore),
        (actual, _) => actual,
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

/// A sealed shape through PHPStan's ConstantArrayTypeBuilder (`array{}` for no items).
fn finish_shape(shape: ConstArray) -> Type {
    let mut builder = pun_phpstan::types::ShapeBuilder::new();
    for (i, (k, v)) in shape.keys.iter().zip(&shape.values).enumerate() {
        builder.set(Some(k.clone()), v.clone(), shape.optional.contains(&i));
    }
    builder.get()
}
