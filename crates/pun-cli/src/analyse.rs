//! `pun analyse` (`pun analyze`): PHPStan's rules on pun-analyse, configured by pun.toml's `[analyse]`
//! (context/plan/analyse.md Build 1-2).
//!
//! The files come from `[analyse] paths` (or `[files] paths`) minus the excludes, which also drop files from the
//! symbol scan. Errors whose identifier belongs to a rule the preset or `[analyse.rules]` turns off are dropped, then
//! `@pun-ignore` comments and the Pun baseline. Humans get problem blocks and the summary; `--format` (PHPStan's
//! `--error-format`) prints PHPStan's machine formats byte for byte (`analyse_report`). The exit code is PHPStan's:
//! 1 with errors, 0 without, and 2 when Pun couldn't run.

use crate::render::{Look, Renderer, RunError, Summary};
use crate::report::Abort;
use crate::{agent, analyse_report, baseline, github, render, root};
use anyhow::Result;
use clap::Args;
use pun_config::rule_of_identifier;
use pun_config::{Config, Options};
use pun_phpstan::php::normalize_path;
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
    #[arg(long, value_name = "FILE", num_args = 0..=1)]
    generate_baseline: Option<Option<PathBuf>>,
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
    pub(crate) file: PathBuf,
    /// Relative to the project root (PHPStan's simple relative path, the cwd being the root).
    pub(crate) rel: String,
    pub(crate) line: u32,
    pub(crate) identifier: String,
    pub(crate) rule: String,
    pub(crate) message: String,
    pub(crate) tip: Option<String>,
}

impl Finding {
    /// PHPStan's `canBeIgnored()`: false for the errors about ignores themselves (`ignore.*`).
    pub(crate) fn ignorable(&self) -> bool {
        !self.identifier.starts_with("ignore.")
    }

    /// PHPStan's `getLine()`: none (0 here) for an `ignoreErrors` entry nothing matched.
    pub(crate) fn line(&self) -> Option<u32> {
        (self.line > 0).then_some(self.line)
    }
}

/// What an analysis found.
pub(crate) struct Analysed {
    pub(crate) findings: Vec<Finding>,
    /// PHPStan's not-file-specific errors: `[[analyse.ignore]]` entries without a file path nothing matched.
    pub(crate) general: Vec<String>,
    /// The files analysed.
    pub(crate) files: usize,
    /// Baseline entries (counts) no error matched any more.
    pub(crate) unmatched_baseline: usize,
    /// `@pun-ignore` comments without a rule ID, as problems (`ignore-comment`).
    pub(crate) malformed: Vec<Problem>,
    /// The analysed paths (absolute): the path arguments, else the configured paths, else the root.
    pub(crate) paths: Vec<PathBuf>,
}

pub(crate) fn load_config(root: &Path, config: Option<PathBuf>) -> Result<Config> {
    let opts = Options {
        config,
        ..Default::default()
    };
    pun_config::load(root, &opts).map_err(|e| run_error(format!("{e:#}"), None))
}

/// The engine's rule for PHPStan's errors about ignores (`ignore.unmatched`, `ignore.count`, ...), which no rule
/// reports.
const UNMATCHED_IGNORE: &str = "unmatched-ignore";

fn run_error(message: String, hint: Option<String>) -> anyhow::Error {
    let error = RunError {
        path: None,
        line: None,
        message: message.clone(),
        hint,
    };
    Abort::pun(message, error).into()
}

/// The project pun-analyse works on: the analysed files, Composer's autoloaded files for symbols (minus the
/// excludes), the target PHP version, and `[analyse]` as PHPStan's parameters (the level, `stubFiles`,
/// `ignoreErrors`, Larastan, rule options). Only the enabled rules' identifiers are reported.
fn project(cfg: &Config, analysed: Vec<PathBuf>) -> Result<pun_analyse::Project> {
    use pun_phpstan::ignore::IgnoredErrors;
    use pun_phpstan::{Array, Key, Value};
    let mut params = cfg.analyse.parameters(&cfg.root);
    pun_phpstan::config::validate_ignore_errors(&params).map_err(|e| run_error(e, None))?;
    let mut extensions = Array::new();
    extensions.push(Value::Str("php".into()));
    params.set(Key::Str("fileExtensions".into()), Value::Array(extensions));
    params.set(
        Key::Str("phpVersion".into()),
        Value::Int(i64::from(cfg.php_version().to_id())),
    );
    let mut project = pun_analyse::Project::from_config(&cfg.root, &params);
    let excluded = Exclusions::new(cfg, Command::Analyse)?;
    project.scanned.retain(|p| !excluded.is_excluded(p));
    project
        .autoloaded
        .retain(|a| !excluded.is_excluded(&a.path));
    project.analysed = analysed;
    let ignores = (params.get_str("ignoreErrors").and_then(Value::as_array)).cloned();
    project.ignore_errors =
        IgnoredErrors::new(&ignores.unwrap_or_default(), true).with_rules(rule_of_identifier);
    let on: std::collections::HashSet<String> =
        cfg.analyse.rules.iter().map(|r| r.id.clone()).collect();
    project.reported = Some(std::sync::Arc::new(move |id| {
        rule_of_identifier(id).is_some_and(|r| on.contains(r))
    }));
    Ok(project)
}

