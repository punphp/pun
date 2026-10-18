//! `pun analyse` (`pun analyze`): PHPStan's rules on pun-analyse, configured by pun.toml's `[analyse]`
//! (context/plan/analyse.md Build 1-2).
//!
//! The files come from `[analyse] paths` (or `[files] paths`) minus the excludes, which also drop files from the
//! symbol scan. Errors whose identifier belongs to a rule the preset or `[analyse.rules]` turns off are dropped, then
//! `@pun-ignore` comments and the Pun baseline. Humans get problem blocks and the summary; `--format` (PHPStan's
//! `--error-format`) prints PHPStan's machine formats byte for byte (`analyse_report`). The exit code is PHPStan's:
//! 1 with errors, 0 without, and 2 when Pun couldn't run.

use crate::mapping::rule_of_identifier;
use crate::render::{Look, Renderer, RunError, Summary};
use crate::report::Abort;
use crate::{agent, analyse_report, baseline, github, render, root};
use anyhow::Result;
use clap::Args;
use pun_config::{Config, Options};
use pun_problem::ignore::Ignores;
use pun_problem::{Level, LineIndex, Problem, Span};
use pun_workspace::{Command, Exclusions};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Args, Default, Clone)]
pub(crate) struct AnalyseArgs {
    /// The files or directories to analyse (default: `[analyse] paths`, else `[files] paths`, else the project).
    paths: Vec<PathBuf>,
    /// The configuration that should be used.
    #[arg(long)]
    config: Option<PathBuf>,
    /// PHPStan's error formats: table, raw, json, prettyJson, checkstyle, junit, gitlab, github, teamcity.
    #[arg(long, alias = "error-format", value_name = "FORMAT")]
    format: Option<String>,
    /// Write every error to the baseline (`[analyse] baseline`, else pun-baseline.toml) and exit 0.
    #[arg(long, value_name = "FILE", num_args = 0..=1, default_missing_value = "")]
    generate_baseline: Option<PathBuf>,
    /// The number of threads (default: one per core).
    #[arg(long)]
    max_processes: Option<String>,
    /// Print each finding on one line.
    #[arg(long)]
    concise: bool,
    /// `raw`: add each error's identifier, as PHPStan's `-v` does.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// Print nothing but the machine format or the errors' summary line.
    #[arg(short, long)]
    quiet: bool,
    /// Colour the output: auto, always or never.
    #[arg(long, value_enum, value_name = "WHEN")]
    color: Option<render::ColorChoice>,
    /// Accepted for PHPStan compatibility; Pun shows progress only on a terminal after one second.
    #[arg(long, hide = true)]
    no_progress: bool,
    /// Accepted for PHPStan compatibility; Pun has no memory limit.
    #[arg(long, hide = true)]
    memory_limit: Option<String>,
    /// `GITHUB_ACTIONS=true`: human output gets GitHub annotations after it.
    #[arg(skip)]
    pub(crate) github_actions: bool,
}

impl AnalyseArgs {
    pub(crate) fn max_processes(&self) -> Option<&str> {
        self.max_processes.as_deref()
    }

    pub(crate) fn failure(&self) -> crate::report::Failure {
        let human = self.format.is_none() && !self.quiet && !agent::enabled();
        crate::report::Failure {
            pint: false,
            machine: !human,
            github_format: self.format.as_deref() == Some("github"),
            annotate: human && self.github_actions,
            color: self.color.unwrap_or(render::ColorChoice::Auto),
        }
    }
}

/// One error, as PHPStan reports it, with the Pun rule its identifier belongs to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Finding {
    /// Absolute, as PHPStan's `json` and `raw` print it.
    pub file: PathBuf,
    /// Relative to the project root (PHPStan's simple relative path, the cwd being the root).
    pub rel: String,
    pub line: u32,
    pub identifier: String,
    pub rule: String,
    pub message: String,
    pub tip: Option<String>,
}

/// What an analysis found.
pub(crate) struct Analysed {
    pub findings: Vec<Finding>,
    /// The files analysed.
    pub files: usize,
    /// Baseline entries (counts) no error matched any more.
    pub unmatched_baseline: usize,
    /// `@pun-ignore` comments without a rule ID, as problems (`ignore-comment`).
    pub malformed: Vec<Problem>,
    /// The analysed paths (absolute): the path arguments, else the configured paths, else the root.
    pub paths: Vec<PathBuf>,
}

pub(crate) fn load_config(root: &Path, config: Option<PathBuf>) -> Result<Config> {
    let opts = Options {
        config,
        ..Default::default()
    };
    pun_config::load(root, &opts).map_err(|e| {
        let message = format!("{e:#}");
        let error = RunError {
            path: None,
            line: None,
            message: message.clone(),
            hint: None,
        };
        Abort::pun(message, error).into()
    })
}

