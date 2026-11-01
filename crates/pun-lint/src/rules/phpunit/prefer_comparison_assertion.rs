//! `lint/phpunit/prefer-comparison-assertion`, ported from rector-phpunit's `AssertComparisonToSpecificMethodRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertTrue($a === $b)` becomes `assertSame($b, $a)`, and likewise for `!==`, `==`, `!=`, `<`, `<=`, `>` and `>=`
//! and for `assertFalse`. The expected value goes first: the left operand when its type is a constant (for the
//! equality operators), else the right one.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, expr_text, is_constant_value, is_test_case_call_named, old_args, renamed,
    rewrite_args,
};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct PreferComparisonAssertion;

impl Rule for PreferComparisonAssertion {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-comparison-assertion"
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
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertTrue", "assertFalse"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let Expression::Binary(binary) = unparenthesize(first.value()) else {
            return;
        };
        let (method, not_method, keep_order) = match binary.operator {
            BinaryOperator::Identical(_) => ("assertSame", "assertNotSame", false),
            BinaryOperator::NotIdentical(_) => ("assertNotSame", "assertSame", false),
            BinaryOperator::Equal(_) => ("assertEquals", "assertNotEquals", false),
            BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => {
                ("assertNotEquals", "assertEquals", false)
            }
            BinaryOperator::GreaterThan(_) => ("assertGreaterThan", "assertLessThanOrEqual", true),
            BinaryOperator::LessThan(_) => ("assertLessThan", "assertGreaterThanOrEqual", true),
            BinaryOperator::GreaterThanOrEqual(_) => {
                ("assertGreaterThanOrEqual", "assertLessThan", true)
            }
            BinaryOperator::LessThanOrEqual(_) => {
                ("assertLessThanOrEqual", "assertGreaterThan", true)
            }
            _ => return,
        };
        let Some(name) = renamed(
            &call,
            &[("assertTrue", method), ("assertFalse", not_method)],
        ) else {
            return;
        };
        let (left, right) = (expr_text(ctx, binary.lhs), expr_text(ctx, binary.rhs));
        let (a, b) = if is_constant_value(ctx, binary.lhs) && !keep_order {
            (left, right)
        } else {
            (right, left)
        };
        let args: Vec<NewArg> = [NewArg::New(a.to_string()), NewArg::New(b.to_string())]
            .into_iter()
            .chain(old_args(&call, 1))
            .collect();
        out.push(Report::fixed(
            call.span,
            format!("Use `{name}()` instead of asserting a comparison."),
            Safety::Safe,
            rewrite_args(ctx, &call, Some(name), &args),
        ));
    }
}
