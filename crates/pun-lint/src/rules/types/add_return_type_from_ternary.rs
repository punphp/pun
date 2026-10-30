//! `lint/types/add-return-type-from-ternary`, ported from Rector 2.6.1's `ReturnTypeFromStrictTernaryRector`
//! (rules/TypeDeclaration/Rector/Class_): a function or method whose one `return` returns a ternary with a known
//! native type, and whose returns (`null` aside) make one type, gets the ternary's type. Rector writes the ternary's
//! docblock-aware type; Pun, reading native types only, writes the native one.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Expression, Node, NodeKind};

use super::declare::{return_declaration, union_order};
use super::facts::facts;
use super::native::native_type;
use super::silent_void::Place;
use super::type_factory::{infer_returns, is_union_without_null, without_covered_objects};
use super::{
    SCALAR_TYPES, add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped,
};
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromTernary;

impl Rule for AddReturnTypeFromTernary {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-ternary"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
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
        let scope = class_scope(ancestors);
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let returns = returns_scoped(f.body);
        // shouldSkip: the inferred return type without `null` is a union
        if is_union_without_null(&infer_returns(&place, f.body, &returns)) {
            return;
        }
        let [r] = returns.as_slice() else {
            return;
        };
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        let Some(ternary @ Expression::Conditional(_)) = r.value else {
            return;
        };
        let Some(t) = native_type(&facts, ternary) else {
            return;
        };
        if t.types.iter().any(|t| matches!(t, TAtomic::Mixed(_))) {
            return;
        }
        if let Some(r#type) = return_declaration(
            without_covered_objects(union_order(t.types.as_ref()), &facts.codebase),
            &facts.codebase,
            ctx.php_version,
            &[],
            true,
        ) {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}