/// The project pun-analyse works on: the analysed files, Composer's autoloaded files for symbols (minus the
/// excludes), the target PHP version.
fn project(cfg: &Config, analysed: Vec<PathBuf>) -> Result<pun_analyse::Project> {
    use pun_phpstan::{Array, Key, Value};
    let mut params = Array::new();
    let mut extensions = Array::new();
    extensions.push(Value::Str("php".into()));
    params.set(Key::Str("fileExtensions".into()), Value::Array(extensions));
    params.set(
        Key::Str("phpVersion".into()),
        Value::Int(i64::from(php_version_id(&cfg.php))),
    );
    let mut project = pun_analyse::Project::from_config(&cfg.root, &params);
    let excluded = Exclusions::new(cfg, Command::Analyse)?;
    project.scanned.retain(|p| !excluded.is_excluded(p));
    project
        .autoloaded
        .retain(|a| !excluded.is_excluded(&a.path));
    project.analysed = analysed;
    Ok(project)
}

/// `8.4` as `PHP_VERSION_ID` (80400).
pub(crate) fn php_version_id(version: &str) -> u32 {
    let mut parts = version.split('.').map(|p| p.parse::<u32>().unwrap_or(0));
    let major = parts.next().unwrap_or(8);
    let minor = parts.next().unwrap_or(4);
    let patch = parts.next().unwrap_or(0);
    major * 10000 + minor * 100 + patch
}

/// Analyse `paths` (or the configured paths) under `cfg`: enabled rules only, `@pun-ignore` comments and the
/// baseline applied (unless `baseline` is false, for `--generate-baseline`).
pub(crate) fn analyse(cfg: &Config, paths: &[PathBuf], use_baseline: bool) -> Result<Analysed> {
    let files = pun_workspace::files(cfg, Some(Command::Analyse), paths)?;
    let count = files.len();
    let project = project(cfg, files)?;
    let errors = pun_analyse::run(&project).map_err(|e| {
        let message = e.to_string();
        let error = RunError {
            path: None,
            line: None,
            message: message.clone(),
            hint: None,
        };
        anyhow::Error::new(Abort::pun(message, error))
    })?;
    let on: std::collections::HashSet<&str> =
        cfg.analyse.rules.iter().map(|r| r.id.as_str()).collect();
    let mut findings: Vec<Finding> = errors
        .into_iter()
        .filter_map(|e| {
            let rule = rule_of_identifier(&e.identifier)?;
            on.contains(rule).then(|| {
                let file = PathBuf::from(&e.file);
                Finding {
                    rel: relative(&cfg.root, &file),
                    file,
                    line: e.line,
                    identifier: e.identifier,
                    rule: rule.to_string(),
                    message: e.message,
                    tip: e.tip,
                }
            })
        })
        .collect();
    let malformed = apply_ignores(&mut findings, &cfg.root);
    let mut unmatched_baseline = 0;
    if use_baseline && let Some(path) = &cfg.analyse.baseline {
        let b = baseline::Baseline::read(&cfg.root.join(path))?;
        unmatched_baseline = b.filter(&mut findings);
    }
    let configured: Vec<PathBuf> = match paths.is_empty() {
        false => paths.to_vec(),
        true => cfg.analyse.paths.iter().map(PathBuf::from).collect(),
    };
    let mut paths: Vec<PathBuf> = configured.iter().map(|p| cfg.root.join(p)).collect();
    if paths.is_empty() {
        paths.push(cfg.root.clone());
    }
    Ok(Analysed {
        findings,
        files: count,
        unmatched_baseline,
        malformed,
        paths,
    })
}

fn relative(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .into_owned()
}

/// Drop the findings `@pun-ignore` comments cover; the malformed comments come back as problems.
fn apply_ignores(findings: &mut Vec<Finding>, root: &Path) -> Vec<Problem> {
    let mut by_file: BTreeMap<PathBuf, Vec<Finding>> = BTreeMap::new();
    for f in findings.drain(..) {
        by_file.entry(f.file.clone()).or_default().push(f);
    }
    let mut malformed = vec![];
    for (file, mut list) in by_file {
        let source = std::fs::read(&file).unwrap_or_default();
        let source = String::from_utf8_lossy(&source);
        let mut ignores = Ignores::parse(&source);
        if !ignores.is_empty() {
            list.retain(|f| !ignores.ignores(&f.rule, f.line as usize));
            let rel = relative(root, &file);
            malformed.extend(ignores.malformed.iter().map(|m| ignore_problem(&rel, m)));
        }
        findings.extend(list);
    }
    findings.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    malformed
}

