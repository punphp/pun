//! `lint/dead-code/no-useless-catch`, ported from Rector 2.6.1's `RemoveDeadTryCatchRector`
//! (rules/DeadCode/Rector/TryCatch).
//!
//! When the `finally` block is missing or empty:
//!
//! - a `try` with an empty block is removed, with its catches;
//! - a `try` with one catch whose first statement rethrows the caught variable (`catch (E $e) { throw $e; }`) is
//!   replaced by the statements of its block.
//!
//! "Empty" is php-parser's: no statement, or only comments (one `Nop`). Rector prints a comment that ends the moved
//! block followed by an empty line, which Pun writes too.

use mago_syntax::cst::{Expression, Node, NodeKind, Statement};
use pun_problem::{Edit, Fix, Safety};

use super::php_parser::{
    Frame, block_list, enclosing, line_indent, moved_text, remove, statement_lines, tokens,
};
use crate::syntax::{node_span, same_code, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessCatch;

impl Rule for NoUselessCatch {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-catch"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Try])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Try(r#try) = node else { return };
        let source = ctx.source;
        // A cheap look before the file is read into tokens: an empty `try` block, or one catch that throws.
        let rethrows = r#try.catch_clauses.len() == 1
            && r#try.catch_clauses.as_slice()[0]
                .block
                .statements
                .iter()
                .any(|s| matches!(s, Statement::Expression(e) if matches!(e.expression, Expression::Throw(_))));
        let has_statement = r#try
            .block
            .statements
            .iter()
            .any(|s| !matches!(s, Statement::Noop(_)));
        if !rethrows && has_statement {
            return;
        }
        let tokens = tokens(source);
        // isEmpty: no statement, or one Nop.
        let empty = |b| block_list(&tokens, b).stmts.is_empty();
        // not empty stmts on finally always executed
        if r#try
            .finally_clause
            .as_ref()
            .is_some_and(|f| !empty(&f.block))
        {
            return;
        }
        let (list, key) = enclosing(source, &tokens, ancestors);
        let edits = if empty(&r#try.block) {
            remove(source, &tokens, &list, &[key])
        } else {
            // Not empty, so the cheap look found the one catch that throws.
            let catch = &r#try.catch_clauses.as_slice()[0];
            let catch_list = block_list(&tokens, &catch.block);
            let Some(Statement::Expression(first)) = catch_list.stmts.first() else {
                return;
            };
            let Expression::Throw(throw) = first.expression else {
                return;
            };
            let Some(variable) = &catch.variable else {
                return;
            };
            if !same_code(
                source,
                node_span(variable),
                node_span(unparenthesize(throw.exception)),
            ) {
                return;
            }
            // return $node->stmts: the try's statements take its place.
            let try_list = block_list(&tokens, &r#try.block);
            let span = node_span(r#try);
            let indent = line_indent(source, span.start);
            let text = if list.frame == Frame::Bare && try_list.len() > 1 {
                let inner = format!("{indent}    ");
                format!(
                    "{{\n{inner}{}\n{indent}}}",
                    statement_lines(source, &tokens, &try_list, &inner)
                )
            } else {
                let mut text = moved_text(source, &tokens, &try_list, indent);
                // php-parser prints the Nop of a trailing comment as an empty line.
                if try_list.nop && key + 1 < list.stmts.len() {
                    text.push('\n');
                }
                text
            };
            vec![Edit::new(span, text)]
        };
        out.push(Report {
            span: node_span(r#try),
            message: "This try/catch does nothing; remove it.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}
