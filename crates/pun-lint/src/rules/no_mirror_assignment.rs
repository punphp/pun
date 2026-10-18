//! `lint/dead-code/no-mirror-assignment`, ported from Rector 2.6.1's `SimplifyMirrorAssignRector`
//! (rules/DeadCode/Rector/Expression).
//!
//! A statement that assigns a variable (or any other place) to itself, `$a = $a;`, does nothing and is removed.

use mago_syntax::cst::{Node, NodeKind, Statement};
use pun_problem::{Fix, Safety};

use super::rector_code_quality::equal;
use super::rector_stmts::{assign, enclosing, remove};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoMirrorAssignment;

impl Rule for NoMirrorAssignment {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-mirror-assignment"
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
        let Node::Statement(s @ Statement::Expression(e)) = node else {
            return;
        };
        let Some(assign) = assign(e.expression) else {
            return;
        };
        if !equal(ctx, assign.lhs, assign.rhs) {
            return;
        }
        let (list, index) = enclosing(ctx.source, s, ancestors);
        out.push(Report {
            span: node_span(s),
            message: "This assignment assigns a value to itself.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: remove(ctx.source, &list, &[index]),
            }),
        });
    }
}
