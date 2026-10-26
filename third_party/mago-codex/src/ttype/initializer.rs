//! Initializer unions retain PHPStan's observable member order, unlike ordinary flow unions.

use crate::metadata::CodebaseMetadata;
use crate::ttype::atomic::TAtomic;
use crate::ttype::atomic::scalar::TScalar;
use crate::ttype::atomic::scalar::class_like_string::TClassLikeString;
use crate::ttype::union::TUnion;
use crate::ttype::{combine_union_types, get_mixed};

/// TypeCombinator::union buckets constant scalars after other types, retaining the first occurrence
/// within each bucket. Reflection's getConstant() and first mixed/error selection observe this order.
/// Reuse the existing semantic normalizer; do not change ordinary flow's canonical sorted storage.
pub(crate) fn union(left: &TUnion, right: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    let mut result = combine_union_types(left, right, codebase, Default::default());
    let input: Vec<_> = left.types.iter().chain(right.types.iter()).collect();
    result.types.to_mut().sort_by_key(|atomic| {
        (constant_scalar(atomic), input.iter().position(|original| *original == atomic).unwrap_or(input.len()))
    });
    result
}

fn constant_scalar(atomic: &TAtomic) -> bool {
    match atomic {
        TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { .. })) => true,
        TAtomic::Scalar(TScalar::String(string)) => string.get_known_literal_value().is_some(),
        TAtomic::Scalar(TScalar::Integer(integer)) => integer.is_literal(),
        TAtomic::Scalar(TScalar::Float(float)) => float.is_literal(),
        TAtomic::Scalar(TScalar::Bool(boolean)) => !boolean.is_general(),
        _ => false,
    }
}

/// Expand references in place, preserving each replacement's position in a canonical initializer.
pub(crate) fn expanded(parts: Vec<TAtomic>, codebase: &CodebaseMetadata) -> TUnion {
    let mut parts = parts.into_iter();
    let mut value = parts.next().map(TUnion::from_atomic).unwrap_or_else(get_mixed);
    for part in parts {
        value = union(&value, &TUnion::from_atomic(part), codebase);
    }
    value
}
