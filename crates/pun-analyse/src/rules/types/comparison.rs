//! InvalidComparisonOperationRule (`equal.invalid`, `smaller.invalid`, ...): a number compared with an object or an
//! array (PHP throws or the result is meaningless).

use mago_syntax::cst::*;
use pun_phpstan::types::{Accessory, Level, Type, is_super_type_of};

use super::super::{Checker, declared};

pub(super) const IDENTIFIERS: &[&str] = &[
    "equal.invalid",
    "notEqual.invalid",
    "greater.invalid",
    "greaterOrEqual.invalid",
    "smaller.invalid",
    "smallerOrEqual.invalid",
    "spaceship.invalid",
];

/// The comparison's identifier prefix and sigil.
pub(super) fn operator(op: &BinaryOperator) -> Option<(&'static str, &'static str)> {
    Some(match op {
        BinaryOperator::Equal(_) => ("equal", "=="),
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => ("notEqual", "!="),
        BinaryOperator::GreaterThan(_) => ("greater", ">"),
        BinaryOperator::GreaterThanOrEqual(_) => ("greaterOrEqual", ">="),
        BinaryOperator::LessThan(_) => ("smaller", "<"),
        BinaryOperator::LessThanOrEqual(_) => ("smallerOrEqual", "<="),
        BinaryOperator::Spaceship(_) => ("spaceship", "<=>"),
        _ => return None,
    })
}

fn number() -> Type {
    Type::Union(vec![Type::Int, Type::Float])
}

fn instance(t: &Type, class: &str) -> bool {
    is_super_type_of(&Type::Object(class.into(), vec![], false), t).yes()
}

/// `isNumberType()`: the expression is (possibly) an int or float, as the level checks it.
fn is_number(c: &Checker, e: &Expression, full: &Type) -> bool {
    let Some(t) = c.find_type_to_check(e, &|t| is_super_type_of(&number(), t).yes()).ty else {
        return false;
    };
    if t != *full {
        return false;
    }
    // `$acceptedType->equals($type->toNumber())`: SimpleXMLElement, GMP and numeric-string cast to `float|int`
    let numeric_string = matches!(&t, Type::Intersection(ts) if ts.contains(&Type::Accessory(Accessory::NumericString)));
    !is_super_type_of(&number(), &t).no() || numeric_string || instance(&t, "SimpleXMLElement") || instance(&t, "GMP")
}

fn nullable_object_or_array(t: &Type) -> bool {
    if t.is_null().yes() {
        return false;
    }
    is_super_type_of(&Type::Union(vec![Type::ObjectWithoutClass, Type::Null]), t).yes()
        || is_super_type_of(
            &Type::Union(vec![Type::array(pun_phpstan::types::mixed(), pun_phpstan::types::mixed()), Type::Null]),
            t,
        )
        .yes()
}

pub(in super::super) fn binary(c: &mut Checker, b: &Binary) {
    if c.level < 2 {
        return;
    }
    let Some((name, sigil)) = operator(&b.operator) else {
        return;
    };
    let (Some(left), Some(right)) = (c.phpstan_type(b.lhs), c.phpstan_type(b.rhs)) else {
        return;
    };
    let bcmath = c.php_version >= 80400;
    let error = declared::with_classes(c, || {
        let l = is_number(c, b.lhs, &left);
        let r = is_number(c, b.rhs, &right);
        if l == r {
            return false;
        }
        // GmpOperatorTypeSpecifyingExtension, BcMathNumberOperatorTypeSpecifyingExtension (an error with a float,
        // which checkExtensionsForComparisonOperators off leaves to the checks below)
        if instance(&left, "GMP") || instance(&right, "GMP") {
            return false;
        }
        let other = if instance(&left, "BcMath\\Number") {
            Some(&right)
        } else if instance(&right, "BcMath\\Number") {
            Some(&left)
        } else {
            None
        };
        if bcmath && other.is_some_and(|o| !o.is_float().yes()) {
            return false;
        }
        (l && nullable_object_or_array(&right)) || (r && nullable_object_or_array(&left))
    });
    if !error {
        return;
    }
    let (l, r) = declared::with_classes(c, || (left.describe(Level::Value), right.describe(Level::Value)));
    let message = format!("Comparison operation \"{sigil}\" between {l} and {r} results in an error.");
    let identifier = IDENTIFIERS
        .iter()
        .find(|i| i.strip_suffix(".invalid") == Some(name))
        .expect("an identifier per operator");
    c.report(b.lhs, identifier, message, None);
}
