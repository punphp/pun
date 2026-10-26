//! Level 2's conversions: EchoRule (`echo.nonString`), PrintRule (`print.nonString`),
//! InvalidPartOfEncapsedStringRule (`encapsedStringPart.nonString`), InvalidCastRule (`cast.*`) and
//! InvalidUnaryOperationRule (`unaryOp.invalid`): a value PHP cannot convert, by `findTypeToCheck()`.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::{Cast, Level, StrictMixed, Type, cast_is_error, is_super_type_of};

use super::super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &[
    "echo.nonString",
    "print.nonString",
    "encapsedStringPart.nonString",
    "cast.int",
    "cast.bool",
    "cast.double",
    "cast.string",
    "unaryOp.invalid",
];

pub(super) fn part<'s, 'arena>(p: &'s StringPart<'arena>) -> Option<&'s Expression<'arena>> {
    match p {
        StringPart::Literal(_) => None,
        StringPart::Expression(e) => Some(e),
        StringPart::BracedExpression(b) => Some(b.expression),
    }
}

/// `findTypeToCheck()` with the conversion as the union criteria: the type to report, `None` when fine.
fn invalid(c: &Checker, e: &Expression, cast: Cast) -> Option<Type> {
    let sm = StrictMixed::default();
    declared::with_classes(c, || {
        let found = c.find_type_to_check(e, &|t| !cast_is_error(t, cast, sm)).ty?;
        cast_is_error(&found, cast, sm).then_some(found)
    })
}

fn value(c: &Checker, t: &Type) -> String {
    declared::with_classes(c, || t.describe(Level::Value))
}

pub(in super::super) fn echo(c: &mut Checker, e: &Echo) {
    if c.level < 2 {
        return;
    }
    for (i, v) in e.values.iter().enumerate() {
        if let Some(t) = invalid(c, v, Cast::String) {
            let message = format!(
                "Parameter #{} ({}) of echo cannot be converted to string.",
                i + 1,
                value(c, &t)
            );
            c.report(*v, "echo.nonString", message, None);
        }
    }
}

pub(in super::super) fn print(c: &mut Checker, p: &PrintConstruct) {
    if c.level < 2 {
        return;
    }
    if let Some(t) = invalid(c, p.value, Cast::String) {
        let message = format!("Parameter {} of print cannot be converted to string.", value(c, &t));
        c.report(p.value, "print.nonString", message, None);
    }
}

pub(in super::super) fn interpolated(c: &mut Checker, parts: &[StringPart]) {
    if c.level < 2 {
        return;
    }
    for e in parts.iter().filter_map(part) {
        if let Some(t) = invalid(c, e, Cast::String) {
            // assistant's call: the part as written stands in for php-parser's printer
            let printed = String::from_utf8_lossy(&c.file.contents[e.span().to_range_usize()]).into_owned();
            let message = format!(
                "Part {printed} ({}) of encapsed string cannot be cast to string.",
                value(c, &t)
            );
            c.report(e, "encapsedStringPart.nonString", message, None);
        }
    }
}

/// The conversion a cast or unary operator makes, with its name in messages.
pub(super) fn operation(op: &UnaryPrefixOperator) -> Option<(Cast, &'static str)> {
    Some(match op {
        UnaryPrefixOperator::IntCast(..) | UnaryPrefixOperator::IntegerCast(..) => (Cast::Integer, "int"),
        UnaryPrefixOperator::BoolCast(..) | UnaryPrefixOperator::BooleanCast(..) => (Cast::Boolean, "bool"),
        UnaryPrefixOperator::DoubleCast(..)
        | UnaryPrefixOperator::RealCast(..)
        | UnaryPrefixOperator::FloatCast(..) => (Cast::Float, "double"),
        UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..) => (Cast::String, "string"),
        UnaryPrefixOperator::Plus(_) => (Cast::Number, "+"),
        UnaryPrefixOperator::Negation(_) => (Cast::Number, "-"),
        UnaryPrefixOperator::BitwiseNot(_) => (Cast::BitwiseNot, "~"),
        _ => return None,
    })
}

pub(in super::super) fn unary(c: &mut Checker, u: &UnaryPrefix) {
    if c.level < 2 {
        return;
    }
    let Some((cast, name)) = operation(&u.operator) else {
        return;
    };
    let unary = matches!(cast, Cast::Number | Cast::BitwiseNot);
    let sm = StrictMixed::default();
    let error = declared::with_classes(c, || {
        let Some(t) = c.find_type_to_check(u.operand, &|t| !cast_is_error(t, cast, sm)).ty else {
            return false;
        };
        // BcMathNumber/GmpUnaryOperatorTypeSpecifyingExtension
        let extension = |class: &str| is_super_type_of(&Type::Object(class.into(), vec![]), &t).yes();
        if unary && (extension("GMP") || (cast == Cast::Number && extension("BcMath\\Number"))) {
            return false;
        }
        cast_is_error(&t, cast, sm)
    });
    // (the operand's type is known when it is an error)
    let Some(full) = c.phpstan_type(u.operand).filter(|_| error) else {
        return;
    };
    let described = value(c, &full);
    if unary {
        let message = format!("Unary operation \"{name}\" on {described} results in an error.");
        c.report(u.operand, "unaryOp.invalid", message, None);
    } else {
        let short = if name == "double" { "float" } else { name };
        let message = format!("Cannot cast {described} to {short}.");
        let identifier = IDENTIFIERS
            .iter()
            .find(|i| i.strip_prefix("cast.") == Some(name))
            .expect("an identifier per cast");
        c.report(u, identifier, message, None);
    }
}
