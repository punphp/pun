//! `lint/types/add-string-return-type` and `lint/types/add-numeric-return-type`, ported from Rector 2.8.4's
//! `StringReturnTypeFromStrictStringReturnsRector` and `NumericReturnTypeFromStrictReturnsRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): a function or method whose every `return` returns a value natively
//! typed `string` (or `int`, or `float`), and whose end cannot be reached, gets that return type. Returns of only
//! literals are left to the `-from-scalar` rules.

use mago_codex::ttype::atomic::TAtomic;
use mago_syntax::cst::{CompositeString, Expression, Literal, Node, NodeKind};

use super::add_scalar_return::{returned, unnegated};
use super::facts::facts;
use super::{SCALAR_TYPES, add_return_type};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddStringReturn;

pub struct AddNumericReturn;

impl Rule for AddStringReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-string-return-type"
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
        // hasAlwaysStringScalarReturn
        let literal = |e: &&Expression<'_>| {
            matches!(
                e,
                Expression::Literal(Literal::String(_))
                    | Expression::CompositeString(
                        CompositeString::Interpolated(_) | CompositeString::Document(_)
                    )
            )
        };
        if values.iter().all(literal) {
            return;
        }
        let facts = facts(ctx);
        if values.iter().all(|v| {
            facts.native_type(node_span(*v)).is_some_and(|t| {
                !t.types.is_empty()
                    && t.types
                        .iter()
                        .all(|a| matches!(a, TAtomic::Scalar(s) if s.is_any_string()))
            })
        }) {
            out.push(add_return_type(f.span, f.at, "string"));
        }
    }
}

impl Rule for AddNumericReturn {
    fn id(&self) -> &'static str {
        "lint/types/add-numeric-return-type"
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
        // isAlwaysNumeric
        let all = |kind: fn(&Expression<'_>) -> bool| values.iter().map(unnegated).all(kind);
        if all(|e| matches!(e, Expression::Literal(Literal::Float(_))))
            || all(|e| matches!(e, Expression::Literal(Literal::Integer(_))))
        {
            return;
        }
        let facts = facts(ctx);
        let types: Vec<_> = values
            .iter()
            .map(|v| facts.native_type(node_span(*v)))
            .collect();
        let r#type = if types
            .iter()
            .all(|t| t.is_some_and(mago_codex::ttype::union::TUnion::is_float))
        {
            "float"
        } else if types
            .iter()
            .all(|t| t.is_some_and(|t| !t.types.is_empty() && t.is_int()))
        {
            "int"
        } else {
            return;
        };
        out.push(add_return_type(f.span, f.at, r#type));
    }
}
