//! `lint/complexity/prefer-not-in-array`, ported from Rector 2.6.1's `RepeatedAndNotEqualToNotInArrayRector`
//! (rules/CodeQuality/Rector/BooleanAnd).
//!
//! Three negated comparisons or more of the same expression joined by `&&`,
//! `$a !== 'a' && $a !== 'b' && $a !== 'c'`, become `!in_array($a, ['a', 'b', 'c'], true)`; with `!=` (or a mix of
//! `!=` and `!==`) the call is not strict.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{Prec, equal, in_parent};
use super::rector_stmts::{compared_values, has_binary};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferNotInArray;

impl Rule for PreferNotInArray {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-not-in-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        if !matches!(binary.operator, BinaryOperator::And(_)) {
            return;
        }
        let and = |op: &BinaryOperator<'_>| matches!(op, BinaryOperator::And(_));
        let compare = |op: &BinaryOperator<'_>| op.is_negated_equality();
        let Some(pairs) = compared_values(binary, &and, &compare) else {
            return;
        };
        if pairs.len() < 3 {
            return;
        }
        // All compared expressions must be equal to the last one.
        let (compared, _) = pairs[pairs.len() - 1];
        if !pairs.iter().all(|(c, _)| equal(ctx, c, compared)) {
            return;
        }
        // isStrictComparison: `!==` only; a mix with `!=` is not strict (https://3v4l.org/2SoHZ).
        let not_identicals = has_binary(node, &|op| matches!(op, BinaryOperator::NotIdentical(_)));
        let not_equals = has_binary(node, &|op| {
            matches!(
                op,
                BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_)
            )
        });
        let text = |e: &Expression<'_>| {
            let s = node_span(unparenthesize(e));
            &ctx.source[s.start..s.end]
        };
        let values: Vec<&str> = pairs.iter().map(|(_, v)| text(v)).collect();
        let strict = if not_identicals && !not_equals {
            ", true"
        } else {
            ""
        };
        let span = node_span(binary);
        let code = format!(
            "!in_array({}, [{}]{strict})",
            text(compared),
            values.join(", ")
        );
        out.push(Report {
            span,
            message: "Use `!in_array()` instead of repeated `&&` comparisons.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    span,
                    in_parent(span, ancestors, code, Prec::Prefix(30)),
                )],
            }),
        });
    }
}
