//! `lint/security/no-eval`, ported from Mago 1.57.0's linter rule `no-eval` (rule/safety/no_eval.rs).
//!
//! Every `eval(...)` is reported on its `eval` keyword: it runs arbitrary code.

use mago_syntax::cst::{Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEval;

impl Rule for NoEval {
    fn id(&self) -> &'static str {
        "lint/security/no-eval"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::EvalConstruct])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::EvalConstruct(e) = node else { return };
        out.push(Report {
            span: node_span(&e.eval),
            message: "Unsafe use of `eval` construct.".to_string(),
            help: Some("Avoid using `eval` unless absolutely necessary, and ensure that any dynamically generated code is properly validated and sanitized before execution.".to_string()),
            fix: None,
        });
    }
}
