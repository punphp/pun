//! `lint/types/add-return-type-from-cast`, ported from Rector 2.6.1's `ReturnTypeFromReturnCastRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) and its `AddReturnTypeFromCast`: a function or method whose every
//! `return` returns a cast to the same type, and whose end cannot be reached, gets that return type.
//!
//! Pun leaves a function alone when a cast is of a scalar literal (`(int) '5'`): PHPStan types that as the constant,
//! and Rector's result then depends on how the constants combine.

use mago_syntax::cst::{Block, Expression, Node, NodeKind, UnaryPrefixOperator};

use super::facts::facts;
use super::silent_void::{Place, has_silent_void};
use super::{add_return_type, any_scoped, class_scope, untyped};
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromCast;

impl Rule for AddReturnTypeFromCast {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-cast"
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
        let body = f.body;
        let Some(r#type) = cast_type(body) else {
            return;
        };
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts(ctx),
            ancestors,
            scope: scope.as_ref(),
        };
        if has_silent_void(&place, body.statements.iter()) {
            return;
        }
        out.push(add_return_type(f.span, f.at, r#type));
    }
}

/// The one type every `return` of the body casts to, if each returns a cast and there is one.
fn cast_type(body: &Block<'_>) -> Option<&'static str> {
    let mut types = Vec::new();
    let mut other = false;
    any_scoped(Node::Block(body), &mut |n| {
        if let Node::Yield(_) = n {
            other = true;
        }
        if let Node::Return(r) = n {
            match r.value.map(cast_of) {
                Some(Some(t)) => types.push(t),
                _ => other = true,
            }
        }
        other
    });
    if other {
        return None;
    }
    let first = *types.first()?;
    types.iter().all(|t| *t == first).then_some(first)
}

/// The type a cast expression has, when Pun can tell it as PHPStan would.
fn cast_of(expr: &Expression<'_>) -> Option<&'static str> {
    let expr = match expr {
        Expression::Parenthesized(p) => p.expression,
        other => other,
    };
    let Expression::UnaryPrefix(prefix) = expr else {
        return None;
    };
    let scalar = |t| (!matches!(prefix.operand, Expression::Literal(_))).then_some(t);
    match prefix.operator {
        UnaryPrefixOperator::IntCast(..) | UnaryPrefixOperator::IntegerCast(..) => scalar("int"),
        UnaryPrefixOperator::FloatCast(..)
        | UnaryPrefixOperator::DoubleCast(..)
        | UnaryPrefixOperator::RealCast(..) => scalar("float"),
        UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..) => {
            scalar("string")
        }
        UnaryPrefixOperator::BoolCast(..) | UnaryPrefixOperator::BooleanCast(..) => scalar("bool"),
        UnaryPrefixOperator::ArrayCast(..) => Some("array"),
        UnaryPrefixOperator::ObjectCast(..) => Some("\\stdClass"),
        _ => None,
    }
}
