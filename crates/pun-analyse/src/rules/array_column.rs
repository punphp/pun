//! PHPStan 2.3.5's ArrayColumnFunctionReturnTypeExtension and ArrayColumnHelper for rows that are arrays or `mixed`.
//! Rows that are objects keep Mago's ArrayColumnProvider, and every call this port declines keeps the stub's type.

use std::collections::BTreeMap;
use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::libraries::stdlib::array::ArrayColumnProvider;
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::KnownItems;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    add_union_type, get_array_parameters, get_arraykey, get_empty_keyed_array, get_int,
    get_literal_int, get_literal_string, get_mixed,
};
use pun_phpstan::types::Tri;

static META: ProviderMeta = ProviderMeta::new(
    "pun-array-column",
    "pun",
    "PHPStan's array_column() return type",
);

pub(super) struct ArrayColumn;

impl Provider for ArrayColumn {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for ArrayColumn {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"array_column")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let types = |context: &ProviderContext<'_, '_, '_>| {
            let array = context.get_expression_type(invocation.get_argument(0, &[b"array"])?)?;
            let column =
                context.get_expression_type(invocation.get_argument(1, &[b"column_key"])?)?;
            let index = match invocation.get_argument(2, &[b"index_key"]) {
                Some(index) => Some(context.get_expression_type(index)?),
                None => None,
            };
            Some((array.clone(), column.clone(), index.cloned()))
        };
        let (array, column, index) = types(context)?;
        return_type(&array, &column, index.as_ref(), context.codebase())
            .or_else(|| ArrayColumnProvider.get_return_type(context, invocation))
    }
}

/// `getTypeFromFunctionCall()`: one constant array goes through `handleConstantArray()`, which declines when a row
/// may lack the column; everything else through `handleAnyArray()`.
fn return_type(
    array: &TUnion,
    column: &TUnion,
    index: Option<&TUnion>,
    codebase: &CodebaseMetadata,
) -> Option<TUnion> {
    let array = array.get_single_array()?;
    let index = index.filter(|index| !index.is_null());
    if let Some(constant) = constant_array(array, column, index) {
        return Some(constant);
    }
    any_array(array, column, index, codebase)
}

/// The known entries of an array and its remaining key and value types.
type Entries = (BTreeMap<ArrayKey, (bool, TUnion)>, Option<(TUnion, TUnion)>);

fn entries(array: &TArray) -> Entries {
    match array {
        TArray::Keyed(keyed) => (
            keyed
                .known_items
                .as_deref()
                .map(|items| items.iter().map(|(k, v)| (*k, v.clone())).collect())
                .unwrap_or_default(),
            keyed
                .parameters
                .as_ref()
                .map(|(k, v)| ((**k).clone(), (**v).clone())),
        ),
        TArray::List(list) => (
            list.known_elements
                .as_deref()
                .into_iter()
                .flatten()
                .map(|(i, entry)| {
                    (
                        ArrayKey::Integer(i64::try_from(*i).unwrap_or(i64::MAX)),
                        entry.clone(),
                    )
                })
                .collect(),
            (!list.element_type.is_never()).then(|| (get_int(), (*list.element_type).clone())),
        ),
    }
}

/// `handleConstantArray()`: the column of each row, in order or under the row's constant index key. A row that may
/// lack the column, an optional row, an index key that is no single constant and an unsealed array are left to
/// `handleAnyArray()`.
fn constant_array(array: &TArray, column: &TUnion, index: Option<&TUnion>) -> Option<TUnion> {
    let (known, rest) = entries(array);
    if known.is_empty() || rest.is_some() {
        return None;
    }
    let mut items = KnownItems::default();
    let mut next = 0;
    for (optional, row) in known.values() {
        let (value, certainty) = offset(row, column)?;
        if *optional || certainty != Tri::Yes {
            return None;
        }
        if value.is_never() {
            continue;
        }
        let key = match index {
            Some(index) => Some(offset(row, index)?.0).filter(|key| !key.is_never()),
            None => None,
        };
        let key = match key {
            Some(key) if key.is_single() => key.get_single().to_array_key()?,
            Some(_) => return None,
            None => ArrayKey::Integer(next),
        };
        if let ArrayKey::Integer(taken) = key {
            next = next.max(taken.checked_add(1)?);
        }
        items.insert(key, (false, value));
    }
    if items.is_empty() {
        return Some(get_empty_keyed_array());
    }
    let mut keyed = TKeyedArray::new().with_known_items(items);
    keyed.non_empty = true;
    Some(TUnion::from_atomic(TAtomic::Array(TArray::Keyed(keyed))))
}

