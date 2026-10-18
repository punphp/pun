//! Pun's own human output (output.md §1, Build 1-2): the one renderer every command calls.
//!
//! Four pieces: file lines, problem blocks (rustc-style code frames from a `pun_problem` span, or one line each under
//! `--concise`), the summary line, and run errors with a hint. Each returns its text with a trailing newline; the
//! caller writes it. [`Look`] decides colour, glyphs and links; [`Spinner`] is the progress line. `pun format`'s
//! own pieces (format.md §9) are its summary wording and the `-v` diff.

mod look;
mod spinner;
#[cfg(test)]
mod tests;

pub use look::{ColorChoice, Look, Role};
#[cfg(test)]
pub use look::{Depth, Theme, nearest_256};
pub use spinner::Spinner;

use pun_problem::{Level, LineIndex, Problem, Safety};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// File lines pad the path to this many characters (at least two spaces before the rules), so the rule IDs start in
/// one column.
const PATH_COLUMN: usize = 27;
/// A problem's `fixable` label starts at this character of its header line (at least two spaces after the message).
const LABEL_COLUMN: usize = 77;
/// Code frames show at most this many lines of one span; a longer span shows its first two lines, `…`, its last.
const MAX_SPAN_LINES: usize = 3;
/// A tab in a code frame shows as this many spaces, and deeper indentation is cut to it.
const INDENT: usize = 4;

/// What happened to one file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileStatus {
    /// Fixed (✔).
    Fixed,
    /// Needs fixing, under `--test` (✖).
    NeedsFix,
    /// Unchanged; printed dim, only under `-v`.
    Unchanged,
}

/// One file line: the path (relative to the project root) and the rule IDs that changed it, or would.
#[derive(Clone, Debug)]
pub struct FileLine {
    pub path: PathBuf,
    pub status: FileStatus,
    pub rules: Vec<String>,
}

/// The summary line's counts. `fix_command` is what `fixable` tells the user to run (`pun check --fix`).
#[derive(Clone, Debug)]
pub struct Summary<'a> {
    pub errors: usize,
    pub warnings: usize,
    pub fixable: usize,
    pub fix_command: &'a str,
    pub need_formatting: usize,
    pub files: usize,
    pub elapsed: Duration,
}

/// `pun format`'s counts: files formatted (or needing it, under `--test`), unchanged, failed, and in all.
#[derive(Clone, Debug)]
pub struct FormatSummary {
    pub formatted: usize,
    pub unchanged: usize,
    pub errors: usize,
    pub files: usize,
    pub test: bool,
    pub elapsed: Duration,
}

/// A run error: where (`pun.toml:7`, a path relative to the root, with an optional line), what, and a hint.
#[derive(Clone, Debug)]
pub struct RunError {
    pub path: Option<PathBuf>,
    pub line: Option<usize>,
    pub message: String,
    pub hint: Option<String>,
}

/// The renderer: a [`Look`], the project root (for clickable paths) and the base URL of the rule docs.
#[derive(Clone, Debug)]
pub struct Renderer {
    pub look: Look,
    /// Problems print one line each (`--concise`).
    pub concise: bool,
    root: PathBuf,
    rule_docs: Option<String>,
}

impl Renderer {
    pub fn new(look: Look, root: impl Into<PathBuf>) -> Renderer {
        Renderer {
            look,
            concise: false,
            root: root.into(),
            rule_docs: None,
        }
    }

    /// Link each rule ID to `base` followed by the ID. Test-only until the docs site has rule pages (the docs-base
    /// row in PLAN.md §3 passes the base).
    #[cfg(test)]
    pub fn with_rule_docs(mut self, base: impl Into<String>) -> Renderer {
        self.rule_docs = Some(base.into());
        self
    }

    /// One line per file, the rule IDs in one column after the longest path. Each line is its own string, so a
    /// caller can put a diff under it.
    pub fn file_lines(&self, files: &[FileLine]) -> Vec<String> {
        let width = files
            .iter()
            .map(|f| f.path.display().to_string().chars().count() + 2)
            .fold(PATH_COLUMN, usize::max);
        files.iter().map(|f| self.file_line(f, width)).collect()
    }

