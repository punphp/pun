//! `pun format` / `lint` / `check`: load the config, pick the engines, fix the files, report.
//!
//! A file goes through the native lint rules first (`pun_lint` on Mago's tree, with `@pun-ignore` comments and the
//! per-file levels of crate::lint_plan), then through one token pipeline (the format rules and the lint rules that
//! are PHP-CS-Fixer fixers, in PHP-CS-Fixer's priority order, as Pint runs them). `pun check` then analyses the
//! project (crate::analyse) and reports its errors with the rest.
use crate::lint_plan::{self, CliLevel, Engine, On, Plan};
use crate::render::{Look, RunError, Spinner};
use crate::report::{self, Abort, FileResult, Status};
use crate::{FormatArgs, root};
use anyhow::Result;
use pun_config::{Config, Options, Rule};
use pun_format::{LintError, Pipeline};
use pun_problem::ignore::{Ignores, covers};
use pun_problem::{LineIndex, Problem};
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Which rules a run applies. `fix`: `--fix` (every enabled lint rule's fix but those with `fix = "off"`; without
/// it every enabled lint rule is checked, nothing written).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage {
    Format,
    Lint {
        fix: bool,
    },
    /// Format and lint in one pass over each file, then analyse.
    Check {
        fix: bool,
    },
}

impl Stage {
    fn fix(self) -> bool {
        matches!(self, Stage::Lint { fix: true } | Stage::Check { fix: true })
    }
}

/// `pun lint` and `pun check`'s own flags.
#[derive(Clone, Default)]
pub(crate) struct LintOpts {
    /// `-A`, `-W` and `-D`, in the order given.
    pub(crate) levels: Vec<(CliLevel, String)>,
    pub(crate) deny_warnings: bool,
    pub(crate) max_warnings: Option<usize>,
    pub(crate) concise: bool,
}

/// What a file's token pass returned: the new code and the fixers that changed it, or why it failed.
type Run = Result<Option<(Vec<u8>, Vec<&'static str>)>, LintError>;

/// One file through the pipeline. The outer error aborts the run (a malformed `@php-cs-fixer-ignore` annotation).
fn run_file(p: &Pipeline, file: &Path, code: &[u8]) -> Result<Run> {
    let r = p.run(file, code);
    // Runner::fixFile analyses annotations after linting the input, before any fixer runs.
    if !matches!(
        r,
        Err(LintError {
            after_fix: false,
            ..
        })
    ) {
        p.check_annotations(file, code)?;
    }
    Ok(r.map(|f| f.map(|f| (f.code, f.applied))))
}

pub(crate) fn load_config(a: &FormatArgs, root: &Path) -> Result<Config> {
    let opts = Options {
        config: a.config.clone(),
        no_config: a.no_config,
        preset: a.preset.clone(),
        blade: a.blade,
    };
    load(root, &opts)
}

/// `pun.toml` (or the given config) under `root`; an error is a config error.
pub(crate) fn load(root: &Path, opts: &Options) -> Result<Config> {
    pun_config::load(root, opts).map_err(config_error)
}

/// Hands a native rule its options from `[lint.rules]`. Overrides change levels only, as one instance of the rule
/// lints every file.
fn configure(cfg: &Config, rule: &mut dyn pun_lint::Rule) -> Result<()> {
    let id = rule.id();
    let base = (cfg.lint.rules.iter()).find(|r| r.id == id);
    let overridden = (cfg.lint.overrides.iter())
        .flat_map(|o| &o.rules)
        .any(|s| s.id == id && !s.options.is_null());
    let result = match overridden {
        true => Err("takes options in [lint.rules] only, not in [[lint.overrides]]".to_string()),
        false => rule.configure(base.map_or(&serde_json::Value::Null, |r| &r.options)),
    };
    result.map_err(|e| config_error(anyhow::anyhow!("pun.toml: `{id}` {e}")))
}

/// A config error: Pun's own run error, never a Pint box (Pint never reads pun.toml).
fn config_error(e: anyhow::Error) -> anyhow::Error {
    Abort::plain(format!("{e:#}"), None).into()
}

/// The token pipelines of a run, one per distinct set of token lint rules (overrides and `@pun-ignore-file` change
/// the set per file), each with the run's format rules ahead of them when the file is formatted.
struct Pipelines {
    cfg: Config,
    format: Vec<Rule>,
    built: Mutex<HashMap<Vec<String>, Arc<Pipeline>>>,
}

impl Pipelines {
    fn new(cfg: &Config, format: Vec<Rule>) -> Pipelines {
        Pipelines {
            cfg: cfg.clone(),
            format,
            built: Mutex::new(HashMap::new()),
        }
    }

    /// The pipeline of the format rules (none when `with_format` is false) and `lint`.
    fn get(&self, with_format: bool, lint: &[Rule]) -> Result<Arc<Pipeline>> {
        let mut key = vec![with_format.to_string()];
        key.extend(lint.iter().map(|r| format!("{}={}", r.name, r.options)));
        if let Some(p) = self.built.lock().expect("pipelines").get(&key) {
            return Ok(p.clone());
        }
        let format = if with_format { &self.format[..] } else { &[] };
        let rules = format.iter().chain(lint).cloned().collect();
        let p = Arc::new(pipeline(&Config {
            rules,
            ..self.cfg.clone()
        })?);
        self.built.lock().expect("pipelines").insert(key, p.clone());
        Ok(p)
    }
}

/// Everything a run checks a file with.
struct Engines<'a> {
    stage: Stage,
    /// Lint and check only.
    plan: Option<Plan>,
    /// The native rules on for some file, when there are any.
    linter: Option<pun_lint::Linter>,
    pipelines: Pipelines,
    /// The files lint checks (`[lint] exclude`, no Blade); `None` for `pun format`.
    lint_files: Option<HashSet<PathBuf>>,
    /// The files `pun check` formats; `None` when every file is formatted (format) or none is (lint).
    format_files: Option<HashSet<PathBuf>>,
    root: &'a Path,
}

