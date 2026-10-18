//! `pun` binary. `pun format` takes Pint's flags and keeps its exit codes and machine formats; the human output is
//! Pun's own (crate::render). Every command reads pun.toml.

mod agent;
mod cache;
mod console;
mod fix_report;
#[cfg(feature = "mago-lint")]
mod lint;
mod list_files;
mod migrate;
mod php_diff;
mod pint_error;
mod render;
mod report;
mod run;
#[cfg(feature = "pprof")]
mod sampler;
mod watch;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use pun_config::Options;
use run::{Stage, format};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "pun",
    version,
    about = "Fast PHP toolchain: Pint-compatible formatter"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
    /// Agent output (a summary line, then one line per finding) even when no coding agent is detected.
    #[arg(long, global = true)]
    agent: bool,
    /// Human output even under a coding agent.
    #[arg(long, global = true, conflicts_with = "agent")]
    no_agent: bool,
}

#[derive(Subcommand)]
enum Cmd {
    /// Fix (or check) the layout of the given paths: the `[format]` rules.
    Format(Box<FormatArgs>),
    /// Report the `[lint]` rules' fixes; `--fix` applies them all but those with `fix = "off"`.
    Lint {
        #[command(flatten)]
        cmd: Box<StageCmd>,
        /// Run Mago's lint-only Laravel rules instead (report only).
        #[cfg(feature = "mago-lint")]
        #[arg(long)]
        mago: bool,
    },
    /// Format and lint in one run (analyse joins once `pun analyse` exists): reports, or `--fix` applies.
    Check(Box<StageCmd>),
    /// Write pun.toml from pint.json (or .php-cs-fixer(.dist).php) and rewrite `@php-cs-fixer-ignore` comments.
    Migrate(migrate::MigrateArgs),
    /// Print the files a command would check with the current pun.toml.
    ListFiles(list_files::ListFilesArgs),
    /// Print the resolved configuration.
    Config {
        /// Accepted for compatibility; showing is all `pun config` does.
        #[arg(long)]
        show: bool,
        /// Output format (default: toml).
        #[arg(long, value_enum)]
        format: Option<ConfigFormat>,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        preset: Option<String>,
    },
    /// Print a rule's summary, group, sets, fix safety, upstream names and docs link.
    Explain {
        /// A rule ID (`format/arrays/syntax`), or an upstream or old name, which is answered with its Pun ID.
        rule: String,
    },
}

/// `pun lint` / `pun check`: `pun format`'s flags plus `--fix`.
#[derive(Args)]
struct StageCmd {
    #[command(flatten)]
    args: FormatArgs,
    /// Apply the fixes instead of reporting them.
    #[arg(long)]
    fix: bool,
}

#[derive(Clone, Copy, PartialEq, clap::ValueEnum)]
enum ConfigFormat {
    Toml,
    Json,
}

/// Pint's DefaultCommand flags, plus Pun's own.
#[derive(Args, Default, Clone)]
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
    /// The output format that should be used (json, txt, agent).
    #[arg(long)]
    format: Option<String>,
    /// Output the test results to a file at this path.
    #[arg(long)]
    output_to_file: Option<PathBuf>,
    /// The format that should be used when outputting the test results to a file.
    #[arg(long)]
    output_format: Option<String>,
    /// File path context for stdin input.
    #[arg(long)]
    stdin_filename: Option<PathBuf>,
    /// The path to the cache file (default: a per-project file under ~/.cache/pun).
    #[arg(long)]
    cache_file: Option<String>,
    /// Accepted for Pint compatibility; Pun always runs in parallel.
    #[arg(short, long)]
    parallel: bool,
    /// The number of threads (default: one per core). Read like Pint's intval(), so any value is accepted.
    #[arg(long)]
    max_processes: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long)]
    quiet: bool,
    /// Colour the output: auto (on for a terminal, off when piped or with NO_COLOR), always or never.
    #[arg(long, value_enum, value_name = "WHEN")]
    color: Option<render::ColorChoice>,
    /// `--color=always` (Pint's spelling); also decorates the `--format` reports as Pint does.
    #[arg(long, hide = true)]
    ansi: bool,
    /// `--color=never` (Pint's spelling).
    #[arg(long, hide = true)]
    no_ansi: bool,
    #[arg(short = 'n', long, hide = true)]
    no_interaction: bool,
    #[command(flatten)]
    pun: PunArgs,
}

