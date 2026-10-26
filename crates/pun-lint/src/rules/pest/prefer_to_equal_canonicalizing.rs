//! `lint/pest/prefer-to-equal-canonicalizing`, ported from pest-plugin-rector's `UseToEqualCanonicalizingRector`:
//! `expect(sort($a))->toBe(sort($b))` (or `toEqual()`) becomes `expect($a)->toEqualCanonicalizing($b)`.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{
    argument_value, func_call, is_name, matcher_subject, report, set_call_arguments,
    subject_holder, texts,
};
use crate::rules::rector_code_quality::named;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferToEqualCanonicalizing;

impl Rule for PreferToEqualCanonicalizing {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-equal-canonicalizing"
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
        let [compare] = call.argument_list.arguments.as_slice() else {
            return;
        };
        let (Some(holder), Some(subject)) = (subject_holder(call), matcher_subject(call)) else {
            return;
        };
        let sorted = |e| match func_call(e).filter(|f| named(f, "sort")) {
            Some(f) => match f.argument_list.arguments.as_slice() {
                [argument] => Some(argument_value(argument)),
                _ => None,
            },
            None => None,
        };
        let (Some(actual), Some(expected)) = (sorted(subject), sorted(argument_value(compare)))
        else {
            return;
        };
        let mut edits = holder.set_arguments(ctx, actual);
        edits.extend(set_call_arguments(
            call,
            "toEqualCanonicalizing",
            &texts(ctx, &[expected]),
        ));
        out.push(report(
            node_span(call),
            "Use `toEqualCanonicalizing()` instead of comparing sorted values.".to_string(),
            Safety::Unsafe,
            edits,
        ));
    }
}
