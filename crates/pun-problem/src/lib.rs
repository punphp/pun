//! The problem type that `pun lint`, `pun analyse` and the renderer share (config.md Build 9).
//!
//! A [`Problem`] is one finding: a rule ID, the file, a byte [`Span`], a message, an optional help line, a [`Level`]
//! and an optional [`Fix`]. A fix is a list of text edits on byte ranges (lint.md §6), labelled safe or unsafe.
//! [`LineIndex`] turns a byte offset into the `line:col` the output shows, and [`apply_fixes`] applies a set of fixes
//! to a source, deferring the ones that overlap a fix applied before them.
//!
//! Lines and columns are 1-based. A column counts bytes, not characters (as PHP's `strlen` and Mago's spans do), so
//! `é` advances it by 2. A line ends at `\n`, `\r\n` or a lone `\r`, as in PHP's own scanner.

use std::ops::Range;
use std::path::PathBuf;

pub mod ignore;

/// A byte range `start..end` in a file's source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

/// How bad a problem is. Lint rules are `error` or `warn` (`off` rules report nothing); analyse is always `Error`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Level {
    Error,
    Warning,
}

/// Whether a fix keeps the code's behaviour (`safe`) or may change it (`unsafe`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Safety {
    Safe,
    Unsafe,
}

/// Replace the bytes in `span` with `text`. An empty span inserts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub span: Span,
    pub text: String,
}

impl Edit {
    pub fn new(span: Span, text: impl Into<String>) -> Self {
        Edit {
            span,
            text: text.into(),
        }
    }
}

/// A fix: edits that apply together or not at all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    pub safety: Safety,
    pub edits: Vec<Edit>,
}

/// One finding. `rule` is the full ID (`lint/laravel/no-debug-calls`), `path` is relative to the project root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    pub rule: String,
    pub path: PathBuf,
    pub span: Span,
    pub message: String,
    pub help: Option<String>,
    pub level: Level,
    pub fix: Option<Fix>,
}

/// A 1-based line and a 1-based byte column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

/// The start offset of every line of one source, built once and queried per problem.
#[derive(Clone, Debug)]
pub struct LineIndex {
    starts: Vec<usize>,
    len: usize,
    /// The length of each line's terminator (0 for the last line).
    ends: Vec<usize>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let bytes = source.as_bytes();
        let mut starts = vec![0];
        let mut ends = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let width = match bytes[i] {
                b'\r' if bytes.get(i + 1) == Some(&b'\n') => 2,
                b'\r' | b'\n' => 1,
                _ => 0,
            };
            if width == 0 {
                i += 1;
                continue;
            }
            i += width;
            ends.push(width);
            starts.push(i);
        }
        ends.push(0);
        LineIndex {
            starts,
            len: bytes.len(),
            ends,
        }
    }

    /// The number of lines; a source ending in a line break has an empty last line.
    pub fn line_count(&self) -> usize {
        self.starts.len()
    }

    /// The line and column of a byte offset. An offset past the end is clamped to the end.
    pub fn line_col(&self, offset: usize) -> LineCol {
        let offset = offset.min(self.len);
        let line = self.starts.partition_point(|&s| s <= offset);
        LineCol {
            line,
            column: offset - self.starts[line - 1] + 1,
        }
    }

    /// The byte range of a 1-based line without its terminator, or `None` past the last line.
    pub fn line_range(&self, line: usize) -> Option<Range<usize>> {
        let index = line.checked_sub(1)?;
        let start = *self.starts.get(index)?;
        let end = self
            .starts
            .get(index + 1)
            .map_or(self.len, |&next| next - self.ends[index]);
        Some(start..end)
    }
}

/// What [`apply_fixes`] did, by index into the fixes it was given.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Applied {
    pub source: String,
    pub applied: Vec<usize>,
    /// Fixes that touch a fix applied before them; they can apply in the next round, on the new source.
    pub deferred: Vec<usize>,
    /// Fixes that can never apply: an edit past the end, reversed, splitting a UTF-8 character, or overlapping
    /// another edit of the same fix.
    pub invalid: Vec<usize>,
}

/// Two edits conflict when their ranges overlap, or when one is an insertion at or inside the other's range (the
/// order of the inserted text would be ambiguous). Adjacent replacements do not conflict.
fn conflicts(a: Span, b: Span) -> bool {
    let overlap = a.start < b.end && b.start < a.end;
    let inserts_into = |ins: Span, other: Span| {
        ins.start == ins.end && other.start <= ins.start && ins.start <= other.end
    };
    overlap || inserts_into(a, b) || inserts_into(b, a)
}

/// Apply `fixes` to `source` in the order given, so the caller's order is the priority. A fix applies whole, or is
/// deferred when one of its edits conflicts with an edit of a fix applied before it.
pub fn apply_fixes(source: &str, fixes: &[&Fix]) -> Applied {
    let mut accepted: Vec<&Edit> = Vec::new();
    let mut applied = Vec::new();
    let mut deferred = Vec::new();
    let mut invalid = Vec::new();
    for (index, fix) in fixes.iter().enumerate() {
        let edits = &fix.edits;
        let bad_range = edits.iter().any(|e| {
            let Span { start, end } = e.span;
            start > end
                || end > source.len()
                || !source.is_char_boundary(start)
                || !source.is_char_boundary(end)
        });
        let self_overlap = edits
            .iter()
            .enumerate()
            .any(|(i, a)| edits[i + 1..].iter().any(|b| conflicts(a.span, b.span)));
        if bad_range || self_overlap {
            invalid.push(index);
        } else if edits
            .iter()
            .any(|e| accepted.iter().any(|a| conflicts(a.span, e.span)))
        {
            deferred.push(index);
        } else {
            accepted.extend(edits);
            applied.push(index);
        }
    }
    accepted.sort_by_key(|e| e.span.start);
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0;
    for edit in accepted {
        out.push_str(&source[cursor..edit.span.start]);
        out.push_str(&edit.text);
        cursor = edit.span.end;
    }
    out.push_str(&source[cursor..]);
    Applied {
        source: out,
        applied,
        deferred,
        invalid,
    }
}
