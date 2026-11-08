//! `lint/dead-code/no-empty-loop`, ported from Rector 2.8.4's `RemoveDeadLoopRector` (rules/DeadCode/Rector/For_).
//!
//! A `do`, `for`, `foreach` or `while` loop with no statements goes, unless one of its expressions (the right side,
//! for a plain assignment) has a side effect.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_stmts::{
    HOLDERS, List, block_list, body_list, holder_list, is_plain_assign, open_list, remove,
};
use super::side_effect::has_side_effect;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEmptyLoop;

impl Rule for NoEmptyLoop {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-empty-loop"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::DoWhile,
            NodeKind::For,
            NodeKind::Foreach,
            NodeKind::While,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        _node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let source = ctx.source;
        let [.., grand, Node::Statement(stmt)] = ancestors else {
            return;
        };
        let Some((body, exprs)) = parts(source, stmt) else {
            return;
        };
        if !body.is_empty() {
            return;
        }
        if exprs.iter().any(|&expr| {
            let expr = match crate::syntax::unparenthesize(expr) {
                Expression::Assignment(a) if is_plain_assign(a) => a.rhs,
                _ => expr,
            };
            has_side_effect(ancestors, expr)
        }) {
            return;
        }
        // The loop's place: an item of a statement list, or the one statement of a control structure's body.
        let edits = if HOLDERS.contains(&grand.kind()) {
            // Rector removes the loop wherever it is, in a property hook's body too, where `holder_list` has none.
            let list = match grand {
                Node::Block(block) => block_list(source, block),
                _ => holder_list(source, *grand, &ancestors[..ancestors.len() - 2]).unwrap(),
            };
            let key = list
                .stmts
                .iter()
                .position(|s| std::ptr::eq(*s, *stmt))
                .unwrap();
            remove(ctx, &list, &[key])
        } else {
            remove(ctx, &body_list(source, stmt), &[0])
        };
        out.push(Report {
            span: node_span(*stmt),
            message: "This loop has no body and no side effect; remove it.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

fn offset(span: mago_span::Span) -> (usize, usize) {
    (span.start.offset as usize, span.end.offset as usize)
}

/// The loop's php-parser `stmts` and the expressions Rector checks: `cond` for `do` and `while`, `init`, `cond` and
/// `loop` for `for`, `expr` and `valueVar` for `foreach`.
fn parts<'a>(source: &str, stmt: &'a Statement<'a>) -> Option<(List<'a>, Vec<&'a Expression<'a>>)> {
    Some(match stmt {
        Statement::DoWhile(r#do) => (body_list(source, r#do.statement), vec![r#do.condition]),
        Statement::While(r#while) => {
            let body = match &r#while.body {
                WhileBody::Statement(s) => body_list(source, s),
                WhileBody::ColonDelimited(b) => {
                    colon(source, &b.statements, b.colon, b.end_while.span())
                }
            };
            (body, vec![r#while.condition])
        }
        Statement::For(r#for) => {
            let body = match &r#for.body {
                ForBody::Statement(s) => body_list(source, s),
                ForBody::ColonDelimited(b) => {
                    colon(source, &b.statements, b.colon, b.end_for.span())
                }
            };
            let exprs = r#for
                .initializations
                .iter()
                .chain(r#for.conditions.iter())
                .chain(r#for.increments.iter())
                .copied()
                .collect();
            (body, exprs)
        }
        Statement::Foreach(foreach) => {
            let body = match &foreach.body {
                ForeachBody::Statement(s) => body_list(source, s),
                ForeachBody::ColonDelimited(b) => {
                    colon(source, &b.statements, b.colon, b.end_foreach.span())
                }
            };
            (body, vec![foreach.expression, foreach.target.value()])
        }
        _ => return None,
    })
}

fn colon<'a>(
    source: &str,
    statements: &'a Sequence<'a, Statement<'a>>,
    colon: mago_span::Span,
    end: mago_span::Span,
) -> List<'a> {
    open_list(source, statements, offset(colon).1, offset(end).0)
}
