//! A run's report. Humans get Pun's own output (output.md §1-§2, format.md §9) from [`crate::render`]; `-q`, the
//! abort boxes and every `--format` reporter stay Pint's, written as Symfony formatter markup and rendered by
//! `console::format`, so their colours (`--ansi`, TTY) come from one code path.

use crate::console::{self, format as fmt};
use crate::fix_report::{self, Change};
use crate::render::{
    ColorChoice, FileLine, FileStatus, FormatSummary, Look, Renderer, RunError, Summary,
};
use crate::{github, pint_error};
use anyhow::{Result, bail};
use pun_problem::{Level, LineIndex, Problem, Span};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Status {
    NoChanges,
    Skipped,
    Fixed,
    /// Syntax error in the input (PHP-CS-Fixer TYPE_INVALID).
    Invalid,
    /// Fixers produced invalid code (TYPE_LINT).
    Lint,
    /// A fixer threw (TYPE_EXCEPTION): Pun's fixers panic where upstream throws.
    Exception,
    /// Symlink: listed by the finder, dropped by the runner without an event.
    Link,
}

pub(crate) struct FileResult {
    pub(crate) path: PathBuf,
    pub(crate) rel: String,
    pub(crate) status: Status,
    pub(crate) applied: Vec<&'static str>,
    pub(crate) diff: String,
    pub(crate) message: String,
    /// The first line the fix changes (its GitHub annotation's line); 0 when nothing changes.
    pub(crate) line: usize,
    /// Lint and analyse problems left in the file, sorted by offset; their spans refer to `source`.
    pub(crate) problems: Vec<Problem>,
    /// The file's final content, when it has problems.
    pub(crate) source: String,
    /// The applied fixers whose lint rule is at `warn` for this file.
    pub(crate) warned: Vec<&'static str>,
}

impl FileResult {
    /// A file nothing happened to yet.
    pub(crate) fn new(path: PathBuf, rel: String) -> FileResult {
        FileResult {
            path,
            rel,
            status: Status::NoChanges,
            applied: vec![],
            diff: String::new(),
            message: String::new(),
            line: 0,
            problems: vec![],
            source: String::new(),
            warned: vec![],
        }
    }

    pub(crate) fn is_error(&self) -> bool {
        matches!(
            self.status,
            Status::Invalid | Status::Exception | Status::Lint
        )
    }
}

pub(crate) struct Ctx<'a> {
    pub(crate) total: usize,
    pub(crate) dry_run: bool,
    /// `--fix`: the fixes of the lint rules it doesn't apply, one result per file (status Fixed).
    pub(crate) remaining: &'a [FileResult],
    pub(crate) verbose: bool,
    pub(crate) ansi: bool,
    pub(crate) no_ansi: bool,
    /// The human output's colours, glyphs and links (`--color`).
    pub(crate) look: Look,
    /// The project root: file lines link their paths under it.
    pub(crate) root: &'a Path,
    pub(crate) elapsed: Duration,
    /// Human output under GitHub Actions (`GITHUB_ACTIONS=true`): the annotations follow it.
    pub(crate) github: bool,
    /// Problems print one line each (`--concise`).
    pub(crate) concise: bool,
    /// `pun check`: baseline entries no analyse error matched any more.
    pub(crate) unmatched_baseline: usize,
}

/// `pun lint` and `pun check`'s counts: errors (failed files, lint fixes and problems at `error`, analyse errors),
/// warnings, fixes `--fix` would apply, and files needing format rules.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Counts {
    pub(crate) errors: usize,
    pub(crate) warnings: usize,
    pub(crate) fixable: usize,
    pub(crate) need_formatting: usize,
}

/// A fixer's Pun rule ID (its own name when it has none).
fn rule_id(fixer: &str) -> &str {
    pun_config::rule_of(fixer).unwrap_or(fixer)
}

/// The fixers of `f` that apply lint rules: those at `error`, then those at `warn`.
fn lint_fixers(f: &FileResult) -> (usize, usize) {
    let lint = || (f.applied.iter()).filter(|n| !rule_id(n).starts_with("format/"));
    let warned = lint().filter(|n| f.warned.contains(n)).count();
    (lint().count() - warned, warned)
}

