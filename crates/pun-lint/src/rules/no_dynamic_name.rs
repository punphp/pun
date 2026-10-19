//! `lint/style/no-dynamic-name`, ported from Mago 1.57.0's linter rule `no-variable-variable`
//! (rule/clarity/no_variable_variable.rs).
//!
//! A variable variable (`$$name`) is reported; `${expr}` is not (Mago looks at nested variables only).

use mago_syntax::cst::{Node, NodeKind, Variable};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoDynamicName;

impl Rule for NoDynamicName {
    fn id(&self) -> &'static str {
        "lint/style/no-dynamic-name"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Variable])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Variable(Variable::Nested(nested)) = node else {
            return;
        };
        out.push(Report {
            span: node_span(nested),
            message: "Usage of variable variables (e.g., `$$var`) is discouraged.".to_string(),
            help: Some("Refactor the code to avoid using variable variables.".to_string()),
            fix: None,
        });
    }
}
