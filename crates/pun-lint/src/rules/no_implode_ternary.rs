//! `lint/dead-code/no-implode-ternary`, ported from Rector 2.6.1's `TernaryImplodeToImplodeRector`
//! (rules/CodeQuality/Rector/Ternary).
//!
//! `$values === [] ? '' : implode(',', $values)` becomes `implode(',', $values)`: joining an empty array gives the
//! empty string already. `join()` counts as `implode()`.

use mago_syntax::cst::{BinaryOperator, CompositeString, Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{args, equal, moved, named};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoImplodeTernary;

impl Rule for NoImplodeTernary {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-implode-ternary"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(t) = node else { return };
        let Expression::Binary(identical) = unparenthesize(t.condition) else {
            return;
        };
        if !matches!(identical.operator, BinaryOperator::Identical(_)) {
            return;
        }
        if !is_empty_array(identical.rhs) {
            return;
        }
        if !t.then.is_some_and(is_empty_string) {
            return;
        }
        let else_ = unparenthesize(t.r#else);
        // A first-class callable (`implode(...)`) is a partial application in Mago, no function call.
        let Expression::Call(mago_syntax::cst::Call::Function(call)) = else_ else {
            return;
        };
        if !named(call, "implode") && !named(call, "join") {
            return;
        }
        let Some(second) = args(call).get(1).copied() else {
            return;
        };
        if !equal(ctx, identical.lhs, second.value()) {
            return;
        }
        let span = node_span(t);
        out.push(Report {
            span,
            message: "`implode()` of an empty array is already the empty string; drop the ternary."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, moved(ctx, span, ancestors, else_))],
            }),
        });
    }
}

/// php-parser's `Array_` without items: `[]` or `array()`.
pub(super) fn is_empty_array(expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Array(a) => a.elements.is_empty(),
        Expression::LegacyArray(a) => a.elements.is_empty(),
        _ => false,
    }
}

/// php-parser's `String_` with the value `''`: an empty quoted string, or a heredoc or nowdoc without content.
fn is_empty_string(expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Literal(Literal::String(s)) => s.value.is_some_and(|v| v.is_empty()),
        Expression::CompositeString(CompositeString::Document(d)) => d.parts.is_empty(),
        _ => false,
    }
}
