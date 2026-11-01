//! `lint/pest/prefer-to-contain`, ported from pest-plugin-rector's `UseToContainRector`: `expect(in_array($needle, $haystack))->toBeTrue()`
//! becomes `expect($haystack)->toContain($needle)`, unless a third argument is `false`. The haystack's type must possibly
//! be an array.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::rules::rector_code_quality::is_constant;
use crate::{Context, Report, Rule};

pub struct PreferToContain;

impl Rule for PreferToContain {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-contain"
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
        function_matcher(
            ctx,
            call,
            "in_array",
            "toContain",
            |values| match values {
                [needle, haystack, rest @ ..]
                    if !rest.first().is_some_and(|s| is_constant(s, "false"))
                        && type_is(ctx, haystack, Is::Array) != Trinary::No =>
                {
                    Some((*haystack, vec![*needle]))
                }
                _ => None,
            },
            out,
        );
    }
}
