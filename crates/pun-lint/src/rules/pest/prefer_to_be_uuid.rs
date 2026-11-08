//! `lint/pest/prefer-to-be-uuid`, ported from pest-plugin-rector's `UseToBeUuidRector`:
//! `expect(preg_match('/^[0-9a-f]{8}-...$/i', $value))->toBe(1)` (one of `UUID_REGEX_PATTERNS`; or `toEqual(1)`, or
//! `toBeGreaterThan(0)`) becomes `expect($value)->toBeUuid()` when `$value` is known to be a string.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, func_call, is_name, matcher_subject, set_call_arguments, subject_holder, type_is,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToBeUuid;

impl Rule for PreferToBeUuid {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-uuid"
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
        if !(is_name(&call.method, "toBe")
            || is_name(&call.method, "toBeGreaterThan")
            || is_name(&call.method, "toEqual"))
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
        let Expression::Literal(Literal::String(pattern)) = unparenthesize(pattern.value()) else {
            return;
        };
        if !pattern
            .value
            .is_some_and(|p| UUID_REGEX_PATTERNS.iter().any(|u| u.as_bytes() == p))
        {
            return;
        }
        let value = value.value();
        if type_is(ctx, value, Is::String) != Trinary::Yes {
            return;
        }
        let [expected] = call.argument_list.arguments.as_slice() else {
            return;
        };
        let Expression::Literal(Literal::Integer(expected)) = unparenthesize(expected.value())
        else {
            return;
        };
        let wanted = if is_name(&call.method, "toBeGreaterThan") {
            0
        } else {
            1
        };
        if expected.value != Some(wanted) {
            return;
        }
        let mut edits = holder.set_arguments(ctx, value);
        edits.extend(set_call_arguments(call, "toBeUuid", &[]));
        out.push(Report::fixed(
            node_span(call),
            "Use `toBeUuid()` instead of matching a UUID pattern.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `UUID_REGEX_PATTERNS`.
const UUID_REGEX_PATTERNS: [&str; 3] = [
    "/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i",
    "/^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$/i",
    "/^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$/i",
];
