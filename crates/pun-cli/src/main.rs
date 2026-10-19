//! `pun` binary. `pun format` takes Pint's flags and keeps its exit codes and machine formats; the human output is
//! Pun's own (crate::render). Every command reads pun.toml.
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a command-line tool prints its reports"
)]

mod agent;
mod analyse;
mod analyse_report;
mod baseline;
mod cache;
mod console;
mod fix_report;
mod github;
mod init;
mod lint_plan;
mod list_files;
mod mapping;
mod migrate;
mod php_diff;
mod pint_error;
mod prettier;
mod render;
mod report;
mod run;
#[cfg(feature = "pprof")]
mod sampler;
mod table;
mod watch;

use anyhow::Result;
use clap::{Args, CommandFactory, FromArgMatches, Parser, Subcommand};
use pun_config::Options;
use run::{LintOpts, Stage, format};
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
    /// Report the `[lint]` rules' problems; `--fix` applies their fixes, but those with `fix = "off"`.
    Lint(Box<StageCmd>),
    /// Format, lint and analyse in one run: reports, or `--fix` applies the format and lint fixes.
    Check(Box<StageCmd>),
    /// Find bugs with types (PHPStan's rules): the `[analyse]` rules. Never edits a file.
    #[command(visible_alias = "analyze")]
    Analyse(Box<analyse::AnalyseArgs>),
    /// Write pun.toml from pint.json (or .php-cs-fixer(.dist).php) and rewrite `@php-cs-fixer-ignore` comments.
    Migrate(migrate::MigrateArgs),
    /// Write pun.toml for a new project: asks the code style, lint sets and analysis preset (`--yes` takes the defaults).
    Init(init::InitArgs),
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

/// `pun lint` / `pun check`: `pun format`'s flags plus `--fix` and the lint levels.
#[derive(Args)]
struct StageCmd {
    #[command(flatten)]
    args: FormatArgs,
    /// Apply the fixes instead of reporting them.
    #[arg(long)]
    fix: bool,
    /// Turn a rule or group off (`-A lint/style`), after pun.toml; repeatable, applied in order with -W and -D.
    #[arg(short = 'A', long = "allow", value_name = "RULE")]
    allow: Vec<String>,
    /// Turn a rule or group on as a warning.
    #[arg(short = 'W', long = "warn", value_name = "RULE")]
    warn: Vec<String>,
    /// Turn a rule or group on as an error.
    #[arg(short = 'D', long = "deny", value_name = "RULE")]
    deny: Vec<String>,
    /// Exit 1 when there are warnings.
    #[arg(long)]
    deny_warnings: bool,
    /// Exit 1 when there are more than this many warnings.
    #[arg(long, value_name = "N")]
    max_warnings: Option<usize>,
    /// Print each problem on one line.
    #[arg(long)]
    concise: bool,
    /// `-A`, `-W` and `-D` in command-line order (filled in `main` from clap's indices).
    #[arg(skip)]
    levels: Vec<(lint_plan::CliLevel, String)>,
}

#[derive(Clone, Copy, PartialEq, clap::ValueEnum)]
enum ConfigFormat {
    Toml,
    Json,
}

/// Pint's DefaultCommand flags, plus Pun's own.
#[derive(Args, Default, Clone)]
#[command(about = "Fix the coding style of the given path")]
pub(crate) struct FormatArgs {
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
    /// The output format that should be used (json, txt, agent, checkstyle, junit, xml, gitlab, github).
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
    /// `GITHUB_ACTIONS=true`, read once in `main`: human output gets GitHub annotations after it.
    #[arg(skip)]
    github_actions: bool,
}

impl FormatArgs {
    /// `--color`, or Pint's `--ansi` (which wins over `--no-ansi`, as in Symfony's configureIO).
    fn color(&self) -> render::ColorChoice {
        use render::ColorChoice;
        self.color.unwrap_or(if self.ansi {
            ColorChoice::Always
        } else if self.no_ansi {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        })
    }

    /// How a failed run of `pun format`, `lint` or `check` shows its error.
    fn failure(&self) -> report::Failure {
        let human = self.format.is_none() && !self.quiet && !agent::enabled();
        report::Failure {
            pint: true,
            machine: !human,
            github_format: self.format.as_deref().and_then(|f| f.parse().ok())
                == Some(report::ReportFormat::Github),
            annotate: human && self.github_actions,
            color: self.color(),
        }
    }
}

