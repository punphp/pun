//! `lint/types/add-nullable-return-type` and `lint/types/add-union-return-type`, ported from Rector 2.6.1's
//! `ReturnNullableTypeRector` and `ReturnUnionTypeRector` (rules/TypeDeclaration/Rector/ClassMethod, with
//! `AddUnionReturnType`): a function or method whose returns' native types make a union gets it, as `?T` from the
//! nullable rule (PHP 7.1) and as anything else from the union rule (PHP 8.0).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_syntax::cst::{Expression, Node, NodeKind};

use super::add_bool_return::unparenthesized;
use super::add_return_type_from_new::anonymous_parents;
use super::declare::{union_declaration, union_order, uniquate};
use super::facts::facts;
use super::native::native_type;
use super::silent_void::{Place, has_silent_void};
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
/// (`ReturnTypeInferer::inferFunctionLike`, constants kept) that `UnionTypeMapper` can write.
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
    if matches!(node, Node::Method(_)) && scope.as_ref().is_some_and(|s| s.is_interface) {
        return None;
    }
    let facts = facts(ctx);
    let place = Place {
        facts: &facts,
        ancestors,
        scope: scope.as_ref(),
    };
    let returns = returns_scoped(f.body);
    if returns.is_empty() {
        return None;
    }
    // `NewTypeResolver`: `new class extends A {}` with one parent is that parent (`ObjectWithoutClassTypeMapper`)
    let mut parents: Vec<Option<TAtomic>> = Vec::new();
    for r in &returns {
        parents.push(match r.value.map(unparenthesized) {
            Some(Expression::AnonymousClass(class)) => {
                match anonymous_parents(ancestors, class).as_slice() {
                    [parent] => {
                        let class = facts.codebase.get_class_like(parent.as_bytes())?;
                        Some(TAtomic::Object(TObject::Named(TNamedObject::new(
                            class.name,
                        ))))
                    }
                    _ => None,
                }
            }
            _ => None,
        });
    }
    let mut types: Vec<&TAtomic> = Vec::new();
    let mut has_void = false;
    for (r, parent) in returns.iter().zip(&parents) {
        match (r.value, parent) {
            (None, _) => has_void = true,
            (Some(_), Some(parent)) => types.push(parent),
            (Some(value), None) => {
                types.extend(union_order(native_type(&facts, value)?.types.as_ref()))
            }
        }
    }
    if has_void || has_silent_void(&place, f.body.statements.iter()) {
        // `void` has no place in a union
        return None;
    }
    let types = uniquate(types);
    if types.len() < 2
        || types
            .iter()
            .any(|t| matches!(t, TAtomic::Mixed(_) | TAtomic::Never))
    {
        return None;
    }
    let r#type = union_declaration(&types, &facts.codebase, ctx.php_version)?;
    Some((f, r#type))
}
