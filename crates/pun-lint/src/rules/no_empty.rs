//! `lint/style/no-empty`, ported from phpstan-strict-rules 2.0.12's `DisallowedEmptyRule` (identifier
//! `empty.notAllowed`), the first upstream source of the rule in mapping.toml; Mago's `no-empty` reports the same
//! nodes with another message. Every `empty(...)` is reported.

use mago_syntax::cst::{Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEmpty;

impl Rule for NoEmpty {
    fn id(&self) -> &'static str {
        "lint/style/no-empty"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::EmptyConstruct])
    }

    fn visit<'ast, 'arena>(
        &self,
        n: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::EmptyConstruct(e) = n else { return };
        out.push(Report {
            span: node_span(e),
            message: "Construct empty() is not allowed. Use more strict comparison.".to_string(),
            help: None,
            fix: None,
        });
    }
}
