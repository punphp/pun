//! `lint/types/add-return-type-from-constant`, ported from Rector 2.6.1's `ReturnTypeFromStrictConstantReturnRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a method whose every `return` returns a constant (`FOO`,
//! `Class::FOO`, and `true`, `false` and `null`, which PHP-Parser reads as constants) gets the union of their types,
//! values dropped.

use mago_syntax::cst::{Access, Expression, Literal, Node, NodeKind};

use super::declare::{return_declaration, union_order};
use super::facts::facts;
use super::native::native_type;
use super::silent_void::Place;
use super::{add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped};
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromConstant;

impl Rule for AddReturnTypeFromConstant {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-constant"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
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
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        // matchAlwaysReturnConstFetch
        let mut types = Vec::new();
        // `has_only_return_with_expr` made sure each return has a value.
        for value in returns.iter().filter_map(|r| r.value) {
            if !matches!(
                value,
                Expression::ConstantAccess(_)
                    | Expression::Access(Access::ClassConstant(_))
                    | Expression::Literal(Literal::True(_) | Literal::False(_) | Literal::Null(_))
            ) {
                return;
            }
            let Some(t) = native_type(&facts, value) else {
                return;
            };
            types.extend(union_order(t.types.as_ref()));
        }
        if let Some(r#type) =
            return_declaration(types, &facts.codebase, ctx.php_version, &[], false)
        {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}
