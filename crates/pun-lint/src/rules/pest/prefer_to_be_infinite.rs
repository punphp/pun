//! `lint/pest/prefer-to-be-infinite`, ported from pest-plugin-rector's `UseToBeInfiniteRector`: `expect(is_infinite($value))->toBeTrue()` becomes `expect($value)->toBeInfinite()`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toBeInfinite()`. The checked value's type must possibly be a float or integer.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToBeInfinite;

impl Rule for PreferToBeInfinite {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-infinite"
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
            "is_infinite",
            "toBeInfinite",
            |values| match values {
                [value]
                    if !(type_is(ctx, value, Is::Float) == Trinary::No
                        && type_is(ctx, value, Is::Integer) == Trinary::No) =>
                {
                    Some((*value, vec![]))
                }
                _ => None,
            },
            out,
        );
    }
}
