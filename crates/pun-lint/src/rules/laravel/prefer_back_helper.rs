//! `lint/laravel/prefer-back-helper`, ported from rector-laravel 2.5.0's `RedirectBackToBackHelperRector`
//! (src/Rector/MethodCall).
//!
//! `redirect()->back(...)` and `Redirect::back(...)` (the facade) become `back(...)`.

use mago_syntax::cst::{Node, NodeKind};

use super::redirect_helper;
use crate::{Context, Report, Rule};

pub struct PreferBackHelper;

impl Rule for PreferBackHelper {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-back-helper"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        redirect_helper(node, ancestors, ctx, "back", "back", out);
    }
}
