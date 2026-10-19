//! PHPStan's native types as Rector maps them to declarations, read from Mago's types (see [`super::facts`]): the
//! kind of each atomic type, and the union `ReturnTypeInferer` builds from a body's returns.

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{Block, Return};

use super::facts::Facts;
use super::silent_void::{Place, has_silent_void};
use crate::syntax::node_span;

/// A native type as a declaration knows it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Kind {
    Int,
    Float,
    String,
    Bool,
    Array,
    Null,
    Void,
    /// An object of a class, by its name as Mago spells it.
    Object {
        name: String,
        is_static: bool,
        is_this: bool,
    },
}

/// The kind of an atomic type, when it is one a declaration can name.
pub(crate) fn kind(atomic: &TAtomic) -> Option<Kind> {
    Some(match atomic {
        TAtomic::Scalar(s) if s.is_int() => Kind::Int,
        TAtomic::Scalar(s) if s.is_float() => Kind::Float,
        TAtomic::Scalar(s) if s.is_any_string() => Kind::String,
        TAtomic::Scalar(s) if s.is_bool() => Kind::Bool,
        TAtomic::Array(_) => Kind::Array,
        TAtomic::Null => Kind::Null,
        TAtomic::Void => Kind::Void,
        TAtomic::Object(TObject::Named(named)) if named.type_parameters.is_none() => Kind::Object {
            name: String::from_utf8_lossy(named.name.as_bytes()).into_owned(),
            is_static: named.is_static,
            is_this: named.is_this,
        },
        TAtomic::Object(TObject::Enum(e)) => Kind::Object {
            name: String::from_utf8_lossy(e.get_name().as_bytes()).into_owned(),
            is_static: false,
            is_this: false,
        },
        _ => return None,
    })
}

/// `ReturnedNodesReturnTypeInfererTypeInferer::inferFunctionLike` as kinds without repeats: each returned value's
/// native type, `void` for a bare `return` and for a reachable end. `None` when there are no returns (Rector's mixed)
/// or a type has no kind.
pub(crate) fn returned_kinds(
    place: &Place<'_, '_, '_>,
    body: &Block<'_>,
    returns: &[&Return<'_>],
) -> Option<Vec<Kind>> {
    if returns.is_empty() {
        return None;
    }
    let mut kinds: Vec<Kind> = Vec::new();
    let push = |k: Kind, kinds: &mut Vec<Kind>| {
        if !kinds.contains(&k) {
            kinds.push(k);
        }
    };
    let mut has_void = false;
    for r in returns {
        let Some(value) = r.value else {
            has_void = true;
            push(Kind::Void, &mut kinds);
            continue;
        };
        let t = native_type(place.facts, value)?;
        for atomic in t.types.iter() {
            push(kind(atomic)?, &mut kinds);
        }
    }
    if !has_void && has_silent_void(place, body.statements.iter()) {
        push(Kind::Void, &mut kinds);
    }
    if kinds == [Kind::Void] {
        return None;
    }
    Some(kinds)
}

/// The native type of an expression, when Mago's analysis reached it.
pub(crate) fn native_type<'f>(
    facts: &'f Facts,
    expr: &mago_syntax::cst::Expression<'_>,
) -> Option<&'f mago_codex::ttype::union::TUnion> {
    facts
        .native_type(node_span(expr))
        .filter(|t| !t.types.is_empty())
}

/// `TypeFactory::createMixedPassedOrUnionTypeAndKeepConstant`'s members, as keys: a constant (a literal value)
/// stays apart from its general type unless that type is there too, so `5` and `int` make `int`, `5` and `6` a
/// union; an integer range is not a constant and stays apart. `None` as for [`returned_kinds`]; `null` is left out.
pub(crate) fn returned_constants(
    place: &Place<'_, '_, '_>,
    returns: &[&Return<'_>],
) -> Option<Vec<String>> {
    // (key, the general type's key for a constant)
    let mut keys: Vec<(String, Option<&str>)> = Vec::new();
    for value in returns.iter().filter_map(|r| r.value) {
        for atomic in native_type(place.facts, value)?.types.iter() {
            let id = || String::from_utf8_lossy(atomic.get_id().as_bytes()).into_owned();
            let key = match atomic {
                TAtomic::Null => continue,
                TAtomic::Array(_) => ("array".to_string(), None),
                TAtomic::Scalar(TScalar::Integer(i)) if i.is_unspecified() => {
                    ("int".to_string(), None)
                }
                TAtomic::Scalar(TScalar::Integer(i)) if i.is_literal() => (id(), Some("int")),
                TAtomic::Scalar(TScalar::Float(f)) if f.is_literal() => (id(), Some("float")),
                TAtomic::Scalar(TScalar::String(s)) => match s.get_known_literal_value() {
                    Some(v) => (format!("'{}'", String::from_utf8_lossy(v)), Some("string")),
                    None => ("string".to_string(), None),
                },
                _ => (id(), None),
            };
            if !keys.contains(&key) {
                keys.push(key);
            }
        }
    }
    let generals: Vec<String> = keys
        .iter()
        .filter(|(_, g)| g.is_none())
        .map(|(k, _)| k.clone())
        .collect();
    Some(
        keys.into_iter()
            .filter(|(_, general)| general.is_none_or(|g| !generals.iter().any(|k| k == g)))
            .map(|(k, _)| k)
            .collect(),
    )
}
