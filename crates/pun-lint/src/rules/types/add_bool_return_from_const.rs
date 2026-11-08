//! `lint/types/add-bool-return-type-from-const`, ported from Rector 2.8.4's
//! `BoolReturnTypeFromBooleanConstReturnsRector` (rules/TypeDeclaration/Rector/ClassMethod): a function or method that
//! only returns `true` or `false`, and whose end cannot be reached, returns `bool`.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};

use super::facts::facts;
use super::silent_void::Place;
use super::{add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped};
use crate::{Context, Report, Rule};

pub struct AddBoolReturnFromConst;

impl Rule for AddBoolReturnFromConst {
    fn id(&self) -> &'static str {
        "lint/types/add-bool-return-type-from-const"
    }

    fn min_php_version(&self) -> u32 {
        super::SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        let returns = returns_scoped(f.body);
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts(ctx),
            ancestors,
            scope: scope.as_ref(),
        };
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        let boolean = |e: &Expression<'_>| {
            matches!(e, Expression::Literal(Literal::True(_) | Literal::False(_)))
        };
        if returns.iter().all(|r| r.value.is_some_and(boolean)) {
            out.push(add_return_type(f.span, f.at, "bool"));
        }
    }
}
