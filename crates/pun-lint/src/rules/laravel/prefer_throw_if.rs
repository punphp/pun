//! `ThrowIfRector` from rector-laravel 2.5.0.

use crate::{Context, Report, Rule};
use mago_syntax::cst::{Node, NodeKind};

pub struct PreferThrowIf;

impl Rule for PreferThrowIf {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-throw-if"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        super::conditional_helpers::visit(node, ctx, "throw", out);
    }
}
