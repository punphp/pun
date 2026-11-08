//! `lint/dead-code/no-tautology-ternary`, ported from Rector 2.8.4's `SimplifyTautologyTernaryRector`
//! (rules/CodeQuality/Rector/Ternary) with `BinaryOpManipulator::matchFirstAndSecondConditionNode`.
//!
//! `$a !== $b ? $a : $b` is always `$a`, and `$a === $b ? $a : $b` is always `$b`: the ternary compares the two
//! values it chooses between, in either order.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{equal, moved};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoTautologyTernary;

impl Rule for NoTautologyTernary {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-tautology-ternary"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(t) = node else { return };
        let Expression::Binary(compare) = unparenthesize(t.condition) else {
            return;
        };
        let not_identical = match compare.operator {
            BinaryOperator::NotIdentical(_) => true,
            BinaryOperator::Identical(_) => false,
            _ => return,
        };
        // A short ternary (`?:`) has no `if` to compare with.
        let Some(then) = t.then else { return };
        let (l, r, e) = (compare.lhs, compare.rhs, t.r#else);
        let matched =
            (equal(ctx, l, then) && equal(ctx, r, e)) || (equal(ctx, r, then) && equal(ctx, l, e));
        if !matched {
            return;
        }
        let value = if not_identical { then } else { e };
        let span = node_span(t);
        let code = moved(ctx, span, ancestors, value);
        out.push(Report {
            span,
            message:
                "The ternary compares the two values it chooses between; use the value it always gives."
                    .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}