pub(crate) fn counts(ctx: &Ctx, results: &[FileResult]) -> Counts {
    let mut c = Counts {
        errors: results.iter().filter(|f| f.is_error()).count(),
        ..Counts::default()
    };
    for f in results {
        if ctx.dry_run && f.status == Status::Fixed {
            let (errors, warnings) = lint_fixers(f);
            c.errors += errors;
            c.warnings += warnings;
            c.fixable += errors + warnings;
            c.need_formatting += usize::from(errors + warnings < f.applied.len());
        }
        for p in &f.problems {
            match p.level {
                Level::Error => c.errors += 1,
                Level::Warning => c.warnings += 1,
            }
            c.fixable += usize::from(ctx.dry_run && p.fix.is_some());
        }
    }
    for f in ctx.remaining {
        let (errors, warnings) = lint_fixers(f);
        c.errors += errors;
        c.warnings += warnings;
    }
    c
}

/// A run error: Pun couldn't run (exit 2). Humans get [`RunError`]; Pint's box is kept for `--format` and `-q` runs
/// when Pint aborts the same way.
#[derive(Debug)]
pub(crate) struct Abort {
    /// The message on one line: agent mode's error, the `::error ::` annotation and Pint's box.
    pub(crate) message: String,
    /// Pint aborts with `message` too; `before` is what it prints ahead of the box.
    pub(crate) pint: bool,
    pub(crate) before: String,
    pub(crate) error: RunError,
    /// The exception Pint throws, for its box.
    pub(crate) thrown: pint_error::Thrown,
}

impl Abort {
    /// An abort Pint has too, with Pint's message. A message of several lines shows its first as the run error and
    /// the rest (without blank lines) as the hint, unless a hint is given.
    pub(crate) fn pint(message: impl Into<String>, hint: Option<String>) -> Abort {
        let message = message.into();
        let (first, rest) = message.split_once('\n').unwrap_or((&message, ""));
        let rest: Vec<&str> = rest.lines().filter(|l| !l.trim().is_empty()).collect();
        Abort {
            error: RunError {
                path: None,
                line: None,
                message: first.to_string(),
                hint: hint.or_else(|| (!rest.is_empty()).then(|| rest.join("\n"))),
            },
            message,
            pint: true,
            before: String::new(),
            thrown: pint_error::Thrown::Console,
        }
    }

    /// Pun's own run error (pun.toml, `pun migrate`, ...), which Pint doesn't have.
    pub(crate) fn pun(message: impl Into<String>, error: RunError) -> Abort {
        Abort {
            message: message.into(),
            pint: false,
            before: String::new(),
            error,
            thrown: pint_error::Thrown::Console,
        }
    }
}

impl std::fmt::Display for Abort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Abort {}

/// How a failed run shows its error.
pub(crate) struct Failure {
    /// `pun format`, `lint` and `check`: a plain error is Pint's message (its box), not Pun's own.
    pub(crate) pint: bool,
    /// A `--format` or `-q` run: Pint's box on stdout for the errors Pint has too.
    pub(crate) machine: bool,
    /// `--format=github`: the error as an annotation, and nothing else.
    pub(crate) github_format: bool,
    /// Human output under GitHub Actions: the annotation follows the error.
    pub(crate) annotate: bool,
    pub(crate) color: ColorChoice,
}

/// Print a run's error the way the run prints (agent line, annotation, Pint's box or Pun's run error on stderr);
/// the exit code is 2, Pun couldn't run.
pub(crate) fn fail(e: anyhow::Error, f: &Failure) -> u8 {
    let abort = match e.downcast::<Abort>() {
        Ok(a) => a,
        Err(e) if f.pint => {
            let mut abort = Abort::pint(e.to_string(), None);
            abort.thrown = pint_error::Thrown::of(&e);
            abort
        }
        Err(e) => {
            let message = format!("{e:#}");
            Abort::pun(
                message.clone(),
                RunError {
                    path: None,
                    line: None,
                    message,
                    hint: None,
                },
            )
        }
    };
    if crate::agent::enabled() {
        print!("{}", crate::agent::error(&abort.to_string()));
    } else if f.github_format {
        print!("{}", github::run_error(&abort.message));
    } else if f.machine && abort.pint {
        print!(
            "{}{}",
            abort.before,
            pint_error::render(&abort.message, abort.thrown)
        );
    } else {
        let root = std::env::current_dir().unwrap_or_default();
        let r = Renderer::new(Look::stderr(f.color), root);
        eprint!("{}", r.run_error(&abort.error));
        if f.annotate {
            print!("{}", github::run_error(&abort.message));
        }
    }
    2
}

