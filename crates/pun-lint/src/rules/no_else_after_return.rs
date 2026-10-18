//! `lint/control-flow/no-else-after-return`, ported from Rector 2.6.1's `RemoveAlwaysElseRector`
//! (rules/EarlyReturn/Rector/If_).
//!
//! When a branch of an `if` ends the flow (`return`, `throw`, `continue`, `exit`), the branch after it needs no
//! `else`: the `else` statements move after the `if`, and an `elseif` after such a branch becomes an `if` of its own.
//! Rector's printer writes the new `if`s anew, a statement a line, and an `elseif` or `else` it keeps from the old
//! `if` on a line of its own after the `}`.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_stmts::{
    Frame, IfView, List, braced, comments, enclosing, if_view, inserted_statements,
    is_exit_or_throw, line_indent, moved_statements, replace_statement,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoElseAfterReturn;

impl Rule for NoElseAfterReturn {
    fn id(&self) -> &'static str {
        "lint/control-flow/no-else-after-return"
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
        // A cheap look first: an `elseif` or an `else` to remove.
        let has_more = match &r#if.body {
            IfBody::Statement(b) => !b.else_if_clauses.is_empty() || b.else_clause.is_some(),
            IfBody::ColonDelimited(b) => !b.else_if_clauses.is_empty() || b.else_clause.is_some(),
        };
        if !has_more {
            return;
        }
        let view = if_view(source, r#if);
        if does_not_last_statement_break_flow(source, &view.stmts) {
            return;
        }
        let (list, index) = enclosing(source, s, ancestors);
        let items = if view.elseifs.is_empty() {
            // Without `elseif`s, there is an `else`.
            let (else_stmts, _) = view.else_.as_ref().unwrap();
            let mut items = vec![Item::Kept];
            if !else_stmts.is_empty() {
                items.push(Item::Stmts(else_stmts));
            }
            items
        } else {
            handle_else_ifs(source, &view)
        };
        let clauses = clauses(source, &view);
        let inserted = list.frame != Frame::Bare;
        let edit = if view.elseifs.is_empty() || !inserted {
            replace_statement(source, &list, index, |indent| {
                items
                    .iter()
                    .map(|item| print(ctx, r#if, &clauses, item, indent, inserted))
                    .collect::<Vec<_>>()
                    .join(&format!("\n{indent}"))
            })
        } else {
            // The printer puts the nodes before the kept `if` (the modified one) on lines of their own, the first
            // with the comments of the old `if`, and leaves the kept `if` its old whitespace.
            let before = list.before(source, index);
            let start = list.with_comments(source, index).start;
            let indent = line_indent(source, node_span(r#if).start);
            let mut text = String::new();
            for item in &items {
                if matches!(item, Item::ModifiedIf(..)) {
                    text.push_str(&source[before..start]);
                } else {
                    text.push_str(&format!("\n{indent}"));
                    if matches!(item, Item::NewIf(0)) && start < node_span(r#if).start {
                        text.push_str(&format!(
                            "{}\n{indent}",
                            source[start..node_span(r#if).start].trim_end()
                        ));
                    }
                }
                text.push_str(&print(ctx, r#if, &clauses, item, indent, inserted));
            }
            Edit::new(Span::new(before, node_span(r#if).end), text)
        };
        out.push(Report {
            span: node_span(r#if),
            message: "This `else` follows a branch that always leaves; it is not needed."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}

/// One node of what `refactor()` returns in place of the `if`.
enum Item<'v, 'a> {
    /// The `if` itself, its `else` removed.
    Kept,
    /// A new `if` with the condition and statements of a clause (0 is the `if`, then the `elseif`s), its comments.
    NewIf(usize),
    /// The `if` with a clause's condition and statements, and the `elseif`s it keeps.
    ModifiedIf(usize, Vec<usize>),
    /// A clause printed on its own: an `elseif`, or the `else` (`usize::MAX`).
    Clause(usize),
    /// The statements of the `else`.
    Stmts(&'v List<'a>),
}

/// A clause of the `if`: its condition (none for the `else`), its statements and the comments before it.
struct Clause<'v, 'a> {
    condition: Option<&'a Expression<'a>>,
    stmts: &'v List<'a>,
    comments: Option<Span>,
}

fn clauses<'v, 'a>(source: &str, view: &'v IfView<'a>) -> Vec<Clause<'v, 'a>> {
    let mut out = vec![Clause {
        condition: Some(view.condition),
        stmts: &view.stmts,
        comments: None,
    }];
    let mut previous = view.stmts.inner.end;
    for (condition, stmts, span) in &view.elseifs {
        out.push(Clause {
            condition: Some(condition),
            stmts,
            comments: comments(source, previous, span.start),
        });
        previous = stmts.inner.end;
    }
    if let Some((stmts, span)) = &view.else_ {
        out.push(Clause {
            condition: None,
            stmts,
            comments: comments(source, previous, span.start),
        });
    }
    out
}

/// `handleElseIfs`, on clause numbers: 0 is the `if`, 1.. the `elseif`s.
fn handle_else_ifs<'v, 'a>(source: &str, view: &'v IfView<'a>) -> Vec<Item<'v, 'a>> {
    let breaks =
        |clause: usize| !does_not_last_statement_break_flow(source, clause_stmts(view, clause));
    let mut items = vec![Item::NewIf(0)];
    let mut remaining: Vec<usize> = (1..=view.elseifs.len()).collect();
    while !remaining.is_empty() {
        let current = remaining.remove(0);
        // When the last statement of the `elseif` does not end the flow, it takes the `if`'s place and the `elseif`s
        // after it that end the flow (or are empty) are printed after it, on their own.
        if !breaks(current) {
            let (kept, own): (Vec<usize>, Vec<usize>) = remaining
                .iter()
                .partition(|&&e| !breaks(e) && !clause_stmts(view, e).is_empty());
            items.push(Item::ModifiedIf(current, kept));
            items.extend(own.into_iter().map(Item::Clause));
            break;
        }
        if remaining.is_empty() {
            items.push(Item::ModifiedIf(current, Vec::new()));
            break;
        }
        items.push(Item::NewIf(current));
    }
    if let Some((else_stmts, _)) = &view.else_ {
        let keep_else = items.iter().any(
            |item| matches!(item, Item::NewIf(c) | Item::ModifiedIf(c, _) | Item::Clause(c) if !breaks(*c)),
        );
        if keep_else {
            items.push(Item::Clause(usize::MAX));
        } else if !else_stmts.is_empty() {
            items.push(Item::Stmts(else_stmts));
        }
    }
    items
}

fn clause_stmts<'v, 'a>(view: &'v IfView<'a>, clause: usize) -> &'v List<'a> {
    if clause == 0 {
        &view.stmts
    } else {
        &view.elseifs[clause - 1].1
    }
}

fn print(
    ctx: &Context<'_>,
    r#if: &If<'_>,
    clauses: &[Clause<'_, '_>],
    item: &Item<'_, '_>,
    indent: &str,
    inserted: bool,
) -> String {
    let source = ctx.source;
    let head = |clause: &Clause<'_, '_>| {
        clause
            .comments
            .map(|c| format!("{}\n{indent}", &source[c.start..c.end]))
            .unwrap_or_default()
    };
    let condition = |clause: &Clause<'_, '_>| {
        let c = unparenthesize(clause.condition.unwrap());
        let s = node_span(c);
        &source[s.start..s.end]
    };
    match item {
        Item::Kept => {
            let IfBody::Statement(body) = &r#if.body else {
                let clause = &clauses[0];
                return format!(
                    "if ({}) {}",
                    condition(clause),
                    braced(source, clause.stmts, indent)
                );
            };
            let span = node_span(r#if);
            source[span.start..node_span(body.statement).end].to_string()
        }
        Item::NewIf(c) | Item::ModifiedIf(c, _) => {
            let clause = &clauses[*c];
            let mut text = format!(
                "{}if ({}) {}",
                head(clause),
                condition(clause),
                braced(source, clause.stmts, indent)
            );
            if let Item::ModifiedIf(_, kept) = item {
                // php-parser's `pStmt_If` writes ` elseif` after the `}`; the comments before an `elseif` it keeps come
                // first, the `elseif` on the next line.
                for &e in kept {
                    text.push(' ');
                    text.push_str(&print_clause(ctx, &clauses[e], indent));
                }
            }
            text
        }
        Item::Clause(c) => {
            let clause = if *c == usize::MAX {
                clauses.last().unwrap()
            } else {
                &clauses[*c]
            };
            print_clause(ctx, clause, indent)
        }
        Item::Stmts(list) if inserted => inserted_statements(source, list, indent),
        Item::Stmts(list) => moved_statements(source, list, indent),
    }
}

/// An `elseif` or `else` printed on its own, with the comments before it.
fn print_clause(ctx: &Context<'_>, clause: &Clause<'_, '_>, indent: &str) -> String {
    let source = ctx.source;
    let head = clause
        .comments
        .map(|c| format!("{}\n{indent}", &source[c.start..c.end]))
        .unwrap_or_default();
    match clause.condition {
        Some(condition) => {
            let s = node_span(unparenthesize(condition));
            format!(
                "{head}elseif ({}) {}",
                &source[s.start..s.end],
                braced(source, clause.stmts, indent)
            )
        }
        None => format!("{head}else {}", braced(source, clause.stmts, indent)),
    }
}

/// `doesNotLastStatementBreakFlow`: the last statement is not a `return`, `throw`, `continue` or `exit`, nor an
/// `if`/`else` whose every branch ends in one.
fn does_not_last_statement_break_flow(source: &str, stmts: &List<'_>) -> bool {
    match stmts.last() {
        Some(Statement::If(inner)) => {
            let view = if_view(source, inner);
            match &view.else_ {
                Some((else_stmts, _)) => {
                    does_not_last_statement_break_flow(source, &view.stmts)
                        || does_not_last_statement_break_flow(source, else_stmts)
                        || view
                            .elseifs
                            .iter()
                            .any(|(_, l, _)| does_not_last_statement_break_flow(source, l))
                }
                None => true,
            }
        }
        Some(Statement::Return(_) | Statement::Continue(_)) => false,
        Some(Statement::Expression(e)) => !is_exit_or_throw(e.expression),
        _ => true,
    }
}
