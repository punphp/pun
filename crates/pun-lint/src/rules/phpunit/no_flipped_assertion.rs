//! `lint/phpunit/no-flipped-assertion`, ported from rector-phpunit's `FlipAssertRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertSame($result, 'expected')` becomes `assertSame('expected', $result)`: when only the second argument is a
//! scalar, a constant or a class constant, the first two arguments swap. Likewise for `assertNotSame`,
//! `assertEquals`, `assertNotEquals` and `assertStringContainsString`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, is_scalar_value, is_test_case_call_named, old_args, report,
    rewrite_args,
};
use crate::{Context, Report, Rule};

const METHOD_NAMES: &[&str] = &[
    "assertSame",
    "assertNotSame",
    "assertNotEquals",
    "assertEquals",
    "assertStringContainsString",
];

pub struct NoFlippedAssertion;

impl Rule for NoFlippedAssertion {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-flipped-assertion"
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
        if !is_test_case_call_named(ctx, ancestors, &call, METHOD_NAMES) {
            return;
        }
        let args = call.args();
        let (Some(first), Some(second)) = (args.first().copied(), args.get(1).copied()) else {
            return;
        };
        if is_scalar_value(arg_value(first)) || !is_scalar_value(arg_value(second)) {
            return;
        }
        out.push(report(
            call.span,
            "Put the expected value first.",
            Safety::Safe,
            rewrite_args(
                ctx,
                &call,
                None,
                &[NewArg::Old(1), NewArg::Old(0)]
                    .into_iter()
                    .chain(old_args(&call, 2))
                    .collect::<Vec<_>>(),
            ),
        ));
    }
}
