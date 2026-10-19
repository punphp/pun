//! What lint-b's phpdoc ports share: the doc comment php-parser gives a node, its child nodes as Rector's
//! `BetterPhpDocParser` reads them, and the doc Rector's `PhpDocInfoPrinter::printFormatPreserving()` prints once some
//! of them are removed (`PhpDocInfo::removeByType()`, `removeByName()`, a `NODE_REMOVE` traversal).

use mago_syntax::cst::Node;
use pun_problem::{Edit, Span};

use super::rector_batch5::doc_comment;
use super::rector_stmts::previous_token_end;
use crate::syntax::node_span;

/// phpdoc-parser's token types, as far as the printer tells them apart.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// `TOKEN_OPEN_PHPDOC`: `/**` and one space after it.
    Open,
    /// `TOKEN_CLOSE_PHPDOC`: `*/`.
    Close,
    /// `TOKEN_PHPDOC_EOL`: a line break, the next line's indent, its `*` and one space after it.
    Eol,
    /// `TOKEN_HORIZONTAL_WS`.
    Ws,
    /// Any other token; Pun takes a run of other characters as one.
    Other,
    /// `TOKEN_END`, empty.
    End,
}

/// A doc comment, its tokens and its child nodes.
pub struct Doc<'s> {
    pub span: Span,
    text: &'s str,
    tokens: Vec<(Kind, usize, usize)>,
    /// Each child's first token and the token after its last (`START_AND_END`).
    children: Vec<(usize, usize)>,
}

impl<'s> Doc<'s> {
    /// The doc comment of the node at `start`, whose ancestors are `ancestors`.
    pub fn of(source: &'s str, ancestors: &[Node<'_, '_>], start: usize) -> Option<Self> {
        // The comments come after the last token before the node, which lies inside the nearest ancestor that starts
        // before it.
        let outer = ancestors
            .iter()
            .rev()
            .map(|a| node_span(a).start)
            .find(|&s| s < start)
            .unwrap_or(0);
        let span = doc_comment(source, previous_token_end(source, outer, start), start)?;
        let text = &source[span.start..span.end];
        let tokens = lex(text);
        let children = children(text, &tokens);
        Some(Doc {
            span,
            text,
            tokens,
            children,
        })
    }

    /// The number of child nodes.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// The words of child `i`, its lines joined: `["@param", "mixed", "$a"]`. A blank line has none.
    pub fn words(&self, i: usize) -> Vec<&'s str> {
        let (from, to) = self.children[i];
        self.tokens[from..to]
            .iter()
            .filter(|t| t.0 == Kind::Other)
            .map(|t| &self.text[t.1..t.2])
            .collect()
    }

    /// The edit that removes the children `removed` flags, as Rector prints the doc after; `node_start` is where the
    /// node the doc belongs to starts. A doc with nothing left goes, with the white space after it.
    pub fn remove(&self, removed: &[bool], node_start: usize) -> Edit {
        let printed = self.print(removed);
        if printed.is_empty() {
            return Edit::new(Span::new(self.span.start, node_start), "");
        }
        Edit::new(self.span, printed)
    }

    /// `printFormatPreserving()` with no child changed but the removed ones.
    fn print(&self, removed: &[bool]) -> String {
        // `EmptyPhpDocDetector`: nothing but blank lines left.
        let kept: Vec<usize> = (0..self.children.len()).filter(|&i| !removed[i]).collect();
        if kept
            .iter()
            .all(|&i| self.children[i].0 == self.children[i].1)
        {
            return String::new();
        }
        // `RemoveNodesStartAndEndResolver`: a removed child goes with the token before it (its line break).
        let jumps: Vec<(usize, usize)> = (0..self.children.len())
            .filter(|&i| removed[i])
            .map(|i| (self.children[i].0.saturating_sub(1), self.children[i].1))
            .collect();
        let tokens = |from: usize, to: usize, skip_empty_lines_above: bool| -> String {
            let mut from = from;
            if from > 0 && self.tokens[from - 1].0 == Kind::Ws {
                from -= 1;
            }
            if skip_empty_lines_above
                && self.tokens[from].0 == Kind::Eol
                && self.tokens.get(from + 1).is_some_and(|t| t.0 == Kind::Eol)
            {
                from += 1;
            }
            let mut out = String::new();
            let mut i = from;
            while i < to {
                while let Some(&(_, end)) = jumps.iter().find(|j| j.0 == i) {
                    i = end;
                }
                if let Some(t) = self.tokens.get(i) {
                    out.push_str(&self.text[t.1..t.2]);
                }
                i += 1;
            }
            out
        };
        let mut output = String::new();
        let mut current = 0;
        for &i in &kept {
            let (start, end) = self.children[i];
            // `correctPreviouslyReprintedFirstNode()`: the first child printed is not the first child.
            if current == 0 && i != 0 {
                current = start - usize::from(self.tokens[start - 1].0 == Kind::Eol);
            }
            // The children keep their keys when one goes, so the child whose key is the new count counts as last.
            let printed = tokens(current, end, i + 1 == kept.len());
            output.push_str(printed.trim_end());
            current = end;
        }
        let last = self.children.last().map_or(current, |c| c.1);
        output.push_str(&tokens(last, self.tokens.len(), true));
        if !output.starts_with("/**") {
            output.insert_str(0, "/**");
        }
        if !output.trim_end().ends_with("*/") {
            output.push_str(" */");
        }
        output.replace(" \n", "\n")
    }
}

