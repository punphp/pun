//! `lint/complexity/prefer-coalesce-over-empty-ternary`, ported from Rector 2.8.4's
//! `TernaryEmptyArrayArrayDimFetchToCoalesceRector` (rules/CodeQuality/Rector/Ternary).
//!
//! `! empty($items) ? $items[0] : 'default'` becomes `$items[0] ?? 'default'` when `$items` is an array (PHPStan's
//! `getType()`, read from the docblocks too).

use mago_syntax::cst::{Construct, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, in_parent, infix};
use super::types::facts::facts;
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferCoalesceOverEmptyTernary;

impl Rule for PreferCoalesceOverEmptyTernary {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-coalesce-over-empty-ternary"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(ternary) = node else {
            return;
        };
        let Expression::UnaryPrefix(not) = unparenthesize(ternary.condition) else {
            return;
        };
        if !not.operator.is_not() {
            return;
        }
        let Expression::Construct(Construct::Empty(empty)) = unparenthesize(not.operand) else {
            return;
        };
        let Some(then) = ternary.then else { return };
        let Expression::ArrayAccess(fetch) = unparenthesize(then) else {
            return;
        };
        let array = facts(ctx)
            .full_type(node_span(empty.value))
            .is_some_and(|t| !t.types.is_empty() && t.is_array());
        if !array || !same_expression(ctx.source, empty.value, fetch.array) {
            return;
        }
        let span = node_span(ternary);
        let code = infix(ctx, then, "??", ternary.r#else, (140, 141));
        out.push(Report {
            span,
            message: "Use `??` instead of a ternary on `! empty()` of the array it reads."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    span,
                    in_parent(span, ancestors, code, Prec::Infix(140, 140, 141)),
                )],
            }),
        });
    }
}
