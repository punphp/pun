//! `lint/laravel/prefer-to-route-helper`, ported from rector-laravel 2.5.0's `RedirectRouteToToRouteHelperRector`
//! (src/Rector/MethodCall).
//!
//! `redirect()->route(...)` and `Redirect::route(...)` (the facade) become `to_route(...)`.

use mago_syntax::cst::{Node, NodeKind};

use super::redirect_helper;
use crate::{Context, Report, Rule};

pub struct PreferToRouteHelper;

impl Rule for PreferToRouteHelper {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-to-route-helper"
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
        redirect_helper(node, ancestors, ctx, "route", "to_route", out);
    }
}