/// A file's run: its result and, under `--fix`, the fixes of the token lint rules it doesn't apply.
struct Processed {
    result: FileResult,
    remaining: Option<FileResult>,
}

pub(crate) fn format(a: FormatArgs, stage: Stage, lint: LintOpts) -> Result<u8> {
    let started = std::time::Instant::now();
    let root = root()?;
    let cfg = load_config(&a, &root)?;
    // No PHP fallback: a rule Pun can't run, format or named lint rule, stops every command (exit 2, Pun can't
    // run). A set or a group skips the rules Pun can't run yet.
    let plan = Plan::new(&cfg, &lint.levels).map_err(config_error)?;
    let mut unported = Pipeline::new(&cfg.rules, &pun_fixers::all)?.not_ported;
    let plan = match plan {
        Ok(plan) if unported.is_empty() => plan,
        Ok(_) => return Err(unported_error(&a, &cfg, &unported).into()),
        Err(lint) => {
            unported.extend(lint);
            return Err(unported_error(&a, &cfg, &unported).into());
        }
    };
    let lints = stage != Stage::Format;
    let formats = !matches!(stage, Stage::Lint { .. });
    // EnsurePrettierIsConfigured: `"Pint/laravel_blade": true` (or --blade) needs Node and prettier in the project.
    let blade_on = formats
        && (cfg.rules.iter()).any(|r| r.name == "Pint/laravel_blade" && r.options.is_null());
    // Also clean up on discovery/configuration errors and early --bail exits, including repeated watch runs.
    struct StopProbe;
    impl Drop for StopProbe {
        fn drop(&mut self) {
            pun_fixers::prettier::stop();
        }
    }
    let _probe = StopProbe;
    if let Some(abort) = crate::prettier::ensure(&root, blade_on) {
        return Err(abort.into());
    }
    let pipelines = Pipelines::new(&cfg, if formats { cfg.rules.clone() } else { vec![] });
    // The pipeline of a file no override matches: `--explain` and stdin.
    let base_token = match lints {
        true => token_rules(&plan.for_file(&root.join("-")), stage.fix(), false),
        false => vec![],
    };
    let base = pipelines.get(formats, &base_token)?;
    if a.pun.explain {
        explain(&cfg, &base);
    }

    let stdin_path = a.paths.first().is_some_and(|p| p.as_os_str() == "-");
    if a.pun.stdin || stdin_path || a.stdin_filename.is_some() {
        if !a.stdin_filename.as_ref().is_some_and(|f| is_blade(f)) || a.test || a.bail {
            pun_fixers::prettier::stop();
        }
        return stdin(&a, &cfg, &base);
    }

    let dry_run = a.test || a.bail;
    let selection = if a.dirty {
        Some(pun_workspace::GitSelect::Dirty)
    } else if let Some(b) = &a.diff {
        Some(pun_workspace::GitSelect::Diff(b.clone()))
    } else if a.pun.staged {
        Some(pun_workspace::GitSelect::Staged)
    } else {
        a.pun
            .since
            .as_ref()
            .map(|r| pun_workspace::GitSelect::Since(r.clone()))
    };
    let selected = match &selection {
        Some(sel) => Some(pun_workspace::git_paths(&cfg, sel)?),
        None => None,
    };
    let format_set = match (formats, &selected) {
        (false, _) => vec![],
        (true, Some(files)) => files.clone(),
        (true, None) => pun_workspace::discover(&cfg, &a.paths)?,
    };
    let lint_set = match (lints, &selected) {
        (false, _) => vec![],
        // A git selection, narrowed to the lint file set.
        (true, Some(files)) => {
            let all: HashSet<PathBuf> =
                pun_workspace::files(&cfg, Some(pun_workspace::Command::Lint), &[])?
                    .into_iter()
                    .collect();
            files.iter().filter(|f| all.contains(*f)).cloned().collect()
        }
        (true, None) => pun_workspace::files(&cfg, Some(pun_workspace::Command::Lint), &a.paths)?,
    };
    let mut files = format_set.clone();
    let known: HashSet<&PathBuf> = format_set.iter().collect();
    files.extend(lint_set.iter().filter(|f| !known.contains(f)).cloned());
    drop(known);
    let natives: Vec<Box<dyn pun_lint::Rule>> = match lints {
        true => {
            let on = plan.mentioned();
            let mut natives: Vec<_> = (pun_lint::rules::all().into_iter())
                .filter(|r| on.contains(r.id()))
                .collect();
            for r in &mut natives {
                configure(&cfg, r.as_mut())?;
            }
            if !natives.is_empty() {
                // the lint files are the project's when no path or git selection narrows them
                let project = match (&selected, a.paths.is_empty()) {
                    (None, true) => lint_set.clone(),
                    _ => pun_workspace::files(&cfg, Some(pun_workspace::Command::Lint), &[])?,
                };
                project_codebase(&cfg, project);
            }
            natives
        }
        false => vec![],
    };
    let engines = Engines {
        stage,
        linter: (!natives.is_empty())
            .then(|| pun_lint::Linter::new(natives, cfg.php_version().to_id())),
        lint_files: lints.then(|| lint_set.into_iter().collect()),
        format_files: match stage {
            Stage::Check { .. } => Some(format_set.into_iter().collect()),
            _ => None,
        },
        plan: lints.then_some(plan),
        pipelines,
        root: &root,
    };

    // The probe's worker is already loading Prettier. Stop it before the PHP pass if no Blade file is selected.
    if blade_on && !files.iter().any(|f| is_blade(f)) {
        pun_fixers::prettier::stop();
    }
    let look = Look::stdout(a.color());
    // The progress line, for humans on a terminal (output.md §1); dropped before anything is printed.
    let human = a.format.is_none() && !a.quiet && !crate::agent::enabled();
    let spinner = human
        .then(|| Spinner::stderr(files.len(), &look, std::io::stderr().is_terminal()))
        .flatten();
    let process = |path: &PathBuf, code: std::io::Result<Vec<u8>>| -> Result<Processed> {
        let r = process_file(&a, &engines, dry_run, path, code);
        if let Some(s) = &spinner {
            s.tick();
        }
        r
    };

    let all: Vec<Result<Processed>> = if a.bail {
        // --bail is PHP-CS-Fixer's stop-on-violation: sequential, stop at the first fixable file (lint errors don't
        // stop it; an aborting file does).
        let mut out = Vec::new();
        for f in &files {
            let r = process(f, std::fs::read(f));
            let stop = (r.as_ref()).map_or(true, |r| r.result.status == Status::Fixed);
            out.push(r);
            if stop {
                break;
            }
        }
        out
    } else {
        // main sizes rayon's global pool by `--max-processes`. Blade templates go through Node workers, which are
        // slow to start and to warm up: a few threads of their own, each with one worker, format the templates,
        // largest first, beside rayon's pass over the PHP files.
        let (mut blade, php): (Vec<_>, Vec<_>) =
            (files.iter().cloned().enumerate()).partition(|(_, f)| blade_on && is_blade(f));
        blade.sort_by_cached_key(|(_, f)| {
            std::cmp::Reverse(std::fs::metadata(f).map_or(0, |m| m.len()))
        });
        // A Node worker takes about 0.4 s to start and warm up, so it needs some 30 templates to pay for itself:
        // on Pinkary's 142, four workers beat both two and eight at any thread count.
        let lanes = (blade.len() / 32)
            .clamp(1, rayon::current_num_threads())
            .min(blade.len());
        let next = std::sync::atomic::AtomicUsize::new(0);
        let php_paths: Vec<PathBuf> = php.iter().map(|(_, f)| f.clone()).collect();
        let (mut done, php_done) = std::thread::scope(|s| {
            let lanes: Vec<_> = (0..lanes)
                .map(|_| {
                    s.spawn(|| {
                        let mut out = vec![];
                        while let Some((i, f)) =
                            blade.get(next.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
                        {
                            out.push((*i, process(f, std::fs::read(f))));
                        }
                        out
                    })
                })
                .collect();
            let php_done = read_in_parallel(&php_paths, process);
            let done: Vec<_> = (lanes.into_iter())
                .flat_map(|h| h.join().unwrap_or_else(|p| std::panic::resume_unwind(p)))
                .collect();
            (done, php_done)
        });
        done.extend(php.iter().map(|(i, _)| *i).zip(php_done));
        done.sort_by_key(|d| d.0);
        done.into_iter().map(|d| d.1).collect()
    };
    drop(spinner);
    let processed: Vec<Processed> = all.into_iter().collect::<Result<_>>()?;

    let mut remaining = vec![];
    let mut results = vec![];
    for p in processed {
        remaining.extend(p.remaining);
        results.push(p.result);
    }

    let mut unmatched_baseline = 0;
    let mut total = files.len();
    // `pun check` analyses after the files are fixed, so it reads what was written. A git selection that selects
    // nothing analyses nothing. ponytail: pun-analyse reads each file again; a `run` that takes the sources would
    // share them (request row in PLAN.md).
    if let Stage::Check { .. } = stage
        && !a.bail
        && selected.as_ref().is_none_or(|s| !s.is_empty())
    {
        let paths = selected.as_deref().unwrap_or(&a.paths);
        let analysed = crate::analyse::analyse(&cfg, paths, true)?;
        unmatched_baseline = analysed.unmatched_baseline;
        total = total.max(analysed.files);
        add_findings(&mut results, &analysed, &root);
    }

    let ctx = report::Ctx {
        total,
        dry_run,
        remaining: &remaining,
        verbose: a.verbose > 0,
        ansi: a.ansi,
        no_ansi: a.no_ansi,
        look,
        root: &root,
        elapsed: started.elapsed(),
        github: a.github_actions && human,
        concise: lint.concise,
        unmatched_baseline,
    };
    report::emit(
        &ctx,
        &results,
        a.format.as_deref(),
        a.quiet,
        a.output_to_file.as_deref(),
        a.output_format.as_deref(),
    )?;
    // A fixer that threw or produced invalid code is Pun failing, not the file (exit 2).
    let crashed = (results.iter()).any(|r| matches!(r.status, Status::Exception | Status::Lint));
    let changed = results.iter().any(|r| r.status == Status::Fixed);
    let failed = match stage {
        Stage::Format => {
            ((dry_run || a.repair) && changed) || results.iter().any(FileResult::is_error)
        }
        _ => {
            let c = report::counts(&ctx, &results);
            c.errors > 0
                || c.need_formatting > 0
                || (a.repair && changed)
                || (lint.deny_warnings && c.warnings > 0)
                || lint.max_warnings.is_some_and(|m| c.warnings > m)
        }
    };
    eprint!("{}", base.profile_report());
    Ok(if crashed { 2 } else { u8::from(failed) })
}

/// The rules that read facts look classes up in the project's other files, as Rector's PHPStan reflection does: scan
/// every file `pun lint` would check (not only the ones named on the command line) into the codebase they share, with
/// the classes they name that Composer autoloads (the generated classmap, then psr-4 and psr-0 from composer.json and
/// vendor/composer/installed.json as pun-analyse finds them), so a parent class from a package is known as in
/// Rector's PHPStan.
/// Composer's `files` entries (vendor/composer/autoload_files.php, the helper functions such as Laravel's `collect()`)
/// are scanned too, as PHPStan finds them through the runtime autoloader.
/// `files` are the files `pun lint` would check. The scan waits for the first rule that reads facts
/// (`facts::set_project_scan`), so rules that read none never scan.
fn project_codebase(cfg: &Config, files: Vec<PathBuf>) {
    let cfg = cfg.clone();
    pun_lint::rules::facts::set_project_scan(move || {
        // A rule asks on a worker of rayon's global pool, whose other workers wait for the scan: it runs on a thread
        // and a pool of its own, sized as the global one.
        let threads = rayon::current_num_threads();
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build();
        let pool = pool.expect("the project scan's thread pool");
        std::thread::scope(|s| {
            s.spawn(|| pool.install(|| scan_project(&cfg, &files)))
                .join()
        })
        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    });
}

/// The project's codebase, [`project_codebase`] of `files`.
fn scan_project(cfg: &Config, files: &[PathBuf]) -> Arc<pun_lint::rules::facts::ProjectCodebase> {
    use pun_lint::rules::facts;
    use pun_phpstan::{Array, Key, Value};
    // The rules read PHPStan's built-in signatures right after the scan: they load beside it, on a core the scan's
    // serial steps leave idle.
    std::thread::spawn(|| pun_phpstan::symbols::symbols(pun_phpstan::symbols::PhpVersion::Php84));
    let read = || -> Vec<(String, String)> {
        (files.par_iter())
            .filter_map(|f| {
                let rel = f.strip_prefix(&cfg.root).unwrap_or(f);
                Some((
                    rel.to_string_lossy().into_owned(),
                    std::fs::read_to_string(f).ok()?,
                ))
            })
            .collect()
    };
    let autoloaded = || {
        let mut extensions = Array::new();
        extensions.push(Value::Str("php".into()));
        let mut params = Array::new();
        params.set(Key::Str("fileExtensions".into()), Value::Array(extensions));
        let project = pun_analyse::Project::from_config(&cfg.root, &params);
        // the runtime autoloader Rector loads looks in Composer's generated classmap first
        (project, composer_classmap(&cfg.root))
    };
    let (sources, (project, classmap)) = rayon::join(read, autoloaded);
    let mut sources = sources;
    for path in composer_files(&cfg.root) {
        if let Ok(code) = std::fs::read_to_string(&path) {
            sources.push((path.to_string_lossy().into_owned(), code));
        }
    }
    // and `vendor/autoload.php` requires Composer's own ClassLoader, which no autoload map lists
    let class_loader = cfg.root.join("vendor/composer/ClassLoader.php");
    let mut by_class: HashMap<&[u8], &Path> = HashMap::new();
    by_class.insert(b"composer\\autoload\\classloader", &class_loader);
    for (class, path) in &classmap {
        by_class.insert(class, path);
    }
    for a in &project.autoloaded {
        by_class.entry(&a.class).or_insert(&a.path);
    }
    let psr0 = project.autoloaded.iter().any(|a| a.psr0);
    let autoload = |class: &[u8]| {
        let path = by_class.get(class).or_else(|| {
            // psr-0: `_` in the class's own name is a directory too
            let at = class.iter().rposition(|b| *b == b'\\').map_or(0, |i| i + 1);
            let mut dirs = class.to_vec();
            dirs[at..]
                .iter_mut()
                .filter(|b| **b == b'_')
                .for_each(|b| *b = b'\\');
            psr0.then(|| by_class.get(dirs.as_slice())).flatten()
        })?;
        Some((
            path.to_string_lossy().into_owned(),
            std::fs::read_to_string(path).ok()?,
        ))
    };
    facts::project_codebase_autoloaded(&sources, cfg.php_version().to_id(), &autoload)
}

/// The classes of `vendor/composer/autoload_classmap.php` (Composer's `classmap` entries, and every class when the
/// autoloader is optimized), ASCII-lowercased, and their files.
fn composer_classmap(root: &Path) -> Vec<(Vec<u8>, PathBuf)> {
    (composer_map(root, "autoload_classmap.php").into_iter())
        .map(|(class, path)| {
            let class = class.replace("\\\\", "\\").to_ascii_lowercase();
            (class.into_bytes(), path)
        })
        .collect()
}

/// The files of `vendor/composer/autoload_files.php` that packages declare (Composer's `files` entries), in the order
/// Composer loads them. The project's own are among the files `pun lint` checks.
fn composer_files(root: &Path) -> Vec<PathBuf> {
    let vendor = root.join("vendor");
    (composer_map(root, "autoload_files.php").into_iter())
        .map(|(_, path)| path)
        .filter(|path| path.starts_with(&vendor))
        .collect()
}

/// The `'key' => $vendorDir . '/path',` lines of one of Composer's generated `vendor/composer/<file>` arrays, with
/// each path under the project or its vendor directory.
fn composer_map(root: &Path, file: &str) -> Vec<(String, PathBuf)> {
    let vendor = root.join("vendor");
    // lossy: a class name need not be UTF-8 (symfony/cache names one `\xA9`)
    let code = std::fs::read(vendor.join("composer").join(file)).unwrap_or_default();
    (String::from_utf8_lossy(&code).lines())
        .filter_map(|line| {
            // `'App\\Model' => $baseDir . '/app/Model.php',`
            let (class, path) = line.trim().strip_prefix('\'')?.split_once("' => $")?;
            let (dir, path) = path.strip_suffix("',")?.split_once(" . '/")?;
            // the other is `$baseDir`, the project
            let base = if dir == "vendorDir" {
                vendor.as_path()
            } else {
                root
            };
            Some((class.to_string(), base.join(path)))
        })
        .collect()
}

/// The threads that read the files (context/plan/lint.md, perf lint 4): opening files on more than about four
/// threads at once slows each open on macOS, so a few threads read and every core checks what they read.
const READERS: usize = 3;

/// `process` over every file, its content read by [`READERS`] threads of their own and checked on rayon's pool
/// (sized by `--max-processes` in main) as it arrives; the results keep the files' order.
fn read_in_parallel<T: Send>(
    files: &[PathBuf],
    process: impl Fn(&PathBuf, std::io::Result<Vec<u8>>) -> T + Sync,
) -> Vec<T> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let next = AtomicUsize::new(0);
    let (tx, rx) = std::sync::mpsc::sync_channel(64);
    std::thread::scope(|s| {
        for _ in 0..READERS.min(files.len()) {
            let (tx, next) = (tx.clone(), &next);
            s.spawn(move || {
                // Stops early only when the receiver is gone (the processing panicked).
                while let Some(path) = files.get(next.fetch_add(1, Ordering::Relaxed))
                    && tx.send((path, std::fs::read(path))).is_ok()
                {}
            });
        }
        drop(tx);
        let index: HashMap<&PathBuf, usize> =
            files.iter().enumerate().map(|(i, f)| (f, i)).collect();
        let mut out: Vec<(usize, T)> = (rx.into_iter().par_bridge())
            .map(|(path, code)| (index[path], process(path, code)))
            .collect();
        out.sort_by_key(|o| o.0);
        out.into_iter().map(|o| o.1).collect()
    })
}

