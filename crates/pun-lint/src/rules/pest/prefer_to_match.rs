//! `lint/pest/prefer-to-match`, ported from pest-plugin-rector's `UseToMatchRector`: `expect(preg_match('/pattern/',
//! $value))->toBe(1)` (or `toEqual(1)`) becomes `expect($value)->toMatch('/pattern/')` when `$value` may be a string.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, argument_value, func_call, is_name, matcher_subject, report, set_call_arguments,
    subject_holder, texts, type_is,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToMatch;

impl Rule for PreferToMatch {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-match"
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
        if !(is_name(&call.method, "toBe") || is_name(&call.method, "toEqual")) {
            return;
        }
        let [expected] = call.argument_list.arguments.as_slice() else {
            return;
        };
        if !matches!(unparenthesize(argument_value(expected)), Expression::Literal(Literal::Integer(i)) if i.value == Some(1))
        {
            return;
        }
        let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call)) else {
            return;
        };
        let Some(matcher) = func_call(subject).filter(|f| named(f, "preg_match")) else {
            return;
        };
        let [pattern, value, ..] = matcher.argument_list.arguments.as_slice() else {
            return;
        };
        let value = argument_value(value);
        if type_is(ctx, value, Is::String) == Trinary::No {
            return;
        }
        let mut edits = holder.set_arguments(ctx, value);
        edits.extend(set_call_arguments(
            call,
            "toMatch",
            &texts(ctx, &[argument_value(pattern)]),
        ));
        out.push(report(
            node_span(call),
            "Use `toMatch()` instead of checking `preg_match()`.".to_string(),
            Safety::Unsafe,
            edits,
        ));
    }
}
