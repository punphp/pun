//! `lint/complexity/no-useless-array-values`, ported from Rector 2.8.4's `SimplifyInArrayValuesRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `in_array($needle, array_values($haystack))` becomes `in_array($needle, $haystack)`: `in_array()` ignores the
//! keys that `array_values()` drops.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{args, call_to, named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessArrayValues;

impl Rule for NoUselessArrayValues {
    fn id(&self) -> &'static str {
        "lint/complexity/no-useless-array-values"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        if !named(c, "in_array") {
            return;
        }
        let Some(haystack) = args(c).get(1).copied() else {
            return;
        };
        let Some(values) = call_to(haystack.value(), "array_values") else {
            return;
        };
        // Rector puts `array_values()`'s first argument in place of the second one. Pun keeps the second argument's
        // name or spread and replaces only its value, so `in_array($a, ...array_values($b))` is left alone, as is
        // an `array_values()` without arguments (Rector cannot print the missing argument) or with a spread one.
        let Some(inner) = args(values).first().copied() else {
            return;
        };
        if haystack.is_unpacked() || inner.is_unpacked() {
            return;
        }
        // The parentheses around the call stay, as php-parser keeps the tokens around the argument it replaces.
        let span = node_span(unparenthesize(haystack.value()));
        out.push(Report {
            span,
            message: "`in_array()` needs no `array_values()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, text(ctx, unparenthesize(inner.value())))],
            }),
        });
    }
}
