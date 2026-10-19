//! `lint/functions/require-strict-function-call`, ported from phpstan-strict-rules 2.0.12's `StrictFunctionCallsRule`
//! (identifier `function.strict`).
//!
//! `in_array()` and `array_search()` need their third argument, `base64_decode()` its second and `array_keys()`, when
//! it has a second, its third, set to `true`. Named arguments count at their parameter's place (an unknown name leaves
//! the call alone, as `ArgumentsNormalizer` gives up on it), a skipped optional parameter counts as set. Pun reads
//! the argument's native type, so a value only a doc comment types `true` is reported where PHPStan is content.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{Argument, Expression, Node, NodeKind};

use super::lint_b_names::function_call_matches_any;
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// The functions, their parameters, and the parameter that must be `true`.
const FUNCTIONS: [(&str, &[&str], usize); 4] = [
    ("in_array", &["needle", "haystack", "strict"], 2),
    ("array_search", &["needle", "haystack", "strict"], 2),
    ("base64_decode", &["string", "strict"], 1),
    ("array_keys", &["array", "filter_value", "strict"], 2),
];

pub struct RequireStrictFunctionCall;

impl Rule for RequireStrictFunctionCall {
    fn id(&self) -> &'static str {
        "lint/functions/require-strict-function-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        let names = FUNCTIONS.map(|(name, _, _)| name);
        let Some(name) = function_call_matches_any(ancestors, call, &names) else {
            return;
        };
        let (_, params, position) = FUNCTIONS.iter().find(|(n, _, _)| *n == name).unwrap();
        // `ArgumentsNormalizer::reorderFuncArguments`: each argument at its parameter's place.
        let mut slots: Vec<(usize, &Expression<'_>)> = Vec::new();
        for (index, argument) in call.argument_list.arguments.iter().enumerate() {
            let slot = match argument {
                Argument::Positional(_) => index,
                Argument::Named(named) => {
                    match params.iter().position(|p| p.as_bytes() == named.name.value) {
                        Some(slot) => slot,
                        None => return,
                    }
                }
            };
            slots.push((slot, argument.value()));
        }
        let last = slots.iter().map(|(slot, _)| slot + 1).max().unwrap_or(0);
        if name == "array_keys" && last < 2 {
            return;
        }
        let problem = match slots.iter().find(|(slot, _)| slot == position) {
            None => "set",
            Some((_, value)) if is_true(ctx, value) => return,
            Some(_) => "true",
        };
        out.push(Report {
            span: node_span(call),
            message: format!(
                "Call to function {name}() requires parameter #{} to be {problem}.",
                position + 1
            ),
            help: None,
            fix: None,
        });
    }
}

/// Whether `true` is a supertype of the value's native type.
fn is_true(ctx: &Context<'_>, value: &Expression<'_>) -> bool {
    facts(ctx).native_type(node_span(value)).is_some_and(|t| {
        t.types
            .iter()
            .all(|a| matches!(a, TAtomic::Scalar(TScalar::Bool(b)) if b.is_true()))
    })
}
