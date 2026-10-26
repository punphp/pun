//! `lint/phpunit/prefer-assert-null-true-false`, ported from rector-phpunit's `AssertSameBoolNullToSpecificMethodRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertSame(null, $a)` becomes `assertNull($a)`, and likewise for `true` and `false` and for `assertNotSame`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, arg_value, is_constant, is_test_case_call_named, remove_first_arg, renamed, report,
};
use crate::{Context, Report, Rule};

const CONSTANTS: &[(&str, &str, &str)] = &[
    ("null", "assertNull", "assertNotNull"),
    ("true", "assertTrue", "assertNotTrue"),
    ("false", "assertFalse", "assertNotFalse"),
];

pub struct PreferAssertNullTrueFalse;

impl Rule for PreferAssertNullTrueFalse {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-null-true-false"
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
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertSame", "assertNotSame"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let value = arg_value(first);
        let Some((_, same, not_same)) = CONSTANTS.iter().find(|(c, _, _)| is_constant(value, c))
        else {
            return;
        };
        let Some(name) = renamed(&call, &[("assertSame", same), ("assertNotSame", not_same)])
        else {
            return;
        };
        let edits = remove_first_arg(ctx, &call, name);
        out.push(report(
            call.span,
            &format!("Use `{name}()` for a comparison with a constant."),
            Safety::Unsafe,
            edits,
        ));
    }
}