/// phpdoc-parser's `Lexer` over a doc comment.
fn lex(text: &str) -> Vec<(Kind, usize, usize)> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 3 + usize::from(bytes.get(3) == Some(&b' '));
    tokens.push((Kind::Open, 0, i));
    while i < bytes.len() {
        let start = i;
        let kind = match bytes[i] {
            b'\r' | b'\n' => {
                i += if bytes[i] == b'\r' { 2 } else { 1 };
                while matches!(bytes.get(i), Some(b' ' | b'\t')) {
                    i += 1;
                }
                if bytes.get(i) == Some(&b'*') && bytes.get(i + 1) != Some(&b'/') {
                    i += 1 + usize::from(bytes.get(i + 1) == Some(&b' '));
                }
                Kind::Eol
            }
            b' ' | b'\t' => {
                while matches!(bytes.get(i), Some(b' ' | b'\t')) {
                    i += 1;
                }
                Kind::Ws
            }
            b'*' if bytes.get(i + 1) == Some(&b'/') => {
                i += 2;
                Kind::Close
            }
            _ => {
                while i < bytes.len()
                    && !matches!(bytes[i], b' ' | b'\t' | b'\r' | b'\n')
                    && !(bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/'))
                {
                    i += 1;
                }
                Kind::Other
            }
        };
        tokens.push((kind, start, i));
    }
    tokens.push((Kind::End, bytes.len(), bytes.len()));
    tokens
}

/// `BetterPhpDocParser::parseWithNode()`'s children: a text runs to a blank line or a tag line, a tag's description
/// runs over blank lines and text up to the next tag line, and a blank line is an empty text.
fn children(text: &str, tokens: &[(Kind, usize, usize)]) -> Vec<(usize, usize)> {
    let is_tag = |i: usize| {
        let t = tokens[i];
        t.0 == Kind::Other && {
            let word = text[t.1..t.2].as_bytes();
            word[0] == b'@' && word.get(1).is_some_and(u8::is_ascii_alphabetic)
        }
    };
    let ends = |i: usize| matches!(tokens[i].0, Kind::Close | Kind::End);
    let mut children = Vec::new();
    let mut pos = 1 + usize::from(tokens[1].0 == Kind::Eol);
    if ends(pos) {
        return children;
    }
    loop {
        let start = pos;
        let tag = is_tag(pos);
        while !ends(pos) && tokens[pos].0 != Kind::Eol {
            pos += 1;
            if tokens[pos].0 == Kind::Eol {
                let mut next = pos;
                while tokens[next].0 == Kind::Eol {
                    next += 1;
                }
                let stop = if tag {
                    is_tag(next) || ends(next)
                } else {
                    next > pos + 1 || is_tag(next) || ends(next)
                };
                if !stop {
                    pos = next;
                }
            }
        }
        children.push((start, pos));
        if ends(pos) {
            break;
        }
        pos += 1;
        if ends(pos) {
            break;
        }
    }
    children
}
