//! `lint/phpunit/prefer-test-prefix`, ported from rector-phpunit's `ReplaceTestAnnotationWithPrefixedFunctionRector`
//! (rules/CodeQuality/Rector/ClassMethod).
//!
//! A method of a test class not named `test*` whose doc comment has a line that is exactly `*@test` or `* @test`
//! (trimmed) loses its `@test` tags (`PhpDocTagRemover::removeByName`, the doc going when nothing is left) and is
//! renamed `test` followed by its name with the first letter upper-cased (`ucfirst`).

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{class_body, is_in_test_class, report};
use crate::rules::rector_batch5::{doc_comment, list_removal, member_from, merge};
use crate::rules::rector_stmts::line_indent;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferTestPrefix;

impl Rule for PreferTestPrefix {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-test-prefix"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        let name = method.name.value;
        if name.starts_with(b"test") || !is_in_test_class(ctx, ancestors, Some(method)) {
            return;
        }
        let source = ctx.source;
        let span = node_span(method);
        // php-parser reprints the doc and the signature at its own indentation, in spaces
        if line_indent(source, span.start).contains('\t') {
            return;
        }
        let (members, left, _) = ancestors.iter().rev().find_map(|a| class_body(*a)).unwrap();
        let at = members
            .iter()
            .position(|m| node_span(m).start == span.start)
            .unwrap();
        let from = member_from(members, left, at);
        let Some(doc) = doc_comment(source, from, span.start) else {
            return;
        };
        let text = &source[doc.start..doc.end];
        if !text
            .lines()
            .any(|row| matches!(row.trim(), "*@test" | "* @test"))
        {
            return;
        }
        let new = without_test_tags(text);
        let mut edits = vec![if new.is_empty() {
            Edit::new(Span::new(doc.start, span.start), "")
        } else {
            Edit::new(doc, new)
        }];
        let mut renamed = String::from("test");
        let name = String::from_utf8_lossy(name);
        let mut chars = name.chars();
        if let Some(first) = chars.next() {
            renamed.push(first.to_ascii_uppercase());
            renamed.push_str(chars.as_str());
        }
        edits.push(Edit::new(node_span(&method.name), renamed));
        out.push(report(
            node_span(&method.name),
            "Prefix the test's name with `test` instead of a `@test` tag.",
            Safety::Unsafe,
            merge(edits),
        ));
    }
}

/// The doc comment, which has a `* @test` line, without its `@test` tags, as Rector's phpdoc printer removes tag
/// nodes (with [`list_removal`]'s rules); `""` when nothing is left. PHPStan's phpdoc parser
/// keeps a blank line after a text as an empty text node, and a blank line after a tag with the tag.
fn without_test_tags(doc: &str) -> String {
    let inner_end = doc.len() - 2;
    // (span, is a tag)
    let mut nodes: Vec<(Span, bool)> = Vec::new();
    let mut open = false;
    let mut line_start = 0;
    for line in doc[..inner_end].split_inclusive('\n') {
        let start = line_start;
        line_start += line.len();
        let body = line.trim_end_matches(['\n', '\r']);
        let mut content = body.trim_start();
        if start == 0 {
            content = content.strip_prefix("/**").unwrap_or(content);
        } else if let Some(rest) = content.strip_prefix('*') {
            content = rest;
        }
        let trimmed = content.trim();
        if trimmed.is_empty() {
            if open && nodes.last().is_some_and(|(_, tag)| !tag) {
                let at = start + body.trim_end().len();
                nodes.push((Span::new(at, at), false));
            }
            open = false;
            continue;
        }
        let from = start + body.len() - content.trim_start().len();
        let to = from + trimmed.len();
        match nodes.last_mut() {
            Some((last, _)) if open && !trimmed.starts_with('@') => last.end = to,
            _ => nodes.push((Span::new(from, to), trimmed.starts_with('@'))),
        }
        open = true;
    }
    let removed: Vec<bool> = nodes
        .iter()
        .map(|(span, tag)| {
            *tag && doc[span.start..span.end]
                .split(char::is_whitespace)
                .next()
                .is_some_and(|name| name.trim_matches('@') == "test")
        })
        .collect();
    let spans: Vec<Span> = nodes.iter().map(|(span, _)| *span).collect();
    // an empty text node left alone is no content: the doc goes
    let kept_content = nodes
        .iter()
        .zip(&removed)
        .any(|((span, _), r)| !r && span.start != span.end);
    let Some(edits) = list_removal(&spans, &removed).filter(|_| kept_content) else {
        return String::new();
    };
    let mut out = doc.to_string();
    for edit in edits.iter().rev() {
        out.replace_range(edit.span.start..edit.span.end, &edit.text);
    }
    out
}
