//! `Type::inferTemplateTypes()` and `TemplateTypeHelper::resolveTemplateTypes()`: what a generic function's
//! templates are at a call, from the arguments its parameters receive (GenericParametersAcceptorResolver).

use super::*;

/// A template's key: its name and scope (PHPStan keys a call's map by name; Pun's types also carry the class
/// templates a call has not resolved, so the scope tells them apart).
type Key = (String, String);

/// `TemplateTypeMap`: the inferred types and the lower bounds a callable's parameters give.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TemplateMap {
    types: Vec<(Key, Type)>,
    lower: Vec<(Key, Type)>,
    /// the templates whose `*ERROR*` is an `AbsorbedTemplateArgumentType`: any type inferred elsewhere wins over it
    absorbed: Vec<Key>,
}

fn key(t: &Template) -> Key {
    (t.name.clone(), t.scope.clone())
}

fn merge(into: &mut Vec<(Key, Type)>, from: Vec<(Key, Type)>, combine: impl Fn(Type, Type) -> Option<Type>) {
    for (k, t) in from {
        match into.iter_mut().find(|(o, _)| *o == k) {
            Some((_, old)) => {
                if let Some(new) = combine(old.clone(), t) {
                    *old = new;
                }
            }
            None => into.push((k, t)),
        }
    }
}

/// a lower bound's intersection, kept unless it is `never`
fn narrowed(a: Type, b: Type) -> Option<Type> {
    Some(intersect(vec![a, b])).filter(|t| !t.is_never())
}

impl TemplateMap {
    fn of(t: &Template, ty: Type) -> TemplateMap {
        TemplateMap { types: vec![(key(t), ty)], ..Default::default() }
    }

    /// `union()` and `benevolentUnion()`, an absorbed template giving way to the other side (`combine()`)
    fn combined(mut self, o: TemplateMap, f: fn(Vec<Type>) -> Type) -> TemplateMap {
        for (k, t) in o.types {
            let theirs = o.absorbed.contains(&k);
            match self.types.iter_mut().find(|(m, _)| *m == k) {
                Some((_, old)) if self.absorbed.contains(&k) => {
                    *old = t;
                    self.absorbed.retain(|a| *a != k || theirs);
                }
                Some(_) if theirs => {}
                Some((_, old)) => *old = f(vec![old.clone(), t]),
                None => {
                    if theirs {
                        self.absorbed.push(k.clone());
                    }
                    self.types.push((k, t));
                }
            }
        }
        merge(&mut self.lower, o.lower, narrowed);
        self
    }

    pub fn union(self, o: TemplateMap) -> TemplateMap {
        self.combined(o, union)
    }

    fn benevolent_union(self, o: TemplateMap) -> TemplateMap {
        self.combined(o, |ts| to_benevolent(union(ts)))
    }

    fn intersect(mut self, o: TemplateMap) -> TemplateMap {
        merge(&mut self.types, o.types, |a, b| Some(intersect(vec![a, b])));
        merge(&mut self.lower, o.lower, |a, b| Some(union(vec![a, b])));
        self
    }

    /// `convertToLowerBoundTypes()`
    fn lower_bound(self) -> TemplateMap {
        let mut lower = self.types;
        merge(&mut lower, self.lower, narrowed);
        TemplateMap { lower, ..Default::default() }
    }

    /// `getTypes()`: each template's type, its lower bound when only that is known.
    pub fn types(&self) -> Vec<(&str, &str, &Type)> {
        let lower = self.lower.iter().filter(|(k, _)| !self.types.iter().any(|(o, _)| o == k));
        self.types.iter().chain(lower).map(|((n, s), t)| (n.as_str(), s.as_str(), t)).collect()
    }

    fn get(&self, t: &Template) -> Option<&Type> {
        self.types().into_iter().find(|(n, s, _)| *n == t.name && *s == t.scope).map(|(_, _, t)| t)
    }
}

/// A template of a union, benevolent union or intersection is one (TemplateUnionType extends UnionType).
fn bounded(t: &Type) -> &Type {
    match t {
        Type::Template(tt) if matches!(tt.bound, Type::Union(_) | Type::Benevolent(_) | Type::Intersection(_)) => &tt.bound,
        t => t,
    }
}

/// `$receivedType->inferTemplateTypesOn($pattern)` of a union, benevolent union or intersection received.
fn on_received(pattern: &Type, received: &Type) -> Option<TemplateMap> {
    let fold = |ts: &[Type], f: fn(TemplateMap, TemplateMap) -> TemplateMap| {
        ts.iter().map(|t| infer_template_types(pattern, t)).fold(TemplateMap::default(), f)
    };
    match bounded(received) {
        Type::Union(ts) => Some(fold(ts, TemplateMap::union)),
        Type::Benevolent(ts) => Some(fold(ts, TemplateMap::benevolent_union)),
        Type::Intersection(ts) => Some(fold(ts, TemplateMap::intersect)),
        _ => None,
    }
}

