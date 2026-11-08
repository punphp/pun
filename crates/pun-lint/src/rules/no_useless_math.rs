//! `lint/dead-code/no-useless-math`, ported from Rector 2.8.4's `RemoveDeadZeroAndOneOperationRector`
//! (rules/DeadCode/Rector/Plus).
//!
//! An operation that cannot change a number goes, when its operands' native types are each all integers or all
//! floats: `$a + 0`, `$a - 0`, `$a * 1`, `$a / 1`, `1 * $a`, `0 + $a`, and `0 - $a`, which becomes `-$a`; and
//! `$a += 0`, `$a -= 0`, `$a *= 1` and `$a /= 1`, which become `$a`. The literal must be the integer `0` or `1`. A
//! zero added to (or taken from, on the left) a value that may be a float stays, since it turns `-0.0` into `0.0`;
//! a multiplication or division with a class constant stays.
//!
//! The kept operand keeps its code. Rector's printer puts it in parentheses when it is an operation in parentheses
//! on the right of `1 *`; elsewhere it adds none, even where the new place binds tighter (`($a + $b) * 1 * 2`
//! becomes `$a + $b * 2`). Pun keeps the parentheses that the meaning needs (lint.md: meaning over bytes).

use mago_syntax::cst::{
    Access, AssignmentOperator, BinaryOperator, Expression, Literal, Node, NodeKind,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{keeps_parentheses, operand_precedence, parent_of, tokens};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessMath;

impl Rule for NoUselessMath {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-math"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary, NodeKind::Assignment])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let source = ctx.source;
        let parent = parent_of(ancestors);
        let text = match node {
            Node::Assignment(assignment) => {
                let facts = facts(ctx);
                let rhs = unparenthesize(assignment.rhs);
                let kept = match assignment.operator {
                    AssignmentOperator::Addition(_) | AssignmentOperator::Subtraction(_) => {
                        is_literal(rhs, 0)
                            && (matches!(assignment.operator, AssignmentOperator::Subtraction(_))
                                || is_integer(&facts, assignment.lhs))
                    }
                    AssignmentOperator::Multiplication(_) | AssignmentOperator::Division(_) => {
                        is_literal(rhs, 1)
                    }
                    _ => return,
                };
                if !kept || !is_number(&facts, assignment.lhs) {
                    return;
                }
                operand_text(source, assignment.lhs, parent)
            }
            _ => {
                let Node::Binary(binary) = node else { return };
                let (left, right) = (unparenthesize(binary.lhs), unparenthesize(binary.rhs));
                let plus = matches!(binary.operator, BinaryOperator::Addition(_));
                match binary.operator {
                    BinaryOperator::Addition(_) | BinaryOperator::Subtraction(_) => {
                        let facts = facts(ctx);
                        if !is_number(&facts, binary.lhs) || !is_number(&facts, binary.rhs) {
                            return;
                        }
                        if is_literal(left, 0) {
                            // on a float, `0 + $x` turns -0.0 into 0.0 and `0 - $x` flips its sign
                            if !is_integer(&facts, binary.rhs) {
                                return;
                            }
                            if plus {
                                operand_text(source, binary.rhs, parent)
                            } else {
                                negated(source, right)
                            }
                        } else if is_literal(right, 0) && (!plus || is_integer(&facts, binary.lhs))
                        {
                            operand_text(source, binary.lhs, parent)
                        } else {
                            return;
                        }
                    }
                    BinaryOperator::Multiplication(_) | BinaryOperator::Division(_) => {
                        let class_constant = |e: &Expression<'_>| {
                            matches!(e, Expression::Access(Access::ClassConstant(_)))
                        };
                        if class_constant(left) || class_constant(right) {
                            return;
                        }
                        let facts = facts(ctx);
                        if !is_number(&facts, binary.lhs) || !is_number(&facts, binary.rhs) {
                            return;
                        }
                        let times = matches!(binary.operator, BinaryOperator::Multiplication(_));
                        if times && is_literal(left, 1) {
                            if is_parenthesized_operation(ctx, node_span(binary), binary.rhs) {
                                format!("({})", text_of(source, right))
                            } else {
                                operand_text(source, binary.rhs, parent)
                            }
                        } else if is_literal(right, 1) {
                            operand_text(source, binary.lhs, parent)
                        } else {
                            return;
                        }
                    }
                    _ => return,
                }
            }
        };
        let span = node_span(&node);
        out.push(Report {
            span,
            message: "This operation does not change the number; keep the number.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, text)],
            }),
        });
    }
}

/// `isLiteralZero` and `isLiteralOne`: an integer literal of value `value`.
fn is_literal(expr: &Expression<'_>, value: u64) -> bool {
    matches!(expr, Expression::Literal(Literal::Integer(i)) if i.value == Some(value))
}

/// `getNativeType($expr)->isInteger()->yes()`.
fn is_integer(facts: &Facts, expr: &Expression<'_>) -> bool {
    facts
        .native_type(node_span(unparenthesize(expr)))
        .is_some_and(|t| !t.types.is_empty() && t.is_int())
}

/// `NodeTypeResolver::isNumberType`: the native type is all integers or all floats.
fn is_number(facts: &Facts, expr: &Expression<'_>) -> bool {
    is_integer(facts, expr)
        || facts
            .native_type(node_span(unparenthesize(expr)))
            .is_some_and(|t| t.is_float())
}

fn text_of<'s>(source: &'s str, expr: &Expression<'_>) -> &'s str {
    let span = node_span(expr);
    &source[span.start..span.end]
}

/// The kept operand `expr` where the operation stood under `parent`: its code, in parentheses where the meaning needs
/// them.
fn operand_text(source: &str, expr: &Expression<'_>, parent: Option<Node<'_, '_>>) -> String {
    let inner = unparenthesize(expr);
    if keeps_parentheses(inner, parent) {
        format!("({})", text_of(source, inner))
    } else {
        text_of(source, inner).to_string()
    }
}

/// `new UnaryMinus($right)` printed: `-` and the operand, in parentheses when it is an operation that binds looser
/// than `-` (Pun) or its code starts with `-` (`pPrefixOp`).
fn negated(source: &str, operand: &Expression<'_>) -> String {
    let text = text_of(source, operand);
    if operand_precedence(operand) < 20 || text.starts_with('-') {
        format!("-({text})")
    } else {
        format!("-{text}")
    }
}

/// `isMulParenthesized`: the right operand is an operation, the multiplication's last token is `)`, and the token
/// before the operation is `(`.
fn is_parenthesized_operation(ctx: &crate::Context<'_>, mul: Span, right: &Expression<'_>) -> bool {
    let source = ctx.source;
    let inner = unparenthesize(right);
    if !matches!(inner, Expression::Binary(_)) || !source[..mul.end].ends_with(')') {
        return false;
    }
    let start = node_span(inner).start;
    tokens(ctx)
        .iter()
        .rev()
        .find(|t| t.end <= start && !source[t.start..t.end].trim().is_empty())
        .is_some_and(|t| &source[t.start..t.end] == "(")
}
