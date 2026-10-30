//! `lint/laravel/prefer-view-array`, ported from Mago 1.57.0's `prefer-view-array` (mago-linter
//! src/rule/best_practices/prefer_view_array.rs).
//!
//! `view(...)->with(...)` is reported: the data goes in `view()`'s array argument. The object must be the call
//! itself, not in parentheses, and the function the global `view` as mago-names resolves it. There is no fix.

use mago_syntax::cst::{Call, Expression, Node, NodeKind};

use super::{is_name, member_name};
use crate::rules::lint_b_names::function_call_matches;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferViewArray;

impl Rule for PreferViewArray {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-view-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        let Expression::Call(Call::Function(function)) = call.object else {
            return;
        };
        if !function_call_matches(ancestors, function, "view")
            || !member_name(&call.method).is_some_and(|n| is_name(n, "with"))
        {
            return;
        }
        out.push(Report {
            span: node_span(call),
            message: "Use array parameter in `view()` instead of chaining `with()`.".to_string(),
            help: Some(
                "Refactor the code to use the array parameter in the `view()` function."
                    .to_string(),
            ),
            fix: None,
        });
    }
}
