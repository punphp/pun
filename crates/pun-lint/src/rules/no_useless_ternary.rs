//! `lint/dead-code/no-useless-ternary`, ported from Rector 2.6.1's `RemoveUselessTernaryRector`
//! (rules/DeadCode/Rector/Ternary).
//!
//! `$b ?: false` on a `bool`, `$a ?: []` on an `array` and `$i ?: 0` on an `int` are the condition itself, and so are
//! `$b ? $b : false` and `$b ? true : false`. Rector reads the condition's native type; Pun takes a `bool` from the
//! syntax (`syntax_types::is_bool`) and an `array` or `int` from a parameter declared so and never written.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::syntax_types::{is_bool, parameter_hint};
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessTernary;

#[derive(PartialEq)]
enum Native {
    Bool,
    Array,
    Int,
    Other,
}

impl Rule for NoUselessTernary {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-ternary"
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
        let source = ctx.source;
        let condition = unparenthesize(ternary.condition);
        if matches!(condition, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)))
        {
            return;
        }
        let native = native(ancestors, condition);
        let is = |e: &Expression<'_>, value: bool| {
            matches!(
                (unparenthesize(e), value),
                (Expression::Literal(Literal::True(_)), true)
                    | (Expression::Literal(Literal::False(_)), false)
            )
        };
        let useless = if native == Native::Bool
            && ternary.then.is_some_and(|t| is(t, true))
            && is(ternary.r#else, false)
        {
            true
        } else if ternary
            .then
            .is_some_and(|t| !same_expression(source, t, ternary.condition))
        {
            false
        } else {
            match (native, unparenthesize(ternary.r#else)) {
                (Native::Bool, e) => is(e, false),
                (Native::Array, Expression::Array(a)) => a.elements.is_empty(),
                (Native::Array, Expression::LegacyArray(a)) => a.elements.is_empty(),
                (Native::Int, Expression::Literal(Literal::Integer(i))) => i.value == Some(0),
                _ => false,
            }
        };
        if !useless {
            return;
        }
        let span = node_span(condition);
        out.push(Report {
            span: node_span(ternary),
            message: "This ternary gives back its condition either way; use the condition."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(ternary), &source[span.start..span.end])],
            }),
        });
    }
}

fn native(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Native {
    if is_bool(ancestors, expr) {
        return Native::Bool;
    }
    let Expression::Variable(Variable::Direct(v)) = expr else {
        return Native::Other;
    };
    match parameter_hint(ancestors, v.name) {
        Some(Hint::Array(_)) => Native::Array,
        Some(Hint::Integer(_)) => Native::Int,
        _ => Native::Other,
    }
}
