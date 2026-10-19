//! `lint/dead-code/no-bool-ternary`, ported from Rector 2.6.1's `UnnecessaryTernaryExpressionRector`
//! (rules/CodeQuality/Rector/Ternary).
//!
//! `cond ? true : false` is `cond` (or `(bool) cond` when it may not be a `bool`); `cond ? false : true` is its
//! negation, written as the inverse comparison when `cond` is one. PHPStan's `isBoolean()` is read from the syntax
//! (`syntax_types::is_bool`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::syntax_types::is_bool;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoBoolTernary;

impl Rule for NoBoolTernary {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-bool-ternary"
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
        let (Some(then), Some(r#else)) = (truth(then), truth(ternary.r#else)) else {
            return;
        };
        let source = ctx.source;
        let text = |e: &Expression<'_>| {
            let s = node_span(e);
            source[s.start..s.end].to_string()
        };
        let condition = unparenthesize(ternary.condition);
        let replacement = match condition {
            Expression::Binary(b) if !matches!(b.operator, BinaryOperator::Instanceof(_)) => {
                if then && !r#else {
                    text(condition)
                } else {
                    let Some(inverse) = inverse(&b.operator) else {
                        return;
                    };
                    format!("{} {inverse} {}", text(b.lhs), text(b.rhs))
                }
            }
            Expression::Pipe(_) if then && !r#else => text(condition),
            Expression::Pipe(_) => return,
            _ if then && !r#else => cast(ancestors, condition, &text),
            _ if !then && r#else => match condition {
                Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                    cast(ancestors, u.operand, &text)
                }
                _ if is_bool(ancestors, condition) => format!("!{}", wrapped(condition, &text)),
                _ => format!("!(bool) {}", wrapped(condition, &text)),
            },
            _ => return,
        };
        out.push(Report {
            span: node_span(ternary),
            message:
                "This ternary only turns its condition into `true` or `false`; use the condition."
                    .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(node_span(ternary), replacement)],
            }),
        });
    }
}

/// `ValueResolver::isTrue` / `isFalse`.
fn truth(expr: &Expression<'_>) -> Option<bool> {
    match unparenthesize(expr) {
        Expression::Literal(Literal::True(_)) => Some(true),
        Expression::Literal(Literal::False(_)) => Some(false),
        _ => None,
    }
}

/// `AssignAndBinaryMap::getInversed`.
fn inverse(op: &BinaryOperator<'_>) -> Option<&'static str> {
    Some(match op {
        BinaryOperator::Identical(_) => "!==",
        BinaryOperator::NotIdentical(_) => "===",
        BinaryOperator::Equal(_) => "!=",
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => "==",
        BinaryOperator::GreaterThan(_) => "<=",
        BinaryOperator::LessThan(_) => ">=",
        BinaryOperator::GreaterThanOrEqual(_) => "<",
        BinaryOperator::LessThanOrEqual(_) => ">",
        _ => return None,
    })
}

/// The expression itself when it is a `bool`, else `(bool) expr`.
fn cast(
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    text: &dyn Fn(&Expression<'_>) -> String,
) -> String {
    if is_bool(ancestors, expr) {
        text(unparenthesize(expr))
    } else {
        format!("(bool) {}", wrapped(expr, text))
    }
}

/// The expression's text, in parentheses when a prefix operator would bind tighter than it.
fn wrapped(expr: &Expression<'_>, text: &dyn Fn(&Expression<'_>) -> String) -> String {
    let expr = unparenthesize(expr);
    match expr {
        Expression::Binary(_)
        | Expression::Assignment(_)
        | Expression::Conditional(_)
        | Expression::Yield(_)
        | Expression::Throw(_)
        | Expression::Pipe(_)
        | Expression::ArrowFunction(_)
        | Expression::Closure(_)
        | Expression::Construct(
            Construct::Print(_)
            | Construct::Include(_)
            | Construct::IncludeOnce(_)
            | Construct::Require(_)
            | Construct::RequireOnce(_),
        ) => {
            format!("({})", text(expr))
        }
        _ => text(expr),
    }
}
