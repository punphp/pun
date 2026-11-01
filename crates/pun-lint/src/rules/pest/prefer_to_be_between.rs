//! `lint/pest/prefer-to-be-between`, ported from pest-plugin-rector's `UseToBeBetweenRector`: `expect($value >= $min
//! && $value <= $max)->toBeTrue()` becomes `expect($value)->toBeBetween($min, $max)` when `$value` may be an int or a
//! float.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, first_value, is_name, set_call_arguments, subject_holder, texts, type_is,
};
use crate::syntax::same_expression;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeBetween;

impl Rule for PreferToBeBetween {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-between"
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
        if !is_name(&call.method, "toBeTrue") {
            return;
        }
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let Some(Expression::Binary(and)) = first_value(holder.arguments).map(unparenthesize)
        else {
            return;
        };
        if !matches!(and.operator, BinaryOperator::And(_)) {
            return;
        }
        let (Expression::Binary(left), Expression::Binary(right)) =
            (unparenthesize(and.lhs), unparenthesize(and.rhs))
        else {
            return;
        };
        if !matches!(left.operator, BinaryOperator::GreaterThanOrEqual(_))
            || !matches!(right.operator, BinaryOperator::LessThanOrEqual(_))
            || !same_expression(ctx.source, left.lhs, right.lhs)
        {
            return;
        }
        let value = left.lhs;
        if type_is(ctx, value, Is::Integer) == Trinary::No
            && type_is(ctx, value, Is::Float) == Trinary::No
        {
            return;
        }
        let mut edits = holder.set_arguments(ctx, value);
        edits.extend(set_call_arguments(
            call,
            "toBeBetween",
            &texts(ctx, &[left.rhs, right.rhs]),
        ));
        out.push(Report::fixed(
            node_span(call),
            "Use `toBeBetween()` instead of checking both bounds.",
            Safety::Safe,
            edits,
        ));
    }
}