/// The `on line N.` at the end of a lint message (the exception's line, used for the `-v` snippet).
fn error_line(message: &str) -> usize {
    message
        .rsplit(" on line ")
        .next()
        .and_then(|n| n.trim_end_matches('.').parse().ok())
        .unwrap_or(0)
}

/// ElaborateSummary::execute (output part). Errors (bad `--format`, an unwritable `--output-to-file`) are Pint's
/// aborts, run errors for Pun.
pub(crate) fn emit(
    ctx: &Ctx,
    results: &[FileResult],
    format: Option<&str>,
    quiet: bool,
    to_file: Option<&Path>,
    file_format: Option<&str>,
) -> Result<()> {
    let out_dec = console::decorated(1, ctx.ansi, ctx.no_ansi);
    let unsupported = |e: anyhow::Error| {
        let hint = "use json, txt, agent, checkstyle, junit, xml, gitlab or github";
        anyhow::Error::new(Abort::pint(e.to_string(), Some(hint.into())))
    };
    let err_dec = console::decorated(2, ctx.ansi, ctx.no_ansi);
    let mut out = String::new();
    let parsed = (format.map(str::parse::<ReportFormat>).transpose()).map_err(unsupported)?;
    if crate::agent::enabled() && format.is_none() {
        out = agent(ctx, results);
    } else if parsed == Some(ReportFormat::Github) {
        // Not formatter markup: printed as is.
        print!("{}", annotations(ctx, results));
    } else if let Some(f) = parsed {
        out = reporter(ctx, results, f, out_dec)?;
    } else if quiet {
        for i in issues(results) {
            eprintln!(
                "{}",
                fmt(
                    &format!(
                        "  {} {}: {}",
                        symbol(i.status, ctx.dry_run, out_dec),
                        i.rel,
                        description(i)
                    ),
                    err_dec
                )
            );
        }
    } else {
        console::print_raw(&human(ctx, results))?;
        if ctx.github {
            print!("{}", annotations(ctx, results));
        }
    }
    console::print_raw(&fmt(&out, out_dec))?;
    std::io::Write::flush(&mut std::io::stdout())?;
    if let (Some(path), Some(f)) = (to_file, file_format.or(format))
        && let Err(e) = std::fs::write(
            path,
            console::raw(&reporter(
                ctx,
                results,
                f.parse().map_err(unsupported)?,
                out_dec,
            )?),
        )
    {
        // PHP's warning, rendered as an uncaught ErrorException. Rust adds " (os error N)" after the same strerror.
        let e = e.to_string();
        let e = e.split(" (os error").next().unwrap_or(&e);
        let msg = format!(
            "file_put_contents({}): Failed to open stream: {e}",
            path.display(),
        );
        let mut abort = Abort::pint(msg, None);
        abort.thrown = pint_error::Thrown::ReportNotWritable;
        abort.error = RunError {
            path: None,
            line: None,
            message: format!("can't write the report to {}: {e}", path.display()),
            hint: None,
        };
        return Err(abort.into());
    }
    if format.is_some() {
        for r in errors(results) {
            eprintln!(
                "{}",
                fmt(
                    &format!("  <error>ERROR</error> {}: {}", r.path.display(), r.message),
                    err_dec
                )
            );
        }
    }
    Ok(())
}

