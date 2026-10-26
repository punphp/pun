//! `lint/pest/prefer-to-have-same-size`, ported from pest-plugin-rector's `UseToHaveSameSizeRector`:
//! `expect(count($a))->toBe(count($b))` (or `sizeof()`, or `toEqual()`) becomes `expect($a)->toHaveSameSize($b)`, and
//! `->toHaveCount(count($b))` becomes `->toHaveSameSize($b)`, when the checked value may be an array or iterable.

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::Safety;

use super::{
    Is, Trinary, argument_value, first_value, func_call, is_name, report, set_call_arguments,
    subject_holder, texts, type_is,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferToHaveSameSize;

impl Rule for PreferToHaveSameSize {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-same-size"
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
        let Some(holder) = subject_holder(call) else {
            return;
        };
        // `isCountFunction` and `getCountedValue`.
        fn counted<'a>(e: Option<&'a Expression<'a>>) -> Option<Option<&'a Expression<'a>>> {
            e.and_then(func_call)
                .filter(|f| named(f, "count") || named(f, "sizeof"))
                .map(|f| first_value(&f.argument_list))
        }
        let countable = |e: &Expression<'_>| {
            type_is(ctx, e, Is::Array) != Trinary::No
                || type_is(ctx, e, Is::Iterable) != Trinary::No
        };
        let message = "Use `toHaveSameSize()` instead of comparing counts.".to_string();
        let [argument] = call.argument_list.arguments.as_slice() else {
            return;
        };
        if is_name(&call.method, "toBe") || is_name(&call.method, "toEqual") {
            let (Some(first), Some(second)) = (
                counted(first_value(holder.arguments)),
                counted(Some(argument_value(argument))),
            ) else {
                return;
            };
            let (Some(first), Some(second)) = (first, second) else {
                return;
            };
            if !countable(first) {
                return;
            }
            let mut edits = holder.set_arguments(ctx, first);
            edits.extend(set_call_arguments(
                call,
                "toHaveSameSize",
                &texts(ctx, &[second]),
            ));
            out.push(report(node_span(call), message, Safety::Unsafe, edits));
        } else if is_name(&call.method, "toHaveCount") {
            let Some(Some(second)) = counted(Some(argument_value(argument))) else {
                return;
            };
            if first_value(holder.arguments).is_some_and(|s| !countable(s)) {
                return;
            }
            out.push(report(
                node_span(call),
                message,
                Safety::Unsafe,
                set_call_arguments(call, "toHaveSameSize", &texts(ctx, &[second])),
            ));
        }
    }
}
