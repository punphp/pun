//! `lint/control-flow/no-loop-variable-overwrite`, ported from phpstan-strict-rules 2.0.12's
//! `OverwriteVariablesWithForeachRule` (identifiers `foreach.keyOverwrite` and `foreach.valueOverwrite`).
//!
//! A `foreach` key or value variable (or one in a destructuring value) is reported when the scope already has it
//! for sure, read from the syntax as `no-for-init-overwrite` reads it.

use mago_syntax::cst::{
    Array, ArrayElement, Expression, ForeachTarget, List, Node, NodeKind, UnaryPrefixOperator,
    Variable,
};

use super::no_for_init_overwrite::defined;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoLoopVariableOverwrite;

impl Rule for NoLoopVariableOverwrite {
    fn id(&self) -> &'static str {
        "lint/control-flow/no-loop-variable-overwrite"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Foreach(foreach) = node else { return };
        let value = match &foreach.target {
            ForeachTarget::Value(t) => t.value,
            ForeachTarget::KeyValue(t) => {
                check(node, ancestors, t.key, "key", out);
                t.value
            }
        };
        check_value_var(node, ancestors, value, out);
    }
}

/// `checkValueVar()`: the value variable, or the items of a `list()` or `[]` destructuring.
fn check_value_var(
    foreach: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    out: &mut Vec<Report>,
) {
    match unparenthesize(expr) {
        Expression::Array(Array { elements, .. }) | Expression::List(List { elements, .. }) => {
            for element in elements.iter() {
                match element {
                    ArrayElement::KeyValue(e) => check_value_var(foreach, ancestors, e.value, out),
                    ArrayElement::Value(e) => check_value_var(foreach, ancestors, e.value, out),
                    _ => {}
                }
            }
        }
        _ => check(foreach, ancestors, expr, "value", out),
    }
}

/// A key or value variable (`&$value` too) the scope already has.
fn check(
    foreach: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    kind: &str,
    out: &mut Vec<Report>,
) {
    let expr = match unparenthesize(expr) {
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_)) => {
            u.operand
        }
        expr => expr,
    };
    if let Expression::Variable(Variable::Direct(v)) = expr
        && defined(foreach, ancestors, v.name)
    {
        out.push(Report {
            span: node_span(v),
            message: format!(
                "Foreach overwrites {} with its {kind} variable.",
                String::from_utf8_lossy(v.name)
            ),
            help: None,
            fix: None,
        });
    }
}
