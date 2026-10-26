//! MixedType's excluded set, distinct from ErrorType's subtract override.

use super::{NeverKind, Tri, Type, bx, intersect, is_super_type_of, union};

impl Type {
    /// `instanceof MixedType`, including ErrorType and nested TemplateMixedType bounds. A template is
    /// explicitly mixed independently of the bound's explicitness (`TemplateMixedType::__construct()`).
    pub fn mixed_explicitness(&self) -> Option<bool> {
        match self {
            Type::Mixed { explicit } | Type::SubtractedMixed { explicit, .. } => Some(*explicit),
            Type::Error => Some(false),
            Type::Template(t) if t.bound.mixed_explicitness().is_some() => Some(true),
            _ => None,
        }
    }

    /// RuleLevelHelper's checked copy. TemplateMixedType::toStrictMixedType replaces the entire bound,
    /// including a nested template or subtraction, but retains scope, name, strategy and the ordinary default.
    pub fn to_strict_mixed(&self) -> Type {
        if self.mixed_explicitness().is_none() {
            return self.clone();
        }
        match self {
            Type::Template(t) => {
                let mut t = t.clone();
                t.bound = Type::StrictMixed;
                Type::Template(t)
            }
            _ => Type::StrictMixed,
        }
    }

    /// MixedType::__construct(): a never exclusion is equivalent to no exclusion.
    pub fn mixed_subtracted(explicit: bool, excluded: Type) -> Type {
        if excluded.is_never() {
            Type::Mixed { explicit }
        } else {
            Type::SubtractedMixed {
                explicit,
                excluded: bx(excluded),
            }
        }
    }
}

pub(super) fn is_super(excluded: &Type, other: &Type) -> Tri {
    match other {
        Type::Never(_) => Tri::Yes,
        Type::Mixed { .. } | Type::Error => Tri::Maybe,
        Type::SubtractedMixed {
            excluded: other, ..
        } => {
            if is_super_type_of(other, excluded).yes() {
                Tri::Yes
            } else {
                Tri::Maybe
            }
        }
        _ => is_super_type_of(excluded, other).negate(),
    }
}

pub(super) fn join(explicit: bool, excluded: &Type, other: &Type) -> Type {
    let remaining = match other {
        Type::Mixed { .. } | Type::Error => return other.clone(),
        Type::SubtractedMixed {
            excluded: other, ..
        } => intersect(vec![excluded.clone(), (**other).clone()]),
        Type::Never(_) => excluded.clone(),
        _ => {
            let remaining = remove(excluded, other);
            if !remaining.is_never() && !is_super_type_of(&remaining, other).no() {
                return Type::Mixed { explicit };
            }
            remaining
        }
    };
    Type::mixed_subtracted(explicit, remaining)
}

pub(super) fn intersection(a: &Type, b: &Type) -> Option<Type> {
    let Type::SubtractedMixed { explicit, excluded } = a else {
        return None;
    };
    Some(match b {
        Type::Mixed { .. } | Type::Error => a.clone(),
        Type::SubtractedMixed {
            excluded: other, ..
        } => Type::mixed_subtracted(
            *explicit,
            union(vec![(**excluded).clone(), (**other).clone()]),
        ),
        _ => remove(b, excluded),
    })
}

/// TypeCombinator::remove() for the value sets represented by mixed exclusions. IntegerRangeType's
/// exact difference is representable; unsupported partial differences retain their original type.
fn remove(from: &Type, excluded: &Type) -> Type {
    if let Type::Union(parts) | Type::Benevolent(parts) = excluded {
        return parts
            .iter()
            .fold(from.clone(), |remaining, part| remove(&remaining, part));
    }
    if let Type::Union(parts) | Type::Benevolent(parts) = from {
        return union(parts.iter().map(|part| remove(part, excluded)).collect());
    }
    if is_super_type_of(excluded, from).yes() {
        return Type::Never(NeverKind::Implicit);
    }
    if let (Some((min, max)), Some((lo, hi))) = (from.int_bounds(), excluded.int_bounds()) {
        let (min, max) = (min.unwrap_or(i64::MIN), max.unwrap_or(i64::MAX));
        let (lo, hi) = (lo.unwrap_or(i64::MIN), hi.unwrap_or(i64::MAX));
        if hi < min || lo > max {
            return from.clone();
        }
        let mut parts = Vec::new();
        if lo > min {
            parts.push(Type::int_range(
                (min != i64::MIN).then_some(min),
                Some(lo - 1),
            ));
        }
        if hi < max {
            parts.push(Type::int_range(
                Some(hi + 1),
                (max != i64::MAX).then_some(max),
            ));
        }
        return union(parts);
    }
    from.clone()
}