/// GitHub annotations (output.md Build 4): under `--test` (and `pun lint`/`pun check` without `--fix`) each file
/// that needs changes, at its first changed line, titled with the rule IDs; syntax errors at their place; a fixer's
/// failure at the file. Fixed files print nothing, nothing is left to do in CI.
fn annotations(ctx: &Ctx, results: &[FileResult]) -> String {
    let file = |f: &FileResult| {
        let ids: Vec<String> = (f.applied.iter())
            .map(|n| pun_config::rule_of(n).unwrap_or(n).to_string())
            .collect();
        let lint: Vec<&str> = (ids.iter().map(String::as_str))
            .filter(|id| !id.starts_with("format/"))
            .collect();
        let message = match lint.is_empty() {
            true => "File needs formatting".to_string(),
            false => format!("{} can fix this file", lint.join(", ")),
        };
        github::file(&f.rel, f.line, &ids, &message)
    };
    let mut lines: Vec<(&str, String)> = vec![];
    for f in results {
        let line = match f.status {
            Status::Fixed if ctx.dry_run => file(f),
            Status::Invalid => {
                let (problem, _, index) = syntax_error(f);
                github::problem(&problem, &index)
            }
            Status::Exception | Status::Lint => github::file(
                &f.rel,
                error_line(&f.message).max(1),
                &["fixer-error".into()],
                &f.message,
            ),
            _ => continue,
        };
        lines.push((&f.rel, line));
    }
    lines.extend(ctx.remaining.iter().map(|f| (f.rel.as_str(), file(f))));
    for f in results.iter().filter(|f| !f.problems.is_empty()) {
        let index = LineIndex::new(&f.source);
        lines.extend((f.problems.iter()).map(|p| (f.rel.as_str(), github::problem(p, &index))));
    }
    // A stable sort: each file's annotations keep their order.
    lines.sort_by(|a, b| a.0.cmp(b.0));
    lines.into_iter().map(|l| l.1).collect()
}

/// Pun's own output: one file line per changed file (`-v`: and per unchanged one, dim, and each change's diff),
/// the files that failed, then the summary. `pun format` words its summary as format.md §9 does; `pun lint` and
/// `pun check` count each lint fix as an error (fixable without `--fix`) and each file needing format rules as
/// needing formatting (output.md §1).
fn human(ctx: &Ctx, results: &[FileResult]) -> String {
    let r = Renderer::new(ctx.look, ctx.root);
    let cmd = crate::agent::cmd();
    let ids = |f: &FileResult| -> Vec<String> {
        (f.applied.iter())
            .map(|n| pun_config::rule_of(n).unwrap_or(n).to_string())
            .collect()
    };
    let changed = if ctx.dry_run {
        FileStatus::NeedsFix
    } else {
        FileStatus::Fixed
    };
    let mut lines: Vec<(FileLine, &str)> = vec![];
    for f in results {
        let status = match f.status {
            Status::Fixed => changed,
            Status::NoChanges | Status::Skipped if ctx.verbose => FileStatus::Unchanged,
            _ => continue,
        };
        lines.push((
            FileLine {
                path: f.rel.clone().into(),
                status,
                rules: ids(f),
            },
            &f.diff,
        ));
    }
    // `--fix`: the fixes it doesn't apply, after the file's own line.
    for f in ctx.remaining {
        let line = FileLine {
            path: f.rel.clone().into(),
            status: FileStatus::NeedsFix,
            rules: ids(f),
        };
        lines.push((line, &f.diff));
    }
    lines.sort_by(|a, b| a.0.path.cmp(&b.0.path));
    let (lines, diffs): (Vec<FileLine>, Vec<&str>) = lines.into_iter().unzip();
    let mut out = String::new();
    for (line, diff) in r.file_lines(&lines).into_iter().zip(diffs) {
        out.push_str(&line);
        if ctx.verbose && !diff.is_empty() {
            out.push_str(&r.diff(diff));
        }
    }
    let mut failed: Vec<&FileResult> = results.iter().filter(|f| f.is_error()).collect();
    failed.sort_by(|a, b| a.rel.cmp(&b.rel));
    let syntax: Vec<(Problem, String, LineIndex)> = (failed.iter())
        .filter(|f| f.status == Status::Invalid)
        .map(|f| syntax_error(f))
        .collect();
    let mut blocks: Vec<String> = vec![];
    // The lint and analyse problems, by file and offset, then the syntax errors.
    let mut with_problems: Vec<&FileResult> = (results.iter())
        .filter(|f| !f.problems.is_empty())
        .collect();
    with_problems.sort_by(|a, b| a.rel.cmp(&b.rel));
    let indexes: Vec<LineIndex> = with_problems
        .iter()
        .map(|f| LineIndex::new(&f.source))
        .collect();
    let mut r = r;
    r.concise = ctx.concise;
    let problems: Vec<(&Problem, &str, &LineIndex)> = (with_problems.iter().zip(&indexes))
        .flat_map(|(f, i)| f.problems.iter().map(move |p| (p, f.source.as_str(), i)))
        .chain(syntax.iter().map(|(p, s, i)| (p, s.as_str(), i)))
        .collect();
    if !problems.is_empty() {
        blocks.push(r.problems(problems));
    }
    blocks.extend(
        (failed.iter())
            .filter(|f| f.status != Status::Invalid)
            .map(|f| fixer_error(&r, f)),
    );
    if !out.is_empty() && !blocks.is_empty() {
        out.push('\n');
    }
    out.push_str(&blocks.join("\n"));
    let formatted = results.iter().filter(|f| f.status == Status::Fixed).count();
    if cmd == "format" {
        let unchanged = (results.iter())
            .filter(|f| matches!(f.status, Status::NoChanges | Status::Skipped))
            .count();
        out.push_str(&r.format_summary(&FormatSummary {
            formatted,
            unchanged,
            errors: failed.len(),
            files: ctx.total,
            test: ctx.dry_run,
            elapsed: ctx.elapsed,
        }));
        return out;
    }
    if ctx.unmatched_baseline > 0 {
        out.push_str(&crate::analyse::baseline_note(&r, ctx.unmatched_baseline));
    }
    let c = counts(ctx, results);
    let fix_command = format!("pun {cmd} --fix");
    out.push_str(&r.summary(&Summary {
        errors: c.errors,
        warnings: c.warnings,
        fixable: c.fixable,
        fix_command: &fix_command,
        need_formatting: c.need_formatting,
        files: ctx.total,
        elapsed: ctx.elapsed,
    }));
    out
}

