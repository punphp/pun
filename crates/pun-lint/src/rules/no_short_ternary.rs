//! `lint/style/no-short-ternary`, ported from phpstan-strict-rules 2.0.12's `DisallowedShortTernaryRule`
//! (identifier `ternary.shortNotAllowed`), the first upstream source of the rule in mapping.toml; Mago's
//! `no-shorthand-ternary` reports the same nodes with another message. Every `a ?: b` is reported.

use mago_syntax::cst::{Node, NodeKind};

use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoShortTernary;

impl Rule for NoShortTernary {
    fn id(&self) -> &'static str {
        "lint/style/no-short-ternary"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Conditional])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Conditional(c) = node else { return };
        if c.then.is_some() {
            return;
        }
        out.push(Report {
            span: node_span(c),
            message: "Short ternary operator is not allowed. Use null coalesce operator if applicable or consider using long ternary.".to_string(),
            help: None,
            fix: None,
        });
    }
}
