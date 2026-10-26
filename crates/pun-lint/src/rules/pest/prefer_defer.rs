//! `lint/pest/prefer-defer`, ported from pest-plugin-rector's `Pest2ToPest3\TapToDeferRector`: `->tap()` in an
//! `expect()` chain becomes `->defer()`, its Pest 3 name.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::{is_expect_chain, is_name, rename, report};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferDefer;

impl Rule for PreferDefer {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-defer"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if !is_expect_chain(call) || !is_name(&call.method, "tap") {
            return;
        }
        out.push(report(
            node_span(&call.method),
            "Use `->defer()` instead of the deprecated `->tap()`.".to_string(),
            Safety::Safe,
            vec![rename(call, "defer")],
        ));
    }
}
