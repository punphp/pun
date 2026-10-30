//! Mago 1.57.0's constant folding (mago-linter src/rule/utils/const_value.rs), function by function: the value of an
//! expression made of literals, constants, class constants and operators on them, or `None` when it is not constant.
//! Mago resolves constant and class names with `mago-names`; here [`resolve_class_name`] and
//! [`resolve_constant_name`] do that from the file's `use` imports and namespace.

use std::cmp::Ordering;

use mago_syntax::cst::{
    Access, BinaryOperator, ClassConstantAccess, ClassLikeConstantSelector, Expression, Literal,
    Node, UnaryPrefix, UnaryPrefixOperator,
};

use super::rector_batch5::{resolve_class_name, resolve_constant_name};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantValue {
    Integer(i64),
    Float(f64),
    String(Vec<u8>),
    Boolean(bool),
    Null,
    Constant(String),
    ClassConstant(String, Vec<u8>),
}

impl ConstantValue {
    /// Mago compares values with `OrderedFloat`, where NaN equals NaN.
    pub(crate) fn same(&self, other: &ConstantValue) -> bool {
        match (self, other) {
            (ConstantValue::Float(a), ConstantValue::Float(b)) => {
                a == b || (a.is_nan() && b.is_nan())
            }
            _ => self == other,
        }
    }
}

#[derive(Clone, Copy)]
enum Number {
    Integer(i64),
    Float(f64),
}

pub(crate) fn get_constant_value(
    ancestors: &[Node<'_, '_>],
    expression: &Expression<'_>,
) -> Option<ConstantValue> {
    match expression {
        Expression::Parenthesized(parenthesized) => {
            get_constant_value(ancestors, parenthesized.expression)
        }
        Expression::Literal(literal) => get_literal_value(literal),
        Expression::UnaryPrefix(unary) => get_unary_value(ancestors, unary),
        Expression::Binary(binary) => {
            let left = get_constant_value(ancestors, binary.lhs)?;
            let right = get_constant_value(ancestors, binary.rhs)?;
            get_binary_value(&binary.operator, left, right)
        }
        Expression::ConstantAccess(access) => Some(ConstantValue::Constant(resolve_constant_name(
            ancestors,
            &access.name,
        ))),
        Expression::Access(Access::ClassConstant(access)) => {
            get_class_constant_value(ancestors, access)
        }
        _ => None,
    }
}

fn get_literal_value(literal: &Literal<'_>) -> Option<ConstantValue> {
    match literal {
        Literal::Integer(integer) => integer
            .value
            .map(|value| ConstantValue::Integer(value as i64)),
        Literal::Float(float) => Some(ConstantValue::Float(float.value.0)),
        Literal::String(string) => string.value.map(|v| ConstantValue::String(v.to_vec())),
        Literal::True(_) => Some(ConstantValue::Boolean(true)),
        Literal::False(_) => Some(ConstantValue::Boolean(false)),
        Literal::Null(_) => Some(ConstantValue::Null),
    }
}

fn get_unary_value(ancestors: &[Node<'_, '_>], unary: &UnaryPrefix<'_>) -> Option<ConstantValue> {
    let operand = get_constant_value(ancestors, unary.operand)?;
    match unary.operator {
        UnaryPrefixOperator::Negation(_) => match operand.to_number()? {
            Number::Integer(value) => Some(match value.checked_neg() {
                Some(value) => ConstantValue::Integer(value),
                None => ConstantValue::Float(-(value as f64)),
            }),
            Number::Float(value) => number_to_value(Number::Float(-value)),
        },
        UnaryPrefixOperator::Plus(_) => number_to_value(operand.to_number()?),
        UnaryPrefixOperator::Not(_) => Some(ConstantValue::Boolean(!operand.to_boolean()?)),
        UnaryPrefixOperator::BitwiseNot(_) => Some(ConstantValue::Integer(!operand.to_integer()?)),
        UnaryPrefixOperator::BoolCast(..) | UnaryPrefixOperator::BooleanCast(..) => {
            Some(ConstantValue::Boolean(operand.to_boolean()?))
        }
        UnaryPrefixOperator::IntCast(..) | UnaryPrefixOperator::IntegerCast(..) => {
            Some(ConstantValue::Integer(operand.to_integer()?))
        }
        UnaryPrefixOperator::FloatCast(..)
        | UnaryPrefixOperator::DoubleCast(..)
        | UnaryPrefixOperator::RealCast(..) => number_to_value(Number::Float(operand.to_float()?)),
        _ => None,
    }
}

