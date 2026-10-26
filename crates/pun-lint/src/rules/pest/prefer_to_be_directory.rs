//! `lint/pest/prefer-to-be-directory`, ported from pest-plugin-rector's `UseToBeDirectoryRector`: `expect(is_dir($value))->toBeTrue()` becomes `expect($value)->toBeDirectory()`, and `toBeFalse()` or a
//! `->not` before the matcher becomes `->not->toBeDirectory()`. The checked value's type must possibly be a string.

use mago_syntax::cst::{Node, NodeKind};

use super::{Is, Trinary, function_matcher, type_is};
use crate::{Context, Report, Rule};

pub struct PreferToBeDirectory;

impl Rule for PreferToBeDirectory {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-be-directory"
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
            "is_dir",
            "toBeDirectory",
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
