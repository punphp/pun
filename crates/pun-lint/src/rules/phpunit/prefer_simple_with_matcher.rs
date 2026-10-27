//! `lint/phpunit/prefer-simple-with-matcher`, ported from rector-phpunit's `UseSpecificWithMethodRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, each argument `$this->equalTo($a)` of a call named `with()` becomes `$a`.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{AnyCall, NewArg, arg_value, is_in_test_class, report, rewrite_args, text};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct PreferSimpleWithMatcher;

impl Rule for PreferSimpleWithMatcher {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-simple-with-matcher"
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
        if !is_in_test_class(ctx, ancestors, None) || !call.is_name(&["with"]) {
            return;
        }
        let args = call.args();
        let mut changed = false;
        let mut new = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            if let Expression::Call(Call::Method(m)) = unparenthesize(arg_value(arg))
                && AnyCall::of(Node::MethodCall(m)).is_some_and(|c| c.is_name(&["equalTo"]))
            {
                let Some(inner) = m.argument_list.arguments.first() else {
                    return;
                };
                new.push(NewArg::New(text(ctx, inner).to_string()));
                changed = true;
            } else {
                new.push(NewArg::Old(i));
            }
        }
        if !changed {
            return;
        }
        out.push(report(
            call.span,
            "Pass the value itself instead of `equalTo()`.",
            Safety::Safe,
            rewrite_args(ctx, &call, None, &new),
        ));
    }
}
