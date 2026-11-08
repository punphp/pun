//! `lint/types/add-closure-never-return`, ported from Rector 2.8.4's `AddClosureNeverReturnTypeRector`
//! (rules/TypeDeclaration/Rector/Closure): a closure that never returns gets the `never` return type (see
//! [`super::never`]).

use mago_syntax::cst::{Node, NodeKind};

use super::never::{FunctionLike, NEVER_TYPE, add_never};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddClosureNeverReturn;

impl Rule for AddClosureNeverReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-closure-never-return"
    }

    fn min_php_version(&self) -> u32 {
        NEVER_TYPE
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Closure])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Closure(closure) = node else {
            return;
        };
        let at = match &closure.use_clause {
            Some(uses) => node_span(uses).end,
            None => node_span(&closure.parameter_list).end,
        };
        let f = FunctionLike {
            span: node_span(&closure.function),
            at,
            return_type: closure.return_type_hint.as_ref(),
            body: &closure.body,
            method: None,
        };
        out.extend(add_never(f, ancestors, ctx));
    }
}
