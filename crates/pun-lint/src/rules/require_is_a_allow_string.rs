//! `lint/complexity/require-is-a-allow-string`, ported from Rector 2.8.4's `IsAWithStringWithThirdArgumentRector`
//! (rules/CodeQuality/Rector/FuncCall).
//!
//! `is_a($class, ...)` on a class-name string is always false unless the third argument, `$allow_string`, is `true`.
//! Rector asks PHPStan whether the first argument is a string; Pun asks the syntax (see `no_sole_value_sprintf`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::no_sole_value_sprintf::is_string;
use super::rector_batch5::called_function_name;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireIsAAllowString;

impl Rule for RequireIsAAllowString {
    fn id(&self) -> &'static str {
        "lint/complexity/require-is-a-allow-string"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|n| n.eq_ignore_ascii_case("is_a"))
        {
            return;
        }
        let args: Vec<&Argument> = call.argument_list.arguments.iter().collect();
        let [Argument::Positional(first), Argument::Positional(second)] = args.as_slice() else {
            return;
        };
        if first.ellipsis.is_some()
            || second.ellipsis.is_some()
            || !is_string(ancestors, first.value)
        {
            return;
        }
        let at = node_span(second.value).end;
        out.push(Report {
            span: node_span(call),
            message: "`is_a()` on a class-name string needs `true` as its third argument."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(Span::new(at, at), ", true")],
            }),
        });
    }
}