/// A syntax error in the input, as a problem with a code frame: at the line PHP names, its first non-blank column
/// (the last line when PHP names the line after it).
fn syntax_error(f: &FileResult) -> (Problem, String, LineIndex) {
    let source = String::from_utf8_lossy(&std::fs::read(&f.path).unwrap_or_default()).into_owned();
    let index = LineIndex::new(&source);
    let last = index.line_col(source.len()).line;
    let range = index
        .line_range(error_line(&f.message).clamp(1, last))
        .expect("a line of the file");
    let line = &source[range.clone()];
    let at = range.start + line.len() - line.trim_start().len();
    let message = f
        .message
        .strip_prefix("Parse error: ")
        .unwrap_or(&f.message);
    let message = message.rsplit_once(" on line ").map_or(message, |m| m.0);
    let problem = Problem {
        rule: "syntax-error".into(),
        path: f.rel.clone().into(),
        span: Span::new(at, at),
        message: message.to_string(),
        help: None,
        level: Level::Error,
        fix: None,
    };
    (problem, source, index)
}

/// Invalid code from a fixer, or a fixer's exception: a run error naming the file and the message, with a hint.
fn fixer_error(r: &Renderer, f: &FileResult) -> String {
    r.run_error(&RunError {
        path: Some(f.rel.clone().into()),
        line: None,
        message: f.message.clone(),
        hint: Some(
            "a rule failed on this file; please report it at https://github.com/punphp/pun/issues"
                .into(),
        ),
    })
}

