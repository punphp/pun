//! `lint/functions/require-strict-array-search`, ported from Rector 2.6.1's `StrictArraySearchRector`
//! (rules/CodingStyle/Rector/FuncCall).
//!
//! `array_search($needle, $haystack)` gets `true` as its third argument. Rector adds it after named arguments too,
//! which PHP rejects (a positional argument after a named one); Pun leaves such a call alone.

use mago_syntax::cst::{Argument, Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::lint_b_helpers::append_argument;
use super::rector_code_quality::{args, named};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireStrictArraySearch;

impl Rule for RequireStrictArraySearch {
    fn id(&self) -> &'static str {
        "lint/functions/require-strict-array-search"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        let call = c;
        let args = args(call);
        if !named(call, "array_search")
            || args.len() != 2
            || args.iter().any(|a| matches!(a, Argument::Named(_)))
        {
            return;
        }
        out.push(Report {
            span: node_span(call),
            message: "Pass `true` to `array_search()` for a strict comparison.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![append_argument(ctx.source, &call.argument_list, "true")],
            }),
        });
    }
}
