//! `lint/complexity/no-negated-group`, ported from Rector 2.6.1's `SimplifyDeMorganBinaryRector`
//! (rules/CodeQuality/Rector/BooleanNot) with `BinaryOpManipulator::inverseBooleanOr`.
//!
//! `!($a > 20 || $b <= 50)` becomes `$a <= 20 && $b > 50` by de Morgan's law: each side is negated, a comparison by
//! its inverse operator, a `!` by dropping it, anything else by a new `!`. A chain of `||` is left alone.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, de_morgan, in_parent};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNegatedGroup;

impl Rule for NoNegatedGroup {
    fn id(&self) -> &'static str {
        "lint/complexity/no-negated-group"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::UnaryPrefix(not) = node else { return };
        if !not.operator.is_not() {
            return;
        }
        let Expression::Binary(or) = unparenthesize(not.operand) else {
            return;
        };
        let is_or = |e: &Expression<'_>| matches!(unparenthesize(e), Expression::Binary(b) if matches!(b.operator, BinaryOperator::Or(_)));
        if !matches!(or.operator, BinaryOperator::Or(_)) || is_or(or.lhs) || is_or(or.rhs) {
            return;
        }
        let span = node_span(not);
        let and = Prec::Infix(120, 121, 120);
        let code = in_parent(
            span,
            ancestors,
            de_morgan(ctx, or.lhs, or.rhs, "&&", (121, 120)),
            and,
        );
        out.push(Report {
            span,
            message: "Negate each condition and join them with `&&` instead of negating the `||`."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}
