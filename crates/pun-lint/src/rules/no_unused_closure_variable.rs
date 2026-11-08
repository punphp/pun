//! `lint/functions/no-unused-closure-variable`, ported from Rector 2.8.4's `RemoveUnusedClosureVariableUseRector`
//! (rules/DeadCode/Rector/Closure), with `ExprUsedInNodeAnalyzer`.
//!
//! A variable in a closure's `use (...)` that the closure's body never uses is removed, and the `use` clause with
//! it when none is left. A variable variable, an `include` or a `compact()` naming it counts as a use. Rector prints
//! a closure that lost its whole `use` clause again, its body a statement per line, and Pun writes the body so.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{block_list, line_indent, statement_lines, tokens};
use super::used_variable::used_in;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUnusedClosureVariable;

impl Rule for NoUnusedClosureVariable {
    fn id(&self) -> &'static str {
        "lint/functions/no-unused-closure-variable"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Closure(closure) = node else { return };
        let Some(clause) = &closure.use_clause else {
            return;
        };
        let variables = clause.variables.as_slice();
        let unused: Vec<bool> = variables
            .iter()
            .map(|v| {
                let name = &v.variable.name[1..];
                !closure
                    .body
                    .statements
                    .iter()
                    .any(|s| used_in(Node::Statement(s), None, name))
            })
            .collect();
        if !unused.contains(&true) {
            return;
        }
        let spans: Vec<Span> = variables.iter().map(node_span).collect();
        let edits = if unused.iter().all(|&u| u) {
            // Without its `use` clause the closure is a new node to php-parser's printer, which prints its body a
            // statement per line.
            let tokens = tokens(ctx);
            let indent = line_indent(ctx.source, node_span(closure).start);
            let body = block_list(tokens, &closure.body);
            let text = if body.is_empty() {
                format!("{{\n{indent}}}")
            } else {
                let inner = format!("{indent}    ");
                format!(
                    "{{\n{inner}{}\n{indent}}}",
                    statement_lines(ctx.source, tokens, &body, &inner)
                )
            };
            vec![
                Edit::new(
                    Span::new(
                        closure.parameter_list.right_parenthesis.end.offset as usize,
                        node_span(clause).end,
                    ),
                    "",
                ),
                Edit::new(node_span(&closure.body), text),
            ]
        } else {
            let last_kept = unused.iter().rposition(|&u| !u).expect("one is kept");
            let mut edits: Vec<Edit> = (0..last_kept)
                .filter(|&i| unused[i])
                .map(|i| Edit::new(Span::new(spans[i].start, spans[i + 1].start), ""))
                .collect();
            if last_kept + 1 < spans.len() {
                edits.push(Edit::new(
                    Span::new(spans[last_kept].end, spans[spans.len() - 1].end),
                    "",
                ));
            }
            edits
        };
        let first = unused.iter().position(|&u| u).expect("one is unused");
        out.push(Report {
            span: spans[first],
            message: format!(
                "`{}` is imported into the closure but never used; remove it.",
                String::from_utf8_lossy(variables[first].variable.name)
            ),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}
