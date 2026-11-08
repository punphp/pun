//! `lint/types/add-string-return-type-from-scalar` and `lint/types/add-numeric-return-type-from-scalar`, ported from
//! Rector 2.8.4's `StringReturnTypeFromStrictScalarReturnsRector` and `NumericReturnTypeFromStrictScalarReturnsRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a function or method whose every `return` returns a string literal
//! returns `string`; one whose every `return` returns an integer literal (or a float literal), maybe negated, returns
//! `int` (or `float`). The end of the body must not be reachable.

use mago_syntax::cst::{CompositeString, Expression, Literal, Node, NodeKind, UnaryPrefixOperator};

use super::facts::facts;
use super::silent_void::Place;
use super::{
    SCALAR_TYPES, add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped,
};
use crate::{Context, Report, Rule};

pub struct AddStringReturnFromScalar;

pub struct AddNumericReturnFromScalar;

/// The returned expressions of an untyped function or method whose every `return` has one and whose end cannot be
/// reached, with where its report and type go.
pub(super) fn returned<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &Context<'_>,
) -> Option<(super::Untyped<'ast, 'arena>, Vec<&'ast Expression<'arena>>)> {
    let f = untyped(node, ancestors, ctx)?;
    let returns = returns_scoped(f.body);
    let scope = class_scope(ancestors);
    let place = Place {
        facts: &facts(ctx),
        ancestors,
        scope: scope.as_ref(),
    };
    if !has_only_return_with_expr(&place, f.body, &returns) {
        return None;
    }
    let values = returns.iter().filter_map(|r| r.value).collect();
    Some((f, values))
}

impl Rule for AddStringReturnFromScalar {
    fn id(&self) -> &'static str {
        "lint/types/add-string-return-type-from-scalar"
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
        let Some((f, values)) = returned(node, ancestors, ctx) else {
            return;
        };
        // php-parser's `String_` and `InterpolatedString`: quoted strings, heredocs and nowdocs, not backticks
        let string = |e: &&Expression<'_>| {
            matches!(
                e,
                Expression::Literal(Literal::String(_))
                    | Expression::CompositeString(
                        CompositeString::Interpolated(_) | CompositeString::Document(_)
                    )
            )
        };
        if values.iter().all(string) {
            out.push(add_return_type(f.span, f.at, "string"));
        }
    }
}

impl Rule for AddNumericReturnFromScalar {
    fn id(&self) -> &'static str {
        "lint/types/add-numeric-return-type-from-scalar"
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
        let Some((f, values)) = returned(node, ancestors, ctx) else {
            return;
        };
        let r#type = if values
            .iter()
            .map(unnegated)
            .all(|e| matches!(e, Expression::Literal(Literal::Float(_))))
        {
            "float"
        } else if values
            .iter()
            .map(unnegated)
            .all(|e| matches!(e, Expression::Literal(Literal::Integer(_))))
        {
            "int"
        } else {
            return;
        };
        out.push(add_return_type(f.span, f.at, r#type));
    }
}

/// The expression under a leading `-`.
pub(super) fn unnegated<'a>(e: &&'a Expression<'a>) -> &'a Expression<'a> {
    match e {
        Expression::UnaryPrefix(p) if matches!(p.operator, UnaryPrefixOperator::Negation(_)) => {
            p.operand
        }
        other => other,
    }
}