/// `pun format`'s flags that Pint doesn't have.
#[derive(Args, Default, Clone)]
pub struct PunArgs {
    /// Only fix files with staged changes (Pun).
    #[arg(long)]
    staged: bool,
    /// Only fix files changed since the given commit, plus untracked files (Pun).
    #[arg(long, value_name = "REF")]
    since: Option<String>,
    /// Read code from stdin (Pun); `--stdin-filename` gives it a path for exclusion rules.
    #[arg(long)]
    stdin: bool,
    /// Print the fixers the run applies, in priority order (to stderr).
    #[arg(long)]
    explain: bool,
    /// Check every file, ignoring and not writing the cache (Pun).
    #[arg(long)]
    no_cache: bool,
    /// Keep running and re-format files as they change (Pun).
    #[arg(long, conflicts_with_all = ["stdin", "stdin_filename"])]
    watch: bool,
}

fn main() -> ExitCode {
    #[cfg(feature = "pprof")]
    let sampler = sampler::start();
    let cli = Cli::parse();
    let name = match &cli.cmd {
        Cmd::Format(_) => "format",
        Cmd::Lint { .. } => "lint",
        Cmd::Check(_) => "check",
        Cmd::Migrate(_) => "migrate",
        Cmd::ListFiles(_) => "list-files",
        Cmd::Config { .. } => "config",
        Cmd::Explain { .. } => "explain",
    };
    let flag = (cli.agent || cli.no_agent).then_some(cli.agent);
    agent::init(flag, name);
    let (result, pint_errors) = match cli.cmd {
        Cmd::Format(a) if a.pun.watch => (watch::run(*a), true),
        Cmd::Format(a) => (format(*a, Stage::Format), true),
        #[cfg(feature = "mago-lint")]
        Cmd::Lint { cmd, mago: true } => (lint::run(cmd.args.paths, cmd.args.config), false),
        Cmd::Lint { cmd, .. } => (stage(*cmd, Stage::Lint), true),
        Cmd::Check(cmd) => (stage(*cmd, Stage::Check), true),
        Cmd::Migrate(a) => (migrate::run(a), false),
        Cmd::ListFiles(a) => (list_files::run(a), false),
        Cmd::Config {
            format,
            config,
            preset,
            ..
        } => (show_config(format, config, preset), false),
        Cmd::Explain { rule } => (
            pun_config::explain(&rule).map(|t| {
                print!("{t}");
                0
            }),
            false,
        ),
    };
    #[cfg(feature = "pprof")]
    if let Some(s) = sampler {
        s.report();
    }
    match result {
        Ok(code) => ExitCode::from(code),
        // A run's errors are Pint's error boxes, like the rest of its output.
        Err(e) if pint_errors => {
            print!("{}", pint_error::render(&e.to_string()));
            ExitCode::FAILURE
        }
        Err(e) => {
            match agent::enabled() {
                true => print!("{}", agent::error(&format!("{e:#}"))),
                false => eprintln!("pun: {e:#}"),
            }
            ExitCode::from(config_exit(&e))
        }
    }
}

/// A config error's exit code: 2 for a removed format preset (`--preset per`, `[format] preset = "symfony"`), 1 otherwise.
pub(crate) fn config_exit(e: &anyhow::Error) -> u8 {
    if e.is::<pun_config::RemovedPreset>() {
        2
    } else {
        1
    }
}

/// `pun lint` / `pun check`: a `pun format` run over other rules; without `--fix` nothing is written.
fn stage(cmd: StageCmd, stage: fn(bool) -> Stage) -> Result<u8> {
    let mut a = cmd.args;
    a.test |= !cmd.fix;
    format(a, stage(cmd.fix))
}

fn root() -> Result<PathBuf> {
    Ok(std::env::current_dir()?.canonicalize()?)
}

fn show_config(
    fmt: Option<ConfigFormat>,
    config: Option<PathBuf>,
    preset: Option<String>,
) -> Result<u8> {
    let root = root()?;
    let cfg = pun_config::load(
        &root,
        &Options {
            config,
            preset,
            ..Default::default()
        },
    )?;
    let v = cfg.show();
    let sources: Vec<String> = (cfg.sources.iter())
        .map(|s| s.strip_prefix(&root).unwrap_or(s).display().to_string())
        .collect();
    // TOML starts with the files it was resolved from, as a comment.
    let toml = match sources.is_empty() {
        true => toml::to_string(&v)?,
        false => format!(
            "# Resolved from {}.\n\n{}",
            sources.join(", "),
            toml::to_string(&v)?
        ),
    };
    if fmt.is_none() && agent::enabled() {
        // Agent mode: what was loaded in one line, then the TOML as usual.
        let detail = [
            format!("preset {}", cfg.preset),
            format!("PHP {}", cfg.php),
            agent::count(cfg.rules.len(), "format rule"),
            agent::count(cfg.lint.rules.len(), "lint rule"),
        ];
        let from = format!("from {}", sources.join(", "));
        print!("{}", agent::render("ok", &detail, &from, &[], false));
        print!("{toml}");
    } else if fmt == Some(ConfigFormat::Json) {
        println!("{}", serde_json::to_string_pretty(&v)?);
    } else {
        print!("{toml}");
    }
    Ok(0)
}
