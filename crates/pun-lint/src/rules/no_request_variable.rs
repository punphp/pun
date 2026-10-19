//! `lint/security/no-request-variable`, ported from Mago 1.57.0's linter rule `no-request-variable`
//! (rule/safety/no_request_variable.rs).
//!
//! `$_REQUEST` mixes `$_GET`, `$_POST` and `$_COOKIE`; each use is reported.

use mago_syntax::cst::{Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoRequestVariable;

impl Rule for NoRequestVariable {
    fn id(&self) -> &'static str {
        "lint/security/no-request-variable"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::DirectVariable])
    }

    fn visit<'ast, 'arena>(
        &self,
        n: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::DirectVariable(v) = n else { return };
        if v.name != b"$_REQUEST" {
            return;
        }
        out.push(Report {
            span: node_span(v),
            message: "Unsafe use of `$_REQUEST` variable.".to_string(),
            help: Some(
                "Use `$_GET`, `$_POST`, or `$_COOKIE` instead for better clarity.".to_string(),
            ),
            fix: None,
        });
    }
}
