//! `lint/phpunit/prefer-narrow-with-consecutive`, ported from rector-phpunit's `NarrowIdenticalWithConsecutiveRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, `withConsecutive([1], [1])` whose arguments are all the same becomes `with([1])`, and likewise
//! `willReturnOnConsecutiveCalls()` becomes `willReturn()`. Arguments that create objects are left alone.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, contains, expr_text, is_in_test_class, report, rewrite_args,
};
use crate::syntax::same_expression;
use crate::{Context, Report, Rule};

pub struct PreferNarrowWithConsecutive;

impl Rule for PreferNarrowWithConsecutive {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-narrow-with-consecutive"
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
        let names = ["withConsecutive", "willReturnOnConsecutiveCalls"];
        if !is_in_test_class(ctx, ancestors, None) || !call.is_name(&names) {
            return;
        }
        let args = call.args();
        let Some(first) = args.first().copied() else {
            return;
        };
        if crate::rules::lint_b_helpers::unpacked(first) {
            return;
        }
        let creates = args.iter().any(|a| {
            contains(Node::Expression(arg_value(a)), &|n| {
                matches!(n, Node::Instantiation(_) | Node::AnonymousClass(_))
            })
        });
        if creates {
            return;
        }
        if !args
            .iter()
            .all(|a| same_expression(ctx.source, arg_value(a), arg_value(first)))
        {
            return;
        }
        let name = if call.is_name(&["withConsecutive"]) {
            "with"
        } else {
            "willReturn"
        };
        out.push(report(
            call.span,
            &format!("Use `{name}()` for identical values."),
            Safety::Safe,
            rewrite_args(
                ctx,
                &call,
                Some(name),
                &[NewArg::New(expr_text(ctx, arg_value(first)).to_string())],
            ),
        ));
    }
}
