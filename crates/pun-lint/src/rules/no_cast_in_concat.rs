//! `lint/dead-code/no-cast-in-concat`, ported from Rector 2.8.4's `RemoveConcatAutocastRector`
//! (rules/DeadCode/Rector/Concat).
//!
//! `'a' . (string) $b` becomes `'a' . $b`: `.` casts to string already. php-parser reads `(binary)` as a string cast
//! too. Rector keeps the parentheses of a cast operand that is a binary operation (`WRAPPED_IN_PARENTHESES`) and
//! prints any other operand bare; Pun also keeps them around other operands that bind looser than `.`.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::{Edit, Fix, Safety};

use super::php_parser::keeps_parentheses;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoCastInConcat;

impl Rule for NoCastInConcat {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-cast-in-concat"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        if !matches!(binary.operator, BinaryOperator::StringConcat(_)) {
            return;
        }
        let edits: Vec<Edit> = [binary.lhs, binary.rhs]
            .into_iter()
            .filter_map(|side| remove_string_cast(ctx.source, side))
            .collect();
        if edits.is_empty() {
            return;
        }
        out.push(Report {
            span: node_span(binary),
            message: "`.` casts to string already; remove the `(string)` cast.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `removeStringCast`: the edit that replaces a string cast with its operand.
fn remove_string_cast(source: &str, side: &Expression<'_>) -> Option<Edit> {
    let Expression::UnaryPrefix(cast) = unparenthesize(side) else {
        return None;
    };
    if !matches!(
        cast.operator,
        UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..)
    ) {
        return None;
    }
    let operand = unparenthesize(cast.operand);
    let wrapped = matches!(cast.operand, Expression::Parenthesized(_))
        && match operand {
            // php-parser's `instanceof` is no `BinaryOp`, and binds tighter than `.`.
            Expression::Binary(b) => !matches!(b.operator, BinaryOperator::Instanceof(_)),
            _ => keeps_parentheses(operand, None),
        };
    let text = node_span(if wrapped { cast.operand } else { operand });
    Some(Edit::new(node_span(cast), &source[text.start..text.end]))
}
