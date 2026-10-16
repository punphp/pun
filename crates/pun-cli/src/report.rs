//! Pint's output: progress dots, Termwind summary (undecorated), `-q`, and the json/txt/agent reporters.

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
    fn is_error(&self) -> bool {
        matches!(self.status, Status::Invalid | Status::Lint)
    }
}

pub struct Ctx<'a> {
    pub preset: &'a str,
    pub total: usize,
    pub dry_run: bool,
    pub verbose: bool,
}

const ABOUT: &str = "PHP CS Fixer 3.95.17 Adalbertus by Fabien Potencier, Dariusz Ruminski and contributors.";

/// laravel/agent-detector: Pint switches to its agent reporter under coding agents.
fn running_in_agent() -> bool {
    const VARS: [&str; 19] = [
        "CURSOR_AGENT", "GEMINI_CLI", "CODEX_SANDBOX", "CODEX_CI", "CODEX_THREAD_ID", "AUGMENT_AGENT", "OPENCODE_CLIENT",
        "OPENCODE", "AMP_CURRENT_THREAD_ID", "CLAUDECODE", "CLAUDE_CODE", "REPL_ID", "COPILOT_MODEL", "COPILOT_ALLOW_ALL",
        "COPILOT_GITHUB_TOKEN", "COPILOT_CLI", "ANTIGRAVITY_AGENT", "PI_CODING_AGENT", "KIRO_AGENT_PATH",
    ];
    std::env::var("AI_AGENT").is_ok_and(|v| !v.trim().is_empty())
        || VARS.iter().any(|v| std::env::var_os(v).is_some())
        || Path::new("/opt/.devin").exists()
}

fn width() -> usize {
    // ponytail: Symfony Terminal also asks stty; COLUMNS or 80 covers CI and pipes.
    std::env::var("COLUMNS").ok().and_then(|c| c.parse().ok()).unwrap_or(80)
}

/// ElaborateSummary::execute (output part). Returns 1 for an unsupported format.
pub fn emit(ctx: &Ctx, results: &[FileResult], format: Option<&str>, quiet: bool, to_file: Option<&Path>, file_format: Option<&str>) -> Result<u8> {
    let mut out = String::new();
    if running_in_agent() {
        out = reporter(ctx, results, "agent")?;
    } else if let Some(f) = format {
        out = reporter(ctx, results, f)?;
    } else if quiet {
        for i in issues(results) {
            eprintln!("  {} {}: {}", symbol(i.status, ctx.dry_run), i.rel, description(i));
        }
    } else {
        out = summary(ctx, results);
    }
    print!("{out}");
    std::io::Write::flush(&mut std::io::stdout())?;
    if let (Some(path), Some(f)) = (to_file, file_format.or(format)) {
        std::fs::write(path, reporter(ctx, results, f)?)?;
    }
    if format.is_some() {
        for r in results.iter().filter(|r| r.is_error()) {
            eprintln!("  ERROR {}: {}", r.path.display(), r.message);
        }
    }
    Ok(0)
}

fn symbol(s: Status, testing: bool) -> &'static str {
    match s {
        Status::Fixed if testing => "⨯",
        Status::Fixed => "✓",
        Status::Invalid | Status::Lint => "!",
        _ => ".",
    }
}

fn description(r: &FileResult) -> String {
    if r.is_error() { r.message.clone() } else { r.applied.join(", ") }
}

/// SummaryOutput::getIssues: changes then errors, sorted by Issue property order. Errors keep their absolute
/// path in the compared field, so they sort before relative paths.
fn issues(results: &[FileResult]) -> Vec<&FileResult> {
    let mut v: Vec<&FileResult> = results.iter().filter(|r| r.status == Status::Fixed || r.is_error()).collect();
    let key = |r: &FileResult| if r.is_error() { r.path.to_string_lossy().into_owned() } else { r.rel.clone() };
    v.sort_by(|a, b| key(a).as_bytes().cmp(key(b).as_bytes()));
    v
}

