//! `lint/pest/prefer-to-contain-only-instances-of`, ported from pest-plugin-rector's
//! `UseToContainOnlyInstancesOfRector`: `expect($items)->each->toBeInstanceOf(User::class)` becomes
//! `expect($items)->toContainOnlyInstancesOf(User::class)` when the checked value may be iterable.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    Is, Trinary, is_expect_chain, is_name, matcher_subject, property_fetch, rename, report, type_is,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToContainOnlyInstancesOf;

impl Rule for PreferToContainOnlyInstancesOf {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-contain-only-instances-of"
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
        if !is_expect_chain(call) || !is_name(&call.method, "toBeInstanceOf") {
            return;
        }
        if matcher_subject(call).is_none_or(|s| type_is(ctx, s, Is::Iterable) == Trinary::No) {
            return;
        }
        if call.argument_list.arguments.len() != 1 {
            return;
        }
        let Some(each) = property_fetch(call.object).filter(|p| is_name(&p.property, "each"))
        else {
            return;
        };
        if property_fetch(each.object).is_some_and(|p| is_name(&p.property, "not")) {
            return;
        }
        out.push(report(
            node_span(call),
            "Use `toContainOnlyInstancesOf()` instead of `->each->toBeInstanceOf()`.".to_string(),
            Safety::Unsafe,
            vec![
                // `$node->var = $node->var->var`.
                Edit::new(
                    Span::new(
                        node_span(unparenthesize(each.object)).end,
                        node_span(unparenthesize(call.object)).end,
                    ),
                    "",
                ),
                rename(call, "toContainOnlyInstancesOf"),
            ],
        ));
    }
}
