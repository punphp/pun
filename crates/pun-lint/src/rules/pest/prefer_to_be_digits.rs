//! `lint/pest/prefer-to-be-digits`, ported from pest-plugin-rector's `UseToBeDigitsRector`: `expect(ctype_digit($value))->toBeTrue()` becomes `expect($value)->toBeDigits()`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toBeDigits()`. The checked value's type must be a string.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToBeDigits;

impl Rule for PreferToBeDigits {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-digits"
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
            "ctype_digit",
            "toBeDigits",
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
