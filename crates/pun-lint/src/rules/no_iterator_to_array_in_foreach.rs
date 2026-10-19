//! `lint/complexity/no-iterator-to-array-in-foreach`, ported from Mago 1.57.0's `no-iterator-to-array-in-foreach`
//! (mago-linter src/rule/redundancy/no_iterator_to_array_in_foreach.rs).
//!
//! Reports `foreach (iterator_to_array($it) as ...)`: `foreach` iterates any `Traversable` directly.

use mago_span::HasSpan;
use mago_syntax::cst::{Call, Expression, Node, NodeKind};

use super::rector_batch5::called_function_name;
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoIteratorToArrayInForeach;

impl Rule for NoIteratorToArrayInForeach {
    fn id(&self) -> &'static str {
        "lint/complexity/no-iterator-to-array-in-foreach"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Foreach(foreach) = node else { return };
        let Expression::Call(Call::Function(call)) = foreach.expression else {
            return;
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|name| name.eq_ignore_ascii_case("iterator_to_array"))
        {
            return;
        }
        out.push(Report {
            span: span(call.span()),
            message: "Unnecessary `iterator_to_array()` in `foreach`.".to_string(),
            help: Some(
                "Remove `iterator_to_array()` and pass the iterator directly to `foreach`."
                    .to_string(),
            ),
            fix: None,
        });
    }
}
