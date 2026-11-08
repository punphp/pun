//! `lint/pest/prefer-to-be-nan`, ported from pest-plugin-rector's `UseToBeNanRector`: `expect(is_nan($value))->toBeTrue()` becomes `expect($value)->toBeNan()`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toBeNan()`. The checked value's type must be a float or integer.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToBeNan;

impl Rule for PreferToBeNan {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-nan"
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
            "is_nan",
            "toBeNan",
            |values| match values {
                [value]
                    if (type_is(ctx, value, Is::Float) == Trinary::Yes
                        || type_is(ctx, value, Is::Integer) == Trinary::Yes) =>
                {
                    Some((*value, vec![]))
                }
                _ => None,
            },
            out,
        );
    }
}
