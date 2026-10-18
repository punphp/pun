//! GitHub Actions annotations (output.md Build 4): PHPStan's GithubErrorFormatter
//! (.oracle/phpstan-src/src/Command/ErrorFormatter/GithubErrorFormatter.php), one workflow command per line.
//!
//! `::error file=<path>,line=<n>,col=<c>,title=<rule>::<message>` for a finding, `::warning …` for a lint warning,
//! `::error ::<message>` for a run error. PHPStan writes `col=0`; Pun writes the problem's column when it has one.
//! PHPStan escapes only the message's newlines; Pun also escapes what GitHub's own toolkit (`@actions/core`
//! `escapeData` / `escapeProperty`) does, so a `%` in a message or a `,` or `:` in a path survives (output.md
//! Decisions).

use pcre2::bytes::Regex;
use pun_problem::{Level, LineIndex, Problem};
use std::sync::LazyLock;

/// GithubErrorFormatter::formatMessage: newlines encoded, and each `@mention` put in backticks so GitHub doesn't
/// notify anyone. `%` and `\r` are encoded first (escapeData), so GitHub decodes the message back to what it was.
pub(crate) fn escape_data(message: &str) -> String {
    let message = message
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A");
    mentions(&message)
}

/// Each `@mention` in backticks, as formatMessage's preg_replace does (every non-overlapping match, left to right).
pub(crate) fn mentions(message: &str) -> String {
    static MENTION: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(^|\s)@([a-zA-Z0-9_\-]+)(\s|$)").expect("regex"));
    let bytes = message.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    for caps in MENTION.captures_iter(bytes).flatten() {
        let whole = caps.get(0).expect("match");
        out.extend_from_slice(&bytes[at..whole.start()]);
        out.extend_from_slice(&caps[1]);
        out.push(b'`');
        out.push(b'@');
        out.extend_from_slice(&caps[2]);
        out.push(b'`');
        out.extend_from_slice(&caps[3]);
        at = whole.end();
    }
    out.extend_from_slice(&bytes[at..]);
    String::from_utf8(out).expect("utf-8 in, utf-8 out")
}

/// A property value (`file=`, `title=`): escapeProperty's `%`, `\r`, `\n`, `:` and `,`.
pub(crate) fn escape_property(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

/// One annotation for a place in a file: `col` 0 when there is none, as PHPStan writes it.
fn annotation(
    level: Level,
    path: &str,
    line: usize,
    col: usize,
    title: &str,
    message: &str,
) -> String {
    let command = match level {
        Level::Error => "error",
        Level::Warning => "warning",
    };
    format!(
        "::{command} file={},line={line},col={col},title={}::{}\n",
        escape_property(path),
        escape_property(title),
        escape_data(message)
    )
}

/// A problem (lint, analyse, a syntax error) at its span's start.
pub(crate) fn problem(p: &Problem, index: &LineIndex) -> String {
    let at = index.line_col(p.span.start);
    annotation(
        p.level,
        &p.path.to_string_lossy(),
        at.line,
        at.column,
        &p.rule,
        &p.message,
    )
}

/// A whole file (it needs formatting, or rules can fix it): an error at `line`, titled with the rule IDs.
pub(crate) fn file(path: &str, line: usize, rules: &[String], message: &str) -> String {
    annotation(Level::Error, path, line, 0, &rules.join(", "), message)
}

/// A run error, which has no file: `::error ::<message>`.
pub(crate) fn run_error(message: &str) -> String {
    format!("::error ::{}\n", escape_data(message))
}

/// The first line `new` changes in `old` (1-based): GitlabReporter::getLines' `begin`, the first `+`/`-` line of the
/// diff's first chunk. sebastian/diff drops the common prefix before diffing, so that is the line holding the first
/// byte the two differ in. Lines end at `\r\n`, `\r` or `\n`, as its `\R` split does.
pub(crate) fn first_changed_line(old: &[u8], new: &[u8]) -> usize {
    let same = old.iter().zip(new).take_while(|(a, b)| a == b).count();
    let mut line = 1;
    for (i, &b) in old[..same].iter().enumerate() {
        // A `\r` ends a line unless a `\n` follows it.
        if b == b'\n' || (b == b'\r' && old.get(i + 1) != Some(&b'\n')) {
            line += 1;
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use pun_problem::Span;

    /// Hand-derived from GithubErrorFormatter::formatMessage plus escapeData: `%` first (so `%0A` in the text stays
    /// literal), newlines encoded, mentions at a word start or after whitespace back-ticked, a mid-word `@` kept.
    #[test]
    fn escapes_like_github() {
        assert_eq!(
            escape_data("100% done\r\nping @taylor, a@b and @x_y-1 %0A"),
            "100%25 done%0D%0Aping @taylor, a@b and `@x_y-1` %250A"
        );
        assert_eq!(escape_data("@one @two"), "`@one` @two");
        assert_eq!(
            escape_property("app/a,b:c%\r\n.php"),
            "app/a%2Cb%3Ac%25%0D%0A.php"
        );
    }

    #[test]
    fn annotation_lines() {
        let source = "<?php\n\n  dd($x);\n";
        let index = LineIndex::new(source);
        let mut p = Problem {
            rule: "lint/laravel/no-debug-calls".into(),
            path: "app/a,b:c.php".into(),
            span: Span::new(9, 15),
            message: "dd() left in the code\nremove it".into(),
            help: None,
            level: Level::Error,
            fix: None,
        };
        assert_eq!(
            problem(&p, &index),
            "::error file=app/a%2Cb%3Ac.php,line=3,col=3,title=lint/laravel/no-debug-calls::dd() left in the code%0Aremove it\n"
        );
        p.level = Level::Warning;
        assert!(problem(&p, &index).starts_with("::warning file="));
        assert_eq!(
            file(
                "app/User.php",
                2,
                &["format/arrays/syntax".into(), "lint/x".into()],
                "File needs formatting"
            ),
            "::error file=app/User.php,line=2,col=0,title=format/arrays/syntax%2C lint/x::File needs formatting\n"
        );
        assert_eq!(
            run_error("Format [a%b] is not supported."),
            "::error ::Format [a%25b] is not supported.\n"
        );
    }

    /// Lines of the first changed byte, counted by hand.
    #[test]
    fn first_changed_lines() {
        assert_eq!(first_changed_line(b"<?php\n$a;\n", b"<?php\n$b;\n"), 2);
        assert_eq!(first_changed_line(b"a\r\nb\r\nc", b"a\r\nb\r\nd"), 3);
        assert_eq!(first_changed_line(b"a\rb\rc", b"a\rb\rd"), 3);
        // An inserted line after line 1 begins at line 2; a missing final newline is the last line.
        assert_eq!(first_changed_line(b"a\nb\n", b"a\n\nb\n"), 2);
        assert_eq!(first_changed_line(b"a\nb", b"a\nb\n"), 2);
        // A change right after a `\r` that a `\n` follows in `old` only.
        assert_eq!(first_changed_line(b"a\r\nb", b"a\rb"), 1);
    }
}
