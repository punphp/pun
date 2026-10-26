//! `lint/phpunit/no-expects-any`, ported from rector-phpunit's `RemoveExpectAnyFromMockRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, `$mock->expects($this->any())` becomes `$mock`. Rector runs it only for PHPUnit 11 and later; Pun
//! leaves that to the rule's group.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{AnyCall, arg_value, is_in_test_class, report, text};
use crate::rules::laravel::is_variable;
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct NoExpectsAny;

impl Rule for NoExpectsAny {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-expects-any"
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
        let Some(call) = AnyCall::of(node) else {
            return;
        };
        if !is_in_test_class(ctx, ancestors, None) || !call.is_name(&["expects"]) {
            return;
        }
        let args = call.args();
        if args.len() != 1 {
            return;
        }
        let Expression::Call(Call::Method(any)) = unparenthesize(arg_value(args[0])) else {
            return;
        };
        let is_any = AnyCall::of(Node::MethodCall(any)).is_some_and(|c| c.is_name(&["any"]));
        if !is_any || !is_variable(any.object, "this") {
            return;
        }
        out.push(report(
            call.span,
            "Remove `expects($this->any())`: it expects nothing.",
            Safety::Safe,
            vec![Edit::new(call.span, text(ctx, call.receiver))],
        ));
    }
}