/// The token lint rules of `on`, as fixer rules: under `--fix` those whose fix applies (`pending`: those whose fix
/// doesn't), otherwise all of them.
fn token_rules(on: &BTreeMap<String, On>, fix: bool, pending: bool) -> Vec<Rule> {
    (on.iter())
        .filter_map(|(id, o)| match lint_plan::engine(id) {
            Some(Engine::Token(fixer)) if (!fix && !pending) || (fix && o.fix != pending) => {
                Some(Rule {
                    name: fixer.to_string(),
                    options: o.options.clone(),
                })
            }
            _ => None,
        })
        .collect()
}

/// A fixed native problem's rule ID as the `'static` string the reports list fixers by: a native rule, else the
/// unused-ignore check (a malformed ignore has no fix).
fn static_id(rule: &str) -> &'static str {
    (lint_plan::native_rules().get(rule).copied()).unwrap_or(lint_plan::UNUSED_IGNORE)
}

/// One round of the native rules' problems, as the file's levels and comments leave them: rules that are off for
/// the file dropped, levels set, fixes taken from `fix = "off"` rules, `@pun-ignore` comments applied, and the
/// engine's own problems (malformed and unused ignore comments) added.
fn filter_problems(
    on: &BTreeMap<String, On>,
    rel: &str,
    source: &str,
    problems: &mut Vec<Problem>,
) {
    problems.retain(|p| on.contains_key(&p.rule));
    for p in problems.iter_mut() {
        let o = &on[&p.rule];
        p.level = o.level;
        if !o.fix {
            p.fix = None;
        }
    }
    let mut ignores = Ignores::parse(source);
    if ignores.is_empty() {
        return;
    }
    ignores.retain(problems, &LineIndex::new(source));
    // An ID of a token rule, of another command or of a rule that is off can't be judged.
    let runs = |id: &str, native: bool| {
        (on.keys()).any(|r| {
            covers(id, r)
                && match lint_plan::engine(r) {
                    Some(Engine::Native) => native,
                    Some(Engine::Token(_)) => !native,
                    _ => false,
                }
        })
    };
    if let Some(o) = on.get(lint_plan::UNUSED_IGNORE) {
        for u in ignores.unused(source, |id| runs(id, true) && !runs(id, false)) {
            problems.push(Problem {
                rule: lint_plan::UNUSED_IGNORE.into(),
                path: rel.into(),
                span: u.span,
                message: format!("`{}` suppresses no problem here", u.id),
                help: Some("remove it".into()),
                level: o.level,
                fix: o.fix.then_some(u.fix),
            });
        }
    }
    problems.extend((ignores.malformed.iter()).map(|m| crate::analyse::ignore_problem(rel, m)));
    problems.sort_by_key(|p| p.span.start);
}

