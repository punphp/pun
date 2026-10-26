//! `lint/phpunit/no-float-delta-parameter`, ported from rector-phpunit's
//! `AssertEqualsOrAssertSameFloatParameterToSpecificMethodsTypeRector` (rules/CodeQuality/Rector/MethodCall).
//!
//! A method call `assertEquals(1.5, $a)` or `assertSame(1.5, $a)` with a float literal first becomes
//! `assertEqualsWithDelta(1.5, $a, PHP_FLOAT_EPSILON)`, keeping a third argument (the message) after it.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::{AnyCall, arg_value, is_test_case_call_named, new_call, report, text};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct NoFloatDeltaParameter;

impl Rule for NoFloatDeltaParameter {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-float-delta-parameter"
    }

    fn min_php_version(&self) -> u32 {
        70200
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
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertEquals", "assertSame"]) {
            return;
        }
        let args = call.args();
        let (Some(first), Some(second)) = (args.first().copied(), args.get(1).copied()) else {
            return;
        };
        if !matches!(
            unparenthesize(arg_value(first)),
            Expression::Literal(Literal::Float(_))
        ) {
            return;
        }
        let mut new_args = vec![text(ctx, first), text(ctx, second), "PHP_FLOAT_EPSILON"];
        if let Some(message) = args.get(2) {
            new_args.push(text(ctx, *message));
        }
        out.push(report(
            call.span,
            "Use `assertEqualsWithDelta()` to compare floats.",
            Safety::Safe,
            vec![Edit::new(
                call.span,
                new_call(ctx, &call, "assertEqualsWithDelta", &new_args),
            )],
        ));
    }
}
