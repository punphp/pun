//! `lint/complexity/prefer-simple-condition`, ported from Rector 2.6.1's `SimplifyConditionsRector`
//! (rules/CodeQuality/Rector/Identical) with `AssignAndBinaryMap::getInversed` and
//! `BinaryOpManipulator::matchFirstAndSecondConditionNode`.
//!
//! `!($a !== 'b')` becomes `$a === 'b'`: a negated comparison of two operands that are no operations themselves
//! takes the inverse operator. `($a === null) === false` becomes `$a !== null` and `($a === null) === true` becomes
//! `$a === null`: an identity check of a comparison against `true` or `false`, in either order.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::{Prec, binary, in_parent, infix, inverse, is_constant, moved};
use super::rector_conditions::{is_binary_op, match_first_and_second};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferSimpleCondition;

impl Rule for PreferSimpleCondition {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-simple-condition"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix, NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (span, code) = match node {
            Node::UnaryPrefix(not) if not.operator.is_not() => {
                let Some(code) = boolean_not(ctx, not.operand) else {
                    return;
                };
                let span = node_span(not);
                (span, in_parent(span, ancestors, code.0, code.1))
            }
            Node::Binary(b) if matches!(b.operator, BinaryOperator::Identical(_)) => {
                let span = node_span(b);
                let Some(code) = identical(ctx, span, ancestors, b.lhs, b.rhs) else {
                    return;
                };
                (span, code)
            }
            _ => return,
        };
        out.push(Report {
            span,
            message: "Write the condition without the negation or the comparison with a boolean."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}

/// `processBooleanNot`: the operand of `!` must be a binary operation other than `||` whose operands are no binary
/// operations.
fn boolean_not(ctx: &Context<'_>, operand: &Expression<'_>) -> Option<(String, Prec)> {
    // `instanceof`, which is no `BinaryOp` in php-parser, has no inverse operator either.
    let Expression::Binary(b) = unparenthesize(operand) else {
        return None;
    };
    // shouldSkip
    if matches!(b.operator, BinaryOperator::Or(_)) || is_binary_op(b.lhs) || is_binary_op(b.rhs) {
        return None;
    }
    inversed(ctx, b.lhs, &b.operator, b.rhs)
}

/// `processIdenticalAndNotIdentical`, for `===` only (the rule's node types are `BooleanNot` and `Identical`).
fn identical(
    ctx: &Context<'_>,
    span: Span,
    ancestors: &[Node<'_, '_>],
    lhs: &Expression<'_>,
    rhs: &Expression<'_>,
) -> Option<String> {
    let (first, other) = match_first_and_second(
        lhs,
        rhs,
        |e| {
            matches!(e, Expression::Binary(b)
                if matches!(b.operator, BinaryOperator::Identical(_) | BinaryOperator::NotIdentical(_)))
        },
        |e| is_constant(e, "true") || is_constant(e, "false"),
    )?;
    if let (true, Expression::Binary(b)) = (is_constant(other, "false"), first) {
        let (code, prec) = inversed(ctx, b.lhs, &b.operator, b.rhs)?;
        return Some(in_parent(span, ancestors, code, prec));
    }
    Some(moved(ctx, span, ancestors, first))
}

/// `createInversedBooleanOp`: the comparison with the inverse operator, or none for an operator without one. Each
/// operand is put in parentheses where php-parser's precedence needs them (Rector marks a ternary operand
/// `WRAPPED_IN_PARENTHESES`, the same).
fn inversed(
    ctx: &Context<'_>,
    lhs: &Expression<'_>,
    op: &BinaryOperator<'_>,
    rhs: &Expression<'_>,
) -> Option<(String, Prec)> {
    let inverse = inverse(op)?;
    let (p, l, r) = binary(op);
    Some((infix(ctx, lhs, inverse, rhs, (l, r)), Prec::Infix(p, l, r)))
}
