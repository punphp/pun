//! `lint/tags/no-invalid-open-tag`, ported from Mago 1.57.0's `invalid-open-tag`
//! (mago-linter src/rule/correctness/invalid_open_tag.rs).
//!
//! Inline HTML that starts (after whitespace) with a misspelled `<?php` (`<php?`, `<ph?p`, `<p?hp`, `<ph?`, `<p?`,
//! in any case) is reported, and the fix writes `<?php` in its place.
//!
//! Where whitespace comes before the tag, Mago 1.57.0 reports a span that ends at the tag's length counted from the
//! inline text's start (`Span::subspan` takes an end, not a length) and its fix then fails as a Mago bug; Pun reports
//! and replaces the tag itself (tests/mago/tags-no-invalid-open-tag/Pun).

use mago_span::HasSpan;
use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::{Context, Report, Rule};

const INVALID_TAGS: &[&str] = &["<php?", "<ph?p", "<p?hp", "<ph?", "<p?"];

pub struct NoInvalidOpenTag;

impl Rule for NoInvalidOpenTag {
    fn id(&self) -> &'static str {
        "lint/tags/no-invalid-open-tag"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Inline])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Inline(inline) = node else { return };
        let content = inline.value;
        let start = content
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(content.len());
        let trimmed = &content[start..];
        let Some(tag) = INVALID_TAGS.iter().find(|tag| {
            trimmed.len() >= tag.len() && trimmed[..tag.len()].eq_ignore_ascii_case(tag.as_bytes())
        }) else {
            return;
        };
        let from = inline.span().start.offset as usize + start;
        let span = Span::new(from, from + tag.len());
        out.push(Report {
            span,
            message: format!(
                "Misspelled PHP opening tag `{}`.",
                String::from_utf8_lossy(&trimmed[..tag.len()])
            ),
            help: Some("Replace this with the correct `<?php` opening tag.".to_string()),
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, "<?php")],
            }),
        });
    }
}
