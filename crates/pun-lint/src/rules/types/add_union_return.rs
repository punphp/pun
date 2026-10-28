//! `lint/types/add-nullable-return-type` and `lint/types/add-union-return-type`, ported from Rector 2.6.1's
//! `ReturnNullableTypeRector` and `ReturnUnionTypeRector` (rules/TypeDeclaration/Rector/ClassMethod, with
//! `AddUnionReturnType`): a function or method whose returns' native types make a union gets it, as `?T` from the
//! nullable rule (PHP 7.1) and as anything else from the union rule (PHP 8.0).

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{Node, NodeKind};

use super::declare::union_declaration;
use super::facts::facts;
use super::silent_void::Place;
use super::type_factory::{Created, infer_returns};
use super::{Untyped, add_return_type, class_scope, returns_scoped, untyped};
use crate::{Context, Report, Rule};

pub struct AddNullableReturn;

pub struct AddUnionReturn;

/// `PhpVersionFeature::NULLABLE_TYPE`.
const NULLABLE_TYPE: u32 = 70100;
/// `PhpVersionFeature::UNION_TYPES`.
const UNION_TYPES: u32 = 80000;

impl Rule for AddNullableReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-nullable-return-type"
    }

    fn min_php_version(&self) -> u32 {
        NULLABLE_TYPE
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
        if let Some((f, r#type)) = union_return(node, ancestors, ctx)
            && r#type.starts_with('?')
        {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}

impl Rule for AddUnionReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-union-return-type"
    }

    fn min_php_version(&self) -> u32 {
        UNION_TYPES
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
        if let Some((f, r#type)) = union_return(node, ancestors, ctx)
            && !r#type.starts_with('?')
        {
            out.push(add_return_type(f.span, f.at, &r#type));
        }
    }
}

/// The untyped function or method and the declaration of its inferred return type, when that type is a union
/// (`ReturnTypeInferer::inferFunctionLike`) that `UnionTypeMapper` can write.
fn union_return<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &Context<'_>,
) -> Option<(Untyped<'ast, 'arena>, String)> {
    let f = untyped(node, ancestors, ctx)?;
    if f.body.statements.is_empty() {
        return None;
    }
    let scope = class_scope(ancestors);
    let facts = facts(ctx);
    let place = Place {
        facts: &facts,
        ancestors,
        scope: scope.as_ref(),
    };
    let Created::Union(types) = infer_returns(&place, f.body, &returns_scoped(f.body)) else {
        return None;
    };
    let types: Vec<&TAtomic> = types.iter().collect();
    let r#type = union_declaration(&types, &facts.codebase, ctx.php_version)?;
    Some((f, r#type))
}
