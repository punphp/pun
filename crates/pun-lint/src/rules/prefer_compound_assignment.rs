//! `lint/style/prefer-compound-assignment`, ported from Rector 2.6.1's `CombinedAssignRector`
//! (rules/CodeQuality/Rector/Assign) and `AssignAndBinaryMap`.
//!
//! `$value = $value + 5;` becomes `$value += 5;` when the assigned expression and the binary operation's left side are
//! the same code. The fix is unsafe: for an expression with side effects (`$a[f()] = $a[f()] + 1`) the compound form
//! evaluates it once instead of twice.

use mago_span::HasSpan;
use mago_syntax::cst::{AssignmentOperator, BinaryOperator, Expression, Node};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::{node_span, same_code, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferCompoundAssignment;

impl Rule for PreferCompoundAssignment {
    fn id(&self) -> &'static str {
        "lint/style/prefer-compound-assignment"
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Assignment(assign) = node else {
            return;
        };
        if !matches!(assign.operator, AssignmentOperator::Assign(_)) {
            return;
        }
        let Expression::Binary(binary) = unparenthesize(assign.rhs) else {
            return;
        };
        let Some(op) = compound(&binary.operator) else {
            return;
        };
        let left = unparenthesize(binary.lhs);
        if matches!(left, Expression::ArrayAccess(_))
            && matches!(binary.operator, BinaryOperator::BitwiseXor(_))
        {
            return;
        }
        let var = node_span(assign.lhs);
        if !same_code(ctx.source, var, node_span(left)) {
            return;
        }
        // php-parser keeps no parentheses, so Rector prints the right operand bare. It does so even for `and`, `or`
        // and `xor`, which bind looser than `+=` and change what the statement means; there Pun keeps the parentheses.
        let right = match unparenthesize(binary.rhs) {
            Expression::Binary(b)
                if matches!(
                    b.operator,
                    BinaryOperator::LowAnd(_)
                        | BinaryOperator::LowOr(_)
                        | BinaryOperator::LowXor(_)
                ) =>
            {
                node_span(binary.rhs)
            }
            inner => node_span(inner),
        };
        let end = assign.span().end.offset as usize;
        let mut edits = vec![Edit::new(
            Span::new(var.end, right.start),
            format!(" {op}= "),
        )];
        if right.end < end {
            edits.push(Edit::new(Span::new(right.end, end), ""));
        }
        out.push(Report {
            span: node_span(assign),
            message: format!("Use `{op}=` instead of repeating the assigned variable."),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// `AssignAndBinaryMap::getAlternative`: the compound assignment for a binary operator, if PHP has one. `??` has
/// `??=`, but php-parser's map does not list it.
fn compound(op: &BinaryOperator<'_>) -> Option<&'static str> {
    Some(match op {
        BinaryOperator::Addition(_) => "+",
        BinaryOperator::Subtraction(_) => "-",
        BinaryOperator::Multiplication(_) => "*",
        BinaryOperator::Division(_) => "/",
        BinaryOperator::Modulo(_) => "%",
        BinaryOperator::Exponentiation(_) => "**",
        BinaryOperator::BitwiseAnd(_) => "&",
        BinaryOperator::BitwiseOr(_) => "|",
        BinaryOperator::BitwiseXor(_) => "^",
        BinaryOperator::LeftShift(_) => "<<",
        BinaryOperator::RightShift(_) => ">>",
        BinaryOperator::StringConcat(_) => ".",
        _ => return None,
    })
}
