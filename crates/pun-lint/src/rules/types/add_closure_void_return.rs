//! `lint/types/add-closure-void-return`, ported from Rector 2.6.1's `AddClosureVoidReturnTypeWhereNoReturnRector`
//! (rules/TypeDeclaration/Rector/Closure).
//!
//! A closure without a return type that returns no value and does not yield gets `: void`.

use mago_syntax::cst::{Node, NodeKind};

use super::{VOID_TYPE, add_return_type, has_exclusive_void};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddClosureVoidReturn;

impl Rule for AddClosureVoidReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-closure-void-return"
    }

    fn min_php_version(&self) -> u32 {
        VOID_TYPE
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Closure(closure) = node else {
            return;
        };
        if closure.return_type_hint.is_some() || !has_exclusive_void(&closure.body) {
            return;
        }
        let at = match &closure.use_clause {
            Some(uses) => node_span(uses).end,
            None => node_span(&closure.parameter_list).end,
        };
        out.push(add_return_type(node_span(&closure.function), at, "void"));
    }
}
