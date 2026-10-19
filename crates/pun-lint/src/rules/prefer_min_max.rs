//! `lint/complexity/prefer-min-max`, ported from Rector 2.6.1's `NumberCompareToMaxFuncCallRector`
//! (rules/CodeQuality/Rector/Ternary).
//!
//! `$a < $b ? $b : $a` and `$a > $b ? $a : $b` are `max(...)` when both sides are integers (read from the syntax,
//! `syntax_types::is_int`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::syntax_types::is_int;
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferMinMax;

impl Rule for PreferMinMax {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-min-max"
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
        let (Some(then), Expression::Binary(compare)) =
            (ternary.then, unparenthesize(ternary.condition))
        else {
            return;
        };
        let source = ctx.source;
        let (first, second) = match compare.operator {
            BinaryOperator::LessThan(_) | BinaryOperator::LessThanOrEqual(_) => {
                (compare.rhs, compare.lhs)
            }
            BinaryOperator::GreaterThan(_) | BinaryOperator::GreaterThanOrEqual(_) => {
                (compare.lhs, compare.rhs)
            }
            _ => return,
        };
        if !same_expression(source, first, then) || !same_expression(source, second, ternary.r#else)
        {
            return;
        }
        if !is_int(ancestors, compare.lhs) || !is_int(ancestors, compare.rhs) {
            return;
        }
        let text = |e: &Expression<'_>| {
            let s = node_span(unparenthesize(e));
            source[s.start..s.end].to_string()
        };
        out.push(Report {
            span: node_span(ternary),
            message: "This ternary picks the larger number; use `max()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    node_span(ternary),
                    format!("max({}, {})", text(then), text(ternary.r#else)),
                )],
            }),
        });
    }
}