/// `$pattern->inferTemplateTypes($received)`
pub fn infer_template_types(pattern: &Type, received: &Type) -> TemplateMap {
    let fold = |ts: &[Type], f: fn(TemplateMap, TemplateMap) -> TemplateMap| {
        ts.iter().map(|t| infer_template_types(t, received)).fold(TemplateMap::default(), f)
    };
    match pattern {
        Type::Template(t) => template(t, received),
        Type::Union(ts) => in_union(ts, received),
        Type::Benevolent(ts) => fold(ts, TemplateMap::benevolent_union),
        Type::Intersection(ts) => fold(ts, TemplateMap::intersect),
        Type::Projection(_, inner) => infer_template_types(inner, received),
        Type::Array(..)
        | Type::ConstArray(_)
        | Type::Iterable(..)
        | Type::Callable(Some(_))
        | Type::Closure(_)
        | Type::ClassString(Some(_))
        | Type::ObjectShape(_) => on_received(pattern, received).unwrap_or_else(|| generic(pattern, received)),
        Type::Object(_, args) if !args.is_empty() => {
            on_received(pattern, received).unwrap_or_else(|| generic(pattern, received))
        }
        // ponytail: a conditional type (a late-resolvable one, inferred as what it resolves to) infers nothing; its
        // parameter is not checked (`is_late()`)
        _ => TemplateMap::default(),
    }
}

/// TemplateTypeTrait::inferTemplateTypes()
fn template(t: &Template, received: &Type) -> TemplateMap {
    if let Type::Template(r) = received
        && is_super_type_of(&t.bound, &r.bound).yes()
    {
        return TemplateMap::of(t, received.clone());
    }
    let map = infer_template_types(&t.bound, received);
    let bound = resolve_template_types(&t.bound, &map);
    match is_super_type_of(&bound, received) {
        Tri::Yes => TemplateMap::of(t, received.clone()).union(map),
        Tri::Maybe if let Type::Union(rs) = received => {
            let matching: Vec<Type> = rs.iter().filter(|r| is_super_type_of(&bound, r).yes()).cloned().collect();
            if matching.is_empty() {
                return map;
            }
            TemplateMap::of(t, union(matching)).union(map)
        }
        _ => map,
    }
}

/// UnionType::inferTemplateTypes(): a received type a member definitely accepts is absorbed by it, telling
/// nothing about the others (`T|null` receiving `null` leaves `T` absorbed); only the rest is inferred against the
/// members that are not naked templates, or against all of them when that infers nothing.
fn in_union(ts: &[Type], received: &Type) -> TemplateMap {
    let mut received = match received {
        // `toArrayOrTraversable()`
        Type::Iterable(k, v) => union(vec![
            Type::array((**k).clone(), (**v).clone()),
            Type::Object("Traversable".into(), vec![(**k).clone(), (**v).clone()]),
        ]),
        r => r.clone(),
    };
    let rs = match bounded(&received) {
        Type::Union(rs) | Type::Benevolent(rs) => rs.clone(),
        r => vec![r.clone()],
    };
    let mut map = TemplateMap::default();
    let mut remaining = Vec::new();
    'received: for r in &rs {
        if !r.is_never() {
            for t in ts {
                if is_super_type_of(t, r).yes() {
                    map = map.union(infer_template_types(t, r));
                    continue 'received;
                }
            }
        }
        remaining.push(r.clone());
    }
    if remaining.is_empty() {
        // `getAbsorbedTemplateTypes()`
        for t in ts.iter().filter_map(naked) {
            if map.get(t).is_none() {
                map.types.push((key(t), Type::Error));
                map.absorbed.push(key(t));
            }
        }
        return map;
    }
    if remaining.len() != rs.len() {
        received = union(remaining);
    }
    for t in ts.iter().filter(|t| naked(t).is_none()) {
        map = map.union(infer_template_types(t, &received));
    }
    if !map.types().is_empty() {
        return map;
    }
    ts.iter().fold(map, |map, t| map.union(infer_template_types(t, &received)))
}

/// `getNakedTemplateType()`
fn naked(t: &Type) -> Option<&Template> {
    match t {
        Type::Template(t) => Some(t),
        Type::ClassString(Some(c)) if let Type::Template(t) = &**c => Some(t),
        _ => None,
    }
}

