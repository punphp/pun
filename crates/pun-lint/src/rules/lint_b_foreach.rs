//! What the four `ForeachToArray*Rector` ports share (Rector 2.6.1, rules/Php84/Rector/Foreach_): a `foreach` whose
//! only statement is an `if` that assigns and breaks, or returns, becomes one call of `array_all()`, `array_any()`,
//! `array_find()` or `array_find_key()` with an arrow function of the `if`'s condition.
//!
//! Rector reads only the `if`'s own statements, so it drops an `else` or `elseif` of that `if`; Pun leaves such a
//! `foreach` alone. Rector asks PHPStan whether the loop's subject is natively an array; Pun knows it where the syntax
//! proves it (`lint_b_helpers::is_native_array`).

use mago_syntax::cst::{
    Expression, Foreach, ForeachBody, ForeachTarget, Node, Statement, Variable,
};
use pun_problem::{Edit, Span};

use super::lint_b_helpers::is_native_array;
use super::rector_code_quality::{equal, is_constant, operand, text};
use super::rector_stmts::{List, body_list, expression, holder_list, if_view, line_indent};
use super::used_variable::used_in;
use crate::Context;
use crate::syntax::{node_span, unparenthesize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    All,
    Any,
    Find,
    FindKey,
}

impl Kind {
    fn function(self) -> &'static str {
        match self {
            Kind::All => "array_all",
            Kind::Any => "array_any",
            Kind::Find => "array_find",
            Kind::FindKey => "array_find_key",
        }
    }
}

/// `refactor()` on one statement list: the first `foreach` that matches, as the edits and the span reported.
pub fn refactor(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    list: &List<'_>,
    kind: Kind,
) -> Option<(Vec<Edit>, Span)> {
    let assignment = assignment_pattern(ctx, ancestors, list, kind);
    if assignment.is_some() || matches!(kind, Kind::Find | Kind::FindKey) {
        return assignment;
    }
    early_return_pattern(ctx, ancestors, list, kind)
}

/// The parts of `foreach (subject as [key =>] $value) { if (cond) { ... } }`.
struct Loop<'a> {
    subject: &'a Expression<'a>,
    key: Option<&'a Expression<'a>>,
    value: &'a Expression<'a>,
    condition: &'a Expression<'a>,
    body: Vec<&'a Statement<'a>>,
}

/// The `foreach`'s single statement, an `if` without `else` or `elseif`, and that `if`'s statements.
fn parts<'a>(ctx: &Context<'_>, foreach: &'a Foreach<'a>) -> Option<Loop<'a>> {
    let stmts = match &foreach.body {
        ForeachBody::Statement(s) => body_list(ctx.source, s),
        ForeachBody::ColonDelimited(b) => {
            holder_list(ctx.source, Node::ForeachColonDelimitedBody(b), &[])?
        }
    };
    if stmts.len() != 1 {
        return None;
    }
    let Statement::If(r#if) = stmts.stmts[0] else {
        return None;
    };
    let view = if_view(ctx.source, r#if);
    if view.else_.is_some() || !view.elseifs.is_empty() || view.stmts.nop {
        return None;
    }
    let (key, value) = match &foreach.target {
        ForeachTarget::Value(t) => (None, t.value),
        ForeachTarget::KeyValue(t) => (Some(t.key), t.value),
    };
    Some(Loop {
        subject: foreach.expression,
        key,
        value,
        condition: view.condition,
        body: view.stmts.stmts.clone(),
    })
}

fn is_variable(expr: &Expression<'_>) -> bool {
    matches!(expr, Expression::Variable(Variable::Direct(_)))
}

/// The arrow function's parameters: the value, then the key when the condition uses it
/// (`ForeachKeyUsedInConditionalAnalyzer`).
fn params(ctx: &Context<'_>, l: &Loop<'_>) -> String {
    let mut params = text(ctx, l.value).to_string();
    if let Some(key) = l.key
        && let Expression::Variable(Variable::Direct(k)) = key
        && used_in(Node::Expression(l.condition), None, &k.name[1..])
    {
        params.push_str(", ");
        params.push_str(text(ctx, key));
    }
    params
}

/// `array_*($subject, fn(params) => body)`.
fn call(ctx: &Context<'_>, kind: Kind, l: &Loop<'_>, negate: bool) -> String {
    let body = if negate {
        match unparenthesize(l.condition) {
            Expression::UnaryPrefix(u) if u.operator.is_not() => {
                text(ctx, unparenthesize(u.operand)).to_string()
            }
            c => format!("!{}", operand(ctx, c, 30, None)),
        }
    } else {
        text(ctx, unparenthesize(l.condition)).to_string()
    };
    format!(
        "{}({}, fn({}) => {body})",
        kind.function(),
        text(ctx, unparenthesize(l.subject)),
        params(ctx, l)
    )
}

