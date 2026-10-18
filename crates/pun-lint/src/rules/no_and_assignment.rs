//! `lint/complexity/no-and-assignment`, ported from Rector 2.6.1's `AndAssignsToSeparateLinesRector`
//! (rules/CodeQuality/Rector/LogicalAnd).
//!
//! `$token = 4 and $tokens[] = $token;` becomes two statements, `$token = 4;` and `$tokens[] = $token;`, on lines of
//! their own. Only an expression statement whose whole expression is an `and` of two plain assignments changes.

use mago_syntax::cst::{
    AssignmentOperator, BinaryOperator, Expression, ExpressionStatement, Node, NodeKind,
};
use pun_problem::{Fix, Safety};

use super::rector_code_quality::{statements_edit, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAndAssignment;

impl Rule for NoAndAssignment {
    fn id(&self) -> &'static str {
        "lint/complexity/no-and-assignment"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ExpressionStatement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::ExpressionStatement(statement) = node {
            check(statement, ancestors, ctx, out);
        }
    }
}

fn check(
    s: &ExpressionStatement<'_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
) {
    let Expression::Binary(and) = unparenthesize(s.expression) else {
        return;
    };
    if !matches!(and.operator, BinaryOperator::LowAnd(_)) {
        return;
    }
    let (left, right) = (unparenthesize(and.lhs), unparenthesize(and.rhs));
    if !is_assign(left) || !is_assign(right) {
        return;
    }
    let parts = [text(ctx, left).to_string(), text(ctx, right).to_string()];
    let Some(edit) = statements_edit(ctx, ancestors, s, &parts) else {
        return;
    };
    out.push(Report {
        span: node_span(s),
        message: "Split the two assignments joined by `and` into two statements.".to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits: vec![edit],
        }),
    });
}

/// php-parser's `Assign`: `=`, not a compound assignment or `= &`.
fn is_assign(expr: &Expression<'_>) -> bool {
    matches!(expr, Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_))
        && !matches!(a.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference()))
}
