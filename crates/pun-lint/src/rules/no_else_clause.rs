//! `lint/complexity/no-else-clause`, ported from Mago 1.57.0's `no-else-clause`
//! (mago-linter src/rule/maintainability/no_else_clause.rs).
//!
//! Reports each `elseif` and the `else` of an `if`, in either syntax.

use mago_span::HasSpan;
use mago_syntax::cst::{IfBody, Node, NodeKind};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoElseClause;

impl Rule for NoElseClause {
    fn id(&self) -> &'static str {
        "lint/complexity/no-else-clause"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::If(r#if) = node else { return };
        let (else_ifs, r#else): (Vec<_>, _) = match &r#if.body {
            IfBody::Statement(body) => (
                body.else_if_clauses.iter().map(HasSpan::span).collect(),
                body.else_clause.as_ref().map(HasSpan::span),
            ),
            IfBody::ColonDelimited(body) => (
                body.else_if_clauses.iter().map(HasSpan::span).collect(),
                body.else_clause.as_ref().map(HasSpan::span),
            ),
        };
        for clause in else_ifs {
            out.push(Report {
                span: span(clause),
                message: "Avoid `elseif` clauses.".into(),
                help: Some(
                    "Refactor to use guard clauses (early returns) or a `match` expression for clarity.".into(),
                ),
                fix: None,
            });
        }
        if let Some(clause) = r#else {
            out.push(Report {
                span: span(clause),
                message: "Avoid `else` clauses.".into(),
                help: Some(
                    "Consider refactoring to use an early return (a guard clause) to simplify the control flow."
                        .into(),
                ),
                fix: None,
            });
        }
    }
}
