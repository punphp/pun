//! `lint/style/no-multi-assign`, ported from Rector 2.6.1's `SplitDoubleAssignRector`
//! (rules/CodingStyle/Rector/Assign).
//!
//! `$a = $b = value;` becomes one statement per variable, `$a = value; $b = value;`. A call (or `new`) is not
//! repeated: the variables after the first take the first variable instead, `$a = f(); $b = $a;`. A chain that writes
//! an array item stays. Comments before the statement go with the first new one.
//!
//! Rector also splits a chain whose inner link is `=&` or a compound assignment, putting that whole link in every new
//! statement; Pun leaves such a chain alone, since its effect would run once per variable.

use mago_syntax::cst::{Assignment, AssignmentOperator, Expression, Node, NodeKind, Statement};
use pun_problem::{Fix, Safety};

use super::lint_b_helpers::replace_run;
use super::rector_code_quality::text;
use super::rector_stmts::{Frame, enclosing, line_indent, replace_statement};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoMultiAssign;

impl Rule for NoMultiAssign {
    fn id(&self) -> &'static str {
        "lint/style/no-multi-assign"
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
        let Node::Statement(stmt) = node else {
            return;
        };
        let Some(lines) = split(ctx, stmt) else {
            return;
        };
        let (list, index) = enclosing(ctx.source, stmt, ancestors);
        let edit = if list.frame == Frame::Bare {
            replace_statement(ctx.source, &list, index, |indent| {
                lines.join(&format!("\n{indent}"))
            })
        } else {
            let span = list.with_comments(ctx.source, index);
            let comments = ctx.source[span.start..node_span(stmt).start].trim_end();
            let indent = line_indent(ctx.source, node_span(stmt).start);
            let mut new = String::new();
            if !comments.is_empty() {
                new.push_str(comments);
                new.push('\n');
                new.push_str(indent);
            }
            new.push_str(&lines.join(&format!("\n{indent}")));
            replace_run(ctx.source, &list, index, index, &new)
        };
        out.push(Report {
            span: node_span(stmt),
            message: "Assign one variable per statement.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![edit],
            }),
        });
    }
}

fn plain<'a>(expr: &'a Expression<'a>) -> Option<&'a Assignment<'a>> {
    match expr {
        Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Assign(_)) => Some(a),
        _ => None,
    }
}

/// `refactor()`: the new statements for `$a = $b = ... = value;`.
fn split(ctx: &Context<'_>, stmt: &Statement<'_>) -> Option<Vec<String>> {
    let Statement::Expression(e) = stmt else {
        return None;
    };
    let first = plain(e.expression)?;
    plain(first.rhs)?;
    let mut chain = vec![first];
    let mut value = first.rhs;
    while let Some(a) = plain(value) {
        chain.push(a);
        value = a.rhs;
    }
    if matches!(value, Expression::Assignment(_)) {
        return None;
    }
    let value = unparenthesize(value);
    let mut expr = match value {
        // php-parser prints a low-precedence `and`, `or`, `xor` on the right of `=` in parentheses.
        Expression::Binary(b) if b.operator.is_low_precedence() => {
            format!("({})", text(ctx, value))
        }
        _ => text(ctx, value).to_string(),
    };
    let mut called = matches!(value, Expression::Call(_) | Expression::Instantiation(_));
    let mut lines = Vec::new();
    for a in chain {
        if matches!(
            a.lhs,
            Expression::ArrayAccess(_) | Expression::ArrayAppend(_)
        ) {
            return None;
        }
        lines.push(format!("{} = {expr};", text(ctx, a.lhs)));
        // Only the call itself is replaced: later variables take the first one.
        if called {
            expr = text(ctx, a.lhs).to_string();
            called = false;
        }
    }
    Some(lines)
}
