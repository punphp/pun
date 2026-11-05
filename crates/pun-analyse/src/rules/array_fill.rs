//! PHPStan 2.3.1's ArrayFillFunctionReturnTypeExtension as a Mago return type provider: `array_fill()` of a constant
//! start index and a constant count of at most 100 is a constant array, else a list or array of the value type.

use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{get_arraykey, get_int, get_mixed, get_never};
use pun_phpstan::types::Tri;

const MAX_SIZE_USE_CONSTANT_ARRAY: i64 = 100;

static META: ProviderMeta = ProviderMeta::new("pun-array-fill", "pun", "PHPStan's array_fill() return type");

pub(super) struct ArrayFill;

impl Provider for ArrayFill {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for ArrayFill {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"array_fill")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        // the extension declines, and PHPStan's signature map returns `array` where Mago's prelude infers a list
        if invocation.argument_count() < 3 {
            return Some(TUnion::from_atomic(TAtomic::Array(TArray::Keyed(TKeyedArray::new_with_parameters(
                Arc::new(get_arraykey()),
                Arc::new(get_mixed()),
            )))));
        }
        let start = context.get_expression_type(invocation.get_argument(0, &[b"start_index"])?)?;
        let count = context.get_expression_type(invocation.get_argument(1, &[b"count"])?)?;
        let value = context.get_expression_type(invocation.get_argument(2, &[b"value"])?)?;
        Some(return_type(start, count, value))
    }
}

/// `getTypeFromFunctionCall()` on PHP 8, where a negative count throws a ValueError (no `false`).
fn return_type(start: &TUnion, count: &TUnion, value: &TUnion) -> TUnion {
    let valid = non_negative(count);
    if valid == Tri::No {
        return get_never();
    }
    if let (Some(start), Some(n)) = (start.get_single_literal_int_value(), count.get_single_literal_int_value())
        && n <= MAX_SIZE_USE_CONSTANT_ARRAY
    {
        let mut items = std::collections::BTreeMap::new();
        let mut next = start;
        let mut overflows = false;
        for i in 0..n {
            items.insert(ArrayKey::Integer(next), (false, value.clone()));
            if next < 0 {
                next = 0;
            } else if next == i64::MAX {
                // PHP throws "Cannot add element to the array as the next element is already occupied"
                overflows = i + 1 < n;
                break;
            } else {
                next += 1;
            }
        }
        if !overflows {
            return TUnion::from_atomic(TAtomic::Array(TArray::Keyed(TKeyedArray {
                non_empty: !items.is_empty(),
                known_items: Some(Arc::new(items)),
                ..TKeyedArray::default()
            })));
        }
    }
    let non_empty = count.types.iter().all(|t| int_from(t, 1) == Tri::Yes);
    let array = if start.get_single_literal_int_value() == Some(0) {
        TArray::List(TList { non_empty, ..TList::new(Arc::new(value.clone())) })
    } else {
        TArray::Keyed(TKeyedArray { non_empty, ..TKeyedArray::new_with_parameters(Arc::new(get_int()), Arc::new(value.clone())) })
    };
    TUnion::from_atomic(TAtomic::Array(array))
}

/// `IntegerRangeType::fromInterval(0, null)->isSuperTypeOf($count)`.
fn non_negative(count: &TUnion) -> Tri {
    Tri::extreme_identity(count.types.iter().map(|t| int_from(t, 0)))
}

/// `IntegerRangeType::fromInterval($min, null)->isSuperTypeOf()` of one member.
fn int_from(t: &TAtomic, min: i64) -> Tri {
    match t {
        TAtomic::Scalar(TScalar::Integer(_)) => {
            if t.get_minimum_int_value().is_some_and(|n| n >= min) {
                Tri::Yes
            } else if t.get_maximum_int_value().is_some_and(|n| n < min) {
                Tri::No
            } else {
                Tri::Maybe
            }
        }
        // an integer template is only maybe within a range, whatever its bound (TemplateIntegerType)
        TAtomic::GenericParameter(_) | TAtomic::Mixed(_) | TAtomic::Scalar(TScalar::Generic | TScalar::ArrayKey | TScalar::Numeric) => Tri::Maybe,
        _ => Tri::No,
    }
}
