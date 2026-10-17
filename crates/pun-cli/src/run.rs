//! `pun format` / `lint` / `check`: load the config, pick the engine, fix the files, report.
use crate::report::{self, FileResult, Status};
use crate::{FormatArgs, cache, pint_error, root};
use anyhow::{Context, Result};
use pun_config::{Config, Options};
use pun_format::{LintError, Pipeline};
use rayon::prelude::*;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Which rules a run applies. `bool`: `--fix` (only lint rules with `fix = "apply"`; without it every enabled lint
/// rule is checked, nothing written).
#[derive(Clone, Copy)]
pub(crate) enum Stage {
    Format,
    Lint(bool),
    /// Format and lint fixers in one pipeline, so fixers run in PHP-CS-Fixer's priority order, as Pint runs them.
    /// ponytail: `pun analyse` joins here once pun-analyse has a library entry point.
    Check(bool),
}

pub(crate) enum Engine {
    Native(Pipeline),
    /// Every file through real PHP-CS-Fixer (pun-delegate), with the enabled rules Pun hasn't ported.
    Delegate(pun_delegate::Delegate, Vec<String>),
}

type Run = Result<Option<(Vec<u8>, Vec<&'static str>)>, LintError>;

impl Engine {
    /// The outer error aborts the run (a dead PHP worker, PHP-CS-Fixer's own run-level exceptions).
    fn run(&self, file: &Path, code: &[u8]) -> Result<Run> {
        Ok(match self {
            Engine::Native(p) => {
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
                r.map(|f| f.map(|f| (f.code, f.applied)))
            }
            // Linting stays native (TokenizerLinter's messages), as on the native path.
            Engine::Delegate(d, _) => match pun_format::lint(code) {
                Err(e) => Err(e),
                Ok(()) => match d.fix(file, code, &pun_format::pun_ignored(code))? {
                    pun_delegate::Outcome::Unchanged => Ok(None),
                    pun_delegate::Outcome::Failed(message) => Err(LintError {
                        message,
                        after_fix: true,
                    }),
                    pun_delegate::Outcome::Fixed(new, applied) => match pun_format::lint(&new) {
                        Ok(()) => Ok(Some((new, applied))),
                        Err(e) => Err(LintError {
                            after_fix: true,
                            ..e
                        }),
                    },
                },
            },
        })
    }
}

pub(crate) fn load_config(a: &FormatArgs, root: &Path) -> Result<Config> {
    let opts = Options {
        config: a.config.clone(),
        no_config: a.no_config,
        preset: a.preset.clone(),
        blade: a.blade,
    };
    pun_config::load(root, &opts)
}

pub(crate) fn format(a: FormatArgs, stage: Stage) -> Result<u8> {
    let root = root()?;
    // Config errors are one line (on stderr), not a Pint box: Pint never reads pun.toml.
    let mut cfg = match load_config(&a, &root) {
        Ok(cfg) => cfg,
        Err(e) if crate::agent::enabled() => {
            print!("{}", crate::agent::error(&format!("{e:#}")));
            return Ok(1);
        }
        Err(e) => {
            eprintln!("pun: {e:#}");
            return Ok(1);
        }
    };
    // `--fix`: the lint rules it doesn't apply (no `fix = "apply"`) still run on the result, reported as remaining.
    let pending: Vec<_> = match stage {
        Stage::Lint(true) | Stage::Check(true) => (cfg.lint.iter())
            .filter(|(_, apply)| !apply)
            .map(|(r, _)| r.clone())
            .collect(),
        _ => vec![],
    };
    if let Stage::Lint(fix) | Stage::Check(fix) = stage {
        let lint = (cfg.lint.iter())
            .filter(|(_, apply)| *apply || !fix)
            .map(|(r, _)| r.clone());
        if let Stage::Lint(_) = stage {
            cfg.rules = lint.collect();
            cfg.blade = false;
        } else {
            cfg.rules.extend(lint);
        }
    }
    // EnsurePrettierIsConfigured: `"Pint/laravel_blade": true` (or --blade) needs Node and prettier in the project.
    let blade_on = cfg
        .rules
        .iter()
        .any(|r| r.name == "Pint/laravel_blade" && r.options.is_null());
    if let Some(out) = report::ensure_prettier(&root, blade_on) {
        print!("{out}");
        return Ok(1);
    }
    let pending_engine = (!pending.is_empty())
        .then(|| {
            engine(
                &a,
                &Config {
                    rules: pending.clone(),
                    ..cfg.clone()
                },
                &root,
            )
        })
        .transpose()?;
    let engine = engine(&a, &cfg, &root)?;
    if a.pun.explain || a.verbose > 0 {
        explain(&cfg, &engine, a.pun.explain);
    }

    let stdin_path = a.paths.first().is_some_and(|p| p.as_os_str() == "-");
    if a.pun.stdin || stdin_path || a.stdin_filename.is_some() {
        return stdin(&a, &cfg, &engine);
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
    let files = match &selection {
        Some(sel) => pun_workspace::git_paths(&cfg, sel)?,
        None => pun_workspace::discover(&cfg, &a.paths)?,
    };

    if a.pun.list_files {
        for f in &files {
            println!("{}", f.strip_prefix(&root).unwrap_or(f).display());
        }
        return Ok(0);
    }

    let cache = if a.pun.no_cache {
        None
    } else {
        a.cache_file
            .as_ref()
            .map(|f| root.join(f))
            .or_else(|| cache::default_path(&root))
            .map(|p| {
                let mut sig = cache::signature(&cfg);
                sig["pending"] = pending.iter().map(|r| r.name.clone()).collect();
                if let Engine::Delegate(d, _) = &engine {
                    sig["delegate"] = d.signature();
                }
                cache::Cache::load(p, sig)
            })
    };
    // Unreadable / unwritable files abort the whole run, like Pint (FileReader / Runner exceptions).
    // Besides the result: the content hash when the file is clean (for the cache), whether the cache said so, and
    // the pending rules' fixes (`--fix`).
    type Processed = (FileResult, Option<String>, bool, Option<FileResult>);
    let process = |path: &PathBuf| -> Result<Processed> {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .into_owned();
        let mut r = FileResult {
            path: path.clone(),
            rel,
            status: Status::NoChanges,
            applied: vec![],
            diff: String::new(),
            message: String::new(),
        };
        // Runner FileFilterIterator: symlinks are dropped silently, empty files are "skipped".
        if path.is_symlink() {
            r.status = Status::Link;
            return Ok((r, None, false, None));
        }
        let code = std::fs::read(path)
            .with_context(|| format!("Failed to read content from \"{}\".", path.display()))?;
        if code.is_empty() {
            r.status = Status::Skipped;
            return Ok((r, None, false, None));
        }
        let hash = cache.as_ref().map(|_| cache::hash(&code));
        if let (Some(c), Some(h)) = (&cache, &hash)
            && c.is_clean(&r.rel, h)
        {
            return Ok((r, hash, true, None));
        }
        let mut clean = None;
        let outcome = match isolated(|| engine.run(path, &code)) {
            Ok(run) => run?,
            Err(message) => {
                r.status = Status::Exception;
                r.message = message;
                return Ok((r, None, false, None));
            }
        };
        // Reports show diffs only with -v (sebastian/diff's LCS is quadratic, so skip it otherwise).
        let diff = |old: &[u8], new: &[u8]| {
            if a.verbose == 0 {
                return String::new();
            }
            report::diff(
                &String::from_utf8_lossy(old),
                &String::from_utf8_lossy(new),
                // Agent mode: paths relative to the project, as everywhere in its output.
                if crate::agent::enabled() {
                    Path::new(&r.rel)
                } else {
                    path
                },
            )
        };
        let result = match outcome {
            Err(e) => {
                r.status = if e.after_fix {
                    Status::Lint
                } else {
                    Status::Invalid
                };
                r.message = e.message;
                None
            }
            Ok(None) => {
                clean = hash;
                pending_engine.as_ref().map(|_| code.clone())
            }
            Ok(Some((new, applied))) => {
                r.diff = diff(&code, &new);
                r.applied = applied;
                r.status = Status::Fixed;
                if !dry_run {
                    std::fs::write(path, &new).with_context(|| {
                        format!(
                            "Cannot write to file \"{}\" as it is not writable.",
                            path.display()
                        )
                    })?;
                }
                Some(new)
            }
        };
        // A pending rule's failure (lint error, panic) is not reported: the file was already fixed and checked.
        let left = match (&pending_engine, &result) {
            (Some(e), Some(now)) => isolated(|| e.run(path, now))
                .ok()
                .transpose()?
                .and_then(Result::ok)
                .flatten(),
            _ => None,
        };
        let left = left.map(|(new, applied)| FileResult {
            diff: diff(result.as_deref().unwrap(), &new),
            applied,
            status: Status::Fixed,
            message: String::new(),
            path: path.clone(),
            rel: r.rel.clone(),
        });
        if left.is_some() {
            clean = None;
        }
        Ok((r, clean, false, left))
    };

    let all: Vec<Result<_>> = if a.bail {
        // --bail is PHP-CS-Fixer's stop-on-violation: sequential, stop at the first fixable file (lint errors don't
        // stop it; an aborting file does).
        let mut out = Vec::new();
        for f in &files {
            let r = process(f);
            let stop = r.as_ref().map_or(true, |r| r.0.status == Status::Fixed);
            out.push(r);
            if stop {
                break;
            }
        }
        out
    } else {
        files.par_iter().map(process).collect()
    };
    if a.format.is_none() && !a.quiet && !a.parallel {
        pint_error::progress_before_abort(&all, dry_run, a.ansi, a.no_ansi);
    }
    let results: Vec<Processed> = all.into_iter().collect::<Result<_>>()?;

    let hits = results.iter().filter(|r| r.2).count();
    let mut remaining = vec![];
    let (results, clean): (Vec<FileResult>, Vec<Option<String>>) = (results.into_iter())
        .map(|(r, c, _, left)| {
            remaining.extend(left);
            (r, c)
        })
        .unzip();
    if let Some(c) = cache {
        if a.pun.explain {
            eprintln!(
                "cache: {hits} of {} files unchanged since a clean run, skipped ({})",
                files.len(),
                c.path().display()
            );
        }
        c.save(results.iter().map(|r| r.rel.clone()).zip(clean));
    }

    let ctx = report::Ctx {
        preset: &cfg.preset,
        total: files.len(),
        dry_run,
        remaining: &remaining,
        verbose: a.verbose > 0,
        ansi: a.ansi,
        no_ansi: a.no_ansi,
    };
    let code = report::emit(
        &ctx,
        &results,
        a.format.as_deref(),
        a.quiet,
        a.output_to_file.as_deref(),
        a.output_format.as_deref(),
    );
    let failed = ((dry_run || a.repair) && results.iter().any(|r| r.status == Status::Fixed))
        || results.iter().any(FileResult::is_error)
        || !remaining.is_empty();

    if let Engine::Native(p) = &engine {
        eprint!("{}", p.profile_report());
        if !p.not_ported.is_empty() {
            eprintln!(
                "pun: --native-only skipped {} rules that are not ported: {}",
                p.not_ported.len(),
                p.not_ported.join(", ")
            );
        }
    }
    Ok(code.max(u8::from(failed)))
}

pub(crate) fn engine(a: &FormatArgs, cfg: &Config, root: &Path) -> Result<Engine> {
    pun_fixers::analyzers::active_preset::set(&cfg.preset);
    let mut p = Pipeline::new(&cfg.rules, &pun_fixers::all)?;
    // PUN_PROFILE=1: per-stage timings on stderr after the run.
    if std::env::var_os("PUN_PROFILE").is_some() {
        p.enable_profile();
    }
    Ok(if p.not_ported.is_empty() || a.pun.native_only {
        Engine::Native(p)
    } else {
        // Pint's Config gets the rules as a map (`true` for no options).
        let rules = cfg
            .rules
            .iter()
            .map(|r| {
                let v = if r.options.is_null() {
                    true.into()
                } else {
                    r.options.clone()
                };
                (r.name.clone(), v)
            })
            .collect();
        // PUN_PINT: a Pint PHAR to delegate to when the project has none (the harness, CI images).
        let d = pun_delegate::Delegate::start(
            root,
            pun_delegate::Source::Rules(serde_json::Value::Object(rules)),
            std::env::var_os("PUN_PINT").map(PathBuf::from),
        )?;
        Engine::Delegate(d, p.not_ported)
    })
}

/// `full` (--explain): the rule list too; `-v` prints only why a run is delegated.
fn explain(cfg: &Config, engine: &Engine, full: bool) {
    let mut err = std::io::stderr().lock();
    match engine {
        Engine::Delegate(d, not_ported) => {
            let why = format!(
                "{} of {} rules are not ported ({})",
                not_ported.len(),
                cfg.rules.len(),
                not_ported.join(", ")
            );
            let _ = writeln!(
                err,
                "delegate: {why}, so every file runs through PHP-CS-Fixer in `php` ({})",
                d.runtime.display()
            );
            for (name, priority) in d.fixers.iter().filter(|_| full) {
                let _ = writeln!(err, "  delegated   {priority:>4}  {name}");
            }
        }
        _ if !full => {}
        Engine::Native(p) => {
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
            for n in &p.not_ported {
                let _ = writeln!(err, "  skipped           {n}");
            }
        }
    }
}

/// DefaultCommand::fixStdinInput: excluded paths echo the input; otherwise print the fixed code.
fn stdin(a: &FormatArgs, cfg: &Config, engine: &Engine) -> Result<u8> {
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

/// Stdin / LSP formatting of `code` as `path`: the fixed code, or `None` when excluded, clean or invalid (Pint
/// reports syntax errors through its unshown JSON report and echoes the input).
pub(crate) fn fix_text(
    cfg: &Config,
    engine: &Engine,
    path: &Path,
    check_excluded: bool,
    code: &[u8],
) -> Result<Option<Vec<u8>>> {
    if check_excluded && pun_workspace::is_path_excluded(cfg, path) {
        return Ok(None);
    }
    // A fixer's exception: Pint reports it in the unshown report too.
    let Ok(run) = isolated(|| engine.run(path, code)) else {
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
                default(info)
            }
        }));
    });
    ISOLATED.set(true);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    ISOLATED.set(false);
    r.map_err(|p| {
        p.downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| p.downcast_ref::<String>().cloned())
            .unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    /// A panic inside `isolated` is the file's error and prints nothing; one elsewhere still reaches the default hook.
    #[test]
    fn isolated_panics() {
        assert_eq!(
            super::isolated(|| panic!("boom")),
            Err::<(), _>("boom".into())
        );
        assert!(std::thread::spawn(|| panic!("elsewhere")).join().is_err());
    }
}
