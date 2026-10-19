//! `lint/security/no-insecure-comparison`, ported from Mago 1.57.0's linter rule `no-insecure-comparison`
//! (rule/security/no_insecure_comparison.rs).
//!
//! `==`, `!=`, `===`, `!==` (and `<>`) with a side that looks like sensitive data (a password, token, secret or API
//! key, by name) is open to timing attacks, unless the other side is a simple literal (an empty string, a number,
//! `null`, a boolean). The operator is reported.

use mago_span::Span;
use mago_syntax::cst::{Expression, Literal, Node, NodeKind};

use super::lint_b_security::get_password;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoInsecureComparison;

impl Rule for NoInsecureComparison {
    fn id(&self) -> &'static str {
        "lint/security/no-insecure-comparison"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        if !binary.operator.is_equality() {
            return;
        }
        let lhs = sensitive_value(binary.lhs);
        let rhs = sensitive_value(binary.rhs);
        if lhs.is_none() && rhs.is_none() {
            return;
        }
        if (lhs.is_some() && is_simple_literal(binary.rhs))
            || (rhs.is_some() && is_simple_literal(binary.lhs))
        {
            return;
        }
        out.push(Report {
            span: node_span(&binary.operator),
            message: "Insecure comparison of sensitive data.".to_string(),
            help: Some(
                "Use `hash_equals` for comparing strings or `password_verify` for validating hashes."
                    .to_string(),
            ),
            fix: None,
        });
    }
}

/// `get_sensitive_value`: a literal is a value, not a name.
fn sensitive_value(expr: &Expression<'_>) -> Option<Span> {
    match expr {
        Expression::Parenthesized(p) => sensitive_value(p.expression),
        Expression::Assignment(a) => sensitive_value(a.lhs).or_else(|| sensitive_value(a.rhs)),
        Expression::Literal(_) => None,
        _ => get_password(expr),
    }
}

fn is_simple_literal(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Parenthesized(p) => is_simple_literal(p.expression),
        Expression::Literal(Literal::String(s)) => s.raw.len() == 2,
        Expression::Literal(_) => true,
        _ => false,
    }
}
