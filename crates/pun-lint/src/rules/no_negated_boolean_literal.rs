//! `lint/complexity/no-negated-boolean-literal`, ported from Rector 2.8.4's `ReplaceConstantBooleanNotRector`
//! (rules/CodeQuality/Rector/BooleanNot).
//!
//! `!false` becomes `true` and `!true` becomes `false`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::is_constant;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoNegatedBooleanLiteral;

impl Rule for NoNegatedBooleanLiteral {
    fn id(&self) -> &'static str {
        "lint/complexity/no-negated-boolean-literal"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::UnaryPrefix(not) = node else { return };
        if !not.operator.is_not() {
            return;
        }
        let value = if is_constant(not.operand, "false") {
            "true"
        } else if is_constant(not.operand, "true") {
            "false"
        } else {
            return;
        };
        let span = node_span(not);
        out.push(Report {
            span,
            message: format!("Use `{value}` instead of negating a boolean literal."),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, value)],
            }),
        });
    }
}
