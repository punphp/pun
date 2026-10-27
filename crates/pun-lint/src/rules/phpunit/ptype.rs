//! PHPStan's type classes as rector-phpunit's rules test them (`$type instanceof IntegerType`, `->isString()->yes()`),
//! read from the types Mago gives an expression ([`facts`]).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Expression, Literal};

use crate::Context;
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};

/// An array key of a `ConstantArrayType`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Key {
    Int(i64),
    Str(String),
}

/// The PHPStan class of a type.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PType {
    /// `IntegerType` (a `ConstantIntegerType` when `constant`).
    Integer {
        constant: bool,
    },
    /// `FloatType`.
    Float {
        constant: bool,
    },
    /// `StringType`, with the value of a `ConstantStringType`.
    String {
        constant: Option<String>,
    },
    /// `BooleanType`.
    Boolean,
    /// `ConstantBooleanType`.
    ConstantBoolean,
    /// `ConstantArrayType`: an array whose keys are all known, in order, with their values' types.
    ConstantArray {
        keys: Vec<Key>,
        values: Vec<PType>,
    },
    /// `ArrayType`.
    Array,
    /// `ObjectType` (an enum case too), by class name.
    Object(String),
    Null,
    Mixed,
    Never,
    /// `UnionType`.
    Union,
    /// Any other type.
    Other,
}

impl PType {
    pub(crate) fn is_integer(&self) -> bool {
        matches!(self, PType::Integer { .. })
    }

    pub(crate) fn is_float(&self) -> bool {
        matches!(self, PType::Float { .. })
    }

    pub(crate) fn is_string(&self) -> bool {
        matches!(self, PType::String { .. })
    }
}

/// The PHPStan class of a Mago type.
pub(crate) fn of(t: &TUnion) -> PType {
    let types: Vec<&TAtomic> = t.types.iter().collect();
    match types.as_slice() {
        [] => PType::Never,
        [atomic] => atomic_of(atomic),
        // distinct bools: `bool`, or `true` and `false`, which PHPStan joins into a `BooleanType`
        many if many
            .iter()
            .all(|a| matches!(a, TAtomic::Scalar(s) if s.is_bool())) =>
        {
            PType::Boolean
        }
        _ => PType::Union,
    }
}

/// `TypeCombinator::removeNull` then [`of`].
pub(crate) fn without_null(t: &TUnion) -> PType {
    let mut kept = t.clone();
    let types: Vec<TAtomic> = t
        .types
        .iter()
        .filter(|a| !matches!(a, TAtomic::Null))
        .cloned()
        .collect();
    kept.types = types.into();
    of(&kept)
}

fn atomic_of(atomic: &TAtomic) -> PType {
    match atomic {
        TAtomic::Scalar(TScalar::Integer(i)) => PType::Integer {
            constant: i.is_literal(),
        },
        TAtomic::Scalar(TScalar::Float(f)) => PType::Float {
            constant: f.is_literal(),
        },
        TAtomic::Scalar(TScalar::String(s)) => PType::String {
            constant: s
                .get_known_literal_value()
                .map(|v| String::from_utf8_lossy(v).into_owned()),
        },
        TAtomic::Scalar(TScalar::ClassLikeString(_)) => PType::String { constant: None },
        TAtomic::Scalar(TScalar::Bool(b)) if b.is_general() => PType::Boolean,
        TAtomic::Scalar(TScalar::Bool(_)) => PType::ConstantBoolean,
        TAtomic::Array(TArray::Keyed(k)) if k.parameters.is_none() => {
            let (mut keys, mut values) = (Vec::new(), Vec::new());
            for (key, (_, value)) in k.known_items.iter().flat_map(|m| m.iter()) {
                keys.push(match key {
                    ArrayKey::Integer(i) => Key::Int(*i),
                    ArrayKey::String(s) => Key::Str(s.to_string()),
                    _ => return PType::Array,
                });
                values.push(of(value));
            }
            PType::ConstantArray { keys, values }
        }
        TAtomic::Array(TArray::List(l)) if l.element_type.is_never() => {
            let (mut keys, mut values) = (Vec::new(), Vec::new());
            for (index, (_, value)) in l.known_elements.iter().flat_map(|m| m.iter()) {
                keys.push(Key::Int(*index as i64));
                values.push(of(value));
            }
            PType::ConstantArray { keys, values }
        }
        TAtomic::Array(_) => PType::Array,
        // `getNativeType()`: an anonymous class's object is an `ObjectWithoutClassType`
        TAtomic::Object(TObject::Named(n))
            if n.name.as_bytes().starts_with(b"{anonymous-class:") =>
        {
            PType::Other
        }
        TAtomic::Object(TObject::Named(n)) if n.intersection_types.is_none() => {
            PType::Object(n.name.to_string())
        }
        TAtomic::Object(TObject::Enum(e)) => PType::Object(e.get_name().to_string()),
        TAtomic::Null => PType::Null,
        TAtomic::Mixed(_) => PType::Mixed,
        TAtomic::Never => PType::Never,
        _ => PType::Other,
    }
}

