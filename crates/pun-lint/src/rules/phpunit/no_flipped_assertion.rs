//! `lint/phpunit/no-flipped-assertion`, ported from rector-phpunit's `FlipAssertRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertSame($result, 'expected')` becomes `assertSame('expected', $result)`: when only the second argument is an
//! expected value (a scalar, a constant or a class constant, negated or not, an enum case's `->value` or `->name`, a
//! `new` or a concatenation of such values, or an array of them), or the second is an array of such values or
//! variables and the first is a call, a property or an item read, the first two arguments swap. Named arguments
//! stay. Likewise for `assertNotSame`, `assertEquals`, `assertNotEquals`, `assertStringContainsString`, and the
//! `Canonicalizing`, `IgnoringCase` and `WithDelta` forms of `assertEquals` and `assertNotEquals`.

use mago_syntax::cst::{
    Access, Argument, ArrayElement, BinaryOperator, Call, ClassLikeConstantSelector,
    ClassLikeMemberSelector, CompositeString, Expression, Node, NodeKind, TokenSeparatedSequence,
    UnaryPrefixOperator,
};
use pun_problem::Safety;

use super::{AnyCall, NewArg, is_test_case_call_named, old_args, rewrite_args};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const METHOD_NAMES: &[&str] = &[
    "assertSame",
    "assertNotSame",
    "assertNotEquals",
    "assertEquals",
    "assertStringContainsString",
    "assertEqualsCanonicalizing",
    "assertNotEqualsCanonicalizing",
    "assertEqualsIgnoringCase",
    "assertNotEqualsIgnoringCase",
    "assertEqualsWithDelta",
    "assertNotEqualsWithDelta",
];

pub struct NoFlippedAssertion;

impl Rule for NoFlippedAssertion {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-flipped-assertion"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        if !is_test_case_call_named(ctx, ancestors, &call, METHOD_NAMES) {
            return;
        }
        let args = call.args();
        let (Some(first), Some(second)) = (args.first().copied(), args.get(1).copied()) else {
            return;
        };
        // named args keep their meaning regardless of position
        if matches!(first, Argument::Named(_)) || matches!(second, Argument::Named(_)) {
            return;
        }
        if !is_flip_needed(first.value(), second.value()) {
            return;
        }
        out.push(Report::fixed(
            call.span,
            "Put the expected value first.",
            Safety::Safe,
            rewrite_args(
                ctx,
                &call,
                None,
                &[NewArg::Old(1), NewArg::Old(0)]
                    .into_iter()
                    .chain(old_args(&call, 2))
                    .collect::<Vec<_>>(),
            ),
        ));
    }
}

/// `isFlipNeeded`.
fn is_flip_needed(first: &Expression<'_>, second: &Expression<'_>) -> bool {
    // correct location
    if is_expected_value(first, false) {
        return false;
    }
    if is_expected_value(second, false) {
        return true;
    }
    // e.g. assertSame($obj->getValues(), [$a, $b])
    array_elements(second).is_some_and(|items| is_literal_array(items, true))
        && is_computed_expr(first)
}

/// `isExpectedValue`: with `allow_variables`, a variable too (as an array's key or value, not inside a concatenation
/// or a `new`).
fn is_expected_value(expr: &Expression<'_>, allow_variables: bool) -> bool {
    let expr = unparenthesize(expr);
    if is_scalar_value(expr) || is_enum_case_value(expr) {
        return true;
    }
    match expr {
        Expression::Variable(_) => allow_variables,
        // `isNewWithLiteralArgs`: a written class name, its arguments expected values, none named or unpacked
        Expression::Instantiation(new) => {
            matches!(
                new.class,
                Expression::Identifier(_)
                    | Expression::Self_(_)
                    | Expression::Static(_)
                    | Expression::Parent(_)
            ) && new.argument_list.as_ref().is_none_or(|list| {
                list.arguments.iter().all(|a| match a {
                    Argument::Positional(p) => {
                        p.ellipsis.is_none() && is_expected_value(p.value, false)
                    }
                    Argument::Named(_) => false,
                })
            })
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
            is_expected_value(b.lhs, false) && is_expected_value(b.rhs, false)
        }
        _ => array_elements(expr).is_some_and(|items| is_literal_array(items, allow_variables)),
    }
}

/// The items of a php-parser `Array_` (`[...]` or `array(...)`).
fn array_elements<'a>(
    expr: &'a Expression<'a>,
) -> Option<&'a TokenSeparatedSequence<'a, ArrayElement<'a>>> {
    match unparenthesize(expr) {
        Expression::Array(a) => Some(&a.elements),
        Expression::LegacyArray(a) => Some(&a.elements),
        _ => None,
    }
}

/// `isLiteralArray`: no item unpacked, each key and value an expected value.
fn is_literal_array(
    items: &TokenSeparatedSequence<'_, ArrayElement<'_>>,
    allow_variables: bool,
) -> bool {
    items.iter().all(|item| match item {
        ArrayElement::KeyValue(kv) => {
            is_expected_value(kv.key, allow_variables)
                && is_expected_value(kv.value, allow_variables)
        }
        ArrayElement::Value(v) => is_expected_value(v.value, allow_variables),
        ArrayElement::Variadic(_) | ArrayElement::Missing(_) => false,
    })
}

/// `isComputedExpr`: a call, a property fetch (nullsafe or static) or an array item read.
fn is_computed_expr(expr: &Expression<'_>) -> bool {
    matches!(
        unparenthesize(expr),
        Expression::Call(
            Call::Method(_) | Call::StaticMethod(_) | Call::Function(_) | Call::NullSafeMethod(_)
        ) | Expression::Access(
            Access::Property(_) | Access::NullSafeProperty(_) | Access::StaticProperty(_)
        ) | Expression::ArrayAccess(_)
    )
}

/// `isEnumCaseValue`: `Status::Active->value` or `->name` (the names in any letter case), on a class constant other
/// than `::class`.
fn is_enum_case_value(expr: &Expression<'_>) -> bool {
    let Expression::Access(Access::Property(fetch)) = expr else {
        return false;
    };
    let ClassLikeMemberSelector::Identifier(name) = &fetch.property else {
        return false;
    };
    if !name.value.eq_ignore_ascii_case(b"value") && !name.value.eq_ignore_ascii_case(b"name") {
        return false;
    }
    match unparenthesize(fetch.object) {
        Expression::Access(Access::ClassConstant(c)) => {
            !matches!(&c.constant, ClassLikeConstantSelector::Identifier(i) if i.value.eq_ignore_ascii_case(b"class"))
        }
        _ => false,
    }
}

/// `isScalarValue`: a php-parser `Scalar` (a literal, a magic constant, an interpolated string), a constant or a
/// class constant, optionally negated (`-1`, `-self::LIMIT`).
fn is_scalar_value(expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Negation(_)) => {
            is_scalar_value(u.operand)
        }
        Expression::Literal(_)
        | Expression::MagicConstant(_)
        | Expression::CompositeString(
            CompositeString::Interpolated(_) | CompositeString::Document(_),
        )
        | Expression::ConstantAccess(_)
        | Expression::Access(Access::ClassConstant(_)) => true,
        _ => false,
    }
}
