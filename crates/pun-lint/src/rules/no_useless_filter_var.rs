//! `lint/dead-code/no-useless-filter-var`, ported from Rector 2.8.4's `RemoveFilterVarOnExactTypeRector`
//! (rules/DeadCode/Rector/FuncCall).
//!
//! `filter_var($value, FILTER_VALIDATE_INT)` with exactly two arguments becomes `$value` when the value's native type
//! is an integer; `FILTER_VALIDATE_FLOAT` does so for a float, and `FILTER_VALIDATE_BOOLEAN` or `FILTER_VALIDATE_BOOL`
//! for a boolean. The value keeps its code; php-parser's printer puts parentheses around it where the place it moves
//! to needs them.

use mago_syntax::cst::{Argument, Call, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::php_parser::{keeps_parentheses, parent_of};
use super::rector_batch5::called_function_name;
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessFilterVar;

impl Rule for NoUselessFilterVar {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-filter-var"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Call])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Call(call @ Call::Function(function)) = node else {
            return;
        };
        if !called_function_name(ancestors, function.function)
            .is_some_and(|n| n.eq_ignore_ascii_case("filter_var"))
        {
            return;
        }
        let args: Vec<&Expression<'_>> = function
            .argument_list
            .arguments
            .iter()
            .map(|a| match a {
                Argument::Positional(p) => p.value,
                Argument::Named(n) => n.value,
            })
            .collect();
        let [value, Expression::ConstantAccess(filter)] = args[..] else {
            return;
        };
        let filter = String::from_utf8_lossy(filter.name.value());
        let Some(t) = facts(ctx)
            .native_type(node_span(unparenthesize(value)))
            .cloned()
        else {
            return;
        };
        let exact = match filter.trim_start_matches('\\') {
            "FILTER_VALIDATE_INT" => t.is_int() && !t.types.is_empty(),
            "FILTER_VALIDATE_FLOAT" => t.is_float(),
            "FILTER_VALIDATE_BOOLEAN" | "FILTER_VALIDATE_BOOL" => t.is_bool(),
            _ => false,
        };
        if !exact {
            return;
        }
        let inner = unparenthesize(value);
        let span = node_span(inner);
        let text = &ctx.source[span.start..span.end];
        let text = if keeps_parentheses(inner, parent_of(ancestors)) {
            format!("({text})")
        } else {
            text.to_string()
        };
        let span = node_span(call);
        out.push(Report {
            span,
            message: "This `filter_var()` gets a value of the type it validates; use the value."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, text)],
            }),
        });
    }
}