/// `handleAnyArray()`.
fn any_array(
    array: &TArray,
    column: &TUnion,
    index: Option<&TUnion>,
    codebase: &CodebaseMetadata,
) -> Option<TUnion> {
    if array.is_empty() {
        return Some(get_empty_keyed_array());
    }
    let row = get_array_parameters(array, codebase).1;
    let (value, certainty) = offset(&row, column)?;
    if value.is_never() {
        return Some(get_empty_keyed_array());
    }
    let non_empty = array.is_non_empty() && certainty == Tri::Yes;
    let Some(index) = index else {
        let mut list = TList::new(Arc::new(value));
        list.non_empty = non_empty;
        return Some(TUnion::from_atomic(TAtomic::Array(TArray::List(list))));
    };
    // getReturnIndexType()
    let (key, certainty) = offset(&row, index)?;
    let key = if key.is_never() {
        get_int()
    } else if certainty == Tri::Yes {
        to_array_key(&key, codebase)?
    } else {
        add_union_type(
            to_array_key(&key, codebase)?,
            &get_int(),
            codebase,
            CombinerOptions::default(),
        )
    };
    let mut keyed = TKeyedArray::new_with_parameters(Arc::new(key), Arc::new(value));
    keyed.non_empty = non_empty;
    Some(TUnion::from_atomic(TAtomic::Array(TArray::Keyed(keyed))))
}

/// `getOffsetOrProperty()` of a row type: the type read at `offset` and whether every row has it. `None` for a row
/// this port does not model (an object, a string).
fn offset(row: &TUnion, offset: &TUnion) -> Option<(TUnion, Tri)> {
    if offset.is_null() {
        return Some((row.clone(), Tri::Yes));
    }
    let wanted = Wanted::all(offset)?;
    let mut found: Option<TUnion> = None;
    let mut certainties = Vec::new();
    for atomic in row.types.iter() {
        let (value, certainty) = match atomic {
            TAtomic::Mixed(_) => (Some(get_mixed()), Tri::Maybe),
            TAtomic::Array(array) => union_offset(array, &wanted),
            // neither property access nor offset access
            TAtomic::Null
            | TAtomic::Scalar(TScalar::Integer(_) | TScalar::Float(_) | TScalar::Bool(_)) => {
                (None, Tri::No)
            }
            _ => return None,
        };
        certainties.push(certainty);
        if let Some(value) = value {
            found = Some(match found {
                Some(found) => merge(found, &value),
                None => value,
            });
        }
    }
    let Some(found) = found else {
        return Some((mago_codex::ttype::get_never(), Tri::Yes));
    };
    let certainty = if certainties.iter().all(|c| *c == Tri::Yes) {
        Tri::Yes
    } else {
        Tri::Maybe
    };
    Some((found, certainty))
}

fn merge(left: TUnion, right: &TUnion) -> TUnion {
    let mut atomics = left.types.into_owned();
    for atomic in right.types.iter() {
        if !atomics.contains(atomic) {
            atomics.push(atomic.clone());
        }
    }
    TUnion::from_vec(atomics)
}

/// The column or index argument: one constant key, or any key of some kinds.
enum Wanted {
    Key(ArrayKey),
    Kinds { int: bool, string: bool },
}

impl Wanted {
    /// The members of a union of constant keys one by one, any other offset as a whole.
    fn all(offset: &TUnion) -> Option<Vec<Self>> {
        let constant = |atomic: &TAtomic| match atomic {
            TAtomic::Scalar(TScalar::Integer(integer)) => integer.is_literal(),
            TAtomic::Scalar(TScalar::String(text)) => text.is_known_literal(),
            _ => false,
        };
        if offset.types.len() > 1 && offset.types.iter().all(constant) {
            return offset
                .types
                .iter()
                .map(|atomic| Self::of(&TUnion::from_atomic(atomic.clone())))
                .collect();
        }
        Some(vec![Self::of(offset)?])
    }

