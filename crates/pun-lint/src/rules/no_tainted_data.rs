//! `lint/security/no-tainted-data`, ported from Mago 1.57.0's linter rule `tainted-data-to-sink`
//! (rule/security/tainted_data_to_sink.rs), with its default sinks: `echo`, `print` and `printf`.
//!
//! A value that comes straight from a request superglobal (`is_user_input`) given to `echo`, `print` or `printf` is
//! reported on the value.

use mago_syntax::cst::{Expression, Node, NodeKind};

use super::lint_b_names::function_call_matches;
use super::lint_b_security::is_user_input;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoTaintedData;

impl Rule for NoTaintedData {
    fn id(&self) -> &'static str {
        "lint/security/no-tainted-data"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Echo,
            NodeKind::PrintConstruct,
            NodeKind::FunctionCall,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        match node {
            Node::Echo(echo) => echo.values.iter().for_each(|v| check(v, out)),
            Node::PrintConstruct(print) => check(print.value, out),
            _ => {
                let Node::FunctionCall(c) = node else { return };
                if function_call_matches(ancestors, c, "printf") {
                    c.argument_list
                        .arguments
                        .iter()
                        .for_each(|a| check(a.value(), out));
                }
            }
        }
    }
}

fn check(value: &Expression<'_>, out: &mut Vec<Report>) {
    if !is_user_input(value) {
        return;
    }
    out.push(Report {
        span: node_span(value),
        message: "Tainted data passed to a sink function/construct.".to_string(),
        help: Some("Ensure the data is validated or escaped prior to using this sink.".to_string()),
        fix: None,
    });
}