/// A malformed `@pun-ignore` comment, as an error.
/// The engine's rule for an `@pun-ignore` comment without a rule ID (rules.md §4).
pub(crate) const INVALID_IGNORE: &str = "invalid-ignore-comment";

pub(crate) fn ignore_problem(rel: &str, m: &pun_problem::ignore::Malformed) -> Problem {
    Problem {
        rule: INVALID_IGNORE.into(),
        path: rel.into(),
        span: m.span,
        message: m.message.clone(),
        help: Some("name the rule IDs it ignores: `@pun-ignore lint/group/rule`".into()),
        level: Level::Error,
        fix: None,
    }
}

/// A finding as a problem: at the first non-blank column of its line (PHPStan reports lines only), the tip as help.
pub(crate) fn problem(f: &Finding, source: &str, index: &LineIndex) -> Problem {
    let at = (index.line_range(f.line.max(1) as usize))
        .map(|r| r.start + source[r.clone()].len() - source[r].trim_start().len())
        .unwrap_or(0);
    Problem {
        rule: f.rule.clone(),
        path: f.rel.clone().into(),
        span: Span::new(at, at),
        message: f.message.clone(),
        help: f.tip.clone(),
        level: Level::Error,
        fix: None,
    }
}

pub(crate) fn run(a: AnalyseArgs) -> Result<u8> {
    let started = Instant::now();
    let root = root()?;
    let cfg = load_config(&root, a.config.clone())?;
    if let Some(path) = &a.generate_baseline {
        return generate_baseline(&a, &cfg, path, started);
    }
    let analysed = analyse(&cfg, &a.paths, true)?;
    let errors = analysed.findings.len() + analysed.malformed.len();
    match a.format.as_deref() {
        Some(format) => {
            let cx = analyse_report::Context {
                verbose: a.verbose > 0 || agent::enabled(),
                root: &root,
                paths: &analysed.paths,
            };
            print!(
                "{}",
                analyse_report::render(format, &analysed.findings, &cx)
                    .map_err(|e| unsupported_format(&e))?
            );
        }
        None if agent::enabled() => print!("{}", agent_report(&analysed)),
        None if a.quiet => {}
        None => {
            let look = Look::stdout(a.color.unwrap_or(render::ColorChoice::Auto));
            print!(
                "{}",
                human(&analysed, &root, look, a.concise, started.elapsed())
            );
            if a.github_actions {
                print!("{}", annotations(&analysed));
            }
        }
    }
    Ok(u8::from(errors > 0))
}

fn unsupported_format(e: &anyhow::Error) -> anyhow::Error {
    let message = e.to_string();
    let error = RunError {
        path: None,
        line: None,
        message: message.clone(),
        hint: Some(format!("use {}", analyse_report::FORMATS.join(", "))),
    };
    Abort::pun(message, error).into()
}

/// `--generate-baseline [file]`: every error into the baseline; nothing is reported, the exit code is 0.
fn generate_baseline(a: &AnalyseArgs, cfg: &Config, path: &Path, started: Instant) -> Result<u8> {
    let path = match path.as_os_str().is_empty() {
        false => path.to_path_buf(),
        true => PathBuf::from(cfg.analyse.baseline.as_deref().unwrap_or(baseline::DEFAULT)),
    };
    let analysed = analyse(cfg, &a.paths, false)?;
    let b = baseline::Baseline::of(&analysed.findings);
    b.write(&cfg.root.join(&path))?;
    let n = analysed.findings.len();
    let errors = agent::count(n, "error");
    let shown = path.display();
    if agent::enabled() {
        let detail = [format!("baseline {shown} written with {errors}")];
        print!("{}", agent::render("ok", &detail, "", &[], false));
        return Ok(0);
    }
    if a.quiet {
        return Ok(0);
    }
    let look = Look::stdout(a.color.unwrap_or(render::ColorChoice::Auto));
    let r = Renderer::new(look, &cfg.root);
    let mut message = format!("Baseline generated with {errors} in {shown}");
    if cfg.analyse.baseline.as_deref() != Some(path.to_string_lossy().as_ref()) {
        message.push_str(&format!(
            "; add `baseline = \"{shown}\"` under [analyse] in pun.toml to use it"
        ));
    }
    print!(
        "{}",
        r.done(&format!(
            "{message} ({:.2}s)",
            started.elapsed().as_secs_f64()
        ))
    );
    Ok(0)
}

