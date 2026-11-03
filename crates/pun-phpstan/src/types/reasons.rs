//! `AcceptsResult::$reasons`: why `accepts()` failed, the tips of the argument, return and property type errors
//! (`acceptsReasonsTip()`). Each reason comes from the same branch of `accepts()` as in PHPStan; the answer itself is
//! [`accepts_with()`]'s.

use super::accepts::{StrictMixed, accepts_with};
use super::templates::ancestor;
use super::*;

/// The reasons of `$a->accepts($b, $strictTypes)`.
pub fn accepts_reasons(a: &Type, b: &Type, strict: bool, sm: StrictMixed) -> Vec<String> {
    use Type as T;
    if sm.explicit || sm.implicit {
        return accepts_reasons(a, &sm.transform(b), strict, StrictMixed::default());
    }
    let rec = |x: &Type, y: &Type| accepts_reasons(x, y, strict, StrictMixed::default());
    let compound = b.is_compound();
    match a {
        T::Reasoned(t, _) => rec(t, b),
        T::LateArrayShape(s) => rec(&s.resolve(), b),
        T::Union(types) | T::Benevolent(types) => union(a, types, b, strict, sm),
        T::Intersection(types) => {
            let mut reasons = merged(types.iter().map(|t| accepts_reasons(t, b, strict, sm)));
            if !accepts_with(a, b, strict, sm).yes() {
                let level = recommended_level(a, Some(b));
                let list = b.is_list();
                if a.is_list().yes() && !list.yes() {
                    reasons.push(format!("{} {} a list.", b.describe(level), if list.no() { "is not" } else { "might not be" }));
                }
                let non_empty = b.is_iterable_at_least_once();
                if a.is_iterable_at_least_once().yes() && !non_empty.yes() {
                    reasons.push(format!("{} {} empty.", b.describe(level), if non_empty.no() { "is" } else { "might be" }));
                }
            }
            reasons
        }
        T::Conditional(c) => accepts_reasons(&c.resolve(), b, strict, sm),
        T::Template(t) if !compound => {
            let mut reasons = accepts_reasons(&t.bound, b, strict, sm);
            if t.argument && !accepts_with(&t.bound, b, strict, sm).no() {
                let level = recommended_level(a, Some(b));
                reasons.push(format!(
                    "Type {} is not always the same as {}. It breaks the contract for some argument types, typically subtypes.",
                    b.describe(level),
                    t.name
                ));
            }
            reasons
        }
        T::Callable(_) if matches!(b, T::Callable(_)) || !compound => match a {
            T::Callable(Some(ours)) if !b.is_callable().no() && !matches!(b, T::Callable(None)) => {
                merged(callable_signatures(b).iter().map(|theirs| signature(ours, theirs, strict)))
            }
            _ => vec![],
        },
        T::ConstArray(c) if c.unsealed.is_none() && (!compound || matches!(b, T::Intersection(_))) => our_keys(c, b, strict),
        T::Mixed { .. } | T::StrictMixed | T::SubtractedMixed { .. } | T::Error | T::Wildcard | T::OffsetAccess(..) | T::Never(_) | T::ObjectShape(_) => vec![],
        _ if compound => accepted_by(a, b, strict, sm),
        T::Array(key, item) => match b {
            T::ConstArray(c) => merged(c.keys.iter().zip(&c.values).flat_map(|(k, v)| [rec(key, k), rec(item, v)])),
            T::Array(k, v) => merged([rec(item, v), rec(key, k)]),
            _ => vec![],
        },
        T::Iterable(key, item) if b.is_iterable().yes() && !(b.is_constant_array().yes() && b.is_iterable_at_least_once().no()) => {
            merged([rec(item, &b.iterable_value_type()), rec(key, &b.iterable_key_type())])
        }
        T::Closure(ours) => match b {
            T::Closure(theirs) => signature(ours, theirs, strict),
            _ => vec![],
        },
        T::Object(class, args, _) if !args.is_empty() => generic(class, args, b),
        _ => vec![],
    }
}

