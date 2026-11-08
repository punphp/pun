//! Rector 2.8.4's `TypeFactory::createMixedPassedOrUnionTypeAndKeepConstant` (src/NodeTypeResolver/PHPStan/Type/
//! TypeFactory.php) with the `TypeHasher` it keys types by (src/NodeTypeResolver/PHPStan/TypeHasher.php), on Mago's
//! types standing in for PHPStan's, and `ReturnedNodesReturnTypeInfererTypeInferer::inferFunctionLike`, which builds a
//! body's return type with it.
//!
//! A type is an owned [`TAtomic`]. `new ArrayType($key, $item)` is a keyed array with those parameters; PHPStan's
//! `ConstantArrayType` is a sealed Mago array (known items, no parameters).

use std::sync::Arc;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Block, Expression, Return};

use super::add_return_type_from_new::anonymous_parents;
use super::declare::union_order;
use super::native::native_type;
use super::silent_void::{Place, has_silent_void};
use crate::syntax::unparenthesize as unparenthesized;

/// What `createUnionOrSingleType` gives.
#[derive(Debug)]
pub(crate) enum Created {
    /// `MixedType`: no types, or one of them mixed.
    Mixed,
    Single(TAtomic),
    /// `new UnionType($types)`, in order.
    Union(Vec<TAtomic>),
}

/// `createMixedPassedOrUnionTypeAndKeepConstant`: each type flattened (`unwrapUnionedTypes`, a constant array
/// becoming one `array<key, item>` per item type), made unique by hash with constants kept unless their general type
/// is there (`uniquateTypes`), then mixed, the one type, or their union.
pub(crate) fn create_keep_constant(types: Vec<TAtomic>) -> Created {
    let mut unwrapped = Vec::new();
    for t in types {
        match &t {
            TAtomic::Array(a) if is_constant_array(&t) => {
                unwrapped.extend(unwrap_constant_array(a));
            }
            _ => unwrapped.push(t),
        }
    }
    let mut types = uniquate(unwrapped);
    match types.len() {
        1 => Created::Single(types.remove(0)),
        // no types (an empty union), or one of them mixed
        _ if types.is_empty() || types.iter().any(|t| matches!(t, TAtomic::Mixed(_))) => {
            Created::Mixed
        }
        _ => Created::Union(types),
    }
}

/// `uniquateTypes($types, keepConstant: true)`: one type per hash, the last of them in the place of the first; a
/// constant (a literal, `true`, `false`) is hashed by its kind and dropped when its general type is there too, and
/// `true` or `false` after both were seen is `bool` (`normalizeBooleanType`).
fn uniquate(types: Vec<TAtomic>) -> Vec<TAtomic> {
    let mut unique: Vec<(String, TAtomic)> = Vec::new();
    let mut constants: Vec<(String, String)> = Vec::new();
    let (mut has_false, mut has_true) = (false, false);
    let insert = |list: &mut Vec<(String, TAtomic)>, hash: String, t: TAtomic| match list
        .iter_mut()
        .find(|(h, _)| *h == hash)
    {
        Some(entry) => entry.1 = t,
        None => list.push((hash, t)),
    };
    for t in types {
        let is_true = matches!(&t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_true());
        let is_false = matches!(&t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_false());
        has_true |= is_true;
        has_false |= is_false;
        let t = if has_true && has_false && (is_true || is_false) {
            TAtomic::Scalar(TScalar::bool())
        } else {
            t
        };
        match without_value(&t) {
            Some(general) => {
                let (hash, general_hash) = (type_hash(&t), type_hash(&general));
                match constants.iter_mut().find(|(h, _)| *h == hash) {
                    Some(entry) => entry.1 = general_hash,
                    None => constants.push((hash.clone(), general_hash)),
                }
                insert(&mut unique, hash, t);
            }
            None => insert(&mut unique, type_hash(&t), t),
        }
    }
    for (hash, general) in &constants {
        if unique.iter().any(|(h, _)| h == general) {
            unique.retain(|(h, _)| h != hash);
        }
    }
    unique.into_iter().map(|(_, t)| t).collect()
}

/// `removeValueFromConstantType`: the general type of a constant, `None` for any other type.
fn without_value(t: &TAtomic) -> Option<TAtomic> {
    Some(TAtomic::Scalar(match t {
        TAtomic::Scalar(TScalar::Float(f)) if f.is_literal() => TScalar::float(),
        TAtomic::Scalar(TScalar::String(s)) if s.is_known_literal() => TScalar::string(),
        TAtomic::Scalar(TScalar::Integer(i)) if i.is_literal() => TScalar::int(),
        TAtomic::Scalar(TScalar::Bool(b)) if b.is_true() || b.is_false() => TScalar::bool(),
        _ => return None,
    }))
}