/// The inference of the generic types other than templates and unions, from a received type that is not a union.
fn generic(pattern: &Type, received: &Type) -> TemplateMap {
    let pair = |k: &Type, v: &Type| {
        infer_template_types(k, &received.iterable_key_type()).union(infer_template_types(v, &received.iterable_value_type()))
    };
    match pattern {
        Type::Array(k, v) if received.is_array().yes() => pair(k, v),
        Type::ConstArray(a) => match received {
            // ponytail: a key the received shape lacks is skipped, its unsealed extras are not matched
            Type::ConstArray(b) => a.keys.iter().zip(&a.values).fold(TemplateMap::default(), |map, (k, v)| {
                match b.keys.iter().position(|o| o == k) {
                    Some(i) => map.union(infer_template_types(v, &b.values[i])),
                    None => map,
                }
            }),
            r if r.is_array().yes() => pair(&pattern.iterable_key_type(), &pattern.iterable_value_type()),
            _ => TemplateMap::default(),
        },
        Type::Iterable(k, v) if received.is_iterable().yes() && !received.is_never() => pair(k, v),
        Type::Callable(Some(s)) if received.is_callable().yes() => {
            callable_signatures(received).iter().fold(TemplateMap::default(), |map, theirs| map.union(on_signature(s, theirs)))
        }
        Type::Closure(s) if let Type::Closure(theirs) = received => on_signature(s, theirs),
        Type::ClassString(Some(inner)) => {
            let object = match received {
                Type::ConstString(class) => Type::Object(class.clone(), vec![]),
                Type::ClassString(Some(t)) => (**t).clone(),
                r if r.is_class_string().yes() => {
                    let inner = match &**inner {
                        Type::Template(t) => t.bound.clone(),
                        t => t.clone(),
                    };
                    intersect(vec![inner, Type::ObjectWithoutClass])
                }
                _ => return TemplateMap::default(),
            };
            infer_template_types(inner, &object)
        }
        Type::Object(class, args) => {
            // ponytail: `$this` and enum cases (TypeWithClassName too) infer nothing
            let other = match received {
                Type::Object(name, other) | Type::Static(name, other) => ancestor(name, other, class),
                _ => None,
            };
            let Some(other) = other else {
                return TemplateMap::default();
            };
            args.iter().enumerate().fold(TemplateMap::default(), |map, (i, a)| {
                let o = other.get(i).cloned().unwrap_or(Type::Error);
                map.union(infer_template_types(a, &o))
            })
        }
        Type::ObjectShape(props) if let Type::ObjectShape(theirs) = received => {
            props.iter().fold(TemplateMap::default(), |map, (name, t, _)| match theirs.iter().find(|(n, ..)| n == name) {
                Some((_, o, _)) => map.union(infer_template_types(t, o)),
                None => map,
            })
        }
        _ => TemplateMap::default(),
    }
}

/// The arguments of `class` as an ancestor of `name<args>` (a class without arguments over its templates' bounds).
pub(super) fn ancestor(name: &str, args: &[Type], class: &str) -> Option<Vec<Type>> {
    let c = symbols::lookup(name)?;
    if c.name.eq_ignore_ascii_case(class) {
        let own = c.templates.iter().map(|t| {
            Type::Template(Box::new(Template {
                name: t.name.clone(),
                scope: format!("class {}", c.name),
                bound: crate::resolve::resolve_str(&t.bound, Some(&c.name)),
                default: None,
                argument: false,
            }))
        });
        return Some(own.map(|t| with_class_args(&t, &c, args)).collect());
    }
    ancestor_args(&c, class, args)
}

/// CallableType::inferTemplateTypesOnParametersAcceptor(): the parameters give lower bounds.
fn on_signature(ours: &Signature, theirs: &Signature) -> TemplateMap {
    let params = ours.params.iter().enumerate().fold(TemplateMap::default(), |map, (i, p)| {
        let arg = match (theirs.params.get(i), &p.ty) {
            (Some(o), _) => o.ty.clone(),
            (None, t @ Type::Template(_)) => t.resolve_to_bounds(),
            (None, _) => Type::Never(NeverKind::Implicit),
        };
        map.union(infer_template_types(&p.ty, &arg).lower_bound())
    });
    params.union(infer_template_types(&ours.ret, &theirs.ret))
}

/// `TemplateTypeHelper::resolveTemplateTypes()`: the map's templates replaced, an `*ERROR*` one by its default or
/// bound.
pub fn resolve_template_types(t: &Type, map: &TemplateMap) -> Type {
    if let Type::Template(tt) = t
        && !tt.argument
        && let Some(with) = map.get(tt)
    {
        return match with {
            Type::Error => resolve_template_types(tt.default.as_ref().unwrap_or(&tt.bound), map),
            with => with.clone(),
        };
    }
    t.traverse(&|inner| resolve_template_types(inner, map))
}

impl TemplateMap {
    /// The map of a call (`GenericParametersAcceptorResolver::resolve()`): each of `templates` the arguments did
    /// not infer is an `*ERROR*`.
    pub fn with_errors(mut self, templates: &[&Template]) -> TemplateMap {
        for t in templates {
            if self.get(t).is_none() {
                self.types.push((key(t), Type::Error));
            }
        }
        self
    }
}
