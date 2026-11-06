//! `lint/style/prefer-early-return-over-prepared-value`, ported from Rector 2.6.1's
//! `PreparedValueToEarlyReturnRector` (rules/EarlyReturn/Rector/Return_), with `IfManipulator::isIfWithoutElseAndElseIfs`
//! and the `BareSingleAssignIf` value object.
//!
//! A variable set to a default, then set by `if`s that each hold only that assignment, then returned, becomes early
//! returns: each `if` returns its value, the default assignment goes and the `return` returns the default.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_code_quality::equal;
use super::rector_stmts::{
    HOLDERS, List, assign, expression, has_equal_variable, holder_list, if_view, is_plain_assign,
    remove,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEarlyReturnOverPreparedValue;

impl Rule for PreferEarlyReturnOverPreparedValue {
    fn id(&self) -> &'static str {
        "lint/style/prefer-early-return-over-prepared-value"
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
        // A cheap look first: an `if` and a `return`.
        let raw = crate::syntax::statements(node);
        if !raw.iter().any(|s| matches!(s, Statement::If(_)))
            || !raw.iter().any(|s| matches!(s, Statement::Return(_)))
        {
            return;
        }
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        let Some((edits, span)) = refactor(ctx, &list) else {
            return;
        };
        out.push(Report {
            span,
            message: "Return early from each `if` instead of preparing the value to return."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// An `if` with no `else` whose only statement assigns: `BareSingleAssignIf`.
struct BareSingleAssignIf<'a> {
    condition: &'a Expression<'a>,
    statement: &'a Statement<'a>,
    /// Where the statement's comments start.
    comments_start: usize,
    assign: &'a Assignment<'a>,
}

fn refactor(ctx: &Context<'_>, list: &List<'_>) -> Option<(Vec<Edit>, Span)> {
    let source = ctx.source;
    let mut ifs: Vec<(usize, &If<'_>)> = Vec::new();
    let mut initial: Option<(usize, &Assignment<'_>)> = None;
    let mut found = None;
    for (key, &stmt) in list.stmts.iter().enumerate() {
        let expr = expression(stmt);
        if let Some(Expression::Assignment(a)) = expr
            && !matches!(a.operator, AssignmentOperator::Assign(_))
        {
            return None;
        }
        if matches!(
            stmt,
            Statement::For(_) | Statement::Foreach(_) | Statement::While(_) | Statement::DoWhile(_)
        ) && let Some((_, initial)) = initial
            && reassigns(ctx, Node::Statement(stmt), initial.lhs)
        {
            return None;
        }
        if let Statement::If(r#if) = stmt {
            ifs.push((key, r#if));
            continue;
        }
        if let Some(a) = expr.and_then(assign) {
            initial = Some((key, a));
            ifs.clear();
            continue;
        }
        let Statement::Return(r#return) = stmt else {
            continue;
        };
        found = Some(r#return);
        break;
    }
    // The cheap look found a `return` in the list.
    let r#return = found.unwrap();
    // Match the exact variable.
    let returned = r#return.value.map(unparenthesize)?;
    if !matches!(returned, Expression::Variable(_)) {
        return None;
    }
    let (position, initial) = initial?;
    let bare: Vec<BareSingleAssignIf<'_>> = ifs
        .iter()
        .map(|&(k, r#if)| match_bare_single_assign_if(ctx, list, k, r#if))
        .collect::<Option<_>>()?;
    if bare.is_empty() {
        return None;
    }
    if !is_variable_shared(ctx, &bare, returned, initial) {
        return None;
    }
    let mut edits = remove(ctx, list, &[position]);
    for b in &bare {
        let value = node_span(unparenthesize(b.assign.rhs));
        // A new `Return_` in place of the statement: its comments go.
        edits.push(Edit::new(
            Span::new(b.comments_start, node_span(b.statement).end),
            format!("return {};", &source[value.start..value.end]),
        ));
    }
    let value = node_span(unparenthesize(initial.rhs));
    edits.push(Edit::new(
        node_span(r#return.value.unwrap()),
        &source[value.start..value.end],
    ));
    let span = Span::new(
        node_span(list.stmts[position]).start,
        node_span(r#return).end,
    );
    Some((edits, span))
}

/// Whether a loop assigns the variable again (`findFirst` of an `Assign` to an equal variable, anywhere under it).
fn reassigns(ctx: &Context<'_>, node: Node<'_, '_>, var: &Expression<'_>) -> bool {
    if let Node::Assignment(a) = node
        && is_plain_assign(a)
        && equal(ctx, a.lhs, var)
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| found = found || reassigns(ctx, child, var));
    found
}

fn match_bare_single_assign_if<'a>(
    ctx: &crate::Context<'_>,
    list: &List<'a>,
    key: usize,
    r#if: &'a If<'a>,
) -> Option<BareSingleAssignIf<'a>> {
    let source = ctx.source;
    let view = if_view(source, r#if);
    // Exactly one statement, an assignment, and no `else` or `elseif`.
    if view.stmts.len() != 1 {
        return None;
    }
    let statement = view.stmts.stmts[0];
    let assign = expression(statement).and_then(assign)?;
    if view.else_.is_some() || !view.elseifs.is_empty() {
        return None;
    }
    // The next statement is another `if` or the `return`.
    if !matches!(
        list.stmts.get(key + 1),
        Some(Statement::If(_) | Statement::Return(_))
    ) {
        return None;
    }
    Some(BareSingleAssignIf {
        condition: view.condition,
        statement,
        comments_start: view.stmts.with_comments(ctx, 0).start,
        assign,
    })
}

/// `isVariableSharedInAssignIfsAndReturn`.
fn is_variable_shared(
    ctx: &Context<'_>,
    bare: &[BareSingleAssignIf<'_>],
    returned: &Expression<'_>,
    initial: &Assignment<'_>,
) -> bool {
    if !equal(ctx, returned, initial.lhs) {
        return false;
    }
    let target = node_span(returned);
    bare.iter().all(|b| {
        !has_equal_variable(ctx.source, Node::Expression(b.condition), target)
            && !has_equal_variable(ctx.source, Node::Expression(b.assign.rhs), target)
            && equal(ctx, b.assign.lhs, returned)
    })
}
