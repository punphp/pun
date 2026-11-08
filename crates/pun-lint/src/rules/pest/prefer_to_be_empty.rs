//! `lint/pest/prefer-to-be-empty`, ported from pest-plugin-rector's `UseToBeEmptyRector`: `->toHaveCount(0)` becomes
//! `->toBeEmpty()`, `expect(count($x))->toBe(0)` (or `sizeof()`, or `toEqual()`) becomes `expect($x)->toBeEmpty()`,
//! and `expect(empty($x))->toBeTrue()`, when `$x` is known not to be an object, becomes `expect($x)->toBeEmpty()` (`toBeFalse()` or a `->not` give
//! `->not->toBeEmpty()`; the new call is made on the holder, so what came between is dropped).

use mago_syntax::cst::{Construct, Expression, Literal, MethodCall, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    func_call, has_not, identifier, is_expect_chain, is_name, matcher_subject, set_call_arguments,
    subject_holder, type_is, Is, Trinary,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeEmpty;

impl Rule for PreferToBeEmpty {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-empty"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if !is_expect_chain(call) {
            return;
        }
        let message = "Use `toBeEmpty()` instead of checking the count or `empty()`.".to_string();
        if is_name(&call.method, "toHaveCount") && has_zero_arg(call) {
            out.push(Report::fixed(
                node_span(call),
                message,
                Safety::Unsafe,
                set_call_arguments(call, "toBeEmpty", &[]),
            ));
            return;
        }
        if (is_name(&call.method, "toBe") || is_name(&call.method, "toEqual")) && has_zero_arg(call)
        {
            let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call))
            else {
                return;
            };
            let Some(count) =
                func_call(subject).filter(|f| named(f, "count") || named(f, "sizeof"))
            else {
                return;
            };
            let [argument] = count.argument_list.arguments.as_slice() else {
                return;
            };
            let mut edits = holder.set_arguments(ctx, argument.value());
            edits.extend(set_call_arguments(call, "toBeEmpty", &[]));
            out.push(Report::fixed(
                node_span(call),
                message,
                Safety::Unsafe,
                edits,
            ));
            return;
        }
        if is_name(&call.method, "toBeTrue") || is_name(&call.method, "toBeFalse") {
            let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call))
            else {
                return;
            };
            let Expression::Construct(Construct::Empty(empty)) = unparenthesize(subject) else {
                return;
            };
            if type_is(ctx, empty.value, Is::Object) != Trinary::No {
                return;
            }
            let not = (identifier(&call.method) == Some(b"toBeFalse")) != has_not(call);
            let not = if not { "->not" } else { "" };
            let mut edits = holder.set_arguments(ctx, empty.value);
            edits.push(Edit::new(
                Span::new(holder.span.end, node_span(call).end),
                format!("{not}->toBeEmpty()"),
            ));
            out.push(Report::fixed(
                node_span(call),
                message,
                Safety::Unsafe,
                edits,
            ));
        }
    }
}

/// `hasZeroArg`: the call's one argument is the integer `0`.
fn has_zero_arg(call: &MethodCall<'_>) -> bool {
    matches!(
        call.argument_list.arguments.as_slice(),
        [argument] if matches!(unparenthesize(argument.value()), Expression::Literal(Literal::Integer(i)) if i.value == Some(0))
    )
}
