//! `lint/phpunit/prefer-assert-string-contains`, ported from rector-phpunit's `AssertFalseStrposToContainsRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertNotFalse(strpos($a, $b))` becomes `assertStringContainsString($b, $a)`, and `assertFalse(...)` becomes
//! `assertStringNotContainsString(...)`; `stripos()` is read as `strpos()`, as upstream does.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, expr_name, is_test_case_call_named, old_args, renamed, rewrite_args, text,
};
use crate::rules::side_effect::any_node;
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const RENAME: &[(&str, &str)] = &[
    ("assertFalse", "assertStringNotContainsString"),
    ("assertNotFalse", "assertStringContainsString"),
];

/// Whether `program` has an `assertFalse(strpos($a, $b))` or `assertNotFalse(...)` call this rule rewrites, a change
/// Rector reports.
fn has_reported_change(program: Node<'_, '_>) -> bool {
    any_node(program, &|n| {
        let Some(call) = AnyCall::of(n) else {
            return false;
        };
        if renamed(&call, RENAME).is_none() {
            return false;
        }
        let Some(first) = call.args().first().copied() else {
            return false;
        };
        let Expression::Call(Call::Function(strpos)) = unparenthesize(first.value()) else {
            return false;
        };
        expr_name(unparenthesize(first.value())).is_some_and(|f| {
            f.eq_ignore_ascii_case(b"strpos") || f.eq_ignore_ascii_case(b"stripos")
        }) && strpos.argument_list.arguments.len() >= 2
    })
}

pub struct PreferAssertStringContains;

impl Rule for PreferAssertStringContains {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-string-contains"
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
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertFalse", "assertNotFalse"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let value = unparenthesize(first.value());
        if matches!(
            value,
            Expression::Call(Call::Method(_) | Call::StaticMethod(_))
        ) {
            return;
        }
        let Some(function) = expr_name(value) else {
            return;
        };
        if !function.eq_ignore_ascii_case(b"strpos") && !function.eq_ignore_ascii_case(b"stripos") {
            return;
        }
        let Some(name) = renamed(&call, RENAME) else {
            return;
        };
        let Expression::Call(Call::Function(strpos)) = value else {
            // Upstream renames the call before it finds no function call to take the arguments from, and returns
            // null: Rector 2.8.4 prints a file only when a rule reported a change there, so the rename stays only
            // beside a reported one (Pun counts this rule's own).
            if !ancestors.first().is_some_and(|program| has_reported_change(*program)) {
                return;
            }
            out.push(Report::fixed(
                call.span,
                format!("Use `{name}()` instead of comparing `strpos()` with `false`."),
                Safety::Safe,
                vec![pun_problem::Edit::new(call.name_span(), name)],
            ));
            return;
        };
        let inner: Vec<_> = strpos.argument_list.arguments.iter().collect();
        let (Some(haystack), Some(needle)) = (inner.first(), inner.get(1)) else {
            return;
        };
        out.push(Report::fixed(
            call.span,
            format!("Use `{name}()` instead of comparing `strpos()` with `false`."),
            Safety::Safe,
            rewrite_args(
                ctx,
                &call,
                Some(name),
                &[
                    NewArg::New(text(ctx, *needle).to_string()),
                    NewArg::New(text(ctx, *haystack).to_string()),
                ]
                .into_iter()
                .chain(old_args(&call, 1))
                .collect::<Vec<_>>(),
            ),
        ));
    }
}
