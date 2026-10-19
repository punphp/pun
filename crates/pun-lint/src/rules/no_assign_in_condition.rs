//! `lint/control-flow/no-assign-in-condition`, ported from Mago 1.57.0's `no-assign-in-condition`
//! (mago-linter src/rule/correctness/no_assign_in_condition.rs), with its default `ignore-while-statements = false`.
//!
//! Reports an assignment that is the whole condition (inside any parentheses) of an `if`, `elseif`, `while` or
//! `do ... while`.

use mago_span::HasSpan;
use mago_syntax::cst::{Expression, Node, NodeKind};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoAssignInCondition;

impl Rule for NoAssignInCondition {
    fn id(&self) -> &'static str {
        "lint/control-flow/no-assign-in-condition"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::If,
            NodeKind::While,
            NodeKind::DoWhile,
            NodeKind::IfStatementBodyElseIfClause,
            NodeKind::IfColonDelimitedBodyElseIfClause,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let condition = match node {
            Node::If(r#if) => r#if.condition,
            Node::While(r#while) => r#while.condition,
            Node::DoWhile(do_while) => do_while.condition,
            Node::IfStatementBodyElseIfClause(clause) => clause.condition,
            Node::IfColonDelimitedBodyElseIfClause(clause) => clause.condition,
            _ => return,
        };
        let mut expr = condition;
        while let Expression::Parenthesized(p) = expr {
            expr = p.expression;
        }
        let Expression::Assignment(assignment) = expr else {
            return;
        };
        out.push(Report {
            span: span(assignment.span()),
            message: "Avoid assignments in conditions.".to_string(),
            help: None,
            fix: None,
        });
    }
}