/// Problem blocks, sorted by file and line, then the summary; a note when baseline entries no longer match.
pub(crate) fn human(
    analysed: &Analysed,
    root: &Path,
    look: Look,
    concise: bool,
    elapsed: Duration,
) -> String {
    let mut r = Renderer::new(look, root);
    r.concise = concise;
    let mut out = problem_blocks(&r, analysed);
    if !out.is_empty() {
        out.push('\n');
    }
    if analysed.unmatched_baseline > 0 {
        out.push_str(&baseline_note(&r, analysed.unmatched_baseline));
    }
    out.push_str(&r.summary(&Summary {
        errors: analysed.findings.len() + analysed.malformed.len(),
        warnings: 0,
        fixable: 0,
        fix_command: "",
        need_formatting: 0,
        files: analysed.files,
        elapsed,
    }));
    out
}

/// The problem blocks of an analysis (also `pun check`'s analyse part).
pub(crate) fn problem_blocks(r: &Renderer, analysed: &Analysed) -> String {
    let sources = sources(analysed);
    let mut problems: Vec<(Problem, &str, &LineIndex)> = vec![];
    for f in &analysed.findings {
        let (source, index) = &sources[&f.file];
        problems.push((problem(f, source, index), source, index));
    }
    for m in &analysed.malformed {
        let (source, index) = &sources[&root_join(analysed, &m.path)];
        problems.push((m.clone(), source, index));
    }
    problems.sort_by(|a, b| (&a.0.path, a.0.span.start).cmp(&(&b.0.path, b.0.span.start)));
    r.problems(problems.iter().map(|(p, s, i)| (p, *s, *i)))
}

/// The absolute path of a malformed comment's file: the findings carry absolute paths, malformed ones relative.
fn root_join(analysed: &Analysed, rel: &Path) -> PathBuf {
    (analysed.findings.iter())
        .find(|f| Path::new(&f.rel) == rel)
        .map_or_else(|| rel.to_path_buf(), |f| f.file.clone())
}

/// Each file with a finding or a malformed comment, read once: its source and line index.
fn sources(analysed: &Analysed) -> BTreeMap<PathBuf, (String, LineIndex)> {
    let mut out = BTreeMap::new();
    let files = (analysed.findings.iter().map(|f| f.file.clone())).chain(
        analysed
            .malformed
            .iter()
            .map(|m| root_join(analysed, &m.path)),
    );
    for file in files {
        out.entry(file.clone()).or_insert_with(|| {
            let source =
                String::from_utf8_lossy(&std::fs::read(&file).unwrap_or_default()).into_owned();
            let index = LineIndex::new(&source);
            (source, index)
        });
    }
    out
}

pub(crate) fn baseline_note(r: &Renderer, unmatched: usize) -> String {
    let entries = if unmatched == 1 { "entry" } else { "entries" };
    let glyph = r.look.paint(render::Role::Warning, r.look.glyph("⚠", "!"));
    format!(
        "{glyph} {unmatched} baseline {entries} no longer match: run `pun analyse --generate-baseline` to drop them\n"
    )
}

/// GitHub annotations after the human output (`GITHUB_ACTIONS=true`): one per finding, at its line.
pub(crate) fn annotations(analysed: &Analysed) -> String {
    let sources = sources(analysed);
    let mut out = String::new();
    for f in &analysed.findings {
        let (source, index) = &sources[&f.file];
        out.push_str(&github::problem(&problem(f, source, index), index));
    }
    for m in &analysed.malformed {
        let (_, index) = &sources[&root_join(analysed, &m.path)];
        out.push_str(&github::problem(m, index));
    }
    out
}

/// Agent mode: the summary line, then `path:line rule message` per finding.
fn agent_report(analysed: &Analysed) -> String {
    let items = agent_items(analysed);
    let errors = items.len();
    let (result, detail) = match errors {
        0 => ("pass", vec![]),
        n => {
            let files: std::collections::BTreeSet<&str> =
                analysed.findings.iter().map(|f| f.rel.as_str()).collect();
            let what = format!(
                "{} in {}",
                agent::count(n, "error"),
                agent::count(files.len(), "file")
            );
            ("fail", vec![what])
        }
    };
    let mut detail = detail;
    if analysed.unmatched_baseline > 0 {
        detail.push(format!(
            "{} no longer matched",
            agent::count(analysed.unmatched_baseline, "baseline entry")
                .replace("entrys", "entries")
        ));
    }
    let checked = format!("{} files checked", analysed.files);
    agent::render(result, &detail, &checked, &items, false)
}

/// One agent line per finding: `path:line rule message`.
pub(crate) fn agent_items(analysed: &Analysed) -> Vec<String> {
    let mut items: Vec<String> = (analysed.findings.iter())
        .map(|f| format!("{}:{} {} {}", f.rel, f.line, f.rule, f.message))
        .collect();
    items.extend(
        (analysed.malformed.iter())
            .map(|m| format!("{} {} {}", m.path.display(), m.rule, m.message)),
    );
    items
}
