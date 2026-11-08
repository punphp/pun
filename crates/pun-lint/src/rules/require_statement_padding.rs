//! `lint/style/require-statement-padding`, ported from Rector 2.8.4's `NewlineAfterStatementRector`
//! (rules/CodingStyle/Rector/Stmt).
//!
//! A function, class, trait, interface, `if`, `foreach`, `for`, `while`, `do` or `try` statement, or a `switch`, gets
//! a blank line after it when the next statement (or the comment before it) starts on the very next line. Rector
//! skips a file with inline HTML.

use mago_syntax::cst::{Node, NodeKind, Statement};
use pun_problem::{Edit, Fix, Safety, Span};

use super::lint_b_helpers::{contains_html, lines_between};
use super::rector_stmts::{HOLDERS, holder_list};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireStatementPadding;

impl Rule for RequireStatementPadding {
    fn id(&self) -> &'static str {
        "lint/style/require-statement-padding"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(HOLDERS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(list) = holder_list(ctx.source, node, ancestors) else {
            return;
        };
        let mut edits = Vec::new();
        for (key, stmt) in list.stmts.iter().enumerate() {
            // `STMTS_TO_HAVE_NEXT_NEWLINE`; a class member is never in a statement list Rector walks.
            if !matches!(
                stmt,
                Statement::Function(_)
                    | Statement::Class(_)
                    | Statement::Trait(_)
                    | Statement::Interface(_)
                    | Statement::If(_)
                    | Statement::Foreach(_)
                    | Statement::DoWhile(_)
                    | Statement::While(_)
                    | Statement::For(_)
                    | Statement::Try(_)
                    | Statement::Switch(_)
            ) {
                continue;
            }
            // The next statement, from its first comment (`CommentResolver`), or php-parser's trailing `Nop`.
            let next = match list.stmts.get(key + 1) {
                Some(_) => list.with_comments(ctx, key + 1).start,
                None => match list.nop_span(ctx.source) {
                    Some(nop) => nop.start,
                    None => continue,
                },
            };
            let end = node_span(*stmt).end;
            if lines_between(ctx.source, end, next) != 1 {
                continue;
            }
            // Rector prints the new empty statement as an empty line.
            edits.push(Edit::new(Span::new(end, end), "\n"));
        }
        let Some(first) = edits.first().map(|e| e.span) else {
            return;
        };
        if contains_html(node, ancestors) {
            return;
        }
        out.push(Report {
            span: first,
            message: "Add a blank line after the statement.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}
