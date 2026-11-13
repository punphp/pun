//! PHPStan 2.3.5's ArrayFlipFunctionReturnTypeExtension and ArrayIntersectKeyFunctionReturnTypeExtension, with
//! `Type::flipArray()` and `Type::intersectKeyArray()` of the array types they ask. Mago's prelude generalizes a
//! flipped shape and keeps none of the other arrays' keys in an intersection.

use std::collections::BTreeMap;
use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::{KnownItems, TKeyedArray};
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    combine_union_types, get_array_parameters, get_arraykey, get_empty_keyed_array, get_int,
};

static META: ProviderMeta = ProviderMeta::new(
    "pun-flip-intersect",
    "pun",
    "PHPStan's array_flip() and array_intersect_key() return types",
);

pub(super) struct FlipIntersect;

impl Provider for FlipIntersect {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for FlipIntersect {
    fn targets() -> FunctionTarget {
        FunctionTarget::ExactMultiple(&[b"array_flip", b"array_intersect_key"])
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let codebase = context.codebase();
        if invocation
            .function_name()
            .eq_ignore_ascii_case("array_flip")
        {
            if invocation.argument_count() != 1 {
                return None;
            }
            let array = context.get_expression_type(invocation.get_argument(0, &[b"array"])?)?;
            let non_empty = array
                .types
                .iter()
                .all(|t| matches!(t, TAtomic::Array(a) if a.is_non_empty()));
            return each(array, codebase, |a| flip(a, non_empty, codebase));
        }
        // `...$arrays` unpacked into the call: their value types, which this does not follow
        let arguments: Vec<TUnion> = (0..invocation.argument_count())
            .map(|i| {
                invocation
                    .get_argument(i, &[])
                    .and_then(|a| context.get_expression_type(a).cloned())
            })
            .collect::<Option<_>>()?;
        let (first, others) = arguments.split_first()?;
        if others.is_empty() {
            return Some(first.clone());
        }
        let other = others
            .iter()
            .skip(1)
            .fold(others[0].clone(), |all, t| union(&all, t, codebase));
        let other_arrays = other
            .types
            .iter()
            .map(|t| match t {
                TAtomic::Array(a) => Some(a),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        each(first, codebase, |a| {
            intersect_key(a, &other_arrays, codebase)
        })
    }
}

/// The result for each array member of `t` (`UnionType::flipArray()` & co.), or `None` for any other member.
fn each(
    t: &TUnion,
    codebase: &CodebaseMetadata,
    mut f: impl FnMut(&TArray) -> TUnion,
) -> Option<TUnion> {
    let mut result: Option<TUnion> = None;
    for atomic in t.types.iter() {
        let TAtomic::Array(array) = atomic else {
            return None;
        };
        let part = f(array);
        result = Some(match result {
            Some(all) => union(&all, &part, codebase),
            None => part,
        });
    }
    result
}

fn union(a: &TUnion, b: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    combine_union_types(a, b, codebase, CombinerOptions::default())
}

/// The items of a constant array (ConstantArrayType), in order: none for a general one.
fn constant_items(array: &TArray) -> Option<Vec<(ArrayKey, bool, TUnion)>> {
    match array {
        TArray::Keyed(keyed) if keyed.parameters.is_none() => Some(
            keyed
                .known_items
                .iter()
                .flat_map(|items| items.iter())
                .map(|(key, (optional, value))| (*key, *optional, value.clone()))
                .collect(),
        ),
        TArray::List(list) if list.element_type.is_never() => Some(
            list.known_elements
                .iter()
                .flat_map(|items| items.iter())
                .map(|(index, (optional, value))| {
                    (ArrayKey::Integer(*index as i64), *optional, value.clone())
                })
                .collect(),
        ),
        _ => None,
    }
}

/// `Type::toArrayKey()`.
fn to_array_key(t: &TUnion) -> TUnion {
    let mut out = Vec::new();
    for atomic in t.types.iter() {
        match atomic {
            TAtomic::Scalar(TScalar::Integer(_)) => out.push(atomic.clone()),
            // a decimal integer string is that integer as a key
            TAtomic::Scalar(s @ TScalar::String(_)) => match s.get_known_literal_string_value() {
                Some(value) => out.push(ArrayKey::from_string(mago_word::word(value)).to_atomic()),
                None => out.push(atomic.clone()),
            },
            TAtomic::Scalar(TScalar::Bool(b)) => match b.value {
                Some(value) => out.push(TAtomic::Scalar(TScalar::literal_int(i64::from(value)))),
                None => {
                    out.push(TAtomic::Scalar(TScalar::literal_int(0)));
                    out.push(TAtomic::Scalar(TScalar::literal_int(1)));
                }
            },
            TAtomic::Scalar(TScalar::Float(_)) => out.extend(get_int().types.into_owned()),
            TAtomic::Null => out.push(TAtomic::Scalar(TScalar::literal_string(mago_word::word(
                "",
            )))),
            _ => return get_arraykey(),
        }
    }
    TUnion::from_vec(out)
}

fn literal_key(atomic: &TAtomic) -> Option<ArrayKey> {
    match atomic {
        TAtomic::Scalar(TScalar::Integer(i)) => i.get_literal_value().map(ArrayKey::Integer),
        TAtomic::Scalar(s @ TScalar::String(_)) => s
            .get_known_literal_string_value()
            .map(|v| ArrayKey::from_string(mago_word::word(v))),
        _ => None,
    }
}

/// ConstantArrayTypeBuilder::getArray(): `array{...}` as a list when its keys are 0, 1, ... in order.
fn shape(items: Vec<(ArrayKey, bool, TUnion)>) -> TUnion {
    if items.is_empty() {
        return get_empty_keyed_array();
    }
    let is_list = items
        .iter()
        .enumerate()
        .all(|(i, (key, optional, _))| *key == ArrayKey::Integer(i as i64) && !optional);
    if is_list {
        let elements: BTreeMap<usize, (bool, TUnion)> = items
            .into_iter()
            .enumerate()
            .map(|(i, (_, optional, value))| (i, (optional, value)))
            .collect();
        return TUnion::from_atomic(TAtomic::Array(TArray::List(TList::from_known_elements(
            elements,
        ))));
    }
    let non_empty = items.iter().any(|(_, optional, _)| !optional);
    let mut known = KnownItems::default();
    for (key, optional, value) in items {
        known.insert(key, (optional, value));
    }
    TUnion::from_atomic(TAtomic::Array(TArray::Keyed(
        TKeyedArray::new()
            .with_known_items(known)
            .with_non_empty(non_empty),
    )))
}

/// ConstantArrayTypeBuilder::setOffsetValueType() of a literal key: a later required key replaces the value.
fn set(
    items: &mut Vec<(ArrayKey, bool, TUnion)>,
    key: ArrayKey,
    optional: bool,
    value: TUnion,
    codebase: &CodebaseMetadata,
) {
    match items.iter_mut().find(|(k, _, _)| *k == key) {
        Some(existing) if optional => existing.2 = union(&existing.2, &value, codebase),
        Some(existing) => {
            existing.1 = false;
            existing.2 = value;
        }
        None => items.push((key, optional, value)),
    }
}

fn general(key: TUnion, value: TUnion, non_empty: bool) -> TUnion {
    TUnion::from_atomic(TAtomic::Array(TArray::Keyed(
        TKeyedArray::new()
            .with_parameters(Arc::new(key), Arc::new(value))
            .with_non_empty(non_empty),
    )))
}

/// `flipArray()`, intersected with NonEmptyArrayType when the whole argument is non-empty.
fn flip(array: &TArray, non_empty: bool, codebase: &CodebaseMetadata) -> TUnion {
    if let Some(items) = constant_items(array) {
        let mut flipped = Vec::new();
        let mut degraded: Option<(TUnion, TUnion)> = None;
        for (key, optional, value) in items {
            let offset = to_array_key(&value);
            let literals: Option<Vec<ArrayKey>> = offset.types.iter().map(literal_key).collect();
            match literals {
                Some(literals) if degraded.is_none() => {
                    let several = literals.len() > 1;
                    for literal in literals {
                        set(
                            &mut flipped,
                            literal,
                            optional || several,
                            key.to_union(),
                            codebase,
                        );
                    }
                }
                _ => {
                    degraded = Some(match degraded {
                        Some((k, v)) => (
                            union(&k, &offset, codebase),
                            union(&v, &key.to_union(), codebase),
                        ),
                        None => (offset, key.to_union()),
                    });
                }
            }
        }
        return match degraded {
            // a non-literal key degrades the builder to a general array of every item
            Some((mut keys, mut values)) => {
                for (key, _, value) in flipped {
                    keys = union(&keys, &key.to_union(), codebase);
                    values = union(&values, &value, codebase);
                }
                general(keys, values, non_empty)
            }
            None => shape(flipped),
        };
    }
    let (key, value) = get_array_parameters(array, codebase);
    general(to_array_key(&value), key, non_empty)
}

#[derive(Clone, Copy, PartialEq)]
enum Has {
    Yes,
    Maybe,
    No,
}

fn contained(codebase: &CodebaseMetadata, inner: &TUnion, outer: &TUnion) -> bool {
    union_comparator::is_contained_by(
        codebase,
        inner,
        outer,
        false,
        false,
        false,
        &mut ComparisonResult::new(),
    )
}

/// `hasOffsetValueType()` of the other arrays.
fn has_offset(others: &[&TArray], key: &ArrayKey, codebase: &CodebaseMetadata) -> Has {
    let mut all_yes = true;
    let mut any = false;
    for other in others {
        let has = match constant_items(other) {
            Some(items) => match items.iter().find(|(k, _, _)| k == key) {
                Some((_, false, _)) => Has::Yes,
                Some((_, true, _)) => Has::Maybe,
                None => Has::No,
            },
            None => {
                let (other_key, _) = get_array_parameters(other, codebase);
                if contained(codebase, &key.to_union(), &other_key) {
                    Has::Maybe
                } else {
                    Has::No
                }
            }
        };
        all_yes &= has == Has::Yes;
        any |= has != Has::No;
    }
    match (all_yes, any) {
        (true, _) => Has::Yes,
        (false, true) => Has::Maybe,
        (false, false) => Has::No,
    }
}

/// `intersectKeyArray()`.
fn intersect_key(array: &TArray, others: &[&TArray], codebase: &CodebaseMetadata) -> TUnion {
    if let Some(items) = constant_items(array) {
        let mut kept = Vec::new();
        for (key, optional, value) in items {
            match has_offset(others, &key, codebase) {
                Has::No => {}
                has => kept.push((key, optional || has != Has::Yes, value)),
            }
        }
        return shape(kept);
    }
    let other_key = others
        .iter()
        .map(|other| get_array_parameters(other, codebase).0)
        .reduce(|all, key| union(&all, &key, codebase))
        .unwrap_or_else(get_arraykey);
    let (key, value) = get_array_parameters(array, codebase);
    // ArrayType::intersectKeyArray() keeps the array; NonEmptyArrayType's is mixed, so the result may be empty
    if contained(codebase, &key, &other_key) {
        let mut array = array.clone();
        match &mut array {
            TArray::Keyed(keyed) => keyed.non_empty = false,
            TArray::List(list) => list.non_empty = false,
        }
        return TUnion::from_atomic(TAtomic::Array(array));
    }
    if !union_comparator::can_expression_types_be_identical(
        codebase, &key, &other_key, false, false,
    ) {
        return get_empty_keyed_array();
    }
    // ArrayType::withTypes(), a list only when every other array is one (AccessoryArrayListType)
    if matches!(array, TArray::List(_)) && others.iter().all(|o| matches!(o, TArray::List(_))) {
        return TUnion::from_atomic(TAtomic::Array(TArray::List(TList::new(Arc::new(value)))));
    }
    general(other_key, value, false)
}