/// Whether PHPStan's `isConstantValue()` holds: a literal scalar, `true`, `false` or `null`.
fn is_constant_value(t: &TAtomic) -> bool {
    match t {
        TAtomic::Null => true,
        TAtomic::Scalar(TScalar::Bool(b)) => !b.is_general(),
        TAtomic::Scalar(TScalar::Integer(i)) => i.is_literal(),
        TAtomic::Scalar(TScalar::Float(f)) => f.is_literal(),
        TAtomic::Scalar(TScalar::String(s)) => s.is_known_literal(),
        _ => is_constant_array(t),
    }
}

/// `TypeHasher::createTypeHash`: an array by its item type, key type and item description; an object by its class
/// name (`static` and `$this` too); a constant by its kind; anything else by its description.
fn type_hash(t: &TAtomic) -> String {
    match t {
        TAtomic::Mixed(_) => "mixed".to_string(),
        // a keyed array with parameters is not constant
        TAtomic::Array(TArray::Keyed(TKeyedArray {
            parameters: Some((key, item)),
            non_empty: false,
            ..
        })) => format!(
            "{}{}{}[]",
            union_hash(item),
            union_hash(key),
            describe(item)
        ),
        TAtomic::Object(TObject::Named(n)) if n.type_parameters.is_none() => {
            String::from_utf8_lossy(n.name.as_bytes()).to_ascii_lowercase()
        }
        TAtomic::Object(TObject::Enum(e)) => {
            String::from_utf8_lossy(e.get_name().as_bytes()).to_ascii_lowercase()
        }
        TAtomic::Null => "NullType".to_string(),
        TAtomic::Scalar(TScalar::Bool(b)) if !b.is_general() => "ConstantBooleanType".to_string(),
        TAtomic::Scalar(TScalar::Integer(i)) if i.is_literal() => "ConstantIntegerType".to_string(),
        TAtomic::Scalar(TScalar::Float(f)) if f.is_literal() => "ConstantFloatType".to_string(),
        TAtomic::Scalar(TScalar::String(s)) if s.is_known_literal() => {
            "ConstantStringType".to_string()
        }
        _ if is_constant_value(t) => "ConstantArrayType".to_string(),
        _ => describe_atomic(t),
    }
}

/// The hash of a type that may be a union: a union of constants is hashed by its class, `UnionType`.
fn union_hash(u: &TUnion) -> String {
    match u.types.as_ref() {
        [one] => type_hash(one),
        many if many.iter().all(is_constant_value) => "UnionType".to_string(),
        _ => describe(u),
    }
}

/// A type's description, standing in for PHPStan's `describe()`: equal for equal types.
fn describe(u: &TUnion) -> String {
    let mut ids: Vec<String> = u.types.iter().map(describe_atomic).collect();
    ids.sort();
    ids.join("|")
}

fn describe_atomic(t: &TAtomic) -> String {
    t.get_id().to_string()
}

/// Whether `t` is PHPStan's `ConstantArrayType`: an array whose keys are all known (`[]`, `[1, 'a' => $b]`).
pub(crate) fn is_constant_array(t: &TAtomic) -> bool {
    match t {
        TAtomic::Array(TArray::Keyed(k)) => k.parameters.is_none(),
        TAtomic::Array(TArray::List(l)) => {
            l.known_elements.is_some() && l.element_type.types.iter().all(|t| t.is_never())
        }
        _ => false,
    }
}

/// A constant array's entries: (key, optional, item).
fn constant_entries(t: &TArray) -> Vec<(TAtomic, bool, &TUnion)> {
    match t {
        TArray::Keyed(k) => (k.known_items.iter().flat_map(|i| i.iter()))
            .map(|(key, (optional, item))| (key.to_atomic(), *optional, item))
            .collect(),
        TArray::List(l) => (l.known_elements.iter().flat_map(|e| e.iter()))
            .map(|(i, (optional, item))| {
                (
                    TAtomic::Scalar(TScalar::literal_int(*i as i64)),
                    *optional,
                    item,
                )
            })
            .collect(),
    }
}

