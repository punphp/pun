//! `pun` binary. Invoked as `pint` (argv[0]), it takes Pint's flags and mimics its output and exit codes.

mod report;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use pun_config::{Config, Engine, Options};
use pun_format::{Fixer, LintError, Pipeline};
use rayon::prelude::*;
use report::{FileResult, Status};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "pun", version, about = "Fast PHP toolchain: Pint-compatible formatter")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Fix (or check) the coding style of the given paths.
    Format(FormatArgs),
    /// Inspect configuration.
    Config {
        /// Print the resolved configuration after imports.
        #[arg(long)]
        show: bool,
        /// Output format.
        #[arg(long, default_value = "toml", value_parser = ["toml", "json"])]
        format: String,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        preset: Option<String>,
    },
}

/// Pint's DefaultCommand flags plus Pun's own.
#[derive(Args, Default)]
#[command(about = "Fix the coding style of the given path")]
pub struct FormatArgs {
    /// The path to fix (`-` reads stdin).
    paths: Vec<PathBuf>,
    /// Enable the [Pint/laravel_blade] rule to format Blade files.
    #[arg(long)]
    blade: bool,
    /// The configuration that should be used.
    #[arg(long)]
    config: Option<PathBuf>,
    /// Disable loading any configuration file.
    #[arg(long)]
    no_config: bool,
    /// The preset that should be used.
    #[arg(long)]
    preset: Option<String>,
    /// Test for code style errors without fixing them.
    #[arg(long, alias = "check")]
    test: bool,
    /// Test for code style errors without fixing them and stop on first error.
    #[arg(long)]
    bail: bool,
    /// Fix code style errors but exit with status 1 if there were any changes made.
    #[arg(long)]
    repair: bool,
    /// Only fix files that have changed since branching off from the given branch.
    #[arg(long, value_name = "BRANCH")]
    diff: Option<String>,
    /// Only fix files that have uncommitted changes.
    #[arg(long)]
    dirty: bool,
    /// Only fix files with staged changes (Pun).
    #[arg(long)]
    staged: bool,
    /// The output format that should be used (json, txt, agent).
    #[arg(long)]
    format: Option<String>,
    /// Output the test results to a file at this path.
    #[arg(long)]
    output_to_file: Option<PathBuf>,
    /// The format that should be used when outputting the test results to a file.
    #[arg(long)]
    output_format: Option<String>,
    /// Read code from stdin (Pun); `--stdin-filename` gives it a path for exclusion rules.
    #[arg(long)]
    stdin: bool,
    /// File path context for stdin input.
    #[arg(long)]
    stdin_filename: Option<PathBuf>,
    /// Formatting engine: compat (Pint-identical) or pun (mago-formatter).
    #[arg(long, value_parser = ["compat", "pun"])]
    engine: Option<String>,
    /// Print which rules run natively and which are not ported (to stderr).
    #[arg(long)]
    explain: bool,
    /// Accepted for Pint compatibility; Pun has no cache yet.
    #[arg(long)]
    cache_file: Option<String>,
    /// Accepted for Pint compatibility; Pun always runs in parallel.
    #[arg(short, long)]
    parallel: bool,
    #[arg(long)]
    max_processes: Option<usize>,
    /// Print the discovered files and exit (debugging finder parity).
    #[arg(long, hide = true)]
    list_files: bool,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long)]
    quiet: bool,
    #[arg(long, hide = true)]
    ansi: bool,
    #[arg(long, hide = true)]
    no_ansi: bool,
    #[arg(short = 'n', long, hide = true)]
    no_interaction: bool,
}

