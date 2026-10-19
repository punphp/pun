//! `lint/dead-code/no-false-ternary`, ported from Rector 2.6.1's `TernaryToBooleanOrFalseToBooleanAndRector`
//! (rules/DeadCode/Rector/Ternary).
//!
//! `cond ? $bool : false` is `cond && $bool` when `$bool` is a `bool` (read from the syntax, `syntax_types::is_bool`)
//! other than `true`.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::syntax_types::is_bool;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoFalseTernary;

impl Rule for NoFalseTernary {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-false-ternary"
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
        let Some(then) = ternary.then else {
            return;
        };
        if !matches!(
            unparenthesize(ternary.r#else),
            Expression::Literal(Literal::False(_))
        ) || matches!(unparenthesize(then), Expression::Literal(Literal::True(_)))
            || !is_bool(ancestors, then)
        {
            return;
        }
        let source = ctx.source;
        let text = |e: &Expression<'_>, right: bool| {
            let e = unparenthesize(e);
            let s = node_span(e);
            let t = &source[s.start..s.end];
            if below_and(e, right) {
                format!("({t})")
            } else {
                t.to_string()
            }
        };
        out.push(Report {
            span: node_span(ternary),
            message: "A ternary that falls back to `false` is an `&&`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(
                    node_span(ternary),
                    format!("{} && {}", text(ternary.condition, false), text(then, true)),
                )],
            }),
        });
    }
}

/// Whether php-parser's printer puts `expr` in parentheses as an operand of `&&`: it binds looser (or, on the
/// right of the left-associative `&&`, the same).
fn below_and(expr: &Expression<'_>, right: bool) -> bool {
    match expr {
        Expression::Binary(b) => match b.operator {
            BinaryOperator::And(_) => right,
            BinaryOperator::Or(_)
            | BinaryOperator::NullCoalesce(_)
            | BinaryOperator::LowAnd(_)
            | BinaryOperator::LowOr(_)
            | BinaryOperator::LowXor(_) => true,
            _ => false,
        },
        Expression::Conditional(_)
        | Expression::Assignment(_)
        | Expression::Yield(_)
        | Expression::Throw(_)
        | Expression::ArrowFunction(_)
        | Expression::Pipe(_)
        | Expression::Construct(Construct::Print(_)) => true,
        _ => false,
    }
}
