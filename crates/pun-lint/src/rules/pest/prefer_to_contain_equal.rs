//! `lint/pest/prefer-to-contain-equal`, ported from pest-plugin-rector's `UseToContainEqualRector`: `expect(in_array($needle, $haystack, false))->toBeTrue()`
//! becomes `expect($haystack)->toContainEqual($needle)`. The haystack's type must possibly be an array.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::rules::rector_code_quality::is_constant;
use crate::{Context, Report, Rule};

pub struct PreferToContainEqual;

impl Rule for PreferToContainEqual {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-contain-equal"
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
            "toContainEqual",
            |values| match values {
                [needle, haystack, strict, ..]
                    if is_constant(strict, "false")
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