/// `unwrapConstantArrayTypes`: the item types of a constant array (its iterable value type, flattened), each as an
/// `array<key, item>` with the key at the same place in its flattened key type (`mixed` past its end); an item that
/// is a constant array is unwrapped too, each of its arrays the item of one array. `[]` is `array<never, never>`.
fn unwrap_constant_array(t: &TArray) -> Vec<TAtomic> {
    let entries = constant_entries(t);
    if entries.is_empty() {
        return vec![array_of(TAtomic::Never, TAtomic::Never)];
    }
    // Mago merges `[]` with a shape into one whose items are all optional, where PHPStan keeps `array{}` apart
    let mut out = Vec::new();
    if entries.iter().all(|(_, optional, _)| *optional) {
        out.push(array_of(TAtomic::Never, TAtomic::Never));
    }
    let keys = combine(entries.iter().map(|(k, _, _)| k.clone()).collect());
    let items = combine(
        entries
            .iter()
            .flat_map(|(_, _, item)| union_order(item.types.as_ref()))
            .cloned()
            .collect(),
    );
    for (at, item) in items.into_iter().enumerate() {
        let key = keys
            .get(at)
            .cloned()
            .unwrap_or(TAtomic::Mixed(Default::default()));
        if let TAtomic::Array(a) = &item
            && is_constant_array(&item)
        {
            for inner in unwrap_constant_array(a) {
                out.push(array_of(key.clone(), inner));
            }
            continue;
        }
        out.push(array_of(key, item));
    }
    out
}

/// `new ArrayType($key, $item)`.
fn array_of(key: TAtomic, item: TAtomic) -> TAtomic {
    TAtomic::Array(TArray::Keyed(TKeyedArray::new_with_parameters(
        Arc::new(TUnion::from_atomic(key)),
        Arc::new(TUnion::from_atomic(item)),
    )))
}

/// `TypeCombinator::union`'s members, for the keys and items of a constant array: `mixed` swallows the rest, repeats
/// go, a literal goes when its general type is there, and `true` with `false` is `bool`.
fn combine(types: Vec<TAtomic>) -> Vec<TAtomic> {
    if let Some(mixed) = types.iter().find(|t| matches!(t, TAtomic::Mixed(_))) {
        return vec![mixed.clone()];
    }
    let has = |f: &dyn Fn(&TAtomic) -> bool| types.iter().any(f);
    let has_true = has(&|t| matches!(t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_true()));
    let has_false = has(&|t| matches!(t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_false()));
    let general_int =
        has(&|t| matches!(t, TAtomic::Scalar(TScalar::Integer(i)) if !i.is_literal()));
    let general_float =
        has(&|t| matches!(t, TAtomic::Scalar(TScalar::Float(f)) if !f.is_literal()));
    let general_string =
        has(&|t| matches!(t, TAtomic::Scalar(TScalar::String(s)) if !s.is_known_literal()));
    let general_bool = has(&|t| matches!(t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_general()));
    let mut out: Vec<TAtomic> = Vec::new();
    for t in types {
        let t = match &t {
            TAtomic::Scalar(TScalar::Bool(b))
                if !b.is_general() && (general_bool || (has_true && has_false)) =>
            {
                TAtomic::Scalar(TScalar::bool())
            }
            TAtomic::Scalar(TScalar::Integer(i)) if i.is_literal() && general_int => continue,
            TAtomic::Scalar(TScalar::Float(f)) if f.is_literal() && general_float => continue,
            TAtomic::Scalar(TScalar::String(s)) if s.is_known_literal() && general_string => {
                continue;
            }
            _ => t,
        };
        if !out.iter().any(|o| o == &t) {
            out.push(t);
        }
    }
    out
}

/// `NodeTypeCorrector::correctType` on a native type that is not a union: a non-empty string is `string`, a
/// non-empty array `array`, and a list loses its list-ness (`array<int, T>`).
fn corrected(t: &TAtomic) -> TAtomic {
    match t {
        TAtomic::Scalar(TScalar::String(s)) if !s.is_known_literal() && s.is_non_empty() => {
            TAtomic::Scalar(TScalar::string())
        }
        TAtomic::Array(TArray::Keyed(k)) if !is_constant_array(t) && k.non_empty => array_of(
            TAtomic::Mixed(Default::default()),
            TAtomic::Mixed(Default::default()),
        ),
        TAtomic::Array(TArray::List(l)) if !is_constant_array(t) => {
            if l.non_empty {
                array_of(
                    TAtomic::Mixed(Default::default()),
                    TAtomic::Mixed(Default::default()),
                )
            } else {
                TAtomic::Array(TArray::Keyed(TKeyedArray::new_with_parameters(
                    Arc::new(TUnion::from_atomic(TAtomic::Scalar(TScalar::int()))),
                    l.element_type.clone(),
                )))
            }
        }
        _ => t.clone(),
    }
}

/// `NodeTypeResolver::getNativeType` of a returned value, as the members PHPStan's union has, in its order: `new
/// class extends A {}` with one parent is that parent (`NewTypeResolver`), and a type that is not a union is
/// corrected ([`corrected`]).
fn returned_native_type(
    place: &Place<'_, '_, '_>,
    codebase: &CodebaseMetadata,
    value: &Expression<'_>,
) -> Option<Vec<TAtomic>> {
    if let Expression::AnonymousClass(class) = unparenthesized(value)
        && let Some(parent) = anonymous_parents(place.ancestors, class).first()
    {
        let class = codebase.get_class_like(parent.as_bytes())?;
        return Some(vec![TAtomic::Object(TObject::Named(TNamedObject::new(
            class.name,
        )))]);
    }
    let t = native_type(place.facts, value)?;
    Some(match t.types.as_ref() {
        [one] => vec![corrected(one)],
        many => merged_arrays(union_order(many).into_iter().cloned().collect()),
    })
}