    /// One file line, its rule IDs starting at `width`.
    fn file_line(&self, file: &FileLine, width: usize) -> String {
        let (glyph, role) = match file.status {
            FileStatus::Fixed => (self.look.glyph("✔", "ok"), Role::Ok),
            FileStatus::NeedsFix => (self.look.glyph("✖", "x"), Role::Error),
            FileStatus::Unchanged => (self.look.glyph("✔", "ok"), Role::Dim),
        };
        let shown = file.path.display().to_string();
        let mut path = self.path_link(&file.path, &shown);
        if file.status == FileStatus::Unchanged {
            path = self.look.paint(Role::Dim, &path);
        }
        let mut out = format!(" {} {path}", self.look.paint(role, glyph));
        if !file.rules.is_empty() {
            let rules: Vec<String> = file.rules.iter().map(|r| self.rule(r)).collect();
            let pad = width - shown.chars().count();
            out.push_str(&format!("{}{}", " ".repeat(pad), rules.join(", ")));
        }
        out.push('\n');
        out
    }

    /// `-v`'s diff under a file line (format.md §9), from a unified diff: each hunk's lines with their old and new
    /// line numbers, removed lines red, added lines green, context dim; `…` between hunks. The `---`/`+++` header
    /// and `\ No newline` markers are left out.
    pub fn diff(&self, unified: &str) -> String {
        enum Row<'a> {
            Gap,
            Line(Option<usize>, Option<usize>, char, &'a str),
        }
        let mut rows = vec![];
        let (mut old, mut new, mut hunks) = (0, 0, 0);
        for line in unified.split('\n') {
            let line = line.strip_suffix('\r').unwrap_or(line);
            if let Some(header) = line.strip_prefix("@@ -") {
                let start = |s: &str| s.split([',', ' ']).next().and_then(|n| n.parse().ok());
                old = start(header).unwrap_or(1);
                new = header
                    .split_once(" +")
                    .and_then(|(_, n)| start(n))
                    .unwrap_or(1);
                if hunks > 0 {
                    rows.push(Row::Gap);
                }
                hunks += 1;
                continue;
            }
            if hunks == 0 {
                continue;
            }
            let (sign, text) = match line.chars().next() {
                Some(c @ ('+' | '-' | ' ')) => (c, &line[1..]),
                _ => continue,
            };
            let (o, n) = match sign {
                '-' => (Some(old), None),
                '+' => (None, Some(new)),
                _ => (Some(old), Some(new)),
            };
            old += usize::from(sign != '+');
            new += usize::from(sign != '-');
            rows.push(Row::Line(o, n, sign, text));
        }
        let width = old.max(new).saturating_sub(1).max(1).to_string().len();
        let number = |n: Option<usize>| match n {
            Some(n) => format!("{n:>width$}"),
            None => " ".repeat(width),
        };
        let bar = self.look.paint(Role::Dim, self.look.glyph("│", "|"));
        let mut out = String::new();
        for row in rows {
            let Row::Line(o, n, sign, text) = row else {
                let gap = self.look.glyph("…", "...");
                out.push_str(&format!("   {}\n", self.look.paint(Role::Dim, gap)));
                continue;
            };
            let numbers = self
                .look
                .paint(Role::Dim, &format!("{} {}", number(o), number(n)));
            let code = match sign {
                '-' => self.look.paint(Role::Error, &format!("-{text}")),
                '+' => self.look.paint(Role::Ok, &format!("+{text}")),
                _ if text.is_empty() => String::new(),
                _ => format!(" {}", self.look.paint(Role::Dim, text)),
            };
            let space = if code.is_empty() { "" } else { " " };
            out.push_str(&format!("   {numbers} {bar}{space}{code}\n"));
        }
        out
    }

    /// The problems of one or more files: blocks separated by a blank line, or one line each under `--concise`.
    pub fn problems<'a>(
        &self,
        problems: impl IntoIterator<Item = (&'a Problem, &'a str, &'a LineIndex)>,
    ) -> String {
        let blocks: Vec<String> = problems
            .into_iter()
            .map(|(p, source, index)| self.problem(p, source, index))
            .collect();
        blocks.join(if self.concise { "" } else { "\n" })
    }

    /// One problem: its header and a code frame, or a single line under `--concise`.
    pub fn problem(&self, problem: &Problem, source: &str, index: &LineIndex) -> String {
        let (glyph, role) = level(&self.look, problem.level);
        let at = index.line_col(problem.span.start);
        let location = format!("{}:{}:{}", problem.path.display(), at.line, at.column);
        let label = problem.fix.as_ref().map(|fix| match fix.safety {
            Safety::Safe => "fixable",
            Safety::Unsafe => "fixable (unsafe)",
        });
        let glyph = self.look.paint(role, glyph);
        let rule = self.rule(&problem.rule);
        if self.concise {
            let location = self.path_link(&problem.path, &location);
            let label = label.map_or(String::new(), |l| {
                format!("  {}", self.look.paint(Role::Dim, l))
            });
            return format!(" {glyph} {location}  {rule}  {}{label}\n", problem.message);
        }
        let mut out = format!(" {glyph} {rule}  {}", problem.message);
        if let Some(label) = label {
            let used = 5 + problem.rule.chars().count() + problem.message.chars().count();
            let pad = LABEL_COLUMN.saturating_sub(used).max(2);
            out.push_str(&format!(
                "{}{}",
                " ".repeat(pad),
                self.look.paint(Role::Dim, label)
            ));
        }
        out.push('\n');
        out.push_str(&self.frame(problem, &location, source, index, role));
        out
    }

