//! `lint/pest/prefer-to-be-alpha`, ported from pest-plugin-rector's `UseToBeAlphaRector`: `expect(ctype_alpha($value))->toBeTrue()` becomes `expect($value)->toBeAlpha()`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toBeAlpha()`. The checked value's type must be a string.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToBeAlpha;

impl Rule for PreferToBeAlpha {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-alpha"
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
            "ctype_alpha",
            "toBeAlpha",
            |values| match values {
                [value] if type_is(ctx, value, Is::String) == Trinary::Yes => {
                    Some((*value, vec![]))
                }
                _ => None,
            },
            out,
        );
    }
}
