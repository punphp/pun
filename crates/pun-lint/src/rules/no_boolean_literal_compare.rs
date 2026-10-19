//! `lint/complexity/no-boolean-literal-compare`, ported from Rector 2.6.1's `SimplifyBoolIdenticalTrueRector`
//! (rules/CodeQuality/Rector/Identical).
//!
//! `$bool === true` is `$bool`, `$bool !== false` is `$bool`, `$bool !== true` is `!$bool` and `!$x === false` is
//! `$x`, where `$bool` is natively a `bool` (read from the syntax, `syntax_types::is_bool`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::syntax_types::is_bool;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoBooleanLiteralCompare;

impl Rule for NoBooleanLiteralCompare {
    fn id(&self) -> &'static str {
        "lint/complexity/no-boolean-literal-compare"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else {
            return;
        };
        let identical = match binary.operator {
            BinaryOperator::Identical(_) => true,
            BinaryOperator::NotIdentical(_) => false,
            _ => return,
        };
        let candidate = |e: &Expression<'_>| truth(e).is_none() && is_bool(ancestors, e);
        let (value, other) = if candidate(binary.lhs) {
            (binary.lhs, binary.rhs)
        } else if candidate(binary.rhs) {
            (binary.rhs, binary.lhs)
        } else {
            return;
        };
        let source = ctx.source;
        let text = |e: &Expression<'_>| {
            let s = node_span(unparenthesize(e));
            source[s.start..s.end].to_string()
        };
        let replacement = match (identical, truth(other)) {
            (true, Some(true)) | (false, Some(false)) => text(value),
            (true, Some(false)) => match unparenthesize(value) {
                Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                    text(u.operand)
                }
                _ => return,
            },
            (false, Some(true)) => {
                let inner = unparenthesize(value);
                if matches!(
                    inner,
                    Expression::Binary(_) | Expression::Assignment(_) | Expression::Conditional(_)
                ) {
                    format!("!({})", text(inner))
                } else {
                    format!("!{}", text(inner))
                }
            }
            _ => return,
        };
        out.push(Report {
            span: node_span(binary),
            message:
                "Comparing a `bool` with `true` or `false` is the `bool` itself or its negation."
                    .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(binary), replacement)],
            }),
        });
    }
}

fn truth(expr: &Expression<'_>) -> Option<bool> {
    match unparenthesize(expr) {
        Expression::Literal(Literal::True(_)) => Some(true),
        Expression::Literal(Literal::False(_)) => Some(false),
        _ => None,
    }
}
