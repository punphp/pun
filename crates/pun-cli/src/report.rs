//! Pint's output: progress dots, Termwind summary, `-v` diffs and error snippets, `-q`, and every `--format`
//! reporter. Everything is written as Symfony formatter markup and rendered by `console::format`, so colours
//! (`--ansi`, TTY) and their absence come from one code path.

use crate::console::{self, escape, format as fmt};
use crate::fix_report::{self, Change};
use crate::{highlight, pint_error};
use anyhow::{Result, bail};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
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

pub struct FileResult {
    pub path: PathBuf,
    pub rel: String,
    pub status: Status,
    pub applied: Vec<&'static str>,
    pub diff: String,
    pub message: String,
}

impl FileResult {
    pub fn is_error(&self) -> bool {
        matches!(
            self.status,
            Status::Invalid | Status::Exception | Status::Lint
        )
    }
}

pub struct Ctx<'a> {
    pub preset: &'a str,
    pub total: usize,
    pub dry_run: bool,
    /// `--fix`: the fixes of the lint rules it doesn't apply, one result per file (status Fixed).
    pub remaining: &'a [FileResult],
    pub verbose: bool,
    pub ansi: bool,
    pub no_ansi: bool,
}

pub const ABOUT: &str =
    "PHP CS Fixer 3.95.17 Adalbertus by Fabien Potencier, Dariusz Ruminski and contributors.";

/// Symfony's doRenderThrowable box (without the blank line before it), on stdout; `title` is `-v`'s exception class line.
pub fn abort_box(title: Option<&str>, message: &str, decorated: bool) -> String {
    let width = match console::width() {
        0 => usize::MAX,
        w => w - 1,
    };
    let chunk = width.saturating_sub(4).max(1);
    let mut lines: Vec<String> = vec![];
    for line in message
        .split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
    {
        // splitStringByWidth: every piece is padded to the chunk width once a line needs splitting.
        let chars: Vec<char> = line.chars().collect();
        if chars.len() <= chunk {
            lines.push(line.to_string());
        } else {
            lines.extend(
                chars
                    .chunks(chunk)
                    .map(|c| format!("{:chunk$}", c.iter().collect::<String>())),
            );
        }
    }
    let title_len = title.map_or(0, |t| t.chars().count());
    let len = lines
        .iter()
        .map(|l| l.chars().count() + 4)
        .max()
        .unwrap_or(0)
        .max(title_len);
    let empty = format!("<error>{}</error>", " ".repeat(len));
    let mut o = format!("{empty}\n");
    if let Some(t) = title {
        let _ = writeln!(o, "<error>{t}{}</error>", " ".repeat(len - title_len));
    }
    for l in &lines {
        let _ = writeln!(
            o,
            "<error>  {}  {}</error>",
            escape(l),
            " ".repeat(len - l.chars().count() - 4)
        );
    }
    let _ = write!(o, "{empty}\n\n");
    fmt(&o, decorated)
}

