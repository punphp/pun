//! `lint/pest/prefer-to-have-length`, ported from pest-plugin-rector's `UseToHaveLengthRector`:
//! `expect(strlen($string))->toBe(5)` (or `mb_strlen()`, or `toEqual()`) becomes `expect($string)->toHaveLength(5)`
//! when `$string` may be a string.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, argument_value, first_value, func_call, function_name, identifier,
    is_expect_chain, report, set_call_arguments, set_first_argument, subject_holder, texts,
    type_is,
};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferToHaveLength;

impl Rule for PreferToHaveLength {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-length"
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
        if !is_expect_chain(call) || !matches!(identifier(&call.method), Some(b"toBe" | b"toEqual"))
        {
            return;
        }
        let [length] = call.argument_list.arguments.as_slice() else {
            return;
        };
        let Some(holder) = subject_holder(call) else {
            return;
        };
        let Some(function) = first_value(holder.arguments).and_then(func_call) else {
            return;
        };
        if !matches!(function_name(function), Some(b"strlen" | b"mb_strlen")) {
            return;
        }
        let Some(string) = first_value(&function.argument_list) else {
            return;
        };
        if type_is(ctx, string, Is::String) == Trinary::No {
            return;
        }
        let Some(edit) = set_first_argument(ctx, holder.arguments, string) else {
            return;
        };
        let mut edits = vec![edit];
        edits.extend(set_call_arguments(
            call,
            "toHaveLength",
            &texts(ctx, &[argument_value(length)]),
        ));
        out.push(report(
            node_span(call),
            "Use `toHaveLength()` instead of comparing the string's length.".to_string(),
            Safety::Unsafe,
            edits,
        ));
    }
}
