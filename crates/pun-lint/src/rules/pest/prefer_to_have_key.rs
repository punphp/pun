//! `lint/pest/prefer-to-have-key`, ported from pest-plugin-rector's `UseToHaveKeyRector`: `expect(array_key_exists($other, $subject))->toBeTrue()` becomes `expect($subject)->toHaveKey($other)`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toHaveKey()`. The checked value's type must be an array.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToHaveKey;

impl Rule for PreferToHaveKey {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-key"
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
            "array_key_exists",
            "toHaveKey",
            |values| match values {
                [other, subject] if type_is(ctx, subject, Is::Array) == Trinary::Yes => {
                    Some((*subject, vec![*other]))
                }
                _ => None,
            },
            out,
        );
    }
}
