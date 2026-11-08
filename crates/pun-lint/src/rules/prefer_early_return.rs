//! `lint/style/prefer-early-return`, ported from Rector 2.8.4's `ChangeIfElseValueAssignToEarlyReturnRector`
//! (rules/EarlyReturn/Rector/If_), with `IfManipulator::isIfAndElseWithSameVariableAssignAsLastStmts` and
//! `StmtsManipulator::getUnwrappedLastStmt`. Mago's `prefer-early-return`, which also maps here, joins it later.
//!
//! An `if`/`else` whose branches both end by assigning the same variable, followed by `return` of that variable,
//! returns in the `if` instead: the `if` returns its value, the `else` goes, its other statements move after the `if`
//! and the `return` returns the `else`'s value.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::equal;
use super::rector_stmts::{
    HOLDERS, List, assign, expression, holder_list, if_view, line_indent, reindent,
};

use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEarlyReturn;

impl Rule for PreferEarlyReturn {
    fn id(&self) -> &'static str {
        "lint/style/prefer-early-return"
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
        // A cheap look first: an `if` right before a `return`.
        let raw = crate::syntax::statements(node);
        if !raw
            .windows(2)
            .any(|w| matches!((w[0], w[1]), (Statement::If(_), Statement::Return(_))))
        {
            return;
        }
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        for key in 1..list.stmts.len() {
            let Statement::Return(r#return) = list.stmts[key] else {
                continue;
            };
            let Some(returned) = r#return.value else {
                continue;
            };
            let Statement::If(r#if) = list.stmts[key - 1] else {
                continue;
            };
            if let Some(edits) = refactor(ctx, &list, key, r#if, returned) {
                out.push(Report {
                    span: node_span(r#if),
                    message: "Return in the `if` instead of assigning a variable to return after the `else`."
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
}

/// The assignment `getUnwrappedLastStmt` finds at the end of a list, with the statement that holds it.
fn last_assign<'a>(list: &List<'a>) -> Option<(&'a Statement<'a>, &'a Assignment<'a>)> {
    let last = list.last()?;
    Some((last, expression(last).and_then(assign)?))
}

fn refactor(
    ctx: &Context<'_>,
    list: &List<'_>,
    key: usize,
    r#if: &If<'_>,
    returned: &Expression<'_>,
) -> Option<Vec<Edit>> {
    let source = ctx.source;
    let view = if_view(source, r#if);
    // isIfAndElseWithSameVariableAssignAsLastStmts.
    let (else_stmts, else_span) = view.else_.as_ref()?;
    if !view.elseifs.is_empty() {
        return None;
    }
    let (if_last, if_assign) = last_assign(&view.stmts)?;
    let (else_last, else_assign) = last_assign(else_stmts)?;
    if !matches!(unparenthesize(if_assign.lhs), Expression::Variable(_))
        || !equal(ctx, if_assign.lhs, else_assign.lhs)
        || !equal(ctx, returned, else_assign.lhs)
    {
        return None;
    }
    let indent = line_indent(source, node_span(r#if).start);
    let text = |e: &Expression<'_>| {
        let s = node_span(unparenthesize(e));
        &source[s.start..s.end]
    };
    let mut edits = Vec::new();
    // The `if` returns its value; the statement keeps its comments.
    edits.push(Edit::new(
        node_span(if_last),
        format!("return {};", text(if_assign.rhs)),
    ));
    // The `else` goes.
    let else_start = match &r#if.body {
        IfBody::Statement(body) => node_span(body.statement).end,
        IfBody::ColonDelimited(_) => {
            let inner = view.stmts.inner;
            inner.start + source[inner.start..inner.end].trim_end().len()
        }
    };
    // The `else`'s other statements go before the list's last statement: right after the `if` when the `return` is
    // the last statement.
    let moved: String = (0..else_stmts.stmts.len() - 1)
        .map(|i| {
            let span = else_stmts.with_comments(ctx, i);
            format!(
                "\n{indent}{}",
                reindent(source, span, line_indent(source, span.start), indent)
            )
        })
        .collect();
    let last = list.stmts.len() - 1;
    // Edits of one fix must not touch: text that goes where another edit ends joins that edit.
    let mut after_return = String::new();
    if !view.colon && last == key {
        edits.push(Edit::new(Span::new(else_start, else_span.end), moved));
    } else {
        edits.push(Edit::new(Span::new(else_start, else_span.end), ""));
        if last - 1 == key {
            after_return = moved;
        } else {
            let at = node_span(list.stmts[last - 1]).end;
            edits.push(Edit::new(Span::new(at, at), moved));
        }
    }
    // The `return` returns the `else`'s value, with the comments of the `else`'s last statement instead of its own.
    let return_span = list.with_comments(ctx, key);
    let else_last_span = else_stmts.with_comments(ctx, else_stmts.stmts.len() - 1);
    let comments = &source[else_last_span.start..node_span(else_last).start];
    let comments = comments.trim_end();
    let head = if comments.is_empty() {
        String::new()
    } else {
        let span = Span::new(else_last_span.start, else_last_span.start + comments.len());
        format!(
            "{}\n{indent}",
            reindent(source, span, line_indent(source, span.start), indent)
        )
    };
    edits.push(Edit::new(
        return_span,
        format!("{head}return {};{after_return}", text(else_assign.rhs)),
    ));
    Some(edits)
}
