//! `lint/pest/prefer-to-start-with`, ported from pest-plugin-rector's `UseToStartWithRector`: `expect(str_starts_with($subject, $other))->toBeTrue()` becomes `expect($subject)->toStartWith($other)`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toStartWith()`. The checked value's type must be a string.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToStartWith;

impl Rule for PreferToStartWith {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-start-with"
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
            "str_starts_with",
            "toStartWith",
            |values| match values {
                [subject, other] if type_is(ctx, subject, Is::String) == Trinary::Yes => {
                    Some((*subject, vec![*other]))
                }
                _ => None,
            },
            out,
        );
    }
}
