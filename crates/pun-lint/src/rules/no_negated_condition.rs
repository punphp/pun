//! `lint/refactor/no-negated-condition`, ported from Mago 1.57.0's `no-negated-ternary`
//! (mago-linter src/rule/clarity/no_negated_ternary.rs).
//!
//! Reports a ternary (`a ? b : c`, not `a ?: c`) whose condition is negated: `!$a`, or a `!==`, `!=` or `<>`
//! comparison. Mago only reports; Pun's unsafe fix does what Mago's help says, removing the `!` (or turning the
//! operator into `===` / `==`) and swapping the two branches.

use mago_span::HasSpan;
use mago_syntax::cst::{
    Binary, BinaryOperator, Conditional, Expression, Node, NodeKind, UnaryPrefix,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct NoNegatedCondition;

impl Rule for NoNegatedCondition {
    fn id(&self) -> &'static str {
        "lint/refactor/no-negated-condition"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(Conditional {
            condition,
            then: Some(then),
            r#else,
            ..
        }) = node
        else {
            return;
        };
        let (reported, help, condition_edit) = match condition {
            Expression::UnaryPrefix(UnaryPrefix { operator, operand }) if operator.is_not() => {
                let operand_text =
                    &ctx.source[span(operand.span()).start..span(operand.span()).end];
                let text = if matches!(operand, Expression::Assignment(_)) {
                    format!("({operand_text})")
                } else {
                    operand_text.to_string()
                };
                (
                    span(condition.span()),
                    "Remove the `!` and swap the `then` and `else` branches.",
                    Edit::new(span(condition.span()), text),
                )
            }
            Expression::Binary(Binary { operator, .. }) => {
                let (help, replacement) = match operator {
                    BinaryOperator::NotIdentical(_) => (
                        "Replace `!==` with `===` and swap the `then` and `else` branches.",
                        "===",
                    ),
                    BinaryOperator::NotEqual(_) => (
                        "Replace `!=` with `==` and swap the `then` and `else` branches.",
                        "==",
                    ),
                    BinaryOperator::AngledNotEqual(_) => (
                        "Replace `<>` with `==` and swap the `then` and `else` branches.",
                        "==",
                    ),
                    _ => return,
                };
                let at = span(operator.span());
                (at, help, Edit::new(at, replacement))
            }
            _ => return,
        };
        let text = |s: Span| ctx.source[s.start..s.end].to_string();
        // The `then` branch moves to the `else` place, where an operator weaker than `?:` needs parentheses.
        let then_text = if needs_parentheses_as_else(then) {
            format!("({})", text(span(then.span())))
        } else {
            text(span(then.span()))
        };
        let (then, r#else) = (span(then.span()), span(r#else.span()));
        out.push(Report {
            span: reported,
            message: "Negated ternary condition.".to_string(),
            help: Some(help.to_string()),
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![
                    condition_edit,
                    Edit::new(then, text(r#else)),
                    Edit::new(r#else, then_text),
                ],
            }),
        });
    }
}

/// Whether `expr`, written bare after the `:` of a ternary, would not parse as that whole operand: an assignment, a
/// ternary, `yield` or a low-precedence `and` / `or` / `xor`.
fn needs_parentheses_as_else(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Assignment(_) | Expression::Conditional(_) | Expression::Yield(_) => true,
        Expression::Binary(binary) => matches!(
            binary.operator,
            BinaryOperator::LowAnd(_) | BinaryOperator::LowOr(_) | BinaryOperator::LowXor(_)
        ),
        _ => false,
    }
}