/// The type of a literal, from its syntax (Mago types no expression in code after a `never` call, where PHPStan
/// still does).
fn literal(expr: &Expression<'_>) -> Option<PType> {
    Some(match unparenthesize(expr) {
        // php-parser lexes an integer above `PHP_INT_MAX` as a float
        Expression::Literal(Literal::Integer(i))
            if i.value.is_some_and(|v| v > i64::MAX as u64) =>
        {
            PType::Float { constant: true }
        }
        Expression::Literal(Literal::Integer(_)) => PType::Integer { constant: true },
        Expression::Literal(Literal::Float(_)) => PType::Float { constant: true },
        Expression::Literal(Literal::String(s)) => PType::String {
            constant: s.value.map(|v| String::from_utf8_lossy(v).into_owned()),
        },
        Expression::Literal(Literal::True(_) | Literal::False(_)) => PType::ConstantBoolean,
        Expression::Literal(Literal::Null(_)) => PType::Null,
        _ => return None,
    })
}

/// `NodeTypeResolver::getNativeType`, as a PHPStan class; `None` where Mago has no type.
pub(crate) fn native(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<PType> {
    literal(expr).or_else(|| {
        facts(ctx)
            .native_type(node_span(unparenthesize(expr)))
            .map(of)
    })
}

/// [`native`] with `null` removed from the type.
pub(crate) fn native_without_null(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<PType> {
    // `TypeCombinator::removeNull(null)` is `never`
    literal(expr)
        .map(|t| if t == PType::Null { PType::Never } else { t })
        .or_else(|| {
            facts(ctx)
                .native_type(node_span(unparenthesize(expr)))
                .map(without_null)
        })
}

/// `$type->isInteger()->yes()` on the native type of `expr` without `null`: every part is an integer.
pub(crate) fn native_is_integer(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    native_parts(ctx, expr, |s| s.is_int())
}

/// `$type->isString()->yes()` on the native type of `expr` without `null`: every part is a string.
pub(crate) fn native_is_string(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    native_parts(ctx, expr, |s| s.is_any_string())
}

fn native_parts(ctx: &Context<'_>, expr: &Expression<'_>, part: fn(&TScalar) -> bool) -> bool {
    if let Some(t) = literal(expr) {
        return match t {
            PType::Integer { .. } => part(&TScalar::int()),
            PType::String { .. } => part(&TScalar::string()),
            _ => false,
        };
    }
    let facts = facts(ctx);
    let Some(t) = facts.native_type(node_span(unparenthesize(expr))) else {
        return false;
    };
    let parts: Vec<&TAtomic> = t
        .types
        .iter()
        .filter(|a| !matches!(a, TAtomic::Null))
        .collect();
    !parts.is_empty()
        && parts
            .iter()
            .all(|a| matches!(a, TAtomic::Scalar(s) if part(s)))
}
