//! `lint/complexity/no-negated-comparison`, ported from Rector 2.6.1's `BooleanNotIdenticalToNotIdenticalRector`
//! (rules/CodeQuality/Rector/Identical).
//!
//! `!($a === $b)` and `!$a === $b` become `$a !== $b` when both sides are `bool`.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, in_parent, infix};
use super::syntax_types::is_bool;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNegatedComparison;

impl Rule for NoNegatedComparison {
    fn id(&self) -> &'static str {
        "lint/complexity/no-negated-comparison"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary, NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        // refactor(): a `!` over `===`, or processIdentical() on an `===` whose left side is a `!`.
        let (left, right) = match node {
            Node::UnaryPrefix(not) if not.operator.is_not() => {
                let Expression::Binary(identical) = unparenthesize(not.operand) else {
                    return;
                };
                if !matches!(identical.operator, BinaryOperator::Identical(_))
                    || !is_bool(ancestors, identical.lhs)
                    || !is_bool(ancestors, identical.rhs)
                {
                    return;
                }
                (identical.lhs, identical.rhs)
            }
            Node::Binary(identical) if matches!(identical.operator, BinaryOperator::Identical(_)) => {
                if !is_bool(ancestors, identical.lhs) || !is_bool(ancestors, identical.rhs) {
                    return;
                }
                let Expression::UnaryPrefix(not) = unparenthesize(identical.lhs) else {
                    return;
                };
                if !not.operator.is_not() {
                    return;
                }
                (not.operand, identical.rhs)
            }
            _ => return,
        };
        let span = node_span(&node);
        let code = infix(ctx, left, "!==", right, (80, 80));
        out.push(Report {
            span,
            message: "Use `!==` instead of negating an `===` comparison.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    span,
                    in_parent(span, ancestors, code, Prec::Infix(80, 80, 80)),
                )],
            }),
        });
    }
}