    /// The code frame under a problem's header: path, the lines of its span with one line of context before, the
    /// marker under the span, and the help line.
    fn frame(
        &self,
        problem: &Problem,
        location: &str,
        source: &str,
        index: &LineIndex,
        role: Role,
    ) -> String {
        let span = problem.span;
        let first = index.line_col(span.start).line;
        let last = index
            .line_col(span.end.max(span.start + 1) - 1)
            .line
            .max(first);
        let text = |line: usize| {
            let r = index.line_range(line).expect("a line of the span");
            (r.start, &source[r])
        };
        let mut lines: Vec<Option<usize>> = Vec::new();
        if first > 1 && !text(first - 1).1.trim().is_empty() {
            lines.push(Some(first - 1));
        }
        if last - first + 1 > MAX_SPAN_LINES {
            lines.extend([Some(first), Some(first + 1), None, Some(last)]);
        } else {
            lines.extend((first..=last).map(Some));
        }
        let strip = lines
            .iter()
            .flatten()
            .map(|&l| text(l).1)
            .filter(|t| !t.trim().is_empty())
            .map(|t| columns(&t[..t.len() - t.trim_start_matches([' ', '\t']).len()]))
            .min()
            .unwrap_or(0)
            .saturating_sub(INDENT)
            .min({
                let (start, code) = text(first);
                columns(&code[..(span.start - start).min(code.len())])
            });
        let width = last.to_string().len();
        let bar = self.look.paint(Role::Dim, self.look.glyph("│", "|"));
        let mut out = format!(
            " {:width$}{} {}\n",
            "",
            self.look.paint(Role::Dim, self.look.glyph("╭─", ",-")),
            self.path_link(&problem.path, location),
        );
        for line in lines {
            let Some(line) = line else {
                let more = self.look.glyph("…", "...");
                out.push_str(&format!(
                    " {}\n",
                    self.look.paint(Role::Dim, &format!("{more:>width$}"))
                ));
                continue;
            };
            let (start, code) = text(line);
            let shown: String = expand(code).chars().skip(strip).collect();
            let number = self.look.paint(Role::Dim, &format!("{line:>width$}"));
            out.push_str(format!(" {number}{bar} {shown}").trim_end());
            out.push('\n');
            if (first..=last).contains(&line) && (first == last || !code.trim().is_empty()) {
                let indent = code.len() - code.trim_start().len();
                let from = if line == first {
                    (span.start - start).min(code.len())
                } else {
                    indent
                };
                let to = if line == last {
                    (span.end - start).min(code.len())
                } else {
                    code.len()
                };
                let pad = columns(&code[..from]) - strip;
                let marks = columns(&code[from..to.max(from)]).max(1);
                let marker = self.look.glyph("━", "^").repeat(marks);
                out.push_str(&format!(
                    " {:width$}{bar} {}{}\n",
                    "",
                    " ".repeat(pad),
                    self.look.paint(role, &marker)
                ));
            }
        }
        let corner = self.look.paint(Role::Dim, self.look.glyph("╰─", "`-"));
        match &problem.help {
            Some(help) => out.push_str(&format!(" {:width$}{corner} help: {help}\n", "")),
            None => out.push_str(&format!(" {:width$}{corner}\n", "")),
        }
        out
    }

    /// The last line of a run: the non-zero counts in a fixed order, then the file count and the time.
    pub fn summary(&self, s: &Summary) -> String {
        let plural =
            |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
        let mut parts = Vec::new();
        if s.errors > 0 {
            parts.push(
                self.look
                    .paint(Role::Error, &plural(s.errors, "error", "errors")),
            );
        }
        if s.warnings > 0 {
            parts.push(
                self.look
                    .paint(Role::Warning, &plural(s.warnings, "warning", "warnings")),
            );
        }
        if s.fixable > 0 {
            parts.push(format!("{} fixable (run {})", s.fixable, s.fix_command));
        }
        if s.need_formatting > 0 {
            parts.push(plural(
                s.need_formatting,
                "file needs formatting",
                "files need formatting",
            ));
        }
        let (glyph, role) = if s.errors > 0 || s.need_formatting > 0 {
            (self.look.glyph("✖", "x"), Role::Error)
        } else if s.warnings > 0 {
            (self.look.glyph("⚠", "!"), Role::Warning)
        } else {
            parts.push(self.look.paint(Role::Ok, "No problems"));
            (self.look.glyph("✔", "ok"), Role::Ok)
        };
        let files = plural(s.files, "file", "files");
        parts.push(self.look.paint(
            Role::Dim,
            &format!("{files} in {:.2}s", s.elapsed.as_secs_f64()),
        ));
        let separator = self.look.paint(Role::Dim, self.look.glyph("·", "-"));
        format!(
            "{} {}\n",
            self.look.paint(role, glyph),
            parts.join(&format!(" {separator} "))
        )
    }