/// `ReturnedNodesReturnTypeInfererTypeInferer::inferFunctionLike`: the type of each `return`'s value (`void` for a
/// bare `return`, and for an end the body can reach), made one type by [`create_keep_constant`]. `Mixed` without
/// returns, for only `void`, and where Mago has no type for a value.
pub(crate) fn infer_returns(
    place: &Place<'_, '_, '_>,
    body: &Block<'_>,
    returns: &[&Return<'_>],
) -> Created {
    if returns.is_empty() {
        return Created::Mixed;
    }
    let codebase = &place.facts.codebase;
    let mut types = Vec::new();
    let mut has_void = false;
    for r in returns {
        match r.value {
            None => {
                has_void = true;
                types.push(TAtomic::Void);
            }
            Some(value) => match returned_native_type(place, codebase, value) {
                Some(t) => types.extend(t),
                None => return Created::Mixed,
            },
        }
    }
    if !has_void && has_silent_void(place, body.statements.iter()) {
        types.push(TAtomic::Void);
    }
    match create_keep_constant(types) {
        Created::Single(TAtomic::Void) => Created::Mixed,
        created => created,
    }
}

/// Whether `TypeCombinator::removeNull` leaves a `UnionType`: two or more members other than `null` (`UnionType::
/// tryRemove` keeps the rest as they are).
pub(crate) fn is_union_without_null(created: &Created) -> bool {
    matches!(created, Created::Union(types) if types.iter().filter(|t| !matches!(t, TAtomic::Null)).count() > 1)
}

/// The members of a union with each object that another member's class covers dropped, as
/// `TypeCombinator::union` leaves them: `$this` and `static` of a class go with the class, and a subclass with
/// its parent.
pub(crate) fn without_covered_objects<'a>(
    types: Vec<&'a TAtomic>,
    codebase: &CodebaseMetadata,
) -> Vec<&'a TAtomic> {
    let object = |t: &TAtomic| match t {
        TAtomic::Object(TObject::Named(n)) if n.type_parameters.is_none() => Some(n.clone()),
        _ => None,
    };
    let covers = |b: &TNamedObject, a: &TNamedObject| {
        let late = |n: &TNamedObject| n.is_this || n.is_static;
        if late(b) {
            return b.is_static && a.is_this && a.name == b.name;
        }
        codebase.is_instance_of(a.name.as_bytes(), b.name.as_bytes())
            && (late(a) || a.name != b.name)
    };
    let objects: Vec<Option<TNamedObject>> = types.iter().map(|t| object(t)).collect();
    types
        .iter()
        .enumerate()
        .filter(|(i, _)| {
            let Some(a) = &objects[*i] else {
                return true;
            };
            !objects
                .iter()
                .enumerate()
                .any(|(j, b)| j != *i && b.as_ref().is_some_and(|b| covers(b, a)))
        })
        .map(|(_, t)| *t)
        .collect()
}

/// The members of one expression's type with its arrays that are not constant made one array, as
/// `TypeCombinator::union` keeps them: Mago splits `array_map()`'s type into `array<k, v>|list<v>`.
pub(crate) fn merged_arrays(types: Vec<TAtomic>) -> Vec<TAtomic> {
    let general = |t: &TAtomic| matches!(t, TAtomic::Array(_)) && !is_constant_array(t);
    if types.iter().filter(|t| general(t)).count() < 2 {
        return types;
    }
    // the union of non-empty arrays is non-empty (`array_map(...) ?: 1`)
    let non_empty = types.iter().filter(|t| general(t)).all(|t| {
        matches!(
            t,
            TAtomic::Array(TArray::Keyed(TKeyedArray {
                non_empty: true,
                ..
            }))
        ) || matches!(t, TAtomic::Array(TArray::List(l)) if l.non_empty)
    });
    let mut out = Vec::new();
    let mut merged = false;
    for t in types {
        if !general(&t) {
            out.push(t);
        } else if !merged {
            merged = true;
            let mut array = TKeyedArray::new_with_parameters(
                Arc::new(TUnion::from_atomic(TAtomic::Mixed(Default::default()))),
                Arc::new(TUnion::from_atomic(TAtomic::Mixed(Default::default()))),
            );
            array.non_empty = non_empty;
            out.push(TAtomic::Array(TArray::Keyed(array)));
        }
    }
    out
}