/// `array_values(array_unique(array_merge(...)))` of results combined with `and()`/`or()`.
fn merged(all: impl IntoIterator<Item = Vec<String>>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for r in all.into_iter().flatten() {
        if !out.contains(&r) {
            out.push(r);
        }
    }
    out
}

/// UnionType::accepts()
fn union(a: &Type, types: &[Type], b: &Type, strict: bool, sm: StrictMixed) -> Vec<String> {
    use Type as T;
    if let T::Iterable(k, v) = b {
        let alt = T::Union(vec![T::Array(k.clone(), v.clone()), T::Object("Traversable".into(), vec![(**k).clone(), (**v).clone()], false)]);
        return accepts_reasons(a, &alt, strict, sm);
    }
    let inner: Vec<Vec<String>> = sort_types(types).iter().map(|t| accepts_reasons(t, b, strict, sm)).collect();
    let decorate = |i: usize, r: &String| format!("Type #{} from the union: {r}", i + 1);
    let mut reasons = merged(inner.iter().enumerate().map(|(i, rs)| rs.iter().map(|r| decorate(i, r)).collect()));
    if types.iter().map(|t| accepts_with(t, b, strict, sm)).fold(Tri::No, Tri::or).yes() {
        return reasons;
    }
    let common: Vec<&String> = inner[0].iter().filter(|r| inner[1..].iter().all(|o| o.contains(r))).collect();
    if !common.is_empty() {
        reasons = (0..inner.len()).flat_map(|i| common.iter().map(move |r| decorate(i, r))).collect();
    }
    if b.is_compound() && !matches!(b, T::Callable(_) | T::Template(_) | T::Intersection(_)) {
        return accepted_by(a, b, strict, sm);
    }
    if matches!(b, T::Template(t) if matches!(t.bound, T::Union(_) | T::Benevolent(_))) {
        return merged([reasons, accepted_by(a, b, strict, sm)]);
    }
    reasons
}

/// `CompoundType::isAcceptedBy()` of `b`.
fn accepted_by(a: &Type, b: &Type, strict: bool, sm: StrictMixed) -> Vec<String> {
    use Type as T;
    // (a StrictMixedType answers `mixed`, which gives no reasons)
    let rec = |t: &Type| accepts_reasons(a, t, strict, sm);
    match b {
        T::SubtractedMixed { excluded, .. } if is_super_type_of(excluded, a).yes() => vec![format!(
            "Type {} has already been eliminated from mixed.", excluded.describe(Level::Precise)
        )],
        T::Union(types) | T::Benevolent(types) => merged(types.iter().map(rec)),
        // AcceptsResult::lazyMaxMin(): the first member accepted answers
        T::Intersection(types) => match types.iter().find(|t| accepts_with(a, t, strict, sm).yes()) {
            Some(t) => rec(t),
            None => merged(types.iter().map(rec)),
        },
        T::Conditional(c) => rec(&c.resolve()),
        T::Template(t) if !matches!(t.bound, T::Mixed { .. }) => match a {
            T::Template(at) => accepts_reasons(&at.bound, &t.bound, strict, sm),
            _ => rec(&t.bound),
        },
        _ => vec![],
    }
}

/// CallableTypeHelper::isParametersAcceptorSuperTypeOf() with `$treatMixedAsAny`: `ours` accepting `theirs`.
fn signature(ours: &Signature, theirs: &Signature, strict: bool) -> Vec<String> {
    let theirs = specialize_signature(ours, theirs);
    let mut params: Vec<&Param> = theirs.params.iter().collect();
    if let Some(last) = theirs.params.last().filter(|p| p.variadic) {
        while params.len() < ours.params.len() {
            params.push(last);
        }
    }
    let mut reasons = Vec::new();
    for (i, theirs) in params.iter().enumerate() {
        let described =
            if theirs.name.is_empty() { format!("#{}", i + 1) } else { format!("#{} ${}", i + 1, theirs.name) };
        let Some(our) = ours.params.get(i) else {
            if !theirs.optional {
                reasons.push(format!(
                    "Parameter {described} of passed callable is required but accepting callable does not have that parameter. It will be called without it."
                ));
            }
            continue;
        };
        if our.optional && !theirs.optional {
            reasons.push(format!(
                "Parameter {described} of passed callable is required but the parameter of accepting callable is optional. It might be called without it."
            ));
        }
        reasons.extend(accepts_reasons(&theirs.ty, &our.ty, strict, StrictMixed::default()));
        if accepts_with(&theirs.ty, &our.ty, strict, StrictMixed::default()).maybe() {
            let level = recommended_level(&theirs.ty, Some(&our.ty));
            reasons.push(format!(
                "Type {} of parameter {described} of passed callable needs to be same or wider than parameter type {} of accepting callable.",
                theirs.ty.describe(level),
                our.ty.describe(level)
            ));
        }
    }
    reasons.extend(accepts_reasons(&ours.ret, &theirs.ret, true, StrictMixed::default()));
    merged([reasons])
}