    /// `pun format`'s last line (format.md §9): `Formatted N files, M unchanged · T files in Xs`, or under `--test`
    /// `N of T files need formatting · run \`pun format\` to fix · Xs` (`All T files formatted · Xs` when none
    /// does). Files that failed add a red `E errors` before the time.
    pub fn format_summary(&self, s: &FormatSummary) -> String {
        let plural =
            |n: usize, one: &str, many: &str| format!("{n} {}", if n == 1 { one } else { many });
        let time = format!("{:.2}s", s.elapsed.as_secs_f64());
        let files = plural(s.files, "file", "files");
        let mut parts = vec![];
        let mut timing = format!("{files} in {time}");
        if !s.test {
            parts.push(format!(
                "Formatted {}, {} unchanged",
                plural(s.formatted, "file", "files"),
                s.unchanged
            ));
        } else if s.formatted > 0 {
            let need = if s.formatted == 1 { "needs" } else { "need" };
            parts.push(format!("{} of {files} {need} formatting", s.formatted));
            timing = time;
        } else if s.errors == 0 {
            parts.push(format!("All {files} formatted"));
            timing = time;
        }
        if s.errors > 0 {
            parts.push(
                self.look
                    .paint(Role::Error, &plural(s.errors, "error", "errors")),
            );
        }
        if s.test && s.formatted > 0 {
            parts.push("run `pun format` to fix".into());
        }
        parts.push(self.look.paint(Role::Dim, &timing));
        let separator = self.look.paint(Role::Dim, self.look.glyph("·", "-"));
        format!("{}\n", parts.join(&format!(" {separator} ")))
    }

    /// A run error: one red line (`path:line  message`), then the hint indented under it.
    pub fn run_error(&self, e: &RunError) -> String {
        let glyph = self.look.glyph("✖", "x");
        let line = match &e.path {
            Some(path) => {
                let shown = match e.line {
                    Some(line) => format!("{}:{line}", path.display()),
                    None => path.display().to_string(),
                };
                format!("{glyph} {}  {}", self.path_link(path, &shown), e.message)
            }
            None => format!("{glyph} {}", e.message),
        };
        let mut out = format!("{}\n", self.look.paint(Role::Error, &line));
        if let Some(hint) = &e.hint {
            out.push_str(&format!("  {hint}\n"));
        }
        out
    }

    /// A rule ID in the rule colour, linked to its docs page when the base URL is known.
    fn rule(&self, id: &str) -> String {
        let painted = self.look.paint(Role::Rule, id);
        match &self.rule_docs {
            Some(base) => self.look.link(&format!("{base}{id}"), &painted),
            None => painted,
        }
    }

    /// `shown` linked to the file `path` (relative to the root) as a `file://` URL.
    fn path_link(&self, path: &Path, shown: &str) -> String {
        if !self.look.links {
            return shown.to_string();
        }
        self.look.link(&file_url(&self.root.join(path)), shown)
    }
}

/// The glyph and colour of a level.
fn level(look: &Look, level: Level) -> (&'static str, Role) {
    match level {
        Level::Error => (look.glyph("✖", "x"), Role::Error),
        Level::Warning => (look.glyph("⚠", "!"), Role::Warning),
    }
}

/// `text` with each tab as [`INDENT`] spaces.
fn expand(text: &str) -> String {
    text.replace('\t', &" ".repeat(INDENT))
}

/// How many columns `text` takes in a frame: one per character, [`INDENT`] per tab.
fn columns(text: &str) -> usize {
    text.chars()
        .map(|c| if c == '\t' { INDENT } else { 1 })
        .sum()
}

/// A `file://` URL for an absolute path, with every byte outside `A-Za-z0-9-._~/` percent-encoded.
pub fn file_url(path: &Path) -> String {
    let mut url = String::from("file://");
    for &b in path.to_string_lossy().as_bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/".contains(&b) {
            url.push(b as char);
        } else {
            url.push_str(&format!("%{b:02X}"));
        }
    }
    url
}
