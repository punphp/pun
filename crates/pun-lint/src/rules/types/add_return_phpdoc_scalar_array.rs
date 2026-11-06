//! `lint/types/add-return-phpdoc-scalar-array`, ported from Rector 2.6.1's
//! `AddReturnDocblockForScalarArrayFromAssignsRector` (rules/TypeDeclaration/Rector/ClassMethod): a function or method
//! that returns variables it starts as `[]` and only appends scalars of one type to gets `@return T[]`.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_phpstan::types::{Type, mixed};

use super::docblock::{FunctionLike, assignments, doc_report, variable_name};
use super::facts::facts;
use super::silent_void::Place;
use super::{class_scope, has_only_return_with_expr, returns_scoped};
use crate::rules::phpdoc_writer::{change_return_type, native_expr_type};
use crate::{Context, Report, Rule};

pub struct AddReturnPhpdocScalarArray;

impl Rule for AddReturnPhpdocScalarArray {
    fn id(&self) -> &'static str {
        "lint/types/add-return-phpdoc-scalar-array"
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
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        if f.has_non_array_return_type() {
            return;
        }
        let Some(body) = f.body else {
            return;
        };
        let target = f.target(ctx);
        let mut info = target.info(ctx.source);
        // `getReturnType()` must be the implicit `mixed` of no `@return` tag
        if info.return_tag().is_some() {
            return;
        }
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let returns = returns_scoped(body);
        if !has_only_return_with_expr(&place, body, &returns) {
            return;
        }
        // `extractReturnedVariableNames()`
        let mut names: Vec<&str> = Vec::new();
        for r in &returns {
            if let Some(name) = r.value.and_then(variable_name)
                && !names.contains(&name)
            {
                names.push(name);
            }
        }
        if names.is_empty() {
            return;
        }
        let mut types = Vec::new();
        for name in names {
            let Some(t) = scalar_array_type(ctx, node, name) else {
                return;
            };
            types.push(t);
        }
        if types.iter().any(|t| *t != types[0]) {
            return;
        }
        change_return_type(&mut info, &Type::array(mixed(), types[0].clone()));
        out.extend(doc_report(
            f.name,
            "Add the `@return` array type of the scalars appended to the returned variable.",
            ctx.source,
            &target,
            &info,
        ));
    }
}

/// `resolveScalarArrayTypeForVariable()`: `$name` is assigned `[]` and appended scalars of one type.
fn scalar_array_type(ctx: &Context<'_>, function: Node<'_, '_>, name: &str) -> Option<Type> {
    let (mut initialized, mut appended) = (false, false);
    let mut types = Vec::new();
    for a in assignments(function) {
        if variable_name(a.lhs) == Some(name)
            && match a.rhs {
                Expression::Array(arr) => arr.elements.is_empty(),
                Expression::LegacyArray(arr) => arr.elements.is_empty(),
                _ => false,
            }
        {
            initialized = true;
            continue;
        }
        let Expression::ArrayAppend(append) = a.lhs else {
            continue;
        };
        if variable_name(append.array) != Some(name) {
            continue;
        }
        appended = true;
        types.push(scalar_type(ctx, a.rhs)?);
    }
    if !initialized || !appended || types.iter().any(|t| *t != types[0]) {
        return None;
    }
    types.into_iter().next()
}

/// `resolveScalarType()`.
fn scalar_type(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<Type> {
    match expr {
        Expression::Literal(Literal::String(_)) => Some(Type::String),
        Expression::Literal(Literal::Integer(_)) => Some(Type::Int),
        Expression::Literal(Literal::Float(_)) => Some(Type::Float),
        _ => native_expr_type(ctx, expr).filter(is_scalar),
    }
}

/// `$type->isScalar()->yes()`.
fn is_scalar(t: &Type) -> bool {
    match t {
        Type::Bool
        | Type::ConstBool(_)
        | Type::Int
        | Type::ConstInt(_)
        | Type::IntRange(..)
        | Type::Float
        | Type::ConstFloat(_)
        | Type::String
        | Type::ConstString(_)
        | Type::ClassString(_) => true,
        Type::Union(types) => types.iter().all(is_scalar),
        _ => false,
    }
}
