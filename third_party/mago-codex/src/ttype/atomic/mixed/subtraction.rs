//! PHPStan 2.3.1 MixedType subtraction and TypeCombinator's subtractable union transfer.

use std::borrow::Cow;
use std::sync::Arc;

use super::TMixed;
use crate::metadata::CodebaseMetadata;
use crate::ttype::atomic::TAtomic;
use crate::ttype::atomic::array::TArray;
use crate::ttype::atomic::scalar::TScalar;
use crate::ttype::atomic::scalar::int::TInteger;
use crate::ttype::combiner::CombinerOptions;
use crate::ttype::comparator::{ComparisonResult, atomic_comparator, union_comparator};
use crate::ttype::union::TUnion;
use crate::ttype::{combine_union_types, get_empty, get_empty_keyed_array, get_never, get_null, intersect_union_types};

impl TMixed {
    /// The type excluded from this mixed value, independently of Mago's legacy truthiness flags.
    #[must_use]
    pub fn get_subtracted_type(&self) -> Option<&TUnion> {
        self.subtracted_type.as_deref()
    }

    /// Include the negative constraints used by Mago before explicit excluded sets existed.
    pub(crate) fn effective_subtracted_type(&self) -> Option<Cow<'_, TUnion>> {
        if let Some(excluded) = self.get_subtracted_type() {
            Some(Cow::Borrowed(excluded))
        } else if self.is_truthy() {
            Some(Cow::Owned(get_empty()))
        } else if self.is_non_null() {
            Some(Cow::Owned(get_null()))
        } else {
            None
        }
    }

    /// MixedType::changeSubtractedType(): never means no exclusion; this constructs ordinary mixed.
    #[must_use]
    pub fn with_subtracted_type(excluded: TUnion) -> Self {
        Self {
            is_non_null: excluded.has_null(),
            subtracted_type: (!excluded.is_never()).then(|| Arc::new(excluded)),
            ..Self::new()
        }
    }

    /// MixedType::subtract(), including ErrorType's overriding no-op and repeated subtraction.
    /// Unlike TypeCombinator::remove(), subtracting any mixed directly produces never.
    #[must_use]
    pub fn subtract(&self, excluded: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
        if self.is_error() {
            return TUnion::from_atomic(TAtomic::Mixed(self.clone()));
        }
        if excluded.types.iter().any(TAtomic::is_mixed) {
            return get_never();
        }
        let excluded = match self.effective_subtracted_type() {
            Some(previous) => {
                combine_union_types(&previous, excluded, codebase, CombinerOptions::default())
            }
            None => excluded.clone(),
        };
        TUnion::from_atomic(TAtomic::Mixed(Self::with_subtracted_type(excluded).with_is_explicit(self.is_explicit())))
    }

    /// MixedType::unsetOffset(): an unset may create the empty array, but no other excluded value.
    #[must_use]
    pub fn unset_offset(&self, codebase: &CodebaseMetadata) -> Self {
        match self.get_subtracted_type() {
            Some(excluded) => {
                Self::with_subtracted_type(remove(excluded, &get_empty_keyed_array(), codebase))
                    .with_is_explicit(self.is_explicit())
            }
            None => self.clone(),
        }
    }
}

/// Set difference needed by mixed exclusions. Unrepresentable partial differences are left intact;
/// the union transfer below discards such an exclusion instead of claiming an unsound disjoint result.
pub(crate) fn remove(from: &TUnion, excluded: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    let mut result = from.types.to_vec();
    for remove in excluded.types.iter() {
        result = result
            .into_iter()
            .flat_map(|atomic| {
                if atomic_comparator::is_contained_by(
                    codebase,
                    &atomic,
                    remove,
                    false,
                    &mut ComparisonResult::new(),
                ) {
                    Vec::new()
                } else if let (TAtomic::Array(array), TAtomic::Array(empty)) = (&atomic, remove)
                    && empty.is_empty()
                {
                    // PHPStan ArrayType::tryRemove(array{}): retain precisely non-empty arrays.
                    vec![TAtomic::Array(match array {
                        TArray::Keyed(array) => TArray::Keyed(array.clone().with_non_empty(true)),
                        TArray::List(array) => TArray::List(array.clone_non_empty()),
                    })]
                } else if let (
                    TAtomic::Scalar(TScalar::Integer(a)),
                    TAtomic::Scalar(TScalar::Integer(b)),
                ) = (&atomic, remove)
                {
                    TInteger::combine(a.difference(*b, false))
                } else {
                    vec![atomic]
                }
            })
            .collect();
    }
    if result.is_empty() {
        get_never()
    } else {
        TUnion::from_vec(result)
    }
}

/// Union removes only the newly included values from an exclusion; two exclusions intersect.
pub(crate) fn union(excluded: &TUnion, other: &TUnion, codebase: &CodebaseMetadata) -> TMixed {
    let mut excluded = excluded.clone();
    for atomic in other.types.iter().filter(|atomic| !atomic.is_never()) {
        if let TAtomic::Mixed(other) = atomic {
            let Some(other_excluded) = other.effective_subtracted_type() else {
                return TMixed::new();
            };
            excluded = intersect_union_types(&excluded, &other_excluded, codebase)
                .unwrap_or_else(get_never);
        } else {
            let other = TUnion::from_atomic(atomic.clone());
            excluded = remove(&excluded, &other, codebase);
            if union_comparator::can_expression_types_be_identical(
                codebase, &excluded, &other, false, false,
            ) {
                return TMixed::new();
            }
        }
    }
    TMixed::with_subtracted_type(excluded)
}
