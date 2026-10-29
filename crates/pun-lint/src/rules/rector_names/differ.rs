//! php-parser's format-preserving printer on a list whose items were added or removed (`PrettyPrinterAbstract::pArray`
//! with `Differ::diffWithReplacements`), for the two lists the name handling changes: a namespace's or the file's
//! statements (`insertionMap` `"\n"`) and a `use` statement's items (`", "`), and the fallback where `pArray` gives up
//! on a statement list.
//!
//! The printer writes a list as [`Piece`]s: the file's own bytes for what it keeps (the edits other parts of Rector
//! made inside them still apply) and new text for the rest.

use mago_syntax::token::TokenKind;
use pun_problem::Span;

use crate::rules::phpunit::annotations::reformatted;
use crate::rules::phpunit::{Elem, coalesce_replacements, diff};
use crate::rules::rector_conditions::indentation;
use crate::rules::rector_stmts::tokens;

/// A part of a printed list: bytes of the file, or text the printer wrote.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Piece {
    Source(Span),
    Text(String),
}

/// An item of the old list: where its comments start (its start when it has none), where it starts and ends.
#[derive(Clone, Copy, Debug)]
pub(crate) struct OldItem {
    pub comment_start: usize,
    pub start: usize,
    pub end: usize,
}

/// The comments of an item of the new list (`getComments()`).
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Comments {
    /// The comments of the old item at this index, the same objects (printed from the file where the item is kept).
    Of(usize),
    /// Comments printed anew (`pComments()` of these texts, without the line break after them).
    Printed(Vec<String>),
    None,
}

/// An item of the new list.
#[derive(Clone, Debug)]
pub(crate) struct NewItem {
    /// The old item this is (`$a === $b->getAttribute('origNode')`), `None` for a node the printer has not seen.
    pub key: Option<usize>,
    /// What `p()` prints for a node that is not kept.
    pub text: String,
    pub comments: Comments,
}

impl NewItem {
    /// The old item `i`, kept with its comments.
    pub fn kept(i: usize) -> NewItem {
        NewItem {
            key: Some(i),
            text: String::new(),
            comments: Comments::Of(i),
        }
    }
}

/// The old item's comments as `pComments()` prints them when the item moves to the line start at `indent` spaces.
fn print_comments(source: &str, old: &OldItem, indent: usize) -> String {
    let nl = format!("\n{}", " ".repeat(indent));
    comment_spans(source, old)
        .iter()
        .map(|c| reformatted(&source[c.start..c.end]).replace('\n', &nl))
        .collect::<Vec<_>>()
        .join(&nl)
}

/// The comment tokens between an old item's comment start and its start.
fn comment_spans(source: &str, old: &OldItem) -> Vec<Span> {
    tokens(source, old.comment_start, old.start)
        .into_iter()
        .filter(|t| t.kind.is_comment())
        .map(|t| Span::new(t.start, t.end))
        .collect()
}

/// `TokenStream::haveTagInRange()`: an opening or closing tag in `[from, to)`. The file is read from its start, since
/// `from` may follow inline HTML, where the lexer is not inside PHP code.
fn have_tag_in_range(source: &str, from: usize, to: usize) -> bool {
    crate::rules::php_parser::tokens(source).iter().any(|t| {
        (from..to).contains(&t.start)
            && matches!(
                t.kind,
                TokenKind::OpenTag
                    | TokenKind::EchoTag
                    | TokenKind::ShortOpenTag
                    | TokenKind::CloseTag
            )
    })
}

/// `pArray()` on a statement list from `pos` (where the list's subnode starts): the pieces up to the end of the last
/// old item it printed and that end, or `None` where it gives up (the caller falls back).
pub(crate) fn statements(
    source: &str,
    old: &[OldItem],
    new: &[NewItem],
    pos: usize,
    braces: bool,
) -> Option<(Vec<Piece>, usize)> {
    // This was a single statement without braces, but either additional statements have been added, or the single
    // statement has been removed. This requires the addition of braces. For now fall back.
    if old.len() == 1 && new.len() != 1 && !braces {
        return None;
    }
    let keys = keys(old.len(), new);
    let diff = coalesce_replacements(diff(old.len(), &keys));
    let mut out = Vec::new();
    let mut pos = pos;
    let mut before_first_keep = true;
    let mut skip_removed = false;
    let mut delayed: Vec<usize> = Vec::new();
    let mut indent = 0;
    for (i, elem) in diff.iter().enumerate() {
        match *elem {
            Elem::Keep(o, n) | Elem::Replace(o, n) => {
                before_first_keep = false;
                let item = old[o];
                indent = indentation(source, item.start);
                let nl = format!("\n{}", " ".repeat(indent));
                let comment_start = if item.comment_start < pos {
                    item.start
                } else {
                    item.comment_start
                };
                if skip_removed {
                    if have_tag_in_range(source, pos, item.start) {
                        return None;
                    }
                } else {
                    out.push(Piece::Source(Span::new(pos, comment_start)));
                }
                for d in delayed.drain(..) {
                    if let Some(c) = comments_text(source, old, &new[d].comments, indent) {
                        out.push(Piece::Text(format!("{c}{nl}")));
                    }
                    out.push(Piece::Text(format!("{}{nl}", new[d].text)));
                }
                if new[n].comments == Comments::Of(o) {
                    out.push(Piece::Source(Span::new(comment_start, item.start)));
                } else if let Some(c) = comments_text(source, old, &new[n].comments, indent) {
                    out.push(Piece::Text(format!("{c}{nl}")));
                }
                skip_removed = false;
                if matches!(elem, Elem::Keep(..)) {
                    out.push(Piece::Source(Span::new(item.start, item.end)));
                } else {
                    out.push(Piece::Text(new[n].text.clone()));
                }
                pos = item.end;
            }
            Elem::Add(n) => {
                if before_first_keep {
                    delayed.push(n);
                    continue;
                }
                let nl = format!("\n{}", " ".repeat(indent));
                out.push(Piece::Text(nl.clone()));
                if let Some(c) = comments_text(source, old, &new[n].comments, indent) {
                    out.push(Piece::Text(format!("{c}{nl}")));
                }
                out.push(Piece::Text(new[n].text.clone()));
            }
            Elem::Remove(o) => {
                let item = old[o];
                if i == 0 {
                    // If we're removing from the start, keep the tokens before the node and drop those after it.
                    out.push(Piece::Source(Span::new(pos, item.comment_start)));
                    skip_removed = true;
                } else if have_tag_in_range(source, pos, item.comment_start) {
                    return None;
                }
                pos = item.end;
            }
        }
    }
    if skip_removed || !delayed.is_empty() {
        return None;
    }
    Some((out, pos))
}

