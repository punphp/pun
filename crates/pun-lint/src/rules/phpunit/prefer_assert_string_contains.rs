//! `lint/phpunit/prefer-assert-string-contains`, ported from rector-phpunit's `AssertFalseStrposToContainsRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertNotFalse(strpos($a, $b))` becomes `assertStringContainsString($b, $a)`, and `assertFalse(...)` becomes
//! `assertStringNotContainsString(...)`; `stripos()` is read as `strpos()`, as upstream does.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, is_test_case_call_named, old_args, renamed, report, rewrite_args,
    text,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const RENAME: &[(&str, &str)] = &[
    ("assertFalse", "assertStringNotContainsString"),
    ("assertNotFalse", "assertStringContainsString"),
];

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
        let Some(call) = AnyCall::of(node) else {
            return;
        };
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertFalse", "assertNotFalse"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let Expression::Call(Call::Function(strpos)) = unparenthesize(arg_value(first)) else {
            return;
        };
        if !named(strpos, "strpos") && !named(strpos, "stripos") {
            return;
        }
        let inner: Vec<_> = strpos.argument_list.arguments.iter().collect();
        let (Some(haystack), Some(needle)) = (inner.first(), inner.get(1)) else {
            return;
        };
        let Some(name) = renamed(&call, RENAME) else {
            return;
        };
        out.push(report(
            call.span,
            &format!("Use `{name}()` instead of comparing `strpos()` with `false`."),
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
