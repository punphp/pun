//! `lint/strings/prefer-empty-string-identical`, ported from Rector 2.6.1's `StrlenZeroToIdenticalEmptyStringRector`
//! (rules/CodeQuality/Rector/Identical).
//!
//! `strlen($s) === 0` is `$s === ''`, and `strlen($s) > 0` (or `0 < strlen($s)`) is `$s !== ''`. A value that is not
//! natively a string is cast with `(string)`; Pun reads "is a string" from the syntax (`no_sole_value_sprintf`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::{unpacked, value};
use super::no_sole_value_sprintf::is_string;
use super::rector_batch5::called_function_name;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEmptyStringIdentical;

impl Rule for PreferEmptyStringIdentical {
    fn id(&self) -> &'static str {
        "lint/strings/prefer-empty-string-identical"
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
        let Node::Binary(binary) = node else {
            return;
        };
        let identical = match binary.operator {
            BinaryOperator::Identical(_) => true,
            BinaryOperator::GreaterThan(_) | BinaryOperator::LessThan(_) => false,
            _ => return,
        };
        let (call, other, call_on_left) = match (binary.lhs, binary.rhs) {
            (Expression::Call(Call::Function(call)), other) => (call, other, true),
            (other, Expression::Call(Call::Function(call))) => (call, other, false),
            _ => return,
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|n| n.eq_ignore_ascii_case("strlen"))
        {
            return;
        }
        if !matches!(unparenthesize(other), Expression::Literal(Literal::Integer(i)) if i.value == Some(0))
        {
            return;
        }
        // `0 > strlen($s)` and `strlen($s) < 0` never hold.
        match binary.operator {
            BinaryOperator::GreaterThan(_) if !call_on_left => return,
            BinaryOperator::LessThan(_) if call_on_left => return,
            _ => {}
        }
        // `getArgs()[0]`, named or not; Rector also takes an unpacked one, which changes the meaning.
        let Some(arg) = call.argument_list.arguments.first() else {
            return;
        };
        if unpacked(arg) {
            return;
        }
        let source = ctx.source;
        let value = unparenthesize(value(arg));
        let s = node_span(value);
        let text = &source[s.start..s.end];
        let subject = if is_string(ancestors, value) {
            text.to_string()
        } else if matches!(
            value,
            Expression::Binary(_)
                | Expression::Assignment(_)
                | Expression::Conditional(_)
                | Expression::Yield(_)
                | Expression::Throw(_)
                | Expression::Pipe(_)
                | Expression::ArrowFunction(_)
        ) {
            format!("(string) ({text})")
        } else {
            format!("(string) {text}")
        };
        let operator = if identical { "===" } else { "!==" };
        out.push(Report {
            span: node_span(binary),
            message: "Compare with `''` instead of checking `strlen()` against `0`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![Edit::new(
                    node_span(binary),
                    format!("{subject} {operator} ''"),
                )],
            }),
        });
    }
}
