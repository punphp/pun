//! `pun` binary. Invoked as `pint` (argv[0]), it takes Pint's flags and mimics its output and exit codes.

#[cfg(feature = "mago-lint")]
mod lint;
mod cache;
mod console;
mod fix_report;
mod highlight;
mod php_diff;
mod report;
#[cfg(feature = "pprof")]
mod sampler;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use pun_config::{Config, Engine, Options};
use pun_format::{LintError, Pipeline};
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
    /// Report Mago's lint-only rules (`--mago`); never edits files. Needs the `mago-lint` build feature.
    #[cfg(feature = "mago-lint")]
    Lint(lint::LintArgs),
    /// Inspect configuration.
    Config {
        /// Print the resolved configuration after imports.
        #[arg(long)]
        show: bool,
        /// Write pun.toml from pint.json, so pun.toml can be the only config file.
        #[arg(long, conflicts_with = "show")]
        init: bool,
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
    /// Only fix files changed since the given commit, plus untracked files (Pun).
    #[arg(long, value_name = "REF")]
    since: Option<String>,
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
    /// Print which rules run natively and which are delegated to PHP-CS-Fixer (to stderr).
    #[arg(long)]
    explain: bool,
    /// Run only Pun's native fixers: skip (and list) rules that would be delegated to PHP.
    #[arg(long)]
    native_only: bool,
    /// The path to the cache file (default: a per-project file under ~/.cache/pun).
    #[arg(long)]
    cache_file: Option<String>,
    /// Check every file, ignoring and not writing the cache (Pun).
    #[arg(long)]
    no_cache: bool,
    /// Accepted for Pint compatibility; Pun always runs in parallel.
    #[arg(short, long)]
    parallel: bool,
    /// Accepted for Pint compatibility, unparsed (Pint's intval() takes any value).
    #[arg(long)]
    max_processes: Option<String>,
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
    #[cfg(feature = "pprof")]
    let sampler = sampler::start();
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
            #[cfg(feature = "mago-lint")]
            Cmd::Lint(a) => lint::run(a),
            Cmd::Config { init: true, .. } => init_config(),
            Cmd::Config { show: _, init: _, format, config, preset } => show_config(&format, config, preset),
        }
    };
    #[cfg(feature = "pprof")]
    if let Some(s) = sampler {
        s.report();
    }
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

