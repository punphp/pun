//! `lint/phpunit/prefer-simple-mock-expectation`, ported from rector-phpunit's `SingleWithConsecutiveToWithRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, `withConsecutive()` with one argument becomes `with()` and `willReturnOnConsecutiveCalls()`
//! becomes `willReturn()`. Each `$this->equalTo($a)` in the argument becomes `$a`, and an array that held one
//! becomes the list of its items.

use mago_syntax::cst::{ArrayElement, Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    AnyCall, NewArg, arg_value, contains, inner_without_equal_to, is_in_test_class,
    is_method_call_named, report, rewrite_args, without_equal_to,
};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct PreferSimpleMockExpectation;

impl Rule for PreferSimpleMockExpectation {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-simple-mock-expectation"
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
        let names = ["withConsecutive", "willReturnOnConsecutiveCalls"];
        if !is_in_test_class(ctx, ancestors, None) || !call.is_name(&names) {
            return;
        }
        let args = call.args();
        if args.len() != 1 || crate::rules::lint_b_helpers::unpacked(args[0]) {
            return;
        }
        let name = if call.is_name(&["withConsecutive"]) {
            "with"
        } else {
            "willReturn"
        };
        let value = arg_value(args[0]);
        let has_assert_inside = contains(Node::Expression(value), &|n| {
            is_method_call_named(n, &["equalTo", "instanceOf"])
        });
        let new: Vec<NewArg> = match unparenthesize(value) {
            Expression::Array(array) if has_assert_inside => {
                let mut new = Vec::new();
                for element in array.elements.iter() {
                    let (prefix, item) = match element {
                        ArrayElement::KeyValue(kv) => ("", kv.value),
                        ArrayElement::Value(v) => ("", v.value),
                        ArrayElement::Variadic(v) => ("...", v.value),
                        ArrayElement::Missing(_) => return,
                    };
                    let Some(code) = inner_without_equal_to(ctx, item) else {
                        return;
                    };
                    new.push(NewArg::New(format!("{prefix}{code}")));
                }
                new
            }
            _ => {
                let Some(code) = without_equal_to(ctx, value) else {
                    return;
                };
                vec![NewArg::New(code)]
            }
        };
        out.push(report(
            call.span,
            &format!("Use `{name}()` for a single set of values."),
            Safety::Safe,
            rewrite_args(ctx, &call, Some(name), &new),
        ));
    }
}
