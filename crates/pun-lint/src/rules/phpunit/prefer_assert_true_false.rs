//! `lint/phpunit/prefer-assert-true-false`, ported from rector-phpunit's `AssertSameTrueFalseToAssertTrueFalseRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! A method call `assertSame(true, $a)` becomes `assertTrue($a)`, and `assertSame(false, $a)` becomes
//! `assertFalse($a)`. As upstream, `assertNotSame` and the names `assertEqual` and `assertNotEqual` are rewritten
//! the same way.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{AnyCall, arg_value, is_constant, is_test_case_call_named, remove_first_arg, report};
use crate::{Context, Report, Rule};

pub struct PreferAssertTrueFalse;

impl Rule for PreferAssertTrueFalse {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-true-false"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
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
            "assertSame",
            "assertEqual",
            "assertNotSame",
            "assertNotEqual",
        ];
        if !is_test_case_call_named(ctx, ancestors, &call, &names) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let name = if is_constant(arg_value(first), "true") {
            "assertTrue"
        } else if is_constant(arg_value(first), "false") {
            "assertFalse"
        } else {
            return;
        };
        let edits = remove_first_arg(ctx, &call, name);
        out.push(report(
            call.span,
            &format!("Use `{name}()` for a comparison with a boolean."),
            Safety::Unsafe,
            edits,
        ));
    }
}