/// `pun format`'s flags that Pint doesn't have.
#[derive(Args, Default, Clone)]
pub(crate) struct PunArgs {
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
    let matches = Cli::command().get_matches();
    let mut cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());
    if let (Cmd::Lint(cmd) | Cmd::Check(cmd), Some((_, sub))) = (&mut cli.cmd, matches.subcommand())
    {
        cmd.levels = levels(sub);
    }
    let name = match &cli.cmd {
        Cmd::Format(_) => "format",
        Cmd::Lint(_) => "lint",
        Cmd::Check(_) => "check",
        Cmd::Analyse(_) => "analyse",
        Cmd::Migrate(_) => "migrate",
        Cmd::Init(_) => "init",
        Cmd::ListFiles(_) => "list-files",
        Cmd::Config { .. } => "config",
        Cmd::Explain { .. } => "explain",
    };
    let flag = (cli.agent || cli.no_agent).then_some(cli.agent);
    agent::init(flag, name);
    let github_actions = std::env::var_os("GITHUB_ACTIONS").is_some_and(|v| v == "true");
    let mut cmd = cli.cmd;
    match &mut cmd {
        Cmd::Format(a) => a.github_actions = github_actions,
        Cmd::Lint(cmd) | Cmd::Check(cmd) => cmd.args.github_actions = github_actions,
        Cmd::Analyse(a) => a.github_actions = github_actions,
        _ => {}
    }
    // One rayon pool for every pass, capped by `--max-processes` (read like Pint's intval()).
    let max_processes = match &cmd {
        Cmd::Format(a) => a.max_processes.as_deref(),
        Cmd::Lint(cmd) | Cmd::Check(cmd) => cmd.args.max_processes.as_deref(),
        Cmd::Analyse(a) => a.max_processes(),
        _ => None,
    };
    if let Some(n) = max_processes
        .map(pun_tokens::php_compat::intval)
        .filter(|&n| n > 0)
    {
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(n as usize)
            .build_global();
    }
    // Other commands' errors are Pun's own run errors.
    let other = report::Failure {
        pint: false,
        machine: false,
        github_format: false,
        annotate: false,
        color: render::ColorChoice::Auto,
    };
    let failure = match &cmd {
        Cmd::Format(a) => a.failure(),
        Cmd::Lint(cmd) | Cmd::Check(cmd) => cmd.args.failure(),
        Cmd::Analyse(a) => a.failure(),
        _ => other,
    };
    let result = match cmd {
        Cmd::Format(a) if a.pun.watch => watch::run(*a),
        Cmd::Format(a) => format(*a, Stage::Format, LintOpts::default()),
        Cmd::Lint(cmd) => stage(*cmd, |fix| Stage::Lint { fix }),
        Cmd::Check(cmd) => stage(*cmd, |fix| Stage::Check { fix }),
        Cmd::Analyse(a) => analyse::run(*a),
        Cmd::Migrate(a) => migrate::run(a),
        Cmd::Init(a) => init::run(a),
        Cmd::ListFiles(a) => list_files::run(a),
        Cmd::Config {
            format,
            config,
            preset,
            ..
        } => show_config(format, config, preset),
        Cmd::Explain { rule } => pun_config::explain(&rule).map(|t| {
            print!("{t}");
            0
        }),
    };
    #[cfg(feature = "pprof")]
    if let Some(s) = sampler {
        s.report();
    }
    // Exit 2: Pun couldn't run (output.md §2).
    ExitCode::from(result.unwrap_or_else(|e| report::fail(e, &failure)))
}

/// `pun lint` / `pun check`: a `pun format` run over other rules; without `--fix` nothing is written.
fn stage(cmd: StageCmd, stage: fn(bool) -> Stage) -> Result<u8> {
    let mut a = cmd.args;
    a.test |= !cmd.fix;
    let lint = LintOpts {
        levels: cmd.levels,
        deny_warnings: cmd.deny_warnings,
        max_warnings: cmd.max_warnings,
        concise: cmd.concise,
    };
    format(a, stage(cmd.fix), lint)
}

/// `-A`, `-W` and `-D` in the order they were given: later ones win.
fn levels(m: &clap::ArgMatches) -> Vec<(lint_plan::CliLevel, String)> {
    use lint_plan::CliLevel;
    let mut out: Vec<(usize, CliLevel, String)> = vec![];
    for (id, level) in [
        ("allow", CliLevel::Allow),
        ("warn", CliLevel::Warn),
        ("deny", CliLevel::Deny),
    ] {
        if let (Some(values), Some(indices)) = (m.get_many::<String>(id), m.indices_of(id)) {
            out.extend(values.zip(indices).map(|(v, i)| (i, level, v.clone())));
        }
    }
    out.sort_by_key(|l| l.0);
    out.into_iter().map(|(_, l, v)| (l, v)).collect()
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
