//! `pun` binary. Invoked as `pint` (argv[0]), it takes Pint's flags and mimics its output and exit codes.

mod cache;
mod console;
mod fix_report;
mod highlight;
#[cfg(feature = "mago-lint")]
mod lint;
mod lsp;
mod php_diff;
mod pint_error;
mod report;
mod run;
#[cfg(feature = "pprof")]
mod sampler;
mod watch;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use pun_config::Options;
use run::format;
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
}

#[derive(Subcommand)]
enum Cmd {
    /// Fix (or check) the coding style of the given paths.
    Format(Box<FormatCmd>),
    /// Report Mago's lint-only rules (`--mago`); never edits files. Needs the `mago-lint` build feature.
    #[cfg(feature = "mago-lint")]
    Lint(lint::LintArgs),
    /// Formatting-only language server on stdio (textDocument/formatting).
    Lsp,
    /// Inspect configuration.
    Config {
        /// Print the resolved configuration after imports.
        #[arg(long)]
        show: bool,
        /// Write pun.toml from pint.json, so pun.toml can be the only config file.
        #[arg(long, conflicts_with_all = ["show", "format", "config", "preset"])]
        init: bool,
        /// Output format.
        #[arg(long, value_enum, default_value_t = ConfigFormat::Toml)]
        format: ConfigFormat,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long)]
        preset: Option<String>,
    },
}

#[derive(Clone, Copy, PartialEq, clap::ValueEnum)]
enum ConfigFormat {
    Toml,
    Json,
}

/// Pint's DefaultCommand flags.
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
    /// Accepted for Pint compatibility, unparsed (Pint's intval() takes any value).
    #[arg(long)]
    max_processes: Option<String>,
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
    /// Set from `pun format`'s own flags; `pint` mode doesn't parse them, so they are unknown options there.
    #[arg(skip)]
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
    /// Print which rules run natively and which are delegated to PHP-CS-Fixer (to stderr).
    #[arg(long)]
    explain: bool,
    /// Run only Pun's native fixers: skip (and list) rules that would be delegated to PHP.
    #[arg(long)]
    native_only: bool,
    /// Check every file, ignoring and not writing the cache (Pun).
    #[arg(long)]
    no_cache: bool,
    /// Keep running and re-format files as they change (Pun).
    #[arg(long, conflicts_with_all = ["stdin", "stdin_filename"])]
    watch: bool,
    /// Print the discovered files and exit (debugging finder parity).
    #[arg(long, hide = true)]
    list_files: bool,
}

#[derive(Args)]
struct FormatCmd {
    #[command(flatten)]
    args: FormatArgs,
    #[command(flatten)]
    pun: PunArgs,
}

fn main() -> ExitCode {
    let argv0 = std::env::args_os()
        .next()
        .map(PathBuf::from)
        .unwrap_or_default();
    let pint_mode = argv0.file_stem().is_some_and(|s| s == "pint");
    #[cfg(feature = "pprof")]
    let sampler = sampler::start();
    let result = if pint_mode {
        #[derive(Parser)]
        // ArgvInput lets a flag repeat.
        #[command(name = "pint", args_override_self = true)]
        struct Pint {
            #[command(flatten)]
            args: FormatArgs,
        }
        match Pint::try_parse() {
            Ok(p) => format(p.args, true),
            Err(e) => Err(anyhow::anyhow!(
                pint_error::from_clap(&e).unwrap_or_else(|| e.exit())
            )),
        }
    } else {
        match Cli::parse().cmd {
            Cmd::Format(c) => {
                let FormatCmd { args, pun } = *c;
                let a = FormatArgs { pun, ..args };
                if a.pun.watch {
                    watch::run(a)
                } else {
                    format(a, false)
                }
            }
            Cmd::Lsp => lsp::run(),
            #[cfg(feature = "mago-lint")]
            Cmd::Lint(a) => lint::run(a),
            Cmd::Config { init: true, .. } => init_config(),
            Cmd::Config {
                format,
                config,
                preset,
                ..
            } => show_config(format, config, preset),
        }
    };
    #[cfg(feature = "pprof")]
    if let Some(s) = sampler {
        s.report();
    }
    match result {
        Ok(code) => ExitCode::from(code),
        Err(e) if pint_mode => {
            print!("{}", pint_error::render(&e.to_string()));
            ExitCode::FAILURE
        }
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
    println!(
        "Wrote {}. pint.json is no longer needed unless you still run Pint.",
        path.display()
    );
    Ok(0)
}

fn show_config(fmt: ConfigFormat, config: Option<PathBuf>, preset: Option<String>) -> Result<u8> {
    let cfg = pun_config::load(
        &root()?,
        &Options {
            config,
            preset,
            ..Default::default()
        },
    )?;
    let v = cfg.show();
    if fmt == ConfigFormat::Json {
        println!("{}", serde_json::to_string_pretty(&v)?);
    } else {
        print!("{}", toml::to_string(&v)?);
    }
    Ok(0)
}