/// Agent mode: the summary line, then per file its format rule IDs on one line and each lint rule on its own line
/// (`[fix:safe|unsafe]`, the rule's summary), errors as `path:line rule message`; `-v` adds each file's diff.
fn agent(ctx: &Ctx, results: &[FileResult]) -> String {
    use crate::agent::count;
    // `--fix`'s remaining fixes after each file's own lines, tagged `[remaining]`.
    let mut sorted: Vec<(&FileResult, bool)> = (results.iter().map(|r| (r, false)))
        .chain(ctx.remaining.iter().map(|r| (r, true)))
        .collect();
    sorted.sort_by(|a, b| (&a.0.rel, a.1).cmp(&(&b.0.rel, b.1)));
    let mut items = vec![];
    let (mut fixed, mut errors, mut left) = (0, 0, 0);
    let mut warnings = 0;
    for (r, remaining) in sorted {
        if !remaining && !r.problems.is_empty() {
            let index = LineIndex::new(&r.source);
            for p in &r.problems {
                let at = index.line_col(p.span.start);
                let fix = match p.fix.as_ref().map(|f| f.safety) {
                    Some(pun_problem::Safety::Safe) => " [fix:safe]",
                    Some(pun_problem::Safety::Unsafe) => " [fix:unsafe]",
                    None => "",
                };
                match p.level {
                    Level::Error => errors += 1,
                    Level::Warning => warnings += 1,
                }
                let level = if p.level == Level::Warning {
                    " [warn]"
                } else {
                    ""
                };
                items.push(format!(
                    "{}:{}:{} {}{fix}{level} {}",
                    r.rel, at.line, at.column, p.rule, p.message
                ));
            }
        }
        if r.is_error() {
            errors += 1;
            // A fixer's exception or broken output is TYPE_LINT / TYPE_EXCEPTION, bad input TYPE_INVALID.
            let rule = if r.status == Status::Invalid {
                "syntax-error"
            } else {
                "fixer-error"
            };
            let at = match error_line(&r.message) {
                0 => String::new(),
                n => format!(":{n}"),
            };
            items.push(format!("{}{at} {rule} {}", r.rel, r.message));
        } else if r.status == Status::Fixed {
            if remaining {
                left += 1;
            } else {
                fixed += 1;
            }
            let tag = if remaining { " [remaining]" } else { "" };
            let ids: Vec<&str> = (r.applied.iter())
                .map(|f| pun_config::rule_of(f).unwrap_or(f))
                .collect();
            let (format, lint): (Vec<&str>, Vec<&str>) =
                ids.into_iter().partition(|id| id.starts_with("format/"));
            let mut item = String::new();
            if !format.is_empty() {
                let _ = writeln!(item, "{} {}", r.rel, format.join(", "));
            }
            for id in lint {
                let (fix, summary) = lint_rule(id);
                let _ = writeln!(item, "{} {id} [fix:{fix}]{tag}{summary}", r.rel);
            }
            item.pop();
            if ctx.verbose {
                item.push('\n');
                item.push_str(r.diff.trim_end_matches('\n'));
            }
            items.push(item);
        }
    }
    let cmd = crate::agent::cmd();
    let mut detail = vec![];
    if errors > 0 {
        detail.push(count(errors, "error"));
    }
    if fixed > 0 {
        let files = count(fixed, "file");
        detail.push(match (cmd, ctx.dry_run) {
            ("format", true) => format!(
                "{files} need{} formatting",
                if fixed == 1 { "s" } else { "" }
            ),
            ("format", false) => format!("{files} formatted"),
            (_, true) => format!("{files} with fixes"),
            (_, false) => format!("{files} fixed"),
        });
    }
    if left > 0 {
        detail.push(format!("{} with remaining fixes", count(left, "file")));
    }
    if warnings > 0 {
        detail.push(count(warnings, "warning"));
    }
    let result = if errors > 0 || left > 0 || (fixed > 0 && ctx.dry_run) {
        "fail"
    } else if fixed > 0 {
        "fixed"
    } else {
        "pass"
    };
    let checked = format!("{} checked", ctx.total);
    crate::agent::render(result, &detail, &checked, &items, ctx.verbose)
}

/// A lint rule's fix safety (`mixed` counts as unsafe) and ` summary` (empty without one).
fn lint_rule(id: &str) -> (&'static str, String) {
    let m = pun_config::rule(id);
    let fix = if m.as_ref().is_some_and(|m| m.fix == "safe") {
        "safe"
    } else {
        "unsafe"
    };
    let summary = m.and_then(|m| m.summary);
    (fix, summary.map_or(String::new(), |s| format!(" {s}")))
}

/// InteractsWithSymbols::getSymbol for `-q`'s issues (changes and errors): markup when stdout is decorated, the
/// bare symbol otherwise.
fn symbol(s: Status, testing: bool, decorated: bool) -> String {
    let (sym, style) = match s {
        Status::Fixed if testing => ("⨯", "options=bold;fg=red"),
        Status::Fixed => ("✓", "options=bold;fg=green"),
        _ => ("!", "options=bold;fg=red"),
    };
    if decorated {
        format!("<{style}>{sym}</>")
    } else {
        sym.to_string()
    }
}

fn description(r: &FileResult) -> String {
    if r.is_error() {
        r.message.clone()
    } else {
        r.applied.join(", ")
    }
}

/// ErrorsManager: invalid errors, then lint errors (each in processing order).
fn errors(results: &[FileResult]) -> impl Iterator<Item = &FileResult> {
    let of = move |s| results.iter().filter(move |r| r.status == s);
    of(Status::Invalid)
        .chain(of(Status::Exception))
        .chain(of(Status::Lint))
}