/// EnsurePrettierIsConfigured, run before anything else when pint.json sets `"Pint/laravel_blade": true` (or
/// `--blade`): Node must exist and prettier, prettier-plugin-blade and prettier-plugin-tailwindcss must resolve
/// from the project at the required versions. Returns Pint's output when it would abort.
/// ponytail: Pint offers to install missing packages when interactive; Pun always takes the non-interactive "no".
pub fn ensure_prettier(root: &Path, enabled: bool) -> Option<String> {
    use std::process::{Command, Stdio};
    if !enabled {
        return None;
    }
    const REQUIRED: [(&str, &str); 3] = [
        ("prettier", "^3.8.4"),
        ("prettier-plugin-blade", "^3.2.2"),
        ("prettier-plugin-tailwindcss", "^0.8.0"),
    ];
    // Pint's `node -v` plus resources/js/version-probe.cjs per package, in one Node start: a JSON list with, per
    // package, null when it doesn't resolve, else its version ("" when no manifest names it).
    const PROBE: &str = r#"const path = require('path'); const fs = require('fs');
const probe = (pkg) => { let entry;
  try { entry = require.resolve(pkg, { paths: [process.cwd()] }); } catch (e) { return null; }
  let dir = path.dirname(entry);
  while (dir !== path.dirname(dir)) { const manifest = path.join(dir, 'package.json');
    if (fs.existsSync(manifest)) { try { const json = JSON.parse(fs.readFileSync(manifest, 'utf8'));
      if (json.name === pkg && json.version) { return String(json.version).trim(); } } catch (e) {} }
    dir = path.dirname(dir); }
  return ''; };
process.stdout.write(JSON.stringify(process.argv.slice(1).map(probe)));"#;
    let out = Command::new("node")
        .args(["-e", PROBE])
        .args(REQUIRED.map(|r| r.0))
        .current_dir(root)
        .stderr(Stdio::null())
        .output();
    let probes = out
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| serde_json::from_slice::<Vec<Option<String>>>(&o.stdout).ok());
    let Some(probes) = probes else {
        return Some(pint_error::render(
            "The rules enabled in your pint configuration require Node.js to be installed.",
        ));
    };
    let manager = if root.join("bun.lock").exists() || root.join("bun.lockb").exists() {
        ("bun", "add -d")
    } else if root.join("pnpm-lock.yaml").exists() {
        ("pnpm", "add -D")
    } else if root.join("yarn.lock").exists() {
        ("yarn", "add -D")
    } else {
        ("npm", "install -D")
    };
    let mut missing = vec![];
    let mut outdated = vec![];
    for ((pkg, constraint), probe) in REQUIRED.into_iter().zip(probes) {
        match probe {
            None => missing.push(pkg),
            Some(v) if !v.is_empty() && !caret_satisfies(&v, &constraint[1..]) => {
                outdated.push((pkg, v, constraint))
            }
            Some(_) => {}
        }
    }
    if !missing.is_empty() {
        let warning = if crate::agent::enabled() {
            String::new()
        } else {
            format!(
                "\n\x1b[33m The rules enabled in your pint configuration require the following prettier dependencies to be installed using [{}]: {}.\x1b[39m\n\n",
                manager.0,
                missing.join(", ")
            )
        };
        let msg = format!(
            "The rules enabled in your pint configuration require the following prettier dependencies to be installed: {}.",
            missing.join(", ")
        );
        return Some(warning + &pint_error::render(&msg));
    }
    if !outdated.is_empty() {
        let list: Vec<String> = outdated
            .iter()
            .map(|(p, v, c)| format!("  - {p} (installed: {v}, required: {c})"))
            .collect();
        let specs: Vec<String> = outdated
            .iter()
            .map(|(p, _, c)| format!("{p}@{c}"))
            .collect();
        let msg = format!(
            "The following prettier dependencies do not satisfy the versions required by your pint configuration:\n{}\n\nUpdate them using [{}]: {} {} {}",
            list.join("\n"),
            manager.0,
            manager.0,
            manager.1,
            specs.join(" ")
        );
        return Some(pint_error::render(&msg));
    }
    None
}

/// Composer Semver `^x.y.z` for plain `a.b.c` versions.
/// ponytail: pre-release/build suffixes are ignored (Composer treats them by stability); real installs are tagged releases.
fn caret_satisfies(version: &str, base: &str) -> bool {
    let parse = |s: &str| -> Vec<u64> {
        s.trim_start_matches('v')
            .split(['.', '-', '+'])
            .take(3)
            .map(|p| p.parse().unwrap_or(0))
            .collect()
    };
    let (v, b) = (parse(version), parse(base));
    let first = b.iter().position(|&n| n != 0).unwrap_or(2);
    v >= b && v[..=first] == b[..=first]
}

/// The `on line N.` at the end of a lint message (the exception's line, used for the `-v` snippet).
fn error_line(message: &str) -> usize {
    message
        .rsplit(" on line ")
        .next()
        .and_then(|n| n.trim_end_matches('.').parse().ok())
        .unwrap_or(0)
}

/// ElaborateSummary::execute (output part). Errors (bad `--format`) are Pint aborts.
pub fn emit(
    ctx: &Ctx,
    results: &[FileResult],
    format: Option<&str>,
    quiet: bool,
    to_file: Option<&Path>,
    file_format: Option<&str>,
) -> u8 {
    let out_dec = console::decorated(1, ctx.ansi, ctx.no_ansi);
    match emit_inner(ctx, results, format, quiet, to_file, file_format, out_dec) {
        Ok(code) => code,
        Err(e) => {
            print!("{}", pint_error::render(&e.to_string()));
            1
        }
    }
}

