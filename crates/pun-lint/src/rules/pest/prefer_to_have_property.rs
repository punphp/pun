//! `lint/pest/prefer-to-have-property`, ported from pest-plugin-rector's `UseToHavePropertyRector`: `expect(property_exists($subject, $other))->toBeTrue()` becomes `expect($subject)->toHaveProperty($other)`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toHaveProperty()`. The checked value's type must possibly be a object.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToHaveProperty;

impl Rule for PreferToHaveProperty {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-have-property"
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
            "property_exists",
            "toHaveProperty",
            |values| match values {
                [subject, other] if !(type_is(ctx, subject, Is::Object) == Trinary::No) => {
                    Some((*subject, vec![*other]))
                }
                _ => None,
            },
            out,
        );
    }
}
