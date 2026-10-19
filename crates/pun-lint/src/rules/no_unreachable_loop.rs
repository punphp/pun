//! `lint/style/no-unreachable-loop`, ported from Mago 1.57.0's linter rule `loop-does-not-iterate`
//! (rule/best_practices/loop_does_not_iterate.rs).
//!
//! A `for`, `foreach`, `while` or `do`-`while` whose body ends its first pass with an unconditional `break` (of one
//! level) or `return`, with no `continue` or `goto` before it that could skip it, is reported on the whole loop.

use mago_syntax::cst::{
    DeclareBody, Expression, ForBody, ForeachBody, IfBody, Literal, LiteralInteger, Node, NodeKind,
    Statement, WhileBody,
};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUnreachableLoop;

impl Rule for NoUnreachableLoop {
    fn id(&self) -> &'static str {
        "lint/style/no-unreachable-loop"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::For,
            NodeKind::Foreach,
            NodeKind::While,
            NodeKind::DoWhile,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let terminates = match node {
            Node::For(l) => match &l.body {
                ForBody::Statement(s) => terminator(s),
                ForBody::ColonDelimited(b) => terminator_in(b.statements.as_slice()),
            },
            Node::Foreach(l) => match &l.body {
                ForeachBody::Statement(s) => terminator(s),
                ForeachBody::ColonDelimited(b) => terminator_in(b.statements.as_slice()),
            },
            Node::While(l) => match &l.body {
                WhileBody::Statement(s) => terminator(s),
                WhileBody::ColonDelimited(b) => terminator_in(b.statements.as_slice()),
            },
            _ => {
                let Node::DoWhile(l) = node else { return };
                terminator(l.statement)
            }
        };
        if !terminates {
            return;
        }
        out.push(Report {
            span: node_span(&node),
            message: "Loop is unconditionally terminated and will not iterate.".to_string(),
            help: Some("Consider refactoring the code. If the loop is intended to run only once, an `if` statement might be clearer.".to_string()),
            fix: None,
        });
    }
}

/// `get_loop_terminator_from_statements`.
fn terminator_in(statements: &[Statement<'_>]) -> bool {
    for statement in statements {
        if might_skip_terminator(statement) {
            return false;
        }
        if terminator(statement) {
            return true;
        }
    }
    false
}

/// `get_loop_terminator_from_statement`.
fn terminator(statement: &Statement<'_>) -> bool {
    match statement {
        Statement::Block(block) => terminator_in(block.statements.as_slice()),
        Statement::Break(b) => matches!(
            b.level,
            None | Some(Expression::Literal(Literal::Integer(LiteralInteger {
                value: Some(1),
                ..
            })))
        ),
        Statement::Return(_) => true,
        _ => false,
    }
}

fn any(statements: &[Statement<'_>]) -> bool {
    statements.iter().any(might_skip_terminator)
}

/// `might_skip_terminator`, without Mago's arm for a namespace: PHP allows no namespace inside a loop, and Pun lints
/// only code PHP parses.
fn might_skip_terminator(statement: &Statement<'_>) -> bool {
    match statement {
        Statement::Continue(_) | Statement::Goto(_) => true,
        Statement::Block(block) => any(block.statements.as_slice()),
        Statement::If(r#if) => match &r#if.body {
            IfBody::Statement(body) => {
                might_skip_terminator(body.statement)
                    || body
                        .else_clause
                        .as_ref()
                        .is_some_and(|c| might_skip_terminator(c.statement))
                    || body
                        .else_if_clauses
                        .iter()
                        .any(|c| might_skip_terminator(c.statement))
            }
            IfBody::ColonDelimited(body) => {
                any(body.statements.as_slice())
                    || body
                        .else_clause
                        .as_ref()
                        .is_some_and(|c| any(c.statements.as_slice()))
                    || body
                        .else_if_clauses
                        .iter()
                        .any(|c| any(c.statements.as_slice()))
            }
        },
        Statement::While(l) => match &l.body {
            WhileBody::Statement(body) => might_skip_terminator(body),
            WhileBody::ColonDelimited(body) => any(body.statements.as_slice()),
        },
        Statement::DoWhile(l) => might_skip_terminator(l.statement),
        Statement::For(l) => match &l.body {
            ForBody::Statement(body) => might_skip_terminator(body),
            ForBody::ColonDelimited(body) => any(body.statements.as_slice()),
        },
        Statement::Foreach(l) => match &l.body {
            ForeachBody::Statement(body) => might_skip_terminator(body),
            ForeachBody::ColonDelimited(body) => any(body.statements.as_slice()),
        },
        Statement::Declare(declare) => match &declare.body {
            DeclareBody::Statement(body) => might_skip_terminator(body),
            DeclareBody::ColonDelimited(body) => any(body.statements.as_slice()),
        },
        Statement::Try(r#try) => {
            any(r#try.block.statements.as_slice())
                || r#try
                    .catch_clauses
                    .iter()
                    .any(|c| any(c.block.statements.as_slice()))
                || r#try
                    .finally_clause
                    .as_ref()
                    .is_some_and(|c| any(c.block.statements.as_slice()))
        }
        Statement::Switch(switch) => switch
            .body
            .cases()
            .iter()
            .any(|case| any(case.statements())),
        _ => false,
    }
}