fn main() -> ExitCode {
    let argv0 = std::env::args_os().next().map(PathBuf::from).unwrap_or_default();
    let pint_mode = argv0.file_stem().is_some_and(|s| s == "pint");
    let result = if pint_mode {
        #[derive(Parser)]
        #[command(name = "pint")]
        struct Pint {
            #[command(flatten)]
            args: FormatArgs,
        }
        format(Pint::parse().args, true)
    } else {
        match Cli::parse().cmd {
            Cmd::Format(a) => format(a, false),
            Cmd::Config { show: _, format, config, preset } => show_config(&format, config, preset),
        }
    };
    match result {
        Ok(code) => ExitCode::from(code),
        Err(e) => {
            eprintln!("\n  {e}\n");
            ExitCode::FAILURE
        }
    }
}

fn root() -> Result<PathBuf> {
    Ok(std::env::current_dir()?.canonicalize()?)
}

fn show_config(fmt: &str, config: Option<PathBuf>, preset: Option<String>) -> Result<u8> {
    let cfg = pun_config::load(&root()?, &Options { config, preset, ..Default::default() })?;
    let v = cfg.show();
    if fmt == "json" {
        println!("{}", serde_json::to_string_pretty(&v)?);
    } else {
        print!("{}", toml::to_string(&v)?);
    }
    Ok(0)
}

/// Native fixers. TODO: `pun_fixers::all()` once pun-fixers exports it (see docs/REQUESTS.md).
fn native_fixers() -> Vec<Box<dyn Fixer>> {
    pun_fixers::all()
}

enum Engine2 {
    Compat(Pipeline),
    Pun(String),
}

impl Engine2 {
    fn run(&self, file: &Path, code: &[u8]) -> Result<Option<(Vec<u8>, Vec<&'static str>)>, LintError> {
        match self {
            Engine2::Compat(p) => Ok(p.run(file, code)?.map(|f| (f.code, f.applied))),
            Engine2::Pun(php) => Ok(pun_format::format_pun(code, php)?.map(|c| (c, vec!["pun"]))),
        }
    }
}

