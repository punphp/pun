//! `AbortIfRector` from rector-laravel 2.7.3.

use crate::{Context, Report, Rule};
use mago_syntax::cst::{Node, NodeKind};

pub struct PreferAbortIf;

impl Rule for PreferAbortIf {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-abort-if"
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
        super::conditional_helpers::visit(node, ctx, "abort", out);
    }
}
