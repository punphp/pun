//! `lint/pest/prefer-to-be-alpha-numeric`, ported from pest-plugin-rector's `UseToBeAlphaNumericRector`: `expect(ctype_alnum($value))->toBeTrue()` becomes `expect($value)->toBeAlphaNumeric()`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toBeAlphaNumeric()`. The checked value's type must possibly be a string.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToBeAlphaNumeric;

impl Rule for PreferToBeAlphaNumeric {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-alpha-numeric"
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
            "ctype_alnum",
            "toBeAlphaNumeric",
            |values| match values {
                [value] if !(type_is(ctx, value, Is::String) == Trinary::No) => {
                    Some((*value, vec![]))
                }
                _ => None,
            },
            out,
        );
    }
}
