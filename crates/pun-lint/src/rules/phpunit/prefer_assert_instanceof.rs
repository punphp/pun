//! `lint/phpunit/prefer-assert-instanceof`, ported from rector-phpunit's `AssertInstanceOfComparisonRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! `assertTrue($a instanceof Foo)` becomes `assertInstanceOf(\Foo::class, $a)` (`assertFalse` becomes
//! `assertNotInstanceOf`), and `assertSame('Foo', get_class($a))` becomes `assertInstanceOf('Foo', $a)`.

use mago_syntax::cst::{BinaryOperator, Call, Expression, Node, NodeKind, Variable};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, expr_text, is_test_case_call_named, old_args, renamed, report,
    resolve, rewrite_args, text,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const RENAME: &[(&str, &str)] = &[
    ("assertTrue", "assertInstanceOf"),
    ("assertFalse", "assertNotInstanceOf"),
];

pub struct PreferAssertInstanceof;

impl Rule for PreferAssertInstanceof {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-assert-instanceof"
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
        let comparisons = [
            "assertSame",
            "assertNotSame",
            "assertEquals",
            "assertNotEquals",
        ];
        if is_test_case_call_named(ctx, ancestors, &call, &comparisons) {
            let Some(second) = call.args().get(1).copied() else {
                return;
            };
            let Expression::Call(Call::Function(get_class)) = unparenthesize(arg_value(second))
            else {
                return;
            };
            if !named(get_class, "get_class") {
                return;
            }
            let Some(inner) = get_class.argument_list.arguments.first() else {
                return;
            };
            let name = if call.is_name(&["assertSame", "assertEquals"]) {
                "assertInstanceOf"
            } else {
                "assertNotInstanceOf"
            };
            out.push(report(
                call.span,
                &format!("Use `{name}()` instead of comparing `get_class()`."),
                Safety::Safe,
                rewrite_args(
                    ctx,
                    &call,
                    Some(name),
                    &std::iter::once(NewArg::Old(0))
                        .chain(std::iter::once(NewArg::New(text(ctx, inner).to_string())))
                        .chain(old_args(&call, 2))
                        .collect::<Vec<_>>(),
                ),
            ));
            return;
        }
        if !is_test_case_call_named(ctx, ancestors, &call, &["assertTrue", "assertFalse"]) {
            return;
        }
        let Some(first) = call.args().first().copied() else {
            return;
        };
        let Expression::Binary(binary) = unparenthesize(arg_value(first)) else {
            return;
        };
        if !matches!(binary.operator, BinaryOperator::Instanceof(_)) {
            return;
        }
        let class = match unparenthesize(binary.rhs) {
            class @ Expression::Variable(Variable::Direct(_)) => expr_text(ctx, class).to_string(),
            Expression::Identifier(id) => format!("\\{}::class", resolve(ancestors, id)),
            _ => return,
        };
        let Some(name) = renamed(&call, RENAME) else {
            return;
        };
        out.push(report(
            call.span,
            &format!("Use `{name}()` instead of asserting `instanceof`."),
            Safety::Safe,
            rewrite_args(
                ctx,
                &call,
                Some(name),
                &[
                    NewArg::New(class),
                    NewArg::New(expr_text(ctx, binary.lhs).to_string()),
                ]
                .into_iter()
                .chain(old_args(&call, 1))
                .collect::<Vec<_>>(),
            ),
        ));
    }
}
