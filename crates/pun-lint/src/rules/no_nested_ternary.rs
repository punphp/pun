//! `lint/style/no-nested-ternary`, ported from Mago 1.57.0's linter rule `no-nested-ternary`
//! (rule/clarity/no_nested_ternary.rs).
//!
//! A full ternary (`a ? b : c`) whose condition or branches hold another full ternary, through any parentheses, is
//! reported on the inner one. Short ternaries (`?:`) are left alone on both sides.

use mago_syntax::cst::{Expression, Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoNestedTernary;

impl Rule for NoNestedTernary {
    fn id(&self) -> &'static str {
        "lint/style/no-nested-ternary"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(c) = node else { return };
        let Some(then) = c.then else { return };
        for expr in [c.condition, then, c.r#else] {
            check_for_nested(expr, out);
        }
    }
}

fn check_for_nested(expr: &Expression<'_>, out: &mut Vec<Report>) {
    match expr {
        Expression::Parenthesized(p) => check_for_nested(p.expression, out),
        Expression::Conditional(nested) if nested.then.is_some() => out.push(Report {
            span: node_span(expr),
            message: "Nested ternary expressions are confusing due to PHP's operator associativity rules.".to_string(),
            help: Some("Consider refactoring the logic into a separate `if/else` statement or a `match` expression for clarity.".to_string()),
            fix: None,
        }),
        _ => {}
    }
}
