//! `lint/pest/prefer-comparison-matchers`, ported from pest-plugin-rector's `SimplifyComparisonExpectationsRector`:
//! `expect($a > $b)->toBeTrue()` becomes `expect($a)->toBeGreaterThan($b)`, and `>=`, `<` and `<=` become
//! `toBeGreaterThanOrEqual()`, `toBeLessThan()` and `toBeLessThanOrEqual()`.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    is_expect_chain, is_name, matcher_subject, report, set_call_arguments, subject_holder, texts,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferComparisonMatchers;

impl Rule for PreferComparisonMatchers {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-comparison-matchers"
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
        if !is_expect_chain(call) || !is_name(&call.method, "toBeTrue") {
            return;
        }
        let Some(Expression::Binary(comparison)) = matcher_subject(call).map(unparenthesize) else {
            return;
        };
        let matcher = match comparison.operator {
            BinaryOperator::GreaterThan(_) => "toBeGreaterThan",
            BinaryOperator::GreaterThanOrEqual(_) => "toBeGreaterThanOrEqual",
            BinaryOperator::LessThan(_) => "toBeLessThan",
            BinaryOperator::LessThanOrEqual(_) => "toBeLessThanOrEqual",
            _ => return,
        };
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let mut edits = holder.set_arguments(ctx, comparison.lhs);
        edits.extend(set_call_arguments(
            call,
            matcher,
            &texts(ctx, &[comparison.rhs]),
        ));
        out.push(report(
            node_span(call),
            format!("Use `{matcher}()` instead of checking a comparison."),
            Safety::Safe,
            edits,
        ));
    }
}