fn format(a: FormatArgs, pint_mode: bool) -> Result<u8> {
    let root = root()?;
    let engine = a.engine.as_deref().map(|e| if e == "pun" { Engine::Pun } else { Engine::Compat });
    let opts = Options { config: a.config.clone(), no_config: a.no_config, preset: a.preset.clone(), engine, blade: a.blade, pint_mode };
    let cfg = pun_config::load(&root, &opts)?;

    let engine = match cfg.engine {
        Engine::Compat => Engine2::Compat(Pipeline::new(&cfg.rules, native_fixers())?),
        Engine::Pun => Engine2::Pun(cfg.php.clone()),
    };
    if a.explain {
        explain(&cfg, &engine);
    }

    let stdin_path = a.paths.first().is_some_and(|p| p.as_os_str() == "-");
    if a.stdin || stdin_path || a.stdin_filename.is_some() {
        return stdin(&a, &cfg, &engine);
    }

    let dry_run = a.test || a.bail;
    let selection = if a.dirty {
        Some(pun_workspace::GitSelect::Dirty)
    } else if let Some(b) = &a.diff {
        Some(pun_workspace::GitSelect::Diff(b.clone()))
    } else if a.staged {
        Some(pun_workspace::GitSelect::Staged)
    } else {
        None
    };
    let files = match &selection {
        // Pint swallows the "no dirty files" / "not a git repo" aborts and reports an empty run.
        Some(sel) => pun_workspace::git_paths(&cfg, sel).unwrap_or_default(),
        None => pun_workspace::discover(&cfg, &a.paths)?,
    };

    if a.list_files {
        for f in &files {
            println!("{}", f.strip_prefix(&root).unwrap_or(f).display());
        }
        return Ok(0);
    }

    let process = |path: &PathBuf| -> FileResult {
        let rel = path.strip_prefix(&root).unwrap_or(path).to_string_lossy().into_owned();
        let mut r = FileResult { path: path.clone(), rel, status: Status::NoChanges, applied: vec![], diff: String::new(), message: String::new() };
        // Runner FileFilterIterator: symlinks are dropped silently, empty files are "skipped".
        if path.is_symlink() {
            r.status = Status::Link;
            return r;
        }
        let Ok(code) = std::fs::read(path) else {
            r.status = Status::Exception;
            r.message = format!("Failed to read file \"{}\".", path.display());
            return r;
        };
        if code.is_empty() || (matches!(engine, Engine2::Pun(_)) && r.rel.ends_with(".blade.php")) {
            r.status = Status::Skipped;
            return r;
        }
        match engine.run(path, &code) {
            Err(e) => {
                r.status = if e.after_fix { Status::Lint } else { Status::Invalid };
                r.message = e.message;
            }
            Ok(None) => {}
            Ok(Some((new, applied))) => {
                r.diff = report::diff(&String::from_utf8_lossy(&code), &String::from_utf8_lossy(&new), path);
                r.applied = applied;
                r.status = Status::Fixed;
                if !dry_run {
                    if let Err(e) = std::fs::write(path, new) {
                        r.status = Status::Exception;
                        r.message = e.to_string();
                    }
                }
            }
        }
        r
    };

    let results: Vec<FileResult> = if a.bail {
        // --bail is PHP-CS-Fixer's stop-on-violation: sequential, stop at the first fixable file (errors don't stop it).
        let mut out = Vec::new();
        for f in &files {
            let r = process(f);
            let stop = r.status == Status::Fixed;
            out.push(r);
            if stop {
                break;
            }
        }
        out
    } else {
        files.par_iter().map(process).collect()
    };

    let ctx = report::Ctx { preset: &cfg.preset, total: files.len(), dry_run, verbose: a.verbose > 0 };
    let code = report::emit(&ctx, &results, a.format.as_deref(), a.quiet, a.output_to_file.as_deref(), a.output_format.as_deref())?;
    let failed = ((dry_run || a.repair) && results.iter().any(|r| r.status == Status::Fixed))
        || results.iter().any(|r| matches!(r.status, Status::Invalid | Status::Lint | Status::Exception));

    if let Engine2::Compat(p) = &engine {
        if !p.not_ported.is_empty() {
            eprintln!(
                "pun: {} of {} enabled rules are not ported yet, so this result is not Pint-identical (see --explain).",
                p.not_ported.len(),
                cfg.rules.len()
            );
            return Ok(1);
        }
    }
    Ok(code.max(u8::from(failed)))
}

fn explain(cfg: &Config, engine: &Engine2) {
    let mut err = std::io::stderr().lock();
    match engine {
        Engine2::Pun(php) => {
            let _ = writeln!(err, "engine: pun (mago-formatter, pint preset, PHP {php}); pint.json rules are ignored");
        }
        Engine2::Compat(p) => {
            let _ = writeln!(err, "engine: compat (PHP {}), preset {}, {} rules", cfg.php, cfg.preset, cfg.rules.len());
            for f in p.fixers() {
                let _ = writeln!(err, "  native      {:>4}  {}", f.priority(), f.name());
            }
            for n in &p.not_ported {
                let _ = writeln!(err, "  not ported        {n}");
            }
        }
    }
}

/// DefaultCommand::fixStdinInput: excluded paths echo the input; otherwise print the fixed code.
fn stdin(a: &FormatArgs, cfg: &Config, engine: &Engine2) -> Result<u8> {
    let mut code = Vec::new();
    std::io::stdin().read_to_end(&mut code)?;
    let mut out = std::io::stdout().lock();
    let ctx = a.stdin_filename.clone().unwrap_or_else(|| "stdin.php".into());
    if a.stdin_filename.is_some() && pun_workspace::is_path_excluded(cfg, &ctx) {
        out.write_all(&code)?;
        return Ok(0);
    }
    match engine.run(&ctx, &code) {
        Ok(Some((new, _))) => out.write_all(&new)?,
        // Pint reports syntax errors through its (unshown) JSON report and echoes the file unchanged.
        Ok(None) | Err(_) => out.write_all(&code)?,
    }
    Ok(0)
}
