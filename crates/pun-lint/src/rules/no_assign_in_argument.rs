//! `lint/functions/no-assign-in-argument`, ported from Mago 1.57.0's linter rule `no-assign-in-argument`
//! (rule/correctness/no_assign_in_argument.rs).
//!
//! An argument that is an assignment, through any parentheses (`f($a = 1)`, `f(x: ($a .= 'b'))`), is reported, in a
//! call's argument list and in a partial application's.

use mago_syntax::cst::{Expression, Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoAssignInArgument;

impl Rule for NoAssignInArgument {
    fn id(&self) -> &'static str {
        "lint/functions/no-assign-in-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ArgumentList, NodeKind::PartialArgumentList])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let values: Vec<&Expression<'_>> = match node {
            Node::PartialArgumentList(list) => {
                list.arguments.iter().filter_map(|a| a.value()).collect()
            }
            _ => {
                let Node::ArgumentList(l) = node else { return };
                l.arguments
                    .iter()
                    .map(mago_syntax::cst::Argument::value)
                    .collect()
            }
        };
        for value in values {
            if let Some(assignment) = assignment(value) {
                out.push(Report {
                    span: node_span(assignment),
                    message: "Avoid assignments in function call arguments.".to_string(),
                    help: Some(
                        "Consider assigning the variable before the function call.".to_string(),
                    ),
                    fix: None,
                });
            }
        }
    }
}

fn assignment<'a>(expression: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    match expression {
        Expression::Parenthesized(p) => assignment(p.expression),
        Expression::Assignment(_) => Some(expression),
        _ => None,
    }
}
