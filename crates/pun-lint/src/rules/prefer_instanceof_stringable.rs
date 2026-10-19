//! `lint/style/prefer-instanceof-stringable`, ported from Mago 1.57.0's linter rule `instanceof-stringable`
//! (rule/clarity/instanceof_stringable.rs).
//!
//! `is_object($x) && method_exists($x, '__toString')` (either way round, with `&&`) is reported on the whole
//! expression: since PHP 8.0 a class with `__toString()` implements `Stringable`.

use mago_syntax::cst::{BinaryOperator, Call, Expression, Literal, Node, NodeKind, Variable};

use super::lint_b_names::function_call_matches;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferInstanceofStringable;

impl Rule for PreferInstanceofStringable {
    fn id(&self) -> &'static str {
        "lint/style/prefer-instanceof-stringable"
    }

    fn min_php_version(&self) -> u32 {
        80000
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        if !matches!(binary.operator, BinaryOperator::And(_)) {
            return;
        }
        let (Expression::Call(Call::Function(left)), Expression::Call(Call::Function(right))) =
            (binary.lhs, binary.rhs)
        else {
            return;
        };
        let (is_object, method_exists) = if function_call_matches(ancestors, left, "is_object")
            && function_call_matches(ancestors, right, "method_exists")
        {
            (left, right)
        } else if function_call_matches(ancestors, left, "method_exists")
            && function_call_matches(ancestors, right, "is_object")
        {
            (right, left)
        } else {
            return;
        };
        let (is_object, method_exists) = (
            &is_object.argument_list.arguments,
            &method_exists.argument_list.arguments,
        );
        if is_object.len() != 1 || method_exists.len() != 2 {
            return;
        }
        let Expression::Variable(Variable::Direct(object)) = is_object.nodes[0].value() else {
            return;
        };
        let Expression::Variable(Variable::Direct(subject)) = method_exists.nodes[0].value() else {
            return;
        };
        if object.name != subject.name {
            return;
        }
        let Expression::Literal(Literal::String(method)) = method_exists.nodes[1].value() else {
            return;
        };
        if !method
            .value
            .is_some_and(|m| m.eq_ignore_ascii_case(b"__toString"))
        {
            return;
        }
        out.push(Report {
            span: node_span(binary),
            message: "Consider using `instanceof Stringable` instead of `is_object() && method_exists()`."
                .to_string(),
            help: Some("`instanceof Stringable` is clearer and more performant.".to_string()),
            fix: None,
        });
    }
}