/// One file: native lint, the token pass, the pending rules under `--fix`; the result written unless a dry run.
/// Unreadable and unwritable files abort the whole run, like Pint (FileReader / Runner exceptions).
fn process_file(
    a: &FormatArgs,
    e: &Engines,
    dry_run: bool,
    path: &PathBuf,
    code: std::io::Result<Vec<u8>>,
) -> Result<Processed> {
    let rel = path
        .strip_prefix(e.root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned();
    let mut r = FileResult::new(path.clone(), rel.clone());
    let done = |result: FileResult| Processed {
        result,
        remaining: None,
    };
    // Runner FileFilterIterator: symlinks are dropped silently, empty files are "skipped".
    if path.is_symlink() {
        r.status = Status::Link;
        return Ok(done(r));
    }
    let code = code.map_err(|_| crate::pint_error::PintError::Read(path.clone()))?;
    if code.is_empty() {
        r.status = Status::Skipped;
        return Ok(done(r));
    }
    let in_format = match &e.format_files {
        Some(f) => f.contains(path),
        None => !matches!(e.stage, Stage::Lint { .. }),
    };
    let in_lint = e.lint_files.as_ref().is_some_and(|f| f.contains(path));
    let on = match (in_lint, &e.plan) {
        (true, Some(plan)) => plan.for_file(path),
        _ => BTreeMap::new(),
    };
    let text = std::str::from_utf8(&code).ok();
    // A token rule changes a whole file, so only `@pun-ignore-file` turns it off.
    let mut file_ignores = match text {
        Some(t) if !on.is_empty() => Ignores::parse(t),
        _ => Ignores::default(),
    };
    let token_on: BTreeMap<String, On> = (on.iter())
        .filter(|(id, _)| matches!(lint_plan::engine(id), Some(Engine::Token(_))))
        .filter(|(id, _)| !file_ignores.ignores_file(id))
        .map(|(id, o)| (id.clone(), o.clone()))
        .collect();
    let fix = e.stage.fix();
    let rel_path = Path::new(&rel);

    // 1. The native rules.
    let mut now = code.clone();
    let mut applied: Vec<&'static str> = vec![];
    let mut problems: Vec<Problem> = vec![];
    let mut native_syntax_error = false;
    let native_on = (on.keys()).any(|id| {
        matches!(
            lint_plan::engine(id),
            Some(Engine::Native | Engine::Builtin)
        )
    });
    if let (Some(linter), Some(text), true) = (&e.linter, text, native_on) {
        let filter = |src: &str, ps: &mut Vec<Problem>| filter_problems(&on, &rel, src, ps);
        let outcome = match fix {
            true => linter.fix_with(rel_path, text, filter).map(|f| {
                for p in &f.applied {
                    let id = static_id(&p.rule);
                    if !applied.contains(&id) {
                        applied.push(id);
                    }
                }
                now = f.source.into_bytes();
                f.remaining
            }),
            false => linter.check(rel_path, text).map(|mut ps| {
                filter_problems(&on, &rel, text, &mut ps);
                ps
            }),
        };
        match outcome {
            Ok(ps) => problems = ps,
            // The token pass (or PHP's grammar below) reports the syntax error.
            Err(pun_lint::LintError::Syntax { .. }) => native_syntax_error = true,
            Err(err) => {
                r.status = Status::Lint;
                r.message = err.to_string();
                return Ok(done(r));
            }
        }
    }

    // 2. The token pass: the format rules and the token lint rules.
    let token = token_rules(&token_on, fix, false);
    if in_format || !token.is_empty() {
        let pipeline = e.pipelines.get(in_format, &token)?;
        let outcome = match isolated(|| run_file(&pipeline, path, &now)) {
            Ok(run) => run?,
            Err(message) => {
                r.status = Status::Exception;
                r.message = message;
                return Ok(done(r));
            }
        };
        match outcome {
            Err(err) => {
                r.status = if err.after_fix {
                    Status::Lint
                } else {
                    Status::Invalid
                };
                r.message = crate::console::carry(&err.message);
                return Ok(done(r));
            }
            Ok(None) => {}
            Ok(Some((new, fixers))) => {
                applied.extend(fixers);
                // The native problems' spans refer to the source before the token pass.
                if !problems.is_empty()
                    && let (Some(linter), Ok(t)) = (&e.linter, std::str::from_utf8(&new))
                    && let Ok(mut ps) = linter.check(rel_path, t)
                {
                    filter_problems(&on, &rel, t, &mut ps);
                    problems = ps;
                }
                now = new;
            }
        }
    } else if native_syntax_error && let Err(err) = pun_format::lint(&code) {
        r.status = Status::Invalid;
        r.message = crate::console::carry(&err.message);
        return Ok(done(r));
    }
    let warned: Vec<&'static str> = (token_on.iter())
        .filter(|(_, o)| o.level == pun_problem::Level::Warning)
        .filter_map(|(id, _)| lint_plan::token_rules().get(id.as_str()).copied())
        .collect();
    r.warned.clone_from(&warned);

    // Reports show diffs only with -v (sebastian/diff's LCS is quadratic, so skip it otherwise).
    let diff = |old: &[u8], new: &[u8]| {
        if a.verbose == 0 {
            return String::new();
        }
        report::diff_bytes(
            old,
            new,
            // Agent mode: paths relative to the project, as everywhere in its output.
            if crate::agent::enabled() {
                rel_path
            } else {
                path
            },
        )
    };
    if now != code {
        r.diff = diff(&code, &now);
        r.line = crate::github::first_changed_line(&code, &now);
        r.applied = applied;
        r.status = Status::Fixed;
        if !dry_run {
            std::fs::write(path, &now)
                .map_err(|_| crate::pint_error::PintError::Write(path.clone()))?;
        }
    }
    if !problems.is_empty() {
        r.source = String::from_utf8_lossy(&now).into_owned();
        r.problems = problems;
    }

    // 3. `--fix`: the token lint rules it doesn't apply, run on the result and reported. Their failure (lint
    // error, panic) is not reported: the file was already fixed and checked.
    let pending = match fix {
        true => token_rules(&token_on, true, true),
        false => vec![],
    };
    let left = match pending.is_empty() {
        true => None,
        false => {
            let p = e.pipelines.get(false, &pending)?;
            isolated(|| run_file(&p, path, &now))
                .ok()
                .transpose()?
                .and_then(Result::ok)
                .flatten()
        }
    };
    let remaining = left.map(|(new, fixers)| {
        let mut f = FileResult::new(path.clone(), rel.clone());
        f.diff = diff(&now, &new);
        f.line = crate::github::first_changed_line(&now, &new);
        f.applied = fixers;
        f.status = Status::Fixed;
        f.warned = warned;
        f
    });
    Ok(Processed {
        result: r,
        remaining,
    })
}

/// `pun check`: the analysis's errors as problems of their files (a file only analyse covers gets a result of its
/// own). A malformed ignore comment lint already reported is not repeated.
fn add_findings(results: &mut Vec<FileResult>, analysed: &crate::analyse::Analysed, root: &Path) {
    let mut at: HashMap<PathBuf, usize> = (results.iter().enumerate())
        .map(|(i, r)| (r.path.clone(), i))
        .collect();
    let mut index_of = |results: &mut Vec<FileResult>, file: &Path| -> usize {
        let i = *at.entry(file.to_path_buf()).or_insert_with(|| {
            let rel = file.strip_prefix(root).unwrap_or(file).to_string_lossy();
            results.push(FileResult::new(file.to_path_buf(), rel.into_owned()));
            results.len() - 1
        });
        let r = &mut results[i];
        if r.source.is_empty() {
            r.source =
                String::from_utf8_lossy(&std::fs::read(file).unwrap_or_default()).into_owned();
        }
        i
    };
    for f in &analysed.findings {
        let i = index_of(results, &f.file);
        let r = &mut results[i];
        let p = crate::analyse::problem(f, &r.source, &LineIndex::new(&r.source));
        r.problems.push(p);
    }
    for m in &analysed.malformed {
        let i = index_of(results, &root.join(&m.path));
        let r = &mut results[i];
        if !r
            .problems
            .iter()
            .any(|q| q.rule == m.rule && q.span == m.span)
        {
            r.problems.push(m.clone());
        }
    }
    for r in results.iter_mut() {
        r.problems.sort_by_key(|p| p.span.start);
    }
}

fn pipeline(cfg: &Config) -> Result<Pipeline> {
    let mut p = Pipeline::new(&cfg.rules, &pun_fixers::all)?;
    p.set_preset(&cfg.preset);
    p.ignore_mismatched_rules(
        std::env::var("PHP_CS_FIXER_IGNORE_MISMATCHED_RULES_EXCEPTIONS")
            .is_ok_and(|v| pun_tokens::php_compat::filter_var_bool(&v)),
    );
    // PUN_PROFILE=1: per-stage timings on stderr after the run.
    if std::env::var_os("PUN_PROFILE").is_some() {
        p.enable_profile();
    }
    Ok(p)
}

/// The rules a run can't apply, under their Pun IDs (upstream names for sets the preset doesn't expand), and where
/// they come from: pun.toml, or the `--preset` / `--no-config` run's preset.
fn unported_error(a: &FormatArgs, cfg: &Config, names: &[String]) -> Abort {
    let ids: Vec<String> = (names.iter())
        .map(|n| format!("`{}`", pun_config::rule_of(n).unwrap_or(n)))
        .collect();
    // Every rule of the laravel, psr12 and empty presets runs natively, so an unported rule comes from pun.toml.
    let file = (cfg.root).join(a.config.as_deref().unwrap_or(Path::new("pun.toml")));
    let rules = if ids.len() == 1 { "rule" } else { "rules" };
    let what = format!("{rules} not supported by Pun yet: {}", ids.join(", "));
    let rel = file.strip_prefix(&cfg.root).unwrap_or(&file).to_path_buf();
    let them = if ids.len() == 1 { "it" } else { "them" };
    let error = RunError {
        hint: Some(format!(
            "remove {them} from {} to run the other rules",
            rel.display()
        )),
        path: Some(rel),
        line: None,
        message: what.clone(),
    };
    Abort::pun(format!("{}: {what}", file.display()), error)
}

/// `--explain`: the run's fixers in priority order, on stderr.
fn explain(cfg: &Config, p: &Pipeline) {
    let mut err = std::io::stderr().lock();
    let _ = writeln!(
        err,
        "native (PHP {}), preset {}, {} rules",
        cfg.php,
        cfg.preset,
        cfg.rules.len()
    );
    for f in p.fixers() {
        let _ = writeln!(err, "  native      {:>4}  {}", f.priority(), f.name());
    }
}

fn is_blade(path: &Path) -> bool {
    path.to_string_lossy().ends_with(".blade.php")
}

/// DefaultCommand::fixStdinInput: excluded paths echo the input; otherwise print the fixed code.
fn stdin(a: &FormatArgs, cfg: &Config, engine: &Pipeline) -> Result<u8> {
    let mut code = Vec::new();
    std::io::stdin().read_to_end(&mut code)?;
    let mut out = std::io::stdout().lock();
    let ctx = a
        .stdin_filename
        .clone()
        .unwrap_or_else(|| "stdin.php".into());
    // --test/--bail: the dry run leaves Pint's temp file untouched, so the input comes back as is.
    let fixed = if a.test || a.bail {
        None
    } else {
        fix_text(cfg, engine, &ctx, a.stdin_filename.is_some(), &code)?
    };
    out.write_all(fixed.as_deref().unwrap_or(&code))?;
    Ok(0)
}

/// Stdin formatting of `code` as `path`: the fixed code, or `None` when excluded, clean or invalid (Pint
/// reports syntax errors through its unshown JSON report and echoes the input).
fn fix_text(
    cfg: &Config,
    engine: &Pipeline,
    path: &Path,
    check_excluded: bool,
    code: &[u8],
) -> Result<Option<Vec<u8>>> {
    if check_excluded && pun_workspace::is_path_excluded(cfg, path) {
        return Ok(None);
    }
    // A fixer's exception: Pint reports it in the unshown report too.
    let Ok(run) = isolated(|| run_file(engine, path, code)) else {
        return Ok(None);
    };
    Ok(run?.ok().flatten().map(|(new, _)| new))
}

thread_local! {
    static ISOLATED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// One file's fixing, where a panic (Pun's fixers panic where upstream throws) fails only that file, with the panic
/// message, like PHP-CS-Fixer's per-file exception errors. Such panics print nothing; others still do.
fn isolated<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    static HOOK: std::sync::Once = std::sync::Once::new();
    HOOK.call_once(|| {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !ISOLATED.get() {
                default(info);
            }
        }));
    });
    ISOLATED.set(true);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    ISOLATED.set(false);
    r.map_err(|p| {
        p.downcast_ref::<&str>()
            .map(std::string::ToString::to_string)
            .or_else(|| p.downcast_ref::<String>().cloned())
            .unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    /// A panic inside `isolated` is the file's error, with its message.
    #[test]
    fn isolated_panics() {
        assert_eq!(
            super::isolated(|| panic!("boom")),
            Err::<(), _>("boom".into())
        );
    }
}