    fn of(offset: &TUnion) -> Option<Self> {
        if let Some(value) = offset.get_single_literal_int_value() {
            return Some(Self::Key(ArrayKey::Integer(value)));
        }
        if let Some(value) = offset.get_single_literal_string_value() {
            // a numeric string is cast to an integer key
            if value
                .first()
                .is_some_and(|b| b.is_ascii_digit() || *b == b'-')
            {
                return None;
            }
            return Some(Self::Key(ArrayKey::String(mago_word::word(value))));
        }
        let (mut int, mut string) = (false, false);
        for atomic in offset.types.iter() {
            match atomic {
                TAtomic::Scalar(TScalar::Integer(integer)) if !integer.is_literal() => int = true,
                TAtomic::Scalar(TScalar::String(text)) if !text.is_known_literal() => {
                    string = true;
                }
                TAtomic::Scalar(TScalar::ArrayKey) => (int, string) = (true, true),
                _ => return None,
            }
        }
        Some(Self::Kinds { int, string })
    }

    fn matches(&self, key: &ArrayKey) -> bool {
        match (self, key) {
            (Self::Key(wanted), key) => wanted == key,
            (Self::Kinds { int, .. }, ArrayKey::Integer(_)) => *int,
            (Self::Kinds { string, .. }, _) => *string,
        }
    }

    /// Whether a key of the array's general key type can be this offset.
    fn within(&self, keys: &TUnion) -> bool {
        let (int, string) = match self {
            Self::Key(ArrayKey::Integer(_)) => (true, false),
            Self::Key(_) => (false, true),
            Self::Kinds { int, string } => (*int, *string),
        };
        keys.types.iter().any(|atomic| match atomic {
            TAtomic::Scalar(TScalar::Integer(_)) => int,
            TAtomic::Scalar(TScalar::String(_) | TScalar::ClassLikeString(_)) => string,
            _ => true,
        })
    }
}

/// `hasOffsetValueType()` and `getOffsetValueType()` of one array for a union offset: the values of every member,
/// and `TrinaryLogic::extremeIdentity()` of their certainties.
fn union_offset(array: &TArray, wanted: &[Wanted]) -> (Option<TUnion>, Tri) {
    let mut found: Option<TUnion> = None;
    let mut certainty: Option<Tri> = None;
    for wanted in wanted {
        let (value, one) = array_offset(array, wanted);
        certainty = Some(match certainty {
            Some(certainty) if certainty != one => Tri::Maybe,
            _ => one,
        });
        if let Some(value) = value {
            found = Some(match found {
                Some(found) => merge(found, &value),
                None => value,
            });
        }
    }
    (found, certainty.unwrap_or(Tri::No))
}

/// `hasOffsetValueType()` and `getOffsetValueType()` of one array.
fn array_offset(array: &TArray, wanted: &Wanted) -> (Option<TUnion>, Tri) {
    let (known, rest) = entries(array);
    let mut found: Option<TUnion> = None;
    let mut certainty = Tri::Maybe;
    for (key, (optional, value)) in &known {
        if !wanted.matches(key) {
            continue;
        }
        if matches!(wanted, Wanted::Key(_)) && !optional {
            certainty = Tri::Yes;
        }
        found = Some(match found {
            Some(found) => merge(found, value),
            None => value.clone(),
        });
    }
    if certainty != Tri::Yes
        && let Some((keys, value)) = &rest
        && wanted.within(keys)
    {
        found = Some(match found {
            Some(found) => merge(found, value),
            None => value.clone(),
        });
    }
    match found {
        Some(found) => (Some(found), certainty),
        None => (None, Tri::No),
    }
}

/// `castToArrayKeyType()` on PHP 8: `Type::toArrayKey()` of everything but arrays.
fn to_array_key(key: &TUnion, codebase: &CodebaseMetadata) -> Option<TUnion> {
    let mut result: Option<TUnion> = None;
    for atomic in key.types.iter() {
        let cast = match atomic {
            TAtomic::Array(_) => continue,
            TAtomic::Scalar(
                TScalar::Integer(_)
                | TScalar::String(_)
                | TScalar::ClassLikeString(_)
                | TScalar::ArrayKey,
            ) => TUnion::from_atomic(atomic.clone()),
            TAtomic::Scalar(TScalar::Float(_)) => get_int(),
            TAtomic::Scalar(TScalar::Bool(bool)) if bool.is_true() => get_literal_int(1),
            TAtomic::Scalar(TScalar::Bool(bool)) if bool.is_false() => get_literal_int(0),
            TAtomic::Scalar(TScalar::Bool(_)) => merge(get_literal_int(0), &get_literal_int(1)),
            TAtomic::Null => get_literal_string(mago_word::word(b"")),
            TAtomic::Mixed(_) => get_arraykey(),
            _ => return None,
        };
        result = Some(match result {
            Some(result) => add_union_type(result, &cast, codebase, CombinerOptions::default()),
            None => cast,
        });
    }
    result
}
