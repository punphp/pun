//! `lint/dead-code/no-unused-foreach-key`, ported from Rector 2.6.1's `RemoveUnusedForeachKeyRector`
//! (rules/DeadCode/Rector/Foreach_), with `ExprUsedInNodeAnalyzer` and `StmtsManipulator::isVariableUsedInNextStmt`.
//!
//! `foreach ($items as $key => $value)` becomes `foreach ($items as $value)` when neither the loop body nor the
//! statements after the loop use `$key`. A `@var` tag for `$key` in the loop's doc comment goes too, and the doc
//! comment with it when nothing else is left in it.

use mago_syntax::cst::{Expression, ForeachBody, ForeachTarget, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{Token, enclosing, previous_token_end, tokens};
use super::used_variable::{try_of, used_in, used_in_next};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedForeachKey;

impl Rule for NoUnusedForeachKey {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-foreach-key"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Foreach(foreach) = node else { return };
        let ForeachTarget::KeyValue(target) = &foreach.target else {
            return;
        };
        let Expression::Variable(key) = unparenthesize(target.key) else {
            return;
        };
        // getName: a variable variable has no name.
        let name: &[u8] = match key {
            mago_syntax::cst::Variable::Direct(v) => &v.name[1..],
            _ => b"",
        };
        let used_in_body = match &foreach.body {
            ForeachBody::Statement(s) => used_in(Node::Statement(s), None, name),
            ForeachBody::ColonDelimited(b) => b
                .statements
                .iter()
                .any(|s| used_in(Node::Statement(s), None, name)),
        };
        if used_in_body {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(ctx);
        let (list, index) = enclosing(source, &tokens, ancestors);
        if used_in_next(&list, index + 1, try_of(ancestors), name) {
            return;
        }
        let key_span = node_span(target.key);
        let mut edits = vec![Edit::new(
            Span::new(key_span.start, node_span(target.value).start),
            "",
        )];
        let statement = node_span(list.stmts[index]);
        if let Some(edit) = var_tag_edit(source, &tokens, statement.start, name) {
            edits.push(edit);
        }
        out.push(Report {
            span: key_span,
            message: format!(
                "The key `{}` is never used; remove it.",
                &source[key_span.start..key_span.end]
            ),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// The edit that removes the `@var ... $name` tags from the doc comment of the statement at `start` (the last doc
/// comment before it, as php-parser's `getDocComment`), or the whole doc comment when nothing else is in it.
///
/// Rector's doc printer, as its output shows: a tag takes the lines after it up to the next tag (phpdoc-parser reads
/// them as its description), and goes from the end of the line before it (or from `/**` on the first line) to the
/// end of its last text, with the empty lines between it and a next tag.
fn var_tag_edit(source: &str, tokens: &[Token], start: usize, name: &[u8]) -> Option<Edit> {
    let from = previous_token_end(source, tokens, start);
    let doc = tokens.iter().rfind(|t| {
        t.start >= from
            && t.end <= start
            && t.kind == mago_syntax::token::TokenKind::DocBlockComment
    })?;
    let text = &source[doc.start..doc.end];
    // Each line: its start, its end before the newline, and its text without `/**`, `*`, `*/` and blanks.
    let mut lines = Vec::new();
    let mut offset = doc.start;
    for line in text.split_inclusive('\n') {
        let content = line
            .trim()
            .trim_start_matches("/**")
            .trim_end_matches("*/")
            .trim()
            .trim_start_matches('*')
            .trim();
        let at = offset + (content.as_ptr() as usize - line.as_ptr() as usize);
        lines.push((
            offset,
            offset + line.trim_end_matches('\n').len(),
            at,
            content,
        ));
        offset += line.len();
    }
    let n = lines.len();
    let mut removed = Vec::new();
    let mut kept = false;
    let mut i = 0;
    while i < n {
        if !is_var_tag_for(lines[i].3, name) {
            kept |= !lines[i].3.is_empty();
            i += 1;
            continue;
        }
        let mut last = i;
        let mut j = i + 1;
        while j < n && !lines[j].3.starts_with('@') {
            if !lines[j].3.is_empty() {
                last = j;
            }
            j += 1;
        }
        let from = if i == 0 {
            doc.start + 3
        } else {
            lines[i].0 - 1
        };
        let to = if j < n {
            lines[j - 1].1
        } else {
            lines[last].2 + lines[last].3.len()
        };
        removed.push(Span::new(from, to));
        i = j;
    }
    if removed.is_empty() {
        return None;
    }
    if !kept {
        // The doc comment goes with the blanks after it, up to the statement.
        return Some(Edit::new(Span::new(doc.start, start), ""));
    }
    let mut rest = String::new();
    let mut cursor = doc.start;
    for span in &removed {
        rest.push_str(&source[cursor..span.start]);
        cursor = span.end;
    }
    rest.push_str(&source[cursor..doc.end]);
    Some(Edit::new(Span::new(doc.start, doc.end), rest))
}

/// Whether a doc comment line is `@var <type> $name` (or `@var $name <type>`) for this variable.
fn is_var_tag_for(content: &str, name: &[u8]) -> bool {
    let Some(rest) = content.strip_prefix("@var") else {
        return false;
    };
    if !rest.starts_with([' ', '\t']) {
        return false;
    }
    rest.split_whitespace()
        .find(|w| w.starts_with('$'))
        .is_some_and(|w| &w.as_bytes()[1..] == name)
}