/// SummaryOutput::getIssues: changes, then `invalid + exception + lint` (a PHP array union: each list only adds
/// the entries at indexes past the lists before it), sorted by Issue property order. Errors keep their absolute
/// path in the compared field, so they sort before relative paths.
fn issues(results: &[FileResult]) -> Vec<&FileResult> {
    let of = |s| results.iter().filter(move |r: &&FileResult| r.status == s);
    let mut v: Vec<&FileResult> = of(Status::Fixed).collect();
    let mut taken = 0;
    for s in [Status::Invalid, Status::Exception, Status::Lint] {
        let n = of(s).count();
        v.extend(of(s).skip(taken));
        taken = taken.max(n);
    }
    let key = |r: &FileResult| {
        if r.is_error() {
            r.path.to_string_lossy().into_owned()
        } else {
            r.rel.clone()
        }
    };
    v.sort_by(|a, b| key(a).as_bytes().cmp(key(b).as_bytes()));
    v
}

/// A `--format` or `--output-format` value. Pint takes any string and aborts only when it reports, after the run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ReportFormat {
    Json,
    Txt,
    Agent,
    Checkstyle,
    Junit,
    Xml,
    Gitlab,
    Github,
}

impl std::str::FromStr for ReportFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "json" => Self::Json,
            "txt" => Self::Txt,
            "agent" => Self::Agent,
            "checkstyle" => Self::Checkstyle,
            "junit" => Self::Junit,
            "xml" => Self::Xml,
            "gitlab" => Self::Gitlab,
            "github" => Self::Github,
            _ => bail!("Format [{s}] is not supported."),
        })
    }
}

fn reporter(
    ctx: &Ctx,
    results: &[FileResult],
    format: ReportFormat,
    decorated: bool,
) -> Result<String> {
    // A file's lint and analyse problems list with its fixers, by rule ID, once each.
    let fixers: Vec<Vec<&str>> = (results.iter())
        .map(|r| {
            let mut all: Vec<&str> = r.applied.to_vec();
            for p in &r.problems {
                if !all.contains(&p.rule.as_str()) {
                    all.push(&p.rule);
                }
            }
            all
        })
        .collect();
    let changes: Vec<Change> = (results.iter().zip(&fixers))
        .filter(|(r, _)| r.status == Status::Fixed || !r.problems.is_empty())
        .map(|(r, fixers)| Change {
            file: &r.rel,
            fixers,
            diff: if ctx.verbose { &r.diff } else { "" },
        })
        .collect();
    Ok(match format {
        ReportFormat::Json if changes.iter().any(|c| console::carries_bytes(c.diff)) => {
            // json_encode under JSON_THROW_ON_ERROR, on a diff of a file that isn't UTF-8.
            let mut abort = Abort::pint(
                "Malformed UTF-8 characters, possibly incorrectly encoded",
                None,
            );
            abort.thrown = pint_error::Thrown::JsonMalformed;
            return Err(abort.into());
        }
        ReportFormat::Json => fix_report::json(&changes, ctx.verbose, decorated),
        ReportFormat::Txt => {
            fix_report::txt(&changes, ctx.verbose, ctx.dry_run, ctx.total, decorated)
        }
        ReportFormat::Agent => {
            let errors: Vec<(&str, &str)> = (errors(results))
                .map(|r| (r.rel.as_str(), r.message.as_str()))
                .collect();
            fix_report::agent(&changes, &errors, ctx.dry_run)
        }
        ReportFormat::Checkstyle => fix_report::checkstyle(&changes, decorated),
        ReportFormat::Junit => fix_report::junit(&changes, ctx.verbose, decorated),
        ReportFormat::Xml => fix_report::xml_report(&changes, ctx.verbose, decorated),
        ReportFormat::Gitlab => fix_report::gitlab(&changes, decorated),
        ReportFormat::Github => annotations(ctx, results),
    })
}

/// PHP-CS-Fixer UnifiedDiffer (sebastian/diff strict unified, 3 lines of context, real path in headers).
pub(crate) fn diff(old: &str, new: &str, path: &Path) -> String {
    diff_bytes(old.as_bytes(), new.as_bytes(), path)
}

/// `diff` of a file's bytes: those that aren't UTF-8 are carried to the output as they are (`console::carry`).
pub(crate) fn diff_bytes(old: &[u8], new: &[u8], path: &Path) -> String {
    let mut p = path.to_string_lossy().into_owned();
    if p.contains(char::is_whitespace) {
        p = format!("\"{p}\"");
    }
    console::carry(&crate::php_diff::unified(old, new, &p))
}
