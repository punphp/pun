//! `lint/complexity/no-constant-condition`, ported from Rector 2.8.4's `RemoveDeadIfBlockRector`
//! (rules/DeadCode/Rector/If_), with `ConditionInverter` and `SideEffectNodeDetector`.
//!
//! Branches of an `if` that do nothing go: an `else` with no statement, and each `elseif` with no statement whose
//! condition has no side effect. When the `if` body itself has no statement, no comment before the `if` and a
//! condition without side effects, the `if` merges into what is left: the first `elseif` becomes
//! `if (!<condition> && <elseif condition>)` (unless an `else` follows), an `else` becomes `if (!<condition>)`, and
//! with neither the statement goes.
//!
//! php-parser's printer decides the bytes. A removed `else` takes the whitespace and comments before it, a removed
//! `elseif` the text from the end of the branch before it (the first one: up to the next kept branch). An `if` whose
//! `elseif`s all go, and the `if` a merge builds, are printed anew: the body in braces a statement a line, the kept
//! branches as written. Rector prints the kept branches of an alternative-syntax `if` (`elseif ...:`, `else:`) after a
//! braced body, which does not parse; Pun prints them in braces too (lint.md: where Rector writes broken code, Pun
//! writes the valid fix).

use mago_syntax::cst::{Expression, Node, NodeKind, Statement};
use pun_problem::{Edit, Fix, Safety, Span};

use super::no_empty_block::inverted_condition;
use super::php_parser::operand_precedence;
use super::rector_stmts::{
    IfView, List, braced, enclosing, first_comment, if_view, line_indent, previous_token_end,
    remove,
};
use super::side_effect::has_side_effect;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoConstantCondition;

impl Rule for NoConstantCondition {
    fn id(&self) -> &'static str {
        "lint/complexity/no-constant-condition"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Statement(s @ Statement::If(r#if)) = node else {
            return;
        };
        let source = ctx.source;
        let view = if_view(source, r#if);
        let empty_else = view.else_.as_ref().is_some_and(|(l, _)| l.is_empty());
        let else_ = view.else_.as_ref().filter(|_| !empty_else);
        let kept: Vec<usize> = (0..view.elseifs.len())
            .filter(|&i| {
                let (condition, list, _) = &view.elseifs[i];
                !list.is_empty() || has_side_effect(ancestors, condition)
            })
            .collect();
        let elseifs_changed = kept.len() != view.elseifs.len();
        let (list, index) = enclosing(source, s, ancestors);
        let span = node_span(s);
        let indent = line_indent(source, span.start);
        // `$node->getComments() !== []`: comments before the `if`, which php-parser keeps with it
        let commented = list.with_comments(ctx, index).start != span.start;
        let dead = view.stmts.is_empty()
            && !commented
            && !has_side_effect(ancestors, view.condition);
        // A trailing else would run when the negated if condition is true, which the original empty if body never
        // did, so an `if` with a kept `elseif` and an `else` keeps its condition
        let edits = if !dead || (!kept.is_empty() && else_.is_some()) {
            if !empty_else && !elseifs_changed {
                return;
            }
            if elseifs_changed && kept.is_empty() {
                // an emptied `elseifs` list: php-parser prints the `if` anew
                let condition = &source[range(unparenthesize(view.condition))];
                let mut text = format!(
                    "if ({condition}) {}",
                    braced(ctx, &view.stmts, indent)
                );
                if let Some((else_list, else_span)) = else_ {
                    text.push(' ');
                    text.push_str(&else_text(ctx, &view, else_list, *else_span, indent));
                }
                vec![Edit::new(span, text)]
            } else {
                kept_edits(ctx, span.start, &view, &kept, empty_else)
            }
        } else if let Some((&first, rest)) = kept.split_first() {
            let (condition, first_list, _) = &view.elseifs[first];
            let condition = format!(
                "{} && {}",
                inverted_condition(source, view.condition),
                and_operand(source, condition)
            );
            let mut text = format!("if ({condition}) {}", braced(ctx, first_list, indent));
            for &i in rest {
                text.push(' ');
                text.push_str(&elseif_text(ctx, &view, i, indent));
            }
            if let Some((else_list, else_span)) = else_ {
                text.push(' ');
                text.push_str(&else_text(ctx, &view, else_list, *else_span, indent));
            }
            vec![Edit::new(span, text)]
        } else if let Some((else_list, _)) = else_ {
            let condition = inverted_condition(source, view.condition);
            vec![Edit::new(
                span,
                format!("if ({condition}) {}", braced(ctx, else_list, indent)),
            )]
        } else {
            remove(ctx, &list, &[index])
        };
        out.push(Report {
            span,
            message: "This `if` has branches that do nothing; remove them.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

fn range(expr: &Expression<'_>) -> std::ops::Range<usize> {
    let span = node_span(expr);
    span.start..span.end
}

/// An operand on the right of a new `&&`: in parentheses where php-parser's printer puts them, around an operator
/// that binds as loosely as `&&` or looser. An assignment stays bare, as php-parser prints it there.
fn and_operand(source: &str, expr: &Expression<'_>) -> String {
    let inner = unparenthesize(expr);
    let text = &source[range(inner)];
    if !matches!(inner, Expression::Assignment(_)) && operand_precedence(inner) <= 7 {
        format!("({text})")
    } else {
        text.to_string()
    }
}

/// A kept `elseif` after a body printed anew: as written, or in braces when the `if` uses the alternative syntax.
fn elseif_text(ctx: &crate::Context<'_>, view: &IfView<'_>, i: usize, indent: &str) -> String {
    let source = ctx.source;
    let (condition, list, span) = &view.elseifs[i];
    if view.colon {
        let condition = &source[range(unparenthesize(condition))];
        return format!("elseif ({condition}) {}", braced(ctx, list, indent));
    }
    source[span.start..span.end].to_string()
}

/// [`elseif_text`] for the `else`.
fn else_text(ctx: &crate::Context<'_>, view: &IfView<'_>, list: &List<'_>, span: Span, indent: &str) -> String {
    let source = ctx.source;
    if view.colon {
        return format!("else {}", braced(ctx, list, indent));
    }
    source[span.start..span.end].to_string()
}

/// php-parser's format-preserving removal of the `elseif`s not in `kept` (at least one is) and, with `empty_else`, of
/// the `else`. A branch's comments are part of it.
fn kept_edits(
    ctx: &crate::Context<'_>,
    from: usize,
    view: &IfView<'_>,
    kept: &[usize],
    empty_else: bool,
) -> Vec<Edit> {
    let source = ctx.source;
    let starts: Vec<usize> = (view.elseifs.iter())
        .map(|(_, _, span)| {
            let before = previous_token_end(ctx, from, span.start);
            first_comment(source, before, span.start).unwrap_or(span.start)
        })
        .collect();
    let ends: Vec<usize> = view.elseifs.iter().map(|(_, _, span)| span.end).collect();
    let mut edits = Vec::new();
    if let Some(&first) = kept.first() {
        if first > 0 {
            edits.push(Edit::new(Span::new(starts[0], starts[first]), ""));
        }
        for i in first + 1..view.elseifs.len() {
            if !kept.contains(&i) {
                edits.push(Edit::new(Span::new(ends[i - 1], ends[i]), ""));
            }
        }
    }
    if empty_else && let Some((_, span)) = &view.else_ {
        let before = previous_token_end(ctx, from, span.start);
        edits.push(Edit::new(Span::new(before, span.end), ""));
    }
    edits
}
