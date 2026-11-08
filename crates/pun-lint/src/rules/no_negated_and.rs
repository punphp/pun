//! `lint/complexity/no-negated-and`, ported from Rector 2.8.4's `NegatedAndsToPositiveOrsRector`
//! (rules/CodeQuality/Rector/BooleanNot) with `BinaryOpManipulator::inverseBooleanAnd`.
//!
//! `!($a > 20 && $b <= 50)` becomes `$a <= 20 || $b > 50` by de Morgan's law: each side is negated, a comparison by
//! its inverse operator, a `!` by dropping it, anything else by a new `!`. A chain of `&&` is left alone.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, de_morgan, in_parent};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNegatedAnd;

impl Rule for NoNegatedAnd {
    fn id(&self) -> &'static str {
        "lint/complexity/no-negated-and"
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
        let Expression::Binary(and) = unparenthesize(not.operand) else {
            return;
        };
        let is_and = |e: &Expression<'_>| matches!(unparenthesize(e), Expression::Binary(b) if matches!(b.operator, BinaryOperator::And(_)));
        if !matches!(and.operator, BinaryOperator::And(_)) || is_and(and.lhs) || is_and(and.rhs) {
            return;
        }
        let span = node_span(not);
        let or = Prec::Infix(130, 131, 130);
        let code = in_parent(
            span,
            ancestors,
            de_morgan(ctx, and.lhs, and.rhs, "||", (131, 130)),
            or,
        );
        out.push(Report {
            span,
            message: "Negate each condition and join them with `||` instead of negating the `&&`."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}