/// Analyse `paths` (or the configured paths) under `cfg`: enabled rules only, `@pun-ignore` comments and the
/// baseline applied (unless `baseline` is false, for `--generate-baseline`).
pub(crate) fn analyse(cfg: &Config, paths: &[PathBuf], use_baseline: bool) -> Result<Analysed> {
    let files = pun_workspace::files(cfg, Some(Command::Analyse), paths)?;
    let count = files.len();
    let project = project(cfg, files)?;
    let analysis =
        pun_analyse::analyse_project(&project).map_err(|e| run_error(e.to_string(), None))?;
    // pun-analyse reported the enabled rules' errors only, and the errors about ignores
    let mut findings: Vec<Finding> = (analysis.errors.into_iter())
        .map(|e| {
            let rule = rule_of_identifier(&e.identifier).unwrap_or(UNMATCHED_IGNORE);
            // PHPStan reports, sorts and relativises normalised paths (a configured `../x` included).
            let file = PathBuf::from(normalize_path(&e.file));
            Finding {
                rel: relative(&cfg.root, &file),
                file,
                // ponytail: PHPStan's -1 (a node without a position) prints as 0
                line: u32::try_from(e.line).unwrap_or(0),
                identifier: e.identifier,
                rule: rule.to_string(),
                message: e.message,
                tip: e.tip,
            }
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
    let mut paths: Vec<PathBuf> = (configured.iter())
        .map(|p| PathBuf::from(normalize_path(&cfg.root.join(p).to_string_lossy())))
        .collect();
    if paths.is_empty() {
        paths.push(cfg.root.clone());
    }
    Ok(Analysed {
        findings,
        general: analysis.general,
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
const INVALID_IGNORE: &str = "invalid-ignore-comment";

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
    // as PHPStan stops without a level (`pun check` runs the other commands instead)
    if cfg.analyse.rules.is_empty() {
        let hint = "set [analyse] preset, or turn rules on under [analyse.rules]";
        return Err(run_error("No rules detected".into(), Some(hint.into())));
    }
    if let Some(path) = &a.generate_baseline {
        return generate_baseline(&a, &cfg, path.as_deref(), started);
    }
    let analysed = analyse(&cfg, &a.paths, true)?;
    let errors = analysed.findings.len() + analysed.malformed.len() + analysed.general.len();
    match a.format.as_deref() {
        Some(format) => {
            let cx = analyse_report::Context {
                verbose: a.verbose > 0 || agent::enabled(),
                root: &root,
                paths: &analysed.paths,
                general: &analysed.general,
                config: (cfg.sources.iter())
                    .find(|s| s.extension().is_some_and(|e| e == "toml"))
                    .map(|s| relative(&root, s)),
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
    let hint = format!("use {}", analyse_report::FORMATS.join(", "));
    run_error(e.to_string(), Some(hint))
}

/// `--generate-baseline [file]`: every error into the baseline; nothing is reported, the exit code is 0.
fn generate_baseline(
    a: &AnalyseArgs,
    cfg: &Config,
    path: Option<&Path>,
    started: Instant,
) -> Result<u8> {
    let path = match path {
        Some(path) => path.to_path_buf(),
        None => PathBuf::from(cfg.analyse.baseline.as_deref().unwrap_or(baseline::DEFAULT)),
    };
    let mut analysed = analyse(cfg, &a.paths, false)?;
    // PHPStan's baseline leaves out the errors that can't be ignored
    analysed.findings.retain(Finding::ignorable);
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
    let glyph = r.look.paint(render::Role::Error, r.look.glyph("✖", "x"));
    for g in &analysed.general {
        out.push_str(&format!("{glyph} {g}\n"));
    }
    if !out.is_empty() {
        out.push('\n');
    }
    if analysed.unmatched_baseline > 0 {
        out.push_str(&baseline_note(&r, analysed.unmatched_baseline));
    }
    out.push_str(&r.summary(&Summary {
        errors: analysed.findings.len() + analysed.malformed.len() + analysed.general.len(),
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
    let (entries, verb, them) = match unmatched {
        1 => ("entry", "matches", "it"),
        _ => ("entries", "match", "them"),
    };
    let glyph = r.look.paint(render::Role::Warning, r.look.glyph("⚠", "!"));
    format!(
        "{glyph} {unmatched} baseline {entries} no longer {verb}: run `pun analyse --generate-baseline` to drop {them}\n"
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
    let checked = format!("{} checked", analysed.files);
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
    items.extend(
        analysed
            .general
            .iter()
            .map(|g| format!("{UNMATCHED_IGNORE} {g}")),
    );
    items
}
