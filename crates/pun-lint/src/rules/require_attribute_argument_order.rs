//! `lint/complexity/require-attribute-argument-order`, ported from Rector 2.6.1's `SortAttributeNamedArgsRector`
//! (rules/CodeQuality/Rector/Attribute).
//!
//! The named arguments of an attribute move into the order of its class constructor's parameters, as
//! `require-named-argument-order` does for calls.

use mago_syntax::cst::{Node, NodeKind, PartialArgument};
use pun_problem::{Edit, Span};

use super::rector_batch5::resolve_class_name;
use super::require_named_argument_order::{report, sorted};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireAttributeArgumentOrder;

impl Rule for RequireAttributeArgumentOrder {
    fn id(&self) -> &'static str {
        "lint/complexity/require-attribute-argument-order"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Attribute])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Attribute(attribute) = node else {
            return;
        };
        let Some(list) = &attribute.argument_list else {
            return;
        };
        let args: Vec<(Span, Option<&[u8]>)> = list
            .arguments
            .iter()
            .map(|arg| match arg {
                PartialArgument::Named(named) => (node_span(arg), Some(named.name.value)),
                _ => (node_span(arg), None),
            })
            .collect();
        if args.len() <= 1 || args.iter().all(|(_, name)| name.is_none()) {
            return;
        }
        let facts = facts(ctx);
        let class = resolve_class_name(ancestors, &attribute.name);
        let Some(constructor) = facts
            .codebase
            .get_declaring_method(class.as_bytes(), b"__construct")
        else {
            return;
        };
        let Some(spans) = sorted(&args, constructor) else {
            return;
        };
        // php-parser prints an attribute's changed arguments again, on one line.
        let code: Vec<&str> = spans.iter().map(|s| &ctx.source[s.start..s.end]).collect();
        let inner = Span::new(
            list.left_parenthesis.end.offset as usize,
            list.right_parenthesis.start.offset as usize,
        );
        out.push(report(node_span(attribute), vec![Edit::new(inner, code.join(", "))]));
    }
}
