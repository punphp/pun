//! `@pun-ignore` comments (context/plan/rules.md §4): which problems a file's comments suppress, and which of their
//! rule IDs suppressed nothing.
//!
//! | Form | Applies to |
//! | --- | --- |
//! | `$x = foo(); // @pun-ignore <id>` | the line it trails |
//! | `// @pun-ignore <id>` on its own line | the line after the comment |
//! | `// @pun-ignore-next-line <id>` | the line after the comment |
//! | `// @pun-ignore-file <id>` | the whole file |
//! | `// @pun-ignore-start <id>` ... `// @pun-ignore-end` | the lines from the start comment to the end comment (to the end of the file when it has none) |
//!
//! An ID is a full rule ID (`lint/security/no-eval`), a rule's `group/name`, a group (`lint/security` or
//! `security`), comma-separated. Text after ` -- ` is the reason. `//`, `#`, `/* */` and `/** */` comments all
//! work. A directive without an ID is malformed. A problem's line is the line its span starts on.

use std::ops::Range;

use pun_tokens::Kind;

use crate::{Edit, Fix, LineIndex, Problem, Safety, Span};

/// Where a directive applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scope {
    /// 1-based lines, inclusive.
    Lines(usize, usize),
    File,
}

/// One rule ID of a directive and where it is written.
#[derive(Clone, Debug)]
struct Id {
    text: String,
    span: Span,
    used: bool,
}

#[derive(Clone, Debug)]
struct Directive {
    scope: Scope,
    ids: Vec<Id>,
    /// The comment holding it.
    comment: Span,
    /// The comment is alone on its line(s), so deleting it deletes the line.
    own_line: bool,
}

/// A directive that names no rule: `// @pun-ignore` alone, or `// @pun-ignore -- reason`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Malformed {
    /// The directive's `@pun-ignore…` word.
    pub span: Span,
    pub message: String,
}

/// An ignore ID that suppressed nothing, with the fix that deletes it: the whole comment (and its line, when the
/// comment is alone on it) when no other ID of the comment was used, otherwise the ID and its comma. The IDs of one
/// all-unused comment share the same deletion; the fix loop applies it once and the rest go with the comment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unused {
    pub id: String,
    pub span: Span,
    pub fix: Fix,
}

/// The ignore directives of one file.
#[derive(Clone, Debug, Default)]
pub struct Ignores {
    directives: Vec<Directive>,
    pub malformed: Vec<Malformed>,
}

const MARK: &str = "@pun-ignore";

/// Whether ignore ID `id` covers `rule`: the same ID, its `group/name`, its `<command>/<group>`, or its group.
pub fn covers(id: &str, rule: &str) -> bool {
    if id == rule {
        return true;
    }
    let mut parts = rule.splitn(3, '/');
    let (Some(command), Some(group), name) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    match id.split_once('/') {
        None => id == group,
        Some((a, b)) if a == command => b == group,
        Some((a, b)) => name.is_some_and(|n| a == group && b == n),
    }
}