fn init_config() -> Result<u8> {
    let root = root()?;
    let path = root.join("pun.toml");
    if path.exists() {
        anyhow::bail!("pun.toml already exists.");
    }
    std::fs::write(&path, pun_config::init_toml(&root)?)?;
    println!("Wrote {}. pint.json is no longer needed unless you still run Pint.", path.display());
    Ok(0)
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


enum Engine2 {
    Compat(Pipeline),
    Pun(String),
    /// Every file through real PHP-CS-Fixer (pun-delegate): unported rules or a .php-cs-fixer.php config.
    /// With the enabled rules Pun hasn't ported (empty for a .php-cs-fixer.php run).
    Delegate(pun_delegate::Delegate, Vec<String>),
}

type Run = Result<Option<(Vec<u8>, Vec<&'static str>)>, LintError>;

impl Engine2 {
    /// The outer error aborts the run (a dead PHP worker, PHP-CS-Fixer's own run-level exceptions).
    fn run(&self, file: &Path, code: &[u8]) -> Result<Run> {
        Ok(match self {
            Engine2::Compat(p) => p.run(file, code).map(|f| f.map(|f| (f.code, f.applied))),
            Engine2::Pun(php) => pun_format::format_pun(code, php).map(|c| c.map(|c| (c, vec!["pun"]))),
            // Linting stays native (TokenizerLinter's messages), as on the native path.
            Engine2::Delegate(d, _) => match pun_format::lint(code) {
                Err(e) => Err(e),
                Ok(()) => match d.fix(file, code)? {
                    pun_delegate::Outcome::Unchanged => Ok(None),
                    pun_delegate::Outcome::Failed(message) => Err(LintError { message, after_fix: true }),
                    pun_delegate::Outcome::Fixed(new, applied) => match pun_format::lint(&new) {
                        Ok(()) => Ok(Some((new, applied))),
                        Err(e) => Err(LintError { after_fix: true, ..e }),
                    },
                },
            },
        })
    }
}

fn format(a: FormatArgs, pint_mode: bool) -> Result<u8> {
    let root = root()?;
    let engine = a.engine.as_deref().map(|e| if e == "pun" { Engine::Pun } else { Engine::Compat });
    let opts = Options { config: a.config.clone(), no_config: a.no_config, preset: a.preset.clone(), engine, blade: a.blade, pint_mode };
    let cfg = pun_config::load(&root, &opts)?;
    // EnsurePrettierIsConfigured: `"Pint/laravel_blade": true` (or --blade) needs Node and prettier in the project.
    let blade_on = cfg.rules.iter().any(|r| r.name == "Pint/laravel_blade" && r.options.is_null());
    if let Some(out) = report::ensure_prettier(&root, blade_on, a.ansi, a.no_ansi) {
        print!("{out}");
        return Ok(1);
    }
    pun_fixers::analyzers::active_preset::set(&cfg.preset);

    let engine = match cfg.engine {
        Engine::Compat => {
            let mut p = Pipeline::new(&cfg.rules, &pun_fixers::all)?;
            // PUN_PROFILE=1: per-stage timings on stderr after the run.
            if std::env::var_os("PUN_PROFILE").is_some() {
                p.enable_profile();
            }
            let source = match &cfg.php_cs_fixer {
                Some(f) => Some(pun_delegate::Source::Config(f.clone())),
                None => (!p.not_ported.is_empty()).then(|| pun_delegate::Source::Rules(cfg.configured_rules.clone().into())),
            };
            match source {
                Some(_) if a.native_only && cfg.php_cs_fixer.is_some() => {
                    anyhow::bail!("--native-only can't read .php-cs-fixer.php (it needs PHP); add a pint.json or pun.toml instead.")
                }
                Some(s) if !a.native_only => {
                    // PUN_PINT: a Pint PHAR to delegate to when the project has none (the harness, CI images).
                    let d = pun_delegate::Delegate::start(&root, s, std::env::var_os("PUN_PINT").map(PathBuf::from))?;
                    Engine2::Delegate(d, p.not_ported)
                }
                _ => Engine2::Compat(p),
            }
        }
        Engine::Pun => Engine2::Pun(cfg.php.clone()),
    };
    if a.explain || (a.verbose > 0 && !pint_mode) {
        explain(&cfg, &engine, a.explain);
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
    } else if let Some(r) = &a.since {
        Some(pun_workspace::GitSelect::Since(r.clone()))
    } else {
        None
    };
    let files = match (&selection, &engine) {
        (Some(sel), _) => pun_workspace::git_paths(&cfg, sel)?,
        // A .php-cs-fixer.php brings its own Finder.
        (None, Engine2::Delegate(d, _)) if cfg.php_cs_fixer.is_some() && a.paths.is_empty() => d.files()?,
        (None, _) => pun_workspace::discover(&cfg, &a.paths)?,
    };

    if a.list_files {
        for f in &files {
            println!("{}", f.strip_prefix(&root).unwrap_or(f).display());
        }
        return Ok(0);
    }

    let cache = if a.no_cache {
        None
    } else {
        a.cache_file.as_ref().map(|f| root.join(f)).or_else(|| cache::default_path(&root)).map(|p| {
            let mut sig = cache::signature(&cfg);
            if let Engine2::Delegate(d, _) = &engine {
                sig["delegate"] = d.signature();
            }
            cache::Cache::load(p, sig)
        })
    };
    // Unreadable / unwritable files abort the whole run, like Pint (FileReader / Runner exceptions).
    // Besides the result: the content hash when the file is clean (for the cache), and whether the cache said so.
    let process = |path: &PathBuf| -> Result<(FileResult, Option<String>, bool)> {
        let rel = path.strip_prefix(&root).unwrap_or(path).to_string_lossy().into_owned();
        let mut r = FileResult { path: path.clone(), rel, status: Status::NoChanges, applied: vec![], diff: String::new(), message: String::new() };
        // Runner FileFilterIterator: symlinks are dropped silently, empty files are "skipped".
        if path.is_symlink() {
            r.status = Status::Link;
            return Ok((r, None, false));
        }
        let code = std::fs::read(path).with_context(|| format!("Failed to read content from \"{}\".", path.display()))?;
        if code.is_empty() || (matches!(engine, Engine2::Pun(_)) && r.rel.ends_with(".blade.php")) {
            r.status = Status::Skipped;
            return Ok((r, None, false));
        }
        let hash = cache.as_ref().map(|_| cache::hash(&code));
        if let (Some(c), Some(h)) = (&cache, &hash) {
            if c.is_clean(&r.rel, h) {
                return Ok((r, hash, true));
            }
        }
        let mut clean = None;
        match engine.run(path, &code)? {
            Err(e) => {
                r.status = if e.after_fix { Status::Lint } else { Status::Invalid };
                r.message = e.message;
            }
            Ok(None) => clean = hash,
            Ok(Some((new, applied))) => {
                // Reports show diffs only with -v (sebastian/diff's LCS is quadratic, so skip it otherwise).
                if a.verbose > 0 {
                    r.diff = report::diff(&String::from_utf8_lossy(&code), &String::from_utf8_lossy(&new), path);
                }
                r.applied = applied;
                r.status = Status::Fixed;
                if !dry_run {
                    std::fs::write(path, new).with_context(|| format!("Cannot write to file \"{}\" as it is not writable.", path.display()))?;
                }
            }
        }
        Ok((r, clean, false))
    };

    let results: Vec<(FileResult, Option<String>, bool)> = if a.bail {
        // --bail is PHP-CS-Fixer's stop-on-violation: sequential, stop at the first fixable file (errors don't stop it).
        let mut out = Vec::new();
        for f in &files {
            let r = process(f)?;
            let stop = r.0.status == Status::Fixed;
            out.push(r);
            if stop {
                break;
            }
        }
        out
    } else {
        files.par_iter().map(process).collect::<Result<_>>()?
    };

    let hits = results.iter().filter(|r| r.2).count();
    let (results, clean): (Vec<FileResult>, Vec<Option<String>>) = results.into_iter().map(|(r, c, _)| (r, c)).unzip();
    if let Some(c) = cache {
        if a.explain {
            eprintln!("cache: {hits} of {} files unchanged since a clean run, skipped ({})", files.len(), c.path().display());
        }
        c.save(results.iter().map(|r| r.rel.clone()).zip(clean));
    }

    let ctx = report::Ctx { preset: &cfg.preset, total: files.len(), dry_run, verbose: a.verbose > 0, ansi: a.ansi, no_ansi: a.no_ansi };
    let code = report::emit(&ctx, &results, a.format.as_deref(), a.quiet, a.output_to_file.as_deref(), a.output_format.as_deref())?;
    let failed = ((dry_run || a.repair) && results.iter().any(|r| r.status == Status::Fixed))
        || results.iter().any(|r| matches!(r.status, Status::Invalid | Status::Lint));

    if let Engine2::Compat(p) = &engine {
        eprint!("{}", p.profile_report());
        if !p.not_ported.is_empty() {
            eprintln!("pun: --native-only skipped {} rules that are not ported: {}", p.not_ported.len(), p.not_ported.join(", "));
        }
    }
    Ok(code.max(u8::from(failed)))
}

/// `full` (--explain): the rule list too; `-v` prints only why a run is delegated.
fn explain(cfg: &Config, engine: &Engine2, full: bool) {
    let mut err = std::io::stderr().lock();
    match engine {
        Engine2::Delegate(d, not_ported) => {
            let why = match &cfg.php_cs_fixer {
                Some(f) => format!("{} configures the run", f.display()),
                None => format!("{} of {} rules are not ported ({})", not_ported.len(), cfg.rules.len(), not_ported.join(", ")),
            };
            let _ = writeln!(err, "delegate: {why}, so every file runs through PHP-CS-Fixer in `php` ({})", d.runtime.display());
            for (name, priority) in d.fixers.iter().filter(|_| full) {
                let _ = writeln!(err, "  delegated   {priority:>4}  {name}");
            }
        }
        _ if !full => {}
        Engine2::Pun(php) => {
            let _ = writeln!(err, "engine: pun (mago-formatter, pint preset, PHP {php}); pint.json rules are ignored");
        }
        Engine2::Compat(p) => {
            let _ = writeln!(err, "engine: compat (PHP {}), preset {}, {} rules", cfg.php, cfg.preset, cfg.rules.len());
            for f in p.fixers() {
                let _ = writeln!(err, "  native      {:>4}  {}", f.priority(), f.name());
            }
            for n in &p.not_ported {
                let _ = writeln!(err, "  skipped           {n}");
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
    // --test/--bail: the dry run leaves Pint's temp file untouched, so the input comes back as is.
    if a.test || a.bail || (a.stdin_filename.is_some() && pun_workspace::is_path_excluded(cfg, &ctx)) {
        out.write_all(&code)?;
        return Ok(0);
    }
    match engine.run(&ctx, &code)? {
        Ok(Some((new, _))) => out.write_all(&new)?,
        // Pint reports syntax errors through its (unshown) JSON report and echoes the file unchanged.
        Ok(None) | Err(_) => out.write_all(&code)?,
    }
    Ok(0)
}
