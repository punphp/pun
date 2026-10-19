//! `lint/security/require-preg-quote-delimiter`, ported from Mago 1.57.0's linter rule
//! `require-preg-quote-delimiter` (rule/security/require_preg_quote_delimiter.rs).
//!
//! A `preg_quote()` call with fewer than two arguments does not escape the pattern's delimiter; the call is reported.

use mago_syntax::cst::{Node, NodeKind};

use super::lint_b_names::function_call_matches;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequirePregQuoteDelimiter;

impl Rule for RequirePregQuoteDelimiter {
    fn id(&self) -> &'static str {
        "lint/security/require-preg-quote-delimiter"
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
        let Node::FunctionCall(c) = node else { return };
        if c.argument_list.arguments.len() >= 2
            || !function_call_matches(ancestors, c, "preg_quote")
        {
            return;
        }
        out.push(Report {
            span: node_span(c),
            message: "Missing delimiter argument in `preg_quote()` call".to_string(),
            help: Some("Add delimiter: `preg_quote($str, '/')`".to_string()),
            fix: None,
        });
    }
}
