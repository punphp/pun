//! `lint/types/add-function-void-return`, ported from Rector 2.6.1's `AddFunctionVoidReturnTypeWhereNoReturnRector`
//! (rules/TypeDeclaration/Rector/Function_).
//!
//! A function without a return type that returns no value and does not yield gets `: void`.

use mago_syntax::cst::{Node, NodeKind};

use super::{VOID_TYPE, add_return_type, has_exclusive_void};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddFunctionVoidReturn;

impl Rule for AddFunctionVoidReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-function-void-return"
    }

    fn min_php_version(&self) -> u32 {
        VOID_TYPE
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Function(function) = node else {
            return;
        };
        if function.return_type_hint.is_some() || !has_exclusive_void(&function.body) {
            return;
        }
        out.push(add_return_type(
            node_span(&function.name),
            node_span(&function.parameter_list).end,
            "void",
        ));
    }
}
