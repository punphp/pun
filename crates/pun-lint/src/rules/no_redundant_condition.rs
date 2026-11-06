//! `lint/dead-code/no-redundant-condition`, ported from Rector 2.6.1's `RemoveNextSameValueConditionRector`
//! (rules/DeadCode/Rector/Stmt).
//!
//! Two `if`s in a row, neither with an `elseif` or `else`, that check the same condition become one: the second's
//! statements move to the end of the first, and its comments go before the first. The condition must have no side
//! effect, and the first `if`'s statements must not use a variable of the condition.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_stmts::{
    Frame, HOLDERS, comments, holder_list, if_view, inserted_statements, line_indent,
    moved_statements, remove,
};
use super::side_effect::has_side_effect;
use crate::syntax::{node_span, same_expression};
use crate::{Context, Report, Rule};

pub struct NoRedundantCondition;

impl Rule for NoRedundantCondition {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-redundant-condition"
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
        let raw = crate::syntax::statements(node);
        if !raw
            .windows(2)
            .any(|w| matches!((w[0], w[1]), (Statement::If(_), Statement::If(_))))
        {
            return;
        }
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        for (key, &stmt) in list.stmts.iter().enumerate() {
            let Statement::If(r#if) = stmt else {
                continue;
            };
            let view = if_view(source, r#if);
            if !view.elseifs.is_empty() || view.else_.is_some() || view.colon {
                continue;
            }
            let Some(Statement::If(next_if)) = list.stmts.get(key + 1) else {
                continue;
            };
            let next = if_view(source, next_if);
            if !same_expression(source, view.condition, next.condition) {
                continue;
            }
            if !next.elseifs.is_empty() || next.else_.is_some() {
                continue;
            }
            if has_side_effect(&scope, view.condition) {
                continue;
            }
            if condition_variable_used(view.condition, &view.stmts.stmts) {
                continue;
            }
            let indent = line_indent(source, node_span(r#if).start);
            let mut edits = Vec::new();
            // The second `if`'s comments join the first's.
            let between = node_span(r#if).end;
            if let Some(span) = comments(source, between, node_span(next_if).start) {
                let text = comment_lines(source, span, indent);
                let start = node_span(r#if).start;
                edits.push(Edit::new(
                    Span::new(start, start),
                    format!("{text}\n{indent}"),
                ));
            }
            // The second `if`'s statements go at the end of the first's.
            if !next.stmts.is_empty() {
                let inner = format!("{indent}    ");
                match view.stmts.frame {
                    Frame::Braces { open, close } if view.stmts.is_empty() => {
                        let moved = moved_statements(ctx, &next.stmts, &inner);
                        edits.push(Edit::new(
                            Span::new(open, close),
                            format!("\n{inner}{moved}\n{indent}"),
                        ));
                    }
                    Frame::Braces { .. } => {
                        let moved = inserted_statements(ctx, &next.stmts, &inner);
                        let at = view
                            .stmts
                            .nop_span(source)
                            .map(|s| s.end)
                            .or_else(|| view.stmts.stmts.last().map(|s| node_span(*s).end))
                            .unwrap();
                        edits.push(Edit::new(Span::new(at, at), format!("\n{inner}{moved}")));
                    }
                    // `Frame::Bare`: a colon-delimited `if` (`Frame::Open`) was skipped above.
                    _ => {
                        let moved = moved_statements(ctx, &next.stmts, &inner);
                        let only = node_span(view.stmts.stmts[0]);
                        let text = format!(
                            "{{\n{inner}{}\n{inner}{moved}\n{indent}}}",
                            &source[only.start..only.end]
                        );
                        edits.push(Edit::new(only, text));
                    }
                }
            }
            edits.extend(remove(ctx, &list, &[key + 1]));
            out.push(Report {
                span: node_span(next_if),
                message: "This `if` checks the same condition as the `if` before it; merge them."
                    .to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
            return;
        }
    }
}

/// The comments of `span`, a line each at `indent`.
fn comment_lines(source: &str, span: Span, indent: &str) -> String {
    super::rector_stmts::tokens(source, span.start, span.end)
        .iter()
        .filter(|t| t.kind.is_comment())
        .map(|t| &source[t.start..t.end])
        .collect::<Vec<_>>()
        .join(&format!("\n{indent}"))
}

/// `isCondVariableUsedInIfBody`: a variable of the condition is named anywhere in the `if`'s statements.
fn condition_variable_used(condition: &Expression<'_>, statements: &[&Statement<'_>]) -> bool {
    let mut names = Vec::new();
    collect_names(Node::Expression(condition), &mut names);
    let mut used = Vec::new();
    for statement in statements {
        collect_names(Node::Statement(statement), &mut used);
    }
    names.iter().any(|n| used.contains(n))
}

fn collect_names<'a>(node: Node<'_, 'a>, out: &mut Vec<&'a [u8]>) {
    if let Node::DirectVariable(v) = node {
        out.push(v.name);
    }
    node.visit_children(|child| collect_names(child, out));
}
