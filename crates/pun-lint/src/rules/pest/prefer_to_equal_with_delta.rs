//! `lint/pest/prefer-to-equal-with-delta`, ported from pest-plugin-rector's `UseToEqualWithDeltaRector`:
//! `expect(abs($a - $b) < $delta)->toBeTrue()` (or `<=`) becomes `expect($a)->toEqualWithDelta($b, $delta)`. The new
//! call is made on the chain's `expect()`, so what came between is dropped.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    argument_value, arguments_text, expect_argument, expect_call, func_call, has_not, identifier,
    is_expect_chain, report, set_first_argument,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToEqualWithDelta;

impl Rule for PreferToEqualWithDelta {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-equal-with-delta"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if !is_expect_chain(call) || identifier(&call.method) != Some(b"toBeTrue") {
            return;
        }
        let Some(Expression::Binary(comparison)) = expect_argument(call).map(unparenthesize) else {
            return;
        };
        if !matches!(
            comparison.operator,
            BinaryOperator::LessThan(_) | BinaryOperator::LessThanOrEqual(_)
        ) {
            return;
        }
        let Some(abs) = func_call(comparison.lhs).filter(|f| named(f, "abs")) else {
            return;
        };
        let [argument] = abs.argument_list.arguments.as_slice() else {
            return;
        };
        let Expression::Binary(minus) = unparenthesize(argument_value(argument)) else {
            return;
        };
        if !matches!(minus.operator, BinaryOperator::Subtraction(_)) {
            return;
        }
        let Some(expect) = expect_call(call) else {
            return;
        };
        let Some(first) = set_first_argument(ctx, &expect.argument_list, minus.lhs) else {
            return;
        };
        let not = if has_not(call) { "->not" } else { "" };
        out.push(report(
            node_span(call),
            "Use `toEqualWithDelta()` instead of comparing the difference.".to_string(),
            Safety::Safe,
            vec![
                first,
                Edit::new(
                    Span::new(node_span(expect).end, node_span(call).end),
                    format!(
                        "{not}->toEqualWithDelta({})",
                        arguments_text(ctx, &[minus.rhs, comparison.rhs])
                    ),
                ),
            ],
        ));
    }
}
