//! `lint/pest/prefer-to-have-count`, ported from pest-plugin-rector's `UseToHaveCountRector`:
//! `expect(count($items))->toBe(3)` (or `sizeof()`, or `toEqual()`) becomes `expect($items)->toHaveCount(3)` when
//! `$items` may be an array or iterable.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, argument_value, func_call, is_expect_chain, is_name, matcher_subject, rename,
    report, set_first_argument, subject_holder, type_is,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferToHaveCount;

impl Rule for PreferToHaveCount {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-count"
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
        if !is_expect_chain(call)
            || !(is_name(&call.method, "toBe") || is_name(&call.method, "toEqual"))
            || call.argument_list.arguments.len() != 1
        {
            return;
        }
        let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call)) else {
            return;
        };
        let Some(count) = func_call(subject).filter(|f| named(f, "count") || named(f, "sizeof"))
        else {
            return;
        };
        let [argument] = count.argument_list.arguments.as_slice() else {
            return;
        };
        let value = argument_value(argument);
        if type_is(ctx, value, Is::Array) == Trinary::No
            && type_is(ctx, value, Is::Iterable) == Trinary::No
        {
            return;
        }
        let Some(edit) = set_first_argument(ctx, holder.arguments, value) else {
            return;
        };
        out.push(report(
            node_span(call),
            "Use `toHaveCount()` instead of comparing `count()`.".to_string(),
            Safety::Unsafe,
            vec![edit, rename(call, "toHaveCount")],
        ));
    }
}