/// `refactorBooleanAssignmentPattern` (and `ForeachToArrayFind*Rector::refactor`): `$x = <init>;` then a `foreach`
/// whose `if` assigns `$x = <found>;` and breaks.
fn assignment_pattern(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    list: &List<'_>,
    kind: Kind,
) -> Option<(Vec<Edit>, Span)> {
    for (key, stmt) in list.stmts.iter().enumerate() {
        let Statement::Foreach(foreach) = stmt else {
            continue;
        };
        let Some(prev) = key.checked_sub(1).and_then(|k| expression(list.stmts[k])) else {
            continue;
        };
        let Expression::Assignment(prev_assign) = prev else {
            continue;
        };
        if !super::rector_stmts::is_plain_assign(prev_assign) {
            continue;
        }
        let init = match kind {
            Kind::All => "true",
            Kind::Any => "false",
            Kind::Find | Kind::FindKey => "null",
        };
        if !is_constant(prev_assign.rhs, init) || !is_variable(unparenthesize(prev_assign.lhs)) {
            continue;
        }
        let Some(l) = parts(ctx, foreach) else {
            continue;
        };
        if !valid_assignment(ctx, ancestors, foreach, &l, prev_assign.lhs, kind) {
            continue;
        }
        let Expression::Variable(Variable::Direct(value)) = l.value else {
            continue;
        };
        if list.stmts[key + 1..]
            .iter()
            .any(|s| used_in(Node::Statement(s), None, &value.name[1..]))
        {
            continue;
        }
        let new = format!(
            "{} = {};",
            text(ctx, unparenthesize(prev_assign.lhs)),
            call(ctx, kind, &l, kind == Kind::All)
        );
        let edit = replace_run(ctx.source, list, key - 1, key, &new);
        return Some((
            vec![edit],
            Span::new(node_span(list.stmts[key - 1]).start, node_span(*stmt).end),
        ));
    }
    None
}

/// `isValidBooleanAssignmentForeachStructure` and `isValidForeachStructure`.
fn valid_assignment(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    foreach: &Foreach<'_>,
    l: &Loop<'_>,
    assigned: &Expression<'_>,
    kind: Kind,
) -> bool {
    if kind == Kind::FindKey && l.key.is_none() {
        return false;
    }
    let [assignment, brk] = l.body[..] else {
        return false;
    };
    let Some(Expression::Assignment(a)) = expression(assignment) else {
        return false;
    };
    if !super::rector_stmts::is_plain_assign(a)
        || !matches!(brk, Statement::Break(_))
        || !equal(ctx, a.lhs, assigned)
    {
        return false;
    }
    let found = match kind {
        Kind::All => is_constant(a.rhs, "false"),
        Kind::Any => is_constant(a.rhs, "true"),
        Kind::Find => equal(ctx, a.rhs, l.value),
        Kind::FindKey => equal(ctx, a.rhs, l.key.unwrap()),
    };
    found && is_native_array(ancestors, l.subject, node_span(foreach).start)
}

/// `refactorEarlyReturnPattern`: a `foreach` whose `if` returns `false` (`true` for `array_any`), then
/// `return true;` (`false`).
fn early_return_pattern(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    list: &List<'_>,
    kind: Kind,
) -> Option<(Vec<Edit>, Span)> {
    let (inside, after) = if kind == Kind::All {
        ("false", "true")
    } else {
        ("true", "false")
    };
    for (key, stmt) in list.stmts.iter().enumerate() {
        let Statement::Foreach(foreach) = stmt else {
            continue;
        };
        let Some(Statement::Return(next)) = list.stmts.get(key + 1) else {
            continue;
        };
        if !next.value.is_some_and(|v| is_constant(v, after)) {
            continue;
        }
        let Some(l) = parts(ctx, foreach) else {
            continue;
        };
        let [Statement::Return(r)] = l.body[..] else {
            continue;
        };
        if !r.value.is_some_and(|v| is_constant(v, inside))
            || !is_variable(l.value)
            || !is_native_array(ancestors, l.subject, node_span(foreach).start)
        {
            continue;
        }
        let new = format!("return {};", call(ctx, kind, &l, kind == Kind::All));
        let edit = replace_run(ctx.source, list, key, key + 1, &new);
        return Some((
            vec![edit],
            Span::new(node_span(*stmt).start, node_span(next).end),
        ));
    }
    None
}

/// The edit for Rector's removal of the statements `first..=last` of `list` with one new statement added in their
/// place, as php-parser's `pArray()` prints that diff (two removals and an addition, never coalesced into a
/// replacement). After a kept statement, the new one goes on the next line at that statement's indent, so the blanks
/// and comments before the removed statements are lost and those after them are kept. From the list's start, the
/// blanks before the removed statements are kept and the new statement runs straight into the next kept one (or the
/// comments of the trailing `Nop`); with nothing kept after them, php-parser reprints the whole list.
fn replace_run(source: &str, list: &List<'_>, first: usize, last: usize, new: &str) -> Edit {
    let end = node_span(list.stmts[last]).end;
    if first > 0 {
        let prev = node_span(list.stmts[first - 1]);
        let indent = line_indent(source, prev.start);
        return Edit::new(Span::new(prev.end, end), format!("\n{indent}{new}"));
    }
    let start = list.with_comments(source, first).start;
    let next = match list.stmts.get(last + 1) {
        Some(_) => Some(list.with_comments(source, last + 1).start),
        None => list.nop_span(source).map(|s| s.start),
    };
    match next {
        Some(next) => Edit::new(
            Span::new(start, next),
            format!("{new}\n{}", line_indent(source, next)),
        ),
        None => Edit::new(Span::new(start, end), new),
    }
}
