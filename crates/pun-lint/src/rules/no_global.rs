//! `lint/security/no-global`, ported from Mago 1.57.0's linter rule `no-global` (rule/safety/no_global.rs).
//!
//! The `global` keyword and the `$GLOBALS` variable bring in global state; each use is reported.

use mago_syntax::cst::{Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoGlobal;

impl Rule for NoGlobal {
    fn id(&self) -> &'static str {
        "lint/security/no-global"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Global, NodeKind::DirectVariable])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (span, message, help) = match node {
            Node::Global(global) => (
                node_span(&global.global),
                "Unsafe use of `global` keyword.",
                "Refactor your code to avoid using the `global` keyword.",
            ),
            Node::DirectVariable(variable) if variable.name == b"$GLOBALS" => (
                node_span(variable),
                "Unsafe use of `$GLOBALS` variable.",
                "Refactor your code to avoid using the `$GLOBALS` variable directly.",
            ),
            _ => return,
        };
        out.push(Report {
            span,
            message: message.to_string(),
            help: Some(help.to_string()),
            fix: None,
        });
    }
}
