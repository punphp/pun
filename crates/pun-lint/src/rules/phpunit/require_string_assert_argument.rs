//! `lint/phpunit/require-string-assert-argument`, ported from rector-phpunit's
//! `StringCastAssertStringContainsStringRector` (rules/CodeQuality/Rector/MethodCall).
//!
//! The haystack of `assertStringContainsString()` and `assertStringNotContainsString()` is cast with `(string)` when
//! its type is not surely a string.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{AnyCall, is_test_case_call_named};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequireStringAssertArgument;

impl Rule for RequireStringAssertArgument {
    fn id(&self) -> &'static str {
        "lint/phpunit/require-string-assert-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        let names = [
            "assertStringContainsString",
            "assertStringNotContainsString",
        ];
        if !is_test_case_call_named(ctx, ancestors, &call, &names) {
            return;
        }
        let Some(second) = call.args().get(1).copied() else {
            return;
        };
        let value = second.value();
        let facts = facts(ctx);
        let is_string = facts.full_type(node_span(value)).is_some_and(|t| {
            !t.types.is_empty()
                && t.types.iter().all(|a| {
                    matches!(a, mago_codex::ttype::atomic::TAtomic::Scalar(s) if s.is_any_string())
                })
        });
        if is_string {
            return;
        }
        let inner = unparenthesize(value);
        let span = node_span(inner);
        let code = &ctx.source[span.start..span.end];
        let cast = if loose(inner) {
            format!("(string) ({code})")
        } else {
            format!("(string) {code}")
        };
        out.push(Report::fixed(
            node_span(value),
            "Cast the haystack to a string.",
            Safety::Safe,
            vec![Edit::new(node_span(value), cast)],
        ));
    }
}

/// Whether the expression binds looser than a cast, so the cast needs parentheses around it.
fn loose(expr: &Expression<'_>) -> bool {
    matches!(
        expr,
        Expression::Binary(_)
            | Expression::Conditional(_)
            | Expression::Assignment(_)
            | Expression::Yield(_)
            | Expression::Throw(_)
            | Expression::Pipe(_)
            | Expression::ArrowFunction(_)
    )
}