fn get_binary_value(
    operator: &BinaryOperator<'_>,
    left: ConstantValue,
    right: ConstantValue,
) -> Option<ConstantValue> {
    match operator {
        BinaryOperator::Addition(_) => number_to_value(add(left.to_number()?, right.to_number()?)),
        BinaryOperator::Subtraction(_) => {
            number_to_value(subtract(left.to_number()?, right.to_number()?))
        }
        BinaryOperator::Multiplication(_) => {
            number_to_value(multiply(left.to_number()?, right.to_number()?))
        }
        BinaryOperator::Division(_) => {
            number_to_value(divide(left.to_number()?, right.to_number()?)?)
        }
        BinaryOperator::Modulo(_) => modulo(left.to_integer()?, right.to_integer()?),
        BinaryOperator::Exponentiation(_) => {
            number_to_value(power(left.to_number()?, right.to_number()?))
        }
        BinaryOperator::BitwiseAnd(_) => Some(ConstantValue::Integer(
            left.to_integer()? & right.to_integer()?,
        )),
        BinaryOperator::BitwiseOr(_) => Some(ConstantValue::Integer(
            left.to_integer()? | right.to_integer()?,
        )),
        BinaryOperator::BitwiseXor(_) => Some(ConstantValue::Integer(
            left.to_integer()? ^ right.to_integer()?,
        )),
        BinaryOperator::LeftShift(_) => shift_left(left.to_integer()?, right.to_integer()?),
        BinaryOperator::RightShift(_) => shift_right(left.to_integer()?, right.to_integer()?),
        BinaryOperator::NullCoalesce(_) => match left {
            ConstantValue::Null => Some(right),
            ConstantValue::Constant(_) | ConstantValue::ClassConstant(..) => None,
            other => Some(other),
        },
        BinaryOperator::Equal(_) => loose_equal(&left, &right).map(ConstantValue::Boolean),
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => {
            loose_equal(&left, &right).map(|value| ConstantValue::Boolean(!value))
        }
        BinaryOperator::Identical(_) => identical(&left, &right).map(ConstantValue::Boolean),
        BinaryOperator::NotIdentical(_) => {
            identical(&left, &right).map(|value| ConstantValue::Boolean(!value))
        }
        BinaryOperator::LessThan(_) => {
            compare(&left, &right).map(|o| ConstantValue::Boolean(o == Ordering::Less))
        }
        BinaryOperator::LessThanOrEqual(_) => {
            compare(&left, &right).map(|o| ConstantValue::Boolean(o != Ordering::Greater))
        }
        BinaryOperator::GreaterThan(_) => {
            compare(&left, &right).map(|o| ConstantValue::Boolean(o == Ordering::Greater))
        }
        BinaryOperator::GreaterThanOrEqual(_) => {
            compare(&left, &right).map(|o| ConstantValue::Boolean(o != Ordering::Less))
        }
        BinaryOperator::Spaceship(_) => compare(&left, &right).map(|order| {
            ConstantValue::Integer(match order {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            })
        }),
        BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => {
            if left.to_boolean()? {
                Some(ConstantValue::Boolean(right.to_boolean()?))
            } else {
                Some(ConstantValue::Boolean(false))
            }
        }
        BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => {
            if left.to_boolean()? {
                Some(ConstantValue::Boolean(true))
            } else {
                Some(ConstantValue::Boolean(right.to_boolean()?))
            }
        }
        BinaryOperator::LowXor(_) => Some(ConstantValue::Boolean(
            left.to_boolean()? ^ right.to_boolean()?,
        )),
        _ => None,
    }
}

fn get_class_constant_value(
    ancestors: &[Node<'_, '_>],
    access: &ClassConstantAccess<'_>,
) -> Option<ConstantValue> {
    let class = match access.class {
        Expression::Identifier(identifier) => resolve_class_name(ancestors, identifier),
        Expression::Self_(keyword) | Expression::Static(keyword) | Expression::Parent(keyword) => {
            String::from_utf8_lossy(keyword.value).into_owned()
        }
        _ => return None,
    };
    let ClassLikeConstantSelector::Identifier(constant) = &access.constant else {
        return None;
    };
    Some(ConstantValue::ClassConstant(class, constant.value.to_vec()))
}

impl ConstantValue {
    fn to_boolean(&self) -> Option<bool> {
        match self {
            ConstantValue::Integer(value) => Some(*value != 0),
            ConstantValue::Float(value) => Some(*value != 0.0),
            ConstantValue::String(value) => Some(!value.is_empty() && value != b"0"),
            ConstantValue::Boolean(value) => Some(*value),
            ConstantValue::Null => Some(false),
            ConstantValue::Constant(_) | ConstantValue::ClassConstant(..) => None,
        }
    }

    fn to_number(&self) -> Option<Number> {
        match self {
            ConstantValue::Integer(value) => Some(Number::Integer(*value)),
            ConstantValue::Float(value) => Some(Number::Float(*value)),
            ConstantValue::Boolean(value) => Some(Number::Integer(i64::from(*value))),
            ConstantValue::Null => Some(Number::Integer(0)),
            _ => None,
        }
    }

    fn to_integer(&self) -> Option<i64> {
        match self.to_number()? {
            Number::Integer(value) => Some(value),
            Number::Float(value) => float_to_integer(value),
        }
    }

    fn to_float(&self) -> Option<f64> {
        match self.to_number()? {
            Number::Integer(value) => Some(value as f64),
            Number::Float(value) => Some(value),
        }
    }

    fn as_numeric(&self) -> Option<Number> {
        match self {
            ConstantValue::Integer(value) => Some(Number::Integer(*value)),
            ConstantValue::Float(value) => Some(Number::Float(*value)),
            _ => None,
        }
    }
}