/// The keys of the new items for the `Differ`: an old item's index, or one past all old ones for a new node.
fn keys(n: usize, new: &[NewItem]) -> Vec<usize> {
    new.iter()
        .enumerate()
        .map(|(j, item)| item.key.unwrap_or(n + j))
        .collect()
}

/// The text of `comments` (`pComments()`), `None` when there are none.
fn comments_text(
    source: &str,
    old: &[OldItem],
    comments: &Comments,
    indent: usize,
) -> Option<String> {
    match comments {
        Comments::Of(o) => {
            let printed = print_comments(source, &old[*o], indent);
            (!printed.is_empty()).then_some(printed)
        }
        Comments::Printed(texts) => {
            let nl = format!("\n{}", " ".repeat(indent));
            Some(
                texts
                    .iter()
                    .map(|t| reformatted(t).replace('\n', &nl))
                    .collect::<Vec<_>>()
                    .join(&nl),
            )
        }
        Comments::None => None,
    }
}

/// `BetterStandardPrinter::pArray()`'s `EXTRA_SPACE_BEFORE_NOP_REGEX` (`#^[ \t]+$#m` replaced by nothing) on a
/// printed list that holds a `Nop`, `scripting` when the text starts in PHP code rather than before `<?php`. Pun
/// leaves the lines inside a string literal alone: Rector's change of them changes the string's value.
pub(crate) fn strip_blank_lines(text: &str, scripting: bool) -> String {
    let strings = string_spans(text, scripting);
    let in_string = |at: usize| strings.iter().any(|s| s.start < at && at < s.end);
    let mut out = String::new();
    let mut line = 0;
    for part in text.split_inclusive('\n') {
        let body = part.trim_end_matches('\n');
        let blank = !body.is_empty() && body.bytes().all(|c| matches!(c, b' ' | b'\t'));
        if blank && !in_string(line) {
            out.push_str(&part[body.len()..]);
        } else {
            out.push_str(part);
        }
        line += part.len();
    }
    out
}

/// `TokenStream::getTokenCode()` with the indentation adjustment `indent` on `text`, PHP code that starts a line:
/// a line break loses `-indent` spaces after it, or gains `indent`, the string literals aside.
pub(crate) fn token_code(text: &str, indent: isize) -> String {
    let strings = string_spans(text, true);
    let spaces = " ".repeat(indent.unsigned_abs());
    let (from, to) = match indent < 0 {
        true => (format!("\n{spaces}"), "\n".to_string()),
        false => ("\n".to_string(), format!("\n{spaces}")),
    };
    let mut out = String::new();
    let mut at = 0;
    for span in strings
        .iter()
        .chain(std::iter::once(&Span::new(text.len(), text.len())))
    {
        out.push_str(&text[at..span.start].replace(&from, &to));
        out.push_str(&text[span.start..span.end]);
        at = span.end;
    }
    out
}

/// The spans of the string literals (quoted strings, heredocs, nowdocs) of `text`.
fn string_spans(text: &str, scripting: bool) -> Vec<Span> {
    let shift = usize::from(scripting);
    let padded = format!("{}{text}", if scripting { " " } else { "" });
    let mut spans = Vec::new();
    let mut heredoc: Option<usize> = None;
    for t in tokens(&padded, shift, padded.len()) {
        let (start, end) = (t.start - shift, t.end - shift);
        match t.kind {
            TokenKind::DocumentStart(_) => heredoc = Some(start),
            TokenKind::DocumentEnd => {
                if let Some(from) = heredoc.take() {
                    spans.push(Span::new(from, end));
                }
            }
            TokenKind::LiteralString | TokenKind::StringPart if heredoc.is_none() => {
                spans.push(Span::new(start, end))
            }
            _ => {}
        }
    }
    spans
}

/// `pArray()` on a list of `use` items from which some are removed: the pieces from `pos` to the end of the last
/// item it printed, or `None` where it gives up (every item removed from the start).
pub(crate) fn removed_items(
    old: &[Span],
    kept: &[bool],
    pos: usize,
) -> Option<(Vec<Piece>, usize)> {
    let mut out = Vec::new();
    let mut pos = pos;
    let mut skip_removed = false;
    for (i, item) in old.iter().enumerate() {
        if kept[i] {
            if !skip_removed {
                out.push(Piece::Source(Span::new(pos, item.start)));
            }
            skip_removed = false;
            out.push(Piece::Source(*item));
        } else if i == 0 {
            // removing from the start: keep the tokens before the item and drop those after it
            out.push(Piece::Source(Span::new(pos, item.start)));
            skip_removed = true;
        }
        pos = item.end;
    }
    (!skip_removed).then_some((out, pos))
}
