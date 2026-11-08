//! `lint/complexity/prefer-in-array`, ported from Rector 2.8.4's `RepeatedOrEqualToInArrayRector`
//! (rules/CodeQuality/Rector/BooleanOr).
//!
//! Three comparisons or more of the same expression joined by `||`, `$a === 'a' || $a === 'b' || $a === 'c'`, become
//! `in_array($a, ['a', 'b', 'c'], true)`; with `==` the call is not strict. A chain that mixes `==` and `===` stays,
//! and so does one with a value that may have side effects (a call), or one in a default or class constant value.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::equal;
use super::rector_stmts::{
    compared_values, has_binary, is_in_constant_expression, is_safe_to_evaluate_eagerly,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferInArray;

impl Rule for PreferInArray {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-in-array"
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
        if !matches!(binary.operator, BinaryOperator::Or(_)) {
            return;
        }
        let or = |op: &BinaryOperator<'_>| matches!(op, BinaryOperator::Or(_));
        let compare = |op: &BinaryOperator<'_>| {
            matches!(op, BinaryOperator::Equal(_) | BinaryOperator::Identical(_))
        };
        // in_array() func call is not allowed in a constant expression
        if is_in_constant_expression(ancestors) {
            return;
        }
        let Some(pairs) = compared_values(binary, &or, &compare) else {
            return;
        };
        if pairs.len() < 3 || !pairs.iter().all(|(_, v)| is_safe_to_evaluate_eagerly(v)) {
            return;
        }
        // All compared expressions must be equal to the last one.
        let (compared, _) = pairs[pairs.len() - 1];
        if !pairs.iter().all(|(c, _)| equal(ctx, c, compared)) {
            return;
        }
        let identicals = has_binary(node, &|op| matches!(op, BinaryOperator::Identical(_)));
        let equals = has_binary(node, &|op| matches!(op, BinaryOperator::Equal(_)));
        // Mixed `===` and `==` stay as they are: https://3v4l.org/24cFl
        if identicals && equals {
            return;
        }
        let text = |e: &Expression<'_>| {
            let s = node_span(unparenthesize(e));
            &ctx.source[s.start..s.end]
        };
        let values: Vec<&str> = pairs.iter().map(|(_, v)| text(v)).collect();
        let strict = if identicals { ", true" } else { "" };
        let code = format!(
            "in_array({}, [{}]{strict})",
            text(compared),
            values.join(", ")
        );
        out.push(Report {
            span: node_span(binary),
            message: "Use `in_array()` instead of repeated `||` comparisons.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(binary), code)],
            }),
        });
    }
}
