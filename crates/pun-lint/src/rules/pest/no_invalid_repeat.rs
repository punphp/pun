//! `lint/pest/no-invalid-repeat`, ported from pest-plugin-rector's `FixInvalidRepeatValueRector`: `->repeat(0)` or
//! `->repeat(-2)` on a Pest test (`it()`, `test()` or `todo()`) becomes `->repeat(1)`.

use mago_syntax::cst::{Expression, Literal, Node, NodeKind, UnaryPrefixOperator};
use pun_problem::{Edit, Safety};

use super::{argument_value, function_name, has_comments, identifier, report, root_func_call};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoInvalidRepeat;

impl Rule for NoInvalidRepeat {
    fn id(&self) -> &'static str {
        "lint/pest/no-invalid-repeat"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if identifier(&call.method) != Some(b"repeat")
            || !root_func_call(call)
                .and_then(function_name)
                .is_some_and(|n| matches!(n, b"it" | b"test" | b"todo"))
        {
            return;
        }
        let Some(first) = call.argument_list.arguments.first() else {
            return;
        };
        let value = argument_value(first);
        let edit = match unparenthesize(value) {
            Expression::Literal(Literal::Integer(int)) => {
                if int.value.is_none_or(|v| v >= 1) {
                    return;
                }
                // php-parser's `pScalar_Int` writes the new value in the literal's kind.
                let raw = int.raw.to_ascii_lowercase();
                let one = if raw.starts_with(b"0x") {
                    "0x1"
                } else if raw.starts_with(b"0b") {
                    "0b1"
                } else if raw.len() > 1 && raw[0] == b'0' {
                    "01"
                } else {
                    "1"
                };
                Edit::new(node_span(int), one)
            }
            Expression::UnaryPrefix(minus)
                if matches!(minus.operator, UnaryPrefixOperator::Negation(_)) =>
            {
                let Expression::Literal(Literal::Integer(int)) = unparenthesize(minus.operand)
                else {
                    return;
                };
                // A comment before `-` belongs to the argument unless the value is in parentheses.
                let span = node_span(minus);
                if (node_span(value).start != span.start && has_comments(ctx, span.start))
                    || has_comments(ctx, node_span(int).start)
                {
                    return;
                }
                Edit::new(span, "1")
            }
            _ => return,
        };
        out.push(report(
            node_span(call),
            "Repeat a test at least once.".to_string(),
            Safety::Safe,
            vec![edit],
        ));
    }
}
