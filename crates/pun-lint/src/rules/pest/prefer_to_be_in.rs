//! `lint/pest/prefer-to-be-in`, ported from pest-plugin-rector's `UseToBeInRector`: `expect(in_array($needle, $haystack, true))->toBeTrue()`
//! becomes `expect($needle)->toBeIn($haystack)`. The haystack's type must possibly be an array or an iterable.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::rules::rector_code_quality::is_constant;
use crate::{Context, Report, Rule};

pub struct PreferToBeIn;

impl Rule for PreferToBeIn {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-in"
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
            "toBeIn",
            |values| match values {
                [needle, haystack, strict]
                    if is_constant(strict, "true")
                        && !(type_is(ctx, haystack, Is::Array) == Trinary::No
                            && type_is(ctx, haystack, Is::Iterable) == Trinary::No) =>
                {
                    Some((*needle, vec![*haystack]))
                }
                _ => None,
            },
            out,
        );
    }
}