/// ConstantArrayType::checkOurKeys()
// ponytail: an unsealed shape's extra keys give no reasons, nor the offsets of a non-array (a string's, an
// ArrayAccess object's)
fn our_keys(c: &ConstArray, b: &Type, strict: bool) -> Vec<String> {
    let constant = match b {
        Type::ConstArray(b) => Some(b),
        _ => None,
    };
    let mut reasons = Vec::new();
    for (i, (k, v)) in c.keys.iter().zip(&c.values).enumerate() {
        let optional = c.optional.contains(&i);
        let (has, other) = match constant {
            Some(b) => match b.keys.iter().position(|o| o == k) {
                Some(j) => (if b.optional.contains(&j) { Tri::Maybe } else { Tri::Yes }, b.values[j].clone()),
                None => (Tri::No, Type::Never(NeverKind::Implicit)),
            },
            None if is_super_type_of(&b.iterable_key_type(), k).no() || b.is_array().no() => (Tri::No, Type::Error),
            None => (Tri::Maybe, b.iterable_value_type()),
        };
        let offset = |no: bool| format!("Array {} have offset {}.", if no { "does not" } else { "might not" }, k.describe(Level::Value));
        if has.no() {
            if optional {
                continue;
            }
            return if constant.is_some() { vec![offset(true)] } else { vec![] };
        }
        if has.maybe() && !optional && constant.is_some() {
            reasons.push(offset(false));
        }
        let level = recommended_level(v, Some(&other));
        let (key, ours, theirs) = (k.describe(Level::Precise), v.describe(level), other.describe(level));
        let mut value: Vec<String> = accepts_reasons(v, &other, strict, StrictMixed::default())
            .into_iter()
            .map(|r| format!("Offset {key} ({ours}) does not accept type {theirs}: {r}"))
            .collect();
        let accepted = accepts_with(v, &other, strict, StrictMixed::default());
        if !accepted.yes() && value.is_empty() && constant.is_some() {
            value.push(format!("Offset {key} ({ours}) does not accept type {theirs}."));
        }
        if accepted.no() {
            return value;
        }
        reasons.extend(value);
    }
    merged([reasons])
}

/// GenericObjectType::isSuperTypeOfInternal() in an accepts context: an invariant template given a subtype is not
/// covariant (TemplateTypeVariance::isValidVariance()).
pub(super) fn generic(class: &str, args: &[Type], b: &Type) -> Vec<String> {
    let (Type::Object(name, other, _) | Type::Static(name, other)) = b else {
        return vec![];
    };
    let (Some(c), Some(theirs)) = (symbols::lookup(class), ancestor(name, other, class)) else {
        return vec![];
    };
    // a raw object of the class itself is no GenericObjectType ancestor
    if other.is_empty() && name.eq_ignore_ascii_case(class) {
        return vec![];
    }
    let mut reasons = Vec::new();
    for ((t, ours), theirs) in c.templates.iter().zip(args).zip(&theirs) {
        if t.variance != "invariant" || matches!(ours, Type::Projection(..)) {
            continue;
        }
        if !valid_variance(ours, theirs, t.variance, false).yes() && is_super_type_of(ours, theirs).yes() {
            reasons.push(format!(
                "Template type {} on class {} is not covariant. Learn more: <fg=cyan>https://phpstan.org/blog/whats-up-with-template-covariant</>",
                t.name, c.name
            ));
        }
    }
    reasons
}
