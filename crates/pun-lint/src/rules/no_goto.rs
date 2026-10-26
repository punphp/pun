//! `lint/complexity/no-goto`, ported from Mago 1.57.0's `no-goto` (mago-linter src/rule/maintainability/no_goto.rs).
//!
//! Reports every `goto` (on its keyword) and every label.

use mago_span::HasSpan;
use mago_syntax::cst::{Node, NodeKind};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoGoto;

impl Rule for NoGoto {
    fn id(&self) -> &'static str {
        "lint/complexity/no-goto"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Goto, NodeKind::Label])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (at, message, help) = match node {
            Node::Goto(goto) => (
                goto.goto.span(),
                "Avoid using `goto`.",
                "Refactor the code to use loops or conditional statements instead of `goto`.",
            ),
            Node::Label(label) => (
                label.span(),
                "`goto` labels are discouraged.",
                "Refactor the code to use structured control flow like loops or functions to eliminate the need for this label.",
            ),
            _ => return,
        };
        out.push(Report {
            span: span(at),
            message: message.into(),
            help: Some(help.into()),
            fix: None,
        });
    }
}