impl Ignores {
    /// The directives in `source`'s comments. A source without `@pun-ignore` is not tokenized.
    pub fn parse(source: &str) -> Ignores {
        let mut out = Ignores::default();
        if !source.contains(MARK) {
            return out;
        }
        let index = LineIndex::new(source);
        let base = source.as_ptr() as usize;
        let mut open: Vec<usize> = Vec::new();
        for token in pun_tokens::tokenize(source.as_bytes()) {
            if !matches!(token.kind, Some(Kind::T_COMMENT | Kind::T_DOC_COMMENT)) {
                continue;
            }
            let start = token.text.as_ptr() as usize - base;
            let comment = Span::new(start, start + token.text.len());
            let text = &source[comment.start..comment.end];
            let Some(at) = text.find(MARK) else {
                continue;
            };
            let rest = &text[at + MARK.len()..];
            let (kind, after) = ["-next-line", "-file", "-start", "-end", ""]
                .into_iter()
                .find_map(|k| rest.strip_prefix(k).map(|r| (k, r)))
                .expect("the empty suffix always matches");
            // `@pun-ignore-foo`: not a directive.
            if after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
                continue;
            }
            let word = Span::new(start + at, start + at + MARK.len() + kind.len());
            let first = index.line_col(comment.start).line;
            let last = index
                .line_col(comment.end.saturating_sub(1).max(comment.start))
                .line;
            let line = index.line_range(first).expect("the comment's line");
            let own_line = source[line.start..comment.start].trim().is_empty()
                && (index.line_range(last))
                    .is_some_and(|r| source[comment.end..r.end].trim().is_empty());
            if kind == "-end" {
                // `open` holds `-start` directives, whose scope is lines.
                if let Some(i) = open.pop()
                    && let Scope::Lines(from, _) = out.directives[i].scope
                {
                    out.directives[i].scope = Scope::Lines(from, last);
                }
                continue;
            }
            let ids = ids(source, word.end, comment.end, text.ends_with("*/"));
            if ids.is_empty() {
                out.malformed.push(Malformed {
                    span: word,
                    message: format!("`{MARK}{kind}` needs one or more rule IDs"),
                });
                continue;
            }
            let scope = match kind {
                "-file" => Scope::File,
                "-start" => Scope::Lines(first, usize::MAX),
                "-next-line" => Scope::Lines(last + 1, last + 1),
                _ if own_line => Scope::Lines(last + 1, last + 1),
                _ => Scope::Lines(first, first),
            };
            if kind == "-start" {
                open.push(out.directives.len());
            }
            out.directives.push(Directive {
                scope,
                ids,
                comment,
                own_line,
            });
        }
        out
    }

    /// No directive and nothing malformed: the common case, which a caller can skip.
    pub fn is_empty(&self) -> bool {
        self.directives.is_empty() && self.malformed.is_empty()
    }

    /// `problems` without those a directive covers; each covering ID is marked used.
    pub fn retain(&mut self, problems: &mut Vec<Problem>, index: &LineIndex) {
        if self.directives.is_empty() {
            return;
        }
        problems.retain(|p| !self.ignores(&p.rule, index.line_col(p.span.start).line));
    }

    /// Whether a directive covers a problem of `rule` on 1-based `line`; every covering ID is marked used.
    pub fn ignores(&mut self, rule: &str, line: usize) -> bool {
        let mut ignored = false;
        for d in &mut self.directives {
            let applies = match d.scope {
                Scope::File => true,
                Scope::Lines(from, to) => (from..=to).contains(&line),
            };
            if !applies {
                continue;
            }
            for id in &mut d.ids {
                if covers(&id.text, rule) {
                    id.used = true;
                    ignored = true;
                }
            }
        }
        ignored
    }

    /// Whether an `@pun-ignore-file` directive covers `rule`, for rules that report a whole file rather than a line;
    /// every covering ID is marked used.
    pub fn ignores_file(&mut self, rule: &str) -> bool {
        let mut ignored = false;
        for d in self
            .directives
            .iter_mut()
            .filter(|d| d.scope == Scope::File)
        {
            for id in d.ids.iter_mut().filter(|i| covers(&i.text, rule)) {
                id.used = true;
                ignored = true;
            }
        }
        ignored
    }

    /// The IDs no problem used, among those `ran` says belong to a rule that ran (an ID of another command's rule,
    /// or of a rule that is off, can't be judged). Each carries the fix that deletes it.
    pub fn unused(&self, source: &str, ran: impl Fn(&str) -> bool) -> Vec<Unused> {
        let mut out = Vec::new();
        for d in &self.directives {
            let judged: Vec<&Id> = d.ids.iter().filter(|i| ran(&i.text)).collect();
            let dead: Vec<&&Id> = judged.iter().filter(|i| !i.used).collect();
            if dead.is_empty() {
                continue;
            }
            let all_dead = dead.len() == d.ids.len();
            for id in dead {
                let edits = if all_dead {
                    vec![Edit::new(delete_comment(source, d), "")]
                } else {
                    vec![Edit::new(delete_id(source, d, id.span), "")]
                };
                out.push(Unused {
                    id: id.text.clone(),
                    span: id.span,
                    fix: Fix {
                        safety: Safety::Safe,
                        edits,
                    },
                });
            }
        }
        out
    }
}

/// The IDs after a directive's word: comma-separated up to ` -- `, the comment's end or the line's end.
fn ids(source: &str, from: usize, comment_end: usize, block: bool) -> Vec<Id> {
    let end = if block { comment_end - 2 } else { comment_end };
    let mut text = &source[from..end];
    if let Some(i) = text.find('\n') {
        text = &text[..i];
    }
    if let Some(i) = text.find(" --") {
        text = &text[..i];
    }
    let mut out = Vec::new();
    let mut offset = from;
    for part in text.split(',') {
        let trimmed = part.trim_start();
        let start = offset + part.len() - trimmed.len();
        let id = trimmed.split_whitespace().next().unwrap_or("");
        let id = id.trim_end_matches('*');
        if !id.is_empty() {
            out.push(Id {
                text: id.to_string(),
                span: Span::new(start, start + id.len()),
                used: false,
            });
        }
        offset += part.len() + 1;
    }
    out
}

/// The bytes to delete for a whole directive: the comment, and its line when it is alone there.
fn delete_comment(source: &str, d: &Directive) -> Span {
    if !d.own_line {
        // Also the spaces between the code and a trailing comment.
        let before = source[..d.comment.start]
            .trim_end_matches([' ', '\t'])
            .len();
        return Span::new(before, d.comment.end);
    }
    let start = source[..d.comment.start].rfind('\n').map_or(0, |i| i + 1);
    let rest = &source[d.comment.end..];
    let end = d.comment.end + rest.find('\n').map_or(rest.len(), |i| i + 1);
    Span::new(start, end)
}

/// The bytes to delete for one ID of a list: the ID and the comma (and spaces) after it, or before it for the last.
fn delete_id(source: &str, d: &Directive, id: Span) -> Span {
    let after: Range<usize> = id.end..d.comment.end;
    let tail = &source[after];
    let skipped = tail.len() - tail.trim_start_matches([' ', '\t']).len();
    if tail[skipped..].starts_with(',') {
        let rest = &tail[skipped + 1..];
        let spaces = rest.len() - rest.trim_start_matches([' ', '\t']).len();
        return Span::new(id.start, id.end + skipped + 1 + spaces);
    }
    let head = &source[d.comment.start..id.start];
    let comma = head.trim_end_matches([' ', '\t']);
    let comma = comma.strip_suffix(',').unwrap_or(comma);
    Span::new(d.comment.start + comma.len(), id.end)
}