fn summary(ctx: &Ctx, results: &[FileResult]) -> String {
    let w = width();
    let inner = w.saturating_sub(4);
    let mut o = String::new();

    // ProgressOutput
    let per_line = w.saturating_sub(4).max(1);
    for (n, r) in results.iter().filter(|r| r.status != Status::Link).enumerate() {
        if n % per_line == 0 {
            o.push_str("\n  ");
        }
        o.push_str(symbol(r.status, ctx.dry_run));
    }

    let issues = issues(results);
    let errors = issues.iter().filter(|i| i.is_error()).count();
    let fixable = issues.len() - errors;
    let label = match ctx.preset {
        "per" => "PER",
        "psr12" => "PSR 12",
        "laravel" => "Laravel",
        "symfony" => "Symfony",
        _ => "Empty",
    };
    let _ = writeln!(o, "\n\n  {} {label}  ", "─".repeat(inner.saturating_sub(1 + label.chars().count())));

    let status = if issues.is_empty() {
        "  PASS  "
    } else if errors == 0 && !ctx.dry_run {
        "  FIXED  "
    } else {
        "  FAIL  "
    };
    let plural = |n: usize, s: &str| if n == 1 { s.to_string() } else { format!("{s}s") };
    let mut right = format!("{} {}", ctx.total, plural(ctx.total, "file"));
    if errors > 0 {
        let _ = write!(right, ", {errors} {}", plural(errors, "error"));
    }
    if fixable > 0 {
        let _ = write!(right, ", {fixable} style {}{}", plural(fixable, "issue"), if ctx.dry_run { "" } else { " fixed" });
    }
    let dots = inner.saturating_sub(status.len() + 2 + right.chars().count());
    let _ = writeln!(o, "  {status} {} {right}  ", ".".repeat(dots));

    for i in &issues {
        let left = format!("{} {}", symbol(i.status, ctx.dry_run), i.rel);
        let room = inner.saturating_sub(left.chars().count() + 1);
        let mut desc = description(i);
        // A path wider than the box leaves no room; Termwind then prints the description whole.
        if room > 0 && desc.chars().count() > room {
            // Termwind: `truncate` ends in "…"; without it (verbose) the flex box just clips.
            desc = if ctx.verbose {
                desc.chars().take(room).collect()
            } else {
                desc.chars().take(room - 1).collect::<String>().trim_end().to_string() + "…"
            };
        }
        let _ = writeln!(o, "  {left} {desc:>room$}  ");
        if ctx.verbose && !i.is_error() && !i.diff.is_empty() {
            // Issue::diff(): drop the ---/+++ header, indent every line by two spaces.
            let body: Vec<&str> = i.diff.split('\n').skip(2).collect();
            let _ = writeln!(o, "  {}", body.join("\n  "));
        }
    }
    o.push('\n');
    o
}

fn reporter(ctx: &Ctx, results: &[FileResult], format: &str) -> Result<String> {
    let changed: Vec<&FileResult> = results.iter().filter(|r| r.status == Status::Fixed).collect();
    Ok(match format {
        "json" => {
            let files: Vec<serde_json::Value> = changed
                .iter()
                .map(|r| {
                    let mut f = serde_json::json!({ "name": r.rel });
                    if ctx.verbose {
                        f["appliedFixers"] = serde_json::json!(r.applied);
                        f["diff"] = serde_json::json!(r.diff);
                    }
                    f
                })
                .collect();
            php_json(&serde_json::json!({ "about": ABOUT, "files": files, "time": { "total": 0 }, "memory": 0 }))
        }
        "txt" => {
            let mut o = String::new();
            for (n, r) in changed.iter().enumerate() {
                let _ = write!(o, "{:4}) {}", n + 1, r.rel);
                if ctx.verbose {
                    let _ = write!(o, " ({})", r.applied.join(", "));
                    if !r.diff.is_empty() {
                        let _ = write!(o, "\n      ---------- begin diff ----------\n{}\n      ----------- end diff -----------\n", r.diff);
                    }
                }
                o.push('\n');
            }
            let (verb, what) = if ctx.dry_run { ("Found", "files that can be fixed") } else { ("Fixed", "files") };
            let _ = writeln!(o, "\n{verb} {} of {} {what} in 0.000 seconds, 0.00 MB memory used", changed.len(), ctx.total);
            o
        }
        "agent" => {
            let errors: Vec<&FileResult> = results.iter().filter(|r| r.is_error()).collect();
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
                v["files"] = changed.iter().map(|r| serde_json::json!({ "path": r.rel, "fixers": r.applied })).collect();
            }
            if !errors.is_empty() {
                v["errors"] = errors.iter().map(|r| serde_json::json!({ "path": r.rel, "message": r.message })).collect();
            }
            php_json(&v)
        }
        "checkstyle" | "gitlab" | "junit" | "xml" => bail!("Format [{format}] is not yet supported by pun."),
        _ => bail!("Format [{format}] is not supported."),
    })
}

/// json_encode() defaults: escape `/` and non-ASCII (as UTF-16 `\uXXXX`).
fn php_json(v: &serde_json::Value) -> String {
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
    similar::TextDiff::from_lines(old, new).unified_diff().context_radius(3).header(&p, &p).to_string()
}
