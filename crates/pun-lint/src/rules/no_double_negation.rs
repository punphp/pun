//! `lint/complexity/no-double-negation`, ported from Rector 2.6.1's `ReplaceMultipleBooleanNotRector`
//! (rules/CodeQuality/Rector/BooleanNot).
//!
//! `!!$value` becomes `(bool) $value`. A longer chain of `!` keeps one when the count is odd (`!!!$value` becomes
//! `!$value`) and becomes a cast when it is even.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::{lhs_limit, operand};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDoubleNegation;

impl Rule for NoDoubleNegation {
    fn id(&self) -> &'static str {
        "lint/complexity/no-double-negation"
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
        // The number of `!` under this one.
        let mut depth = 0;
        let mut expr = unparenthesize(not.operand);
        while let Expression::UnaryPrefix(inner) = expr
            && inner.operator.is_not()
        {
            depth += 1;
            expr = unparenthesize(inner.operand);
        }
        if depth == 0 {
            return;
        }
        let span = node_span(not);
        // php-parser wraps a prefix operator under the new code when that code is the left operand of a binary one.
        let lhs = lhs_limit(span, ancestors);
        // An even count keeps this `!` and its operand is the innermost expression: php-parser keeps the code up to
        // the old operand and prints the new one in its place. An odd count is a new `(bool)` cast.
        let (edit, message) = if depth % 2 == 0 {
            let start = node_span(not.operand).start;
            (
                Edit::new(Span::new(start, span.end), operand(ctx, expr, 30, lhs)),
                "Use one `!` instead of an odd number of them.",
            )
        } else {
            (
                Edit::new(span, format!("(bool) {}", operand(ctx, expr, 10, lhs))),
                "Use a `(bool)` cast instead of `!!`.",
            )
        };
        out.push(Report {
            span,
            message: message.to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}