fn emit_inner(
    ctx: &Ctx,
    results: &[FileResult],
    format: Option<&str>,
    quiet: bool,
    to_file: Option<&Path>,
    file_format: Option<&str>,
    out_dec: bool,
) -> Result<u8> {
    let err_dec = console::decorated(2, ctx.ansi, ctx.no_ansi);
    let mut out = String::new();
    if crate::agent::enabled() && format.is_none() {
        out = agent(ctx, results);
    } else if let Some(f) = format {
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
        out = summary(ctx, results, out_dec);
    }
    print!("{}", fmt(&out, out_dec));
    std::io::Write::flush(&mut std::io::stdout())?;
    if let (Some(path), Some(f)) = (to_file, file_format.or(format))
        && let Err(e) = std::fs::write(path, reporter(ctx, results, f, out_dec)?)
    {
        // PHP's warning, rendered as an uncaught ErrorException. Rust adds " (os error N)" after the same strerror.
        let e = e.to_string();
        let msg = format!(
            "file_put_contents({}): Failed to open stream: {}",
            path.display(),
            e.split(" (os error").next().unwrap_or(&e)
        );
        print!("{}", pint_error::render(&msg));
        return Ok(1);
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
    Ok(0)
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
    for (r, remaining) in sorted {
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
                ids.into_iter().partition(|id| id.starts_with("format."));
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

/// InteractsWithSymbols::getSymbol: markup when stdout is decorated, the bare symbol otherwise.
fn symbol(s: Status, testing: bool, decorated: bool) -> String {
    let (sym, style) = match s {
        Status::Fixed if testing => ("⨯", "options=bold;fg=red"),
        Status::Fixed => ("✓", "options=bold;fg=green"),
        Status::Invalid | Status::Exception | Status::Lint => ("!", "options=bold;fg=red"),
        _ => (".", "fg=gray"),
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

/// ProgressOutput: one symbol per processed file, `width - 4` per line.
pub fn progress<'a>(
    results: impl Iterator<Item = &'a FileResult>,
    width: usize,
    dry_run: bool,
    decorated: bool,
) -> String {
    let per_line = width.saturating_sub(4).max(1);
    let mut o = String::new();
    for (n, r) in results.filter(|r| r.status != Status::Link).enumerate() {
        if n % per_line == 0 {
            o.push_str("\n  ");
        }
        o.push_str(&symbol(r.status, dry_run, decorated));
    }
    o
}

fn summary(ctx: &Ctx, results: &[FileResult], decorated: bool) -> String {
    let w = console::width();
    let inner = w.saturating_sub(4);
    let mut o = progress(results.iter(), w, ctx.dry_run, decorated);

    let issues = issues(results);
    let errors = issues.iter().filter(|i| i.is_error()).count();
    let fixable = issues.len() - errors;
    let label = match ctx.preset {
        "per" => "PER",
        "psr12" => "PSR 12",
        "laravel" => "Laravel",
        "symfony" => "Symfony",
        // pun-config only resolves the five presets.
        _ => "Empty",
    };
    let _ = writeln!(
        o,
        "\n\n  <fg=gray>{}</> <fg=gray>{label}</>  ",
        "─".repeat(inner.saturating_sub(1 + label.chars().count()))
    );

    let left = ctx.remaining.len();
    let (status, style) = if issues.is_empty() && left == 0 {
        ("  PASS  ", "bg=green;fg=gray")
    } else if errors == 0 && left == 0 && !ctx.dry_run {
        ("  FIXED  ", "bg=green;fg=gray")
    } else {
        ("  FAIL  ", "bg=red;fg=white")
    };
    let plural = |n: usize, s: &str| {
        if n == 1 {
            s.to_string()
        } else {
            format!("{s}s")
        }
    };
    let mut right = format!("{} {}", ctx.total, plural(ctx.total, "file"));
    if errors > 0 {
        let _ = write!(right, ", {errors} {}", plural(errors, "error"));
    }
    if fixable > 0 {
        let _ = write!(
            right,
            ", {fixable} style {}{}",
            plural(fixable, "issue"),
            if ctx.dry_run { "" } else { " fixed" }
        );
    }
    if left > 0 {
        let _ = write!(right, ", {left} style {} remaining", plural(left, "issue"));
    }
    let dots = inner.saturating_sub(status.len() + 2 + right.chars().count());
    let _ = writeln!(
        o,
        "  <{style};options=bold>{status}</> <fg=gray>{}</> {right}  ",
        ".".repeat(dots)
    );

    let colors = console::collision_colors();
    // `--fix`'s remaining fixes are listed after the rest, as `--test` lists them.
    let mut remaining: Vec<&FileResult> = ctx.remaining.iter().collect();
    remaining.sort_by(|a, b| a.rel.cmp(&b.rel));
    let listed =
        (issues.iter().map(|i| (*i, ctx.dry_run))).chain(remaining.into_iter().map(|i| (i, true)));
    for (i, testing) in listed {
        let left = format!("{} {}", symbol(i.status, testing, decorated), i.rel);
        let left_len = 2 + tw_len(&i.rel);
        let room = inner.saturating_sub(left_len + 1);
        let mut desc = description(i);
        let mut pad = room;
        // A path wider than the box leaves no room (width 0): Termwind prints the description whole.
        // ponytail: lengths are chars, Termwind's are mb_strwidth (East Asian wide = 2); messages are ASCII.
        if room > 0 {
            let cut =
                |s: &str, n: usize| s.chars().take(n).collect::<String>().trim_end().to_string();
            // Styles::truncate (not with -v): anything over room-1 is cut to room-1, right-trimmed, plus "…".
            if !ctx.verbose && tw_len(&desc) > room - 1 {
                desc = cut(&desc, room - 1) + "…";
            }
            // Styles::applyWidth: pad to the box, or clip (right-trimmed, not padded back) when still too wide.
            if tw_len(&desc) > room {
                desc = cut(&desc, room);
                pad = 0;
            }
        }
        let _ = writeln!(
            o,
            "  {left} <fg=gray>{}{desc}</>  ",
            " ".repeat(pad.saturating_sub(tw_len(&desc)))
        );
        if ctx.verbose {
            // Issue::code() highlights the exception's line, for a fixer's exception a line of PHP-CS-Fixer's source.
            // ponytail: taken as past the end of the file (no snippet); files longer than that line would show one.
            let code = if i.status == Status::Exception {
                String::new()
            } else if i.is_error() {
                // Issue::code(): the file as it is on disk, around the exception's line.
                let content = std::fs::read(&i.path).unwrap_or_default();
                highlight::snippet(
                    &String::from_utf8_lossy(&content),
                    error_line(&i.message),
                    colors,
                )
            } else {
                highlight::diff(&i.diff, colors)
            };
            if !code.is_empty() {
                let _ = writeln!(o, "{code}");
            }
        }
    }
    o.push('\n');
    o
}

/// Termwind's getLength: width without anything STYLING_REGEX takes for a tag (`<x>`, `</a>`, `\e[1m`).
/// ponytail: Termwind's trimText puts such tags back after cutting; we cut them like text (paths with `<…>`).
fn tw_len(s: &str) -> usize {
    static RE: std::sync::LazyLock<pcre2::bytes::Regex> = std::sync::LazyLock::new(|| {
        pcre2::bytes::Regex::new(r"<[\w=#/;,:.&,%?-]+>|\e\[\d+m").unwrap()
    });
    let stripped: usize = RE
        .find_iter(s.as_bytes())
        .flatten()
        .map(|m| String::from_utf8_lossy(m.as_bytes()).chars().count())
        .sum();
    s.chars().count() - stripped
}

/// DiffConsoleFormatter with TextReporter's begin/end template; lines split on PCRE's `\R`.
fn console_diff(diff: &str, decorated: bool) -> String {
    let mut lines = vec![];
    let mut rest = diff;
    while let Some(i) = rest.find(['\r', '\n', '\x0b', '\x0c', '\u{85}', '\u{2028}', '\u{2029}']) {
        lines.push(&rest[..i]);
        let sep = if rest[i..].starts_with("\r\n") {
            2
        } else {
            rest[i..].chars().next().unwrap().len_utf8()
        };
        rest = &rest[i + sep..];
    }
    lines.push(rest);
    let body: Vec<String> = lines
        .iter()
        .map(|l| {
            if !decorated {
                return l.to_string();
            }
            match l.as_bytes().first() {
                Some(b'+') => format!("<fg=green>{}</fg=green>", escape(l)),
                Some(b'-') => format!("<fg=red>{}</fg=red>", escape(l)),
                Some(b'@') => format!("<fg=cyan>{}</fg=cyan>", escape(l)),
                _ => escape(l),
            }
        })
        .collect();
    let (open, close) = if decorated {
        ("<comment>", "</comment>")
    } else {
        ("", "")
    };
    format!(
        "{open}      ---------- begin diff ----------{close}\n{}\n{open}      ----------- end diff -----------{close}",
        body.join("\n")
    )
}

fn reporter(ctx: &Ctx, results: &[FileResult], format: &str, decorated: bool) -> Result<String> {
    let changed: Vec<&FileResult> = results
        .iter()
        .filter(|r| r.status == Status::Fixed)
        .collect();
    let changes: Vec<Change> = changed
        .iter()
        .map(|r| Change {
            file: &r.rel,
            fixers: &r.applied,
            diff: if ctx.verbose { &r.diff } else { "" },
        })
        .collect();
    let escaped = |s: String| if decorated { escape(&s) } else { s };
    Ok(match format {
        "json" => {
            let files: Vec<serde_json::Value> = changes
                .iter()
                .map(|c| {
                    let mut f = serde_json::json!({ "name": c.file });
                    if ctx.verbose {
                        f["appliedFixers"] = serde_json::json!(c.fixers);
                    }
                    if !c.diff.is_empty() {
                        f["diff"] = serde_json::json!(c.diff);
                    }
                    f
                })
                .collect();
            escaped(php_json(
                &serde_json::json!({ "about": ABOUT, "files": files, "time": { "total": 0 }, "memory": 0 }),
            ))
        }
        "txt" => {
            let mut o = String::new();
            for (n, c) in changes.iter().enumerate() {
                let _ = write!(o, "{:4}) {}", n + 1, c.file);
                if ctx.verbose {
                    let list = c.fixers.join(", ");
                    let _ = write!(
                        o,
                        "{}",
                        if decorated {
                            format!(" (<comment>{list}</comment>)")
                        } else {
                            format!(" ({list})")
                        }
                    );
                }
                if !c.diff.is_empty() {
                    let _ = write!(o, "\n{}\n", console_diff(c.diff, decorated));
                }
                o.push('\n');
            }
            let (verb, what) = if ctx.dry_run {
                ("Found", "files that can be fixed")
            } else {
                ("Fixed", "files")
            };
            let _ = writeln!(
                o,
                "\n{verb} {} of {} {what} in 0.000 seconds, 0.00 MB memory used",
                changes.len(),
                ctx.total
            );
            o
        }
        "agent" => {
            let errors: Vec<&FileResult> = errors(results).collect();
            let result = if !errors.is_empty() {
                "fail"
            } else if changed.is_empty() {
                "passed"
            } else if ctx.dry_run {
                "fail"
            } else {
                "fixed"
            };
            let mut v = serde_json::json!({ "tool": "pint", "result": result });
            if !changed.is_empty() {
                v["files"] = changed
                    .iter()
                    .map(|r| serde_json::json!({ "path": r.rel, "fixers": r.applied }))
                    .collect();
            }
            if !errors.is_empty() {
                v["errors"] = errors
                    .iter()
                    .map(|r| serde_json::json!({ "path": r.rel, "message": r.message }))
                    .collect();
            }
            php_json(&v)
        }
        "checkstyle" => fix_report::checkstyle(&changes, decorated),
        "junit" => fix_report::junit(&changes, ctx.verbose, decorated),
        "xml" => fix_report::xml_report(&changes, ctx.verbose, decorated),
        "gitlab" => fix_report::gitlab(&changes, decorated),
        _ => bail!("Format [{format}] is not supported."),
    })
}

/// json_encode() defaults: escape `/` and non-ASCII (as UTF-16 `\uXXXX`).
pub fn php_json(v: &serde_json::Value) -> String {
    let s = serde_json::to_string(v).expect("json");
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '/' => o.push_str("\\/"),
            c if c.is_ascii() => o.push(c),
            c => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    let _ = write!(o, "\\u{u:04x}");
                }
            }
        }
    }
    o
}

/// PHP-CS-Fixer UnifiedDiffer (sebastian/diff strict unified, 3 lines of context, real path in headers).
pub fn diff(old: &str, new: &str, path: &Path) -> String {
    let mut p = path.to_string_lossy().into_owned();
    if p.contains(char::is_whitespace) {
        p = format!("\"{p}\"");
    }
    String::from_utf8_lossy(&crate::php_diff::unified(
        old.as_bytes(),
        new.as_bytes(),
        &p,
    ))
    .into_owned()
}
