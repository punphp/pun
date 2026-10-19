//! `lint/control-flow/no-empty-catch`, ported from Mago 1.57.0's `no-empty-catch-clause`
//! (mago-linter src/rule/correctness/no_empty_catch_clause.rs).
//!
//! Reports a `catch` block that holds no statement (only empty blocks and `;`), which swallows the exception.

use mago_span::HasSpan;
use mago_syntax::cst::{Node, NodeKind, Statement};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoEmptyCatch;

impl Rule for NoEmptyCatch {
    fn id(&self) -> &'static str {
        "lint/control-flow/no-empty-catch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Try])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Try(r#try) = node else { return };
        for catch in r#try.catch_clauses.iter() {
            if !are_statements_empty(catch.block.statements.as_slice()) {
                continue;
            }
            out.push(Report {
                span: span(catch.span()),
                message: "Do not use empty `catch` blocks.".to_string(),
                help: Some(
                    "Either log the exception, re-throw it, or handle it. If ignoring it is intentional, add a \
                     `@pun-ignore` comment explaining why."
                        .to_string(),
                ),
                fix: None,
            });
        }
    }
}

fn is_statement_empty(statement: &Statement<'_>) -> bool {
    match statement {
        Statement::Block(block) => are_statements_empty(block.statements.as_slice()),
        Statement::Noop(_) => true,
        _ => false,
    }
}

fn are_statements_empty(statements: &[Statement<'_>]) -> bool {
    statements.iter().all(is_statement_empty)
}
