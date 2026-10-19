//! `lint/control-flow/no-unsafe-finally`, ported from Mago 1.57.0's `no-unsafe-finally`
//! (mago-linter src/rule/safety/no_unsafe_finally.rs).
//!
//! Reports a `return`, `throw`, `continue` or `break` in a `finally` block, which overrides the control flow of the
//! `try` and `catch` blocks.

use mago_span::HasSpan;
use mago_syntax::cst::{Node, NodeKind};
use mago_syntax::utils::control_flow::find_control_flows_in_block;

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoUnsafeFinally;

impl Rule for NoUnsafeFinally {
    fn id(&self) -> &'static str {
        "lint/control-flow/no-unsafe-finally"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Try])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Try(r#try) = node else { return };
        let Some(finally) = r#try.finally_clause.as_ref() else {
            return;
        };
        for control_flow in find_control_flows_in_block(&finally.block) {
            out.push(Report {
                span: span(control_flow.span()),
                message: "Unsafe control flow in finally block.".to_string(),
                help: Some("Avoid using control flow statements in `finally` blocks.".to_string()),
                fix: None,
            });
        }
    }
}
