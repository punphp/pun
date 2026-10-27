//! `lint/phpunit/no-negated-assertion`, ported from rector-phpunit's `AssertNotOperatorRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertTrue(!$a)` becomes `assertFalse($a)`, and `assertFalse(!$a)` becomes `assertTrue($a)`.

use mago_syntax::cst::{Expression, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, expr_text, is_test_case_call_named, old_args, renamed, report,
    rewrite_args,
};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const RENAME: &[(&str, &str)] = &[("assertTrue", "assertFalse"), ("assertFalse", "assertTrue")];

pub struct NoNegatedAssertion;

impl Rule for NoNegatedAssertion {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-negated-assertion"
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
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertTrue", "assertFalse"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let Expression::UnaryPrefix(not) = unparenthesize(arg_value(first)) else {
            return;
        };
        if !matches!(not.operator, UnaryPrefixOperator::Not(_)) {
            return;
        }
        let Some(name) = renamed(&call, RENAME) else {
            return;
        };
        out.push(report(
            call.span,
            &format!("Use `{name}()` instead of negating the condition."),
            Safety::Safe,
            rewrite_args(
                ctx,
                &call,
                Some(name),
                &std::iter::once(NewArg::New(expr_text(ctx, not.operand).to_string()))
                    .chain(old_args(&call, 1))
                    .collect::<Vec<_>>(),
            ),
        ));
    }
}