impl Number {
    fn as_float(self) -> f64 {
        match self {
            Number::Integer(value) => value as f64,
            Number::Float(value) => value,
        }
    }
}

fn add(left: Number, right: Number) -> Number {
    match (left, right) {
        (Number::Integer(l), Number::Integer(r)) => match l.checked_add(r) {
            Some(value) => Number::Integer(value),
            None => Number::Float(l as f64 + r as f64),
        },
        _ => Number::Float(left.as_float() + right.as_float()),
    }
}

fn subtract(left: Number, right: Number) -> Number {
    match (left, right) {
        (Number::Integer(l), Number::Integer(r)) => match l.checked_sub(r) {
            Some(value) => Number::Integer(value),
            None => Number::Float(l as f64 - r as f64),
        },
        _ => Number::Float(left.as_float() - right.as_float()),
    }
}

fn multiply(left: Number, right: Number) -> Number {
    match (left, right) {
        (Number::Integer(l), Number::Integer(r)) => match l.checked_mul(r) {
            Some(value) => Number::Integer(value),
            None => Number::Float(l as f64 * r as f64),
        },
        _ => Number::Float(left.as_float() * right.as_float()),
    }
}

fn divide(left: Number, right: Number) -> Option<Number> {
    if right.as_float() == 0.0 {
        return None;
    }
    match (left, right) {
        // `checked_rem` fails only where `checked_div` does (`i64::MIN / -1`).
        (Number::Integer(l), Number::Integer(r)) if l.checked_rem(r) == Some(0) => {
            Some(Number::Integer(l / r))
        }
        _ => Some(Number::Float(left.as_float() / right.as_float())),
    }
}

fn power(left: Number, right: Number) -> Number {
    if let (Number::Integer(base), Number::Integer(exponent)) = (left, right)
        && let Ok(exponent) = u32::try_from(exponent)
        && let Some(value) = base.checked_pow(exponent)
    {
        return Number::Integer(value);
    }
    Number::Float(left.as_float().powf(right.as_float()))
}

fn modulo(left: i64, right: i64) -> Option<ConstantValue> {
    if right == 0 {
        return None;
    }
    Some(ConstantValue::Integer(left.checked_rem(right).unwrap_or(0)))
}

fn shift_left(left: i64, right: i64) -> Option<ConstantValue> {
    let bits = u32::try_from(right).ok()?;
    left.checked_shl(bits).map(ConstantValue::Integer)
}

fn shift_right(left: i64, right: i64) -> Option<ConstantValue> {
    let bits = u32::try_from(right).ok()?;
    left.checked_shr(bits).map(ConstantValue::Integer)
}

fn named_equal(left: &ConstantValue, right: &ConstantValue) -> Option<Option<bool>> {
    match (left, right) {
        (ConstantValue::Constant(l), ConstantValue::Constant(r)) => Some((l == r).then_some(true)),
        (ConstantValue::ClassConstant(lc, ln), ConstantValue::ClassConstant(rc, rn)) => {
            Some((lc == rc && ln == rn).then_some(true))
        }
        (ConstantValue::Constant(_) | ConstantValue::ClassConstant(..), _)
        | (_, ConstantValue::Constant(_) | ConstantValue::ClassConstant(..)) => Some(None),
        _ => None,
    }
}

fn identical(left: &ConstantValue, right: &ConstantValue) -> Option<bool> {
    match named_equal(left, right) {
        Some(result) => result,
        None => Some(left.same(right)),
    }
}

fn loose_equal(left: &ConstantValue, right: &ConstantValue) -> Option<bool> {
    if let Some(result) = named_equal(left, right) {
        return result;
    }
    if matches!(left, ConstantValue::Boolean(_)) || matches!(right, ConstantValue::Boolean(_)) {
        return Some(left.to_boolean()? == right.to_boolean()?);
    }
    match (left, right) {
        (ConstantValue::Integer(l), ConstantValue::Integer(r)) => Some(l == r),
        (ConstantValue::String(l), ConstantValue::String(r)) => (l == r).then_some(true),
        (ConstantValue::Null, ConstantValue::Null) => Some(true),
        (ConstantValue::Null, _) | (_, ConstantValue::Null) => None,
        _ => Some(left.as_numeric()?.as_float() == right.as_numeric()?.as_float()),
    }
}

fn compare(left: &ConstantValue, right: &ConstantValue) -> Option<Ordering> {
    match (left, right) {
        (ConstantValue::Integer(l), ConstantValue::Integer(r)) => Some(l.cmp(r)),
        _ => left
            .as_numeric()?
            .as_float()
            .partial_cmp(&right.as_numeric()?.as_float()),
    }
}

fn number_to_value(number: Number) -> Option<ConstantValue> {
    match number {
        Number::Integer(value) => Some(ConstantValue::Integer(value)),
        Number::Float(value) => value.is_finite().then_some(ConstantValue::Float(value)),
    }
}

fn float_to_integer(value: f64) -> Option<i64> {
    if value.is_finite() && value >= i64::MIN as f64 && value <= i64::MAX as f64 {
        Some(value as i64)
    } else {
        None
    }
}
