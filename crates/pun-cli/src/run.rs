//! `pun format` / `lint` / `check`: load the config, pick the engine, fix the files, report.
use crate::render::{Look, RunError, Spinner};
use crate::report::{self, Abort, FileResult, Status};
use crate::{FormatArgs, cache, root};
use anyhow::{Context, Result};
use pun_config::{Config, Options};
use pun_format::{LintError, Pipeline};
use rayon::prelude::*;
use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};

/// Which rules a run applies. `bool`: `--fix` (every enabled lint rule's fix but those with `fix = "off"`; without
/// it every enabled lint rule is checked, nothing written).
#[derive(Clone, Copy)]
pub(crate) enum Stage {
    Format,
    Lint(bool),
    /// Format and lint fixers in one pipeline, so fixers run in PHP-CS-Fixer's priority order, as Pint runs them.
    /// ponytail: `pun analyse` joins here once pun-analyse has a library entry point.
    Check(bool),
}

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
    pun_config::load(root, &opts)
}

pub(crate) fn format(a: FormatArgs, stage: Stage) -> Result<u8> {
    let started = std::time::Instant::now();
    let root = root()?;
    // Config errors are Pun's own run errors, never a Pint box: Pint never reads pun.toml.
    let mut cfg = load_config(&a, &root).map_err(|e| {
        let message = format!("{e:#}");
        let error = RunError {
            path: None,
            line: None,
            message: message.clone(),
            hint: None,
        };
        Abort::pun(message, error)
    })?;
    // No PHP fallback: a rule without a native fixer, format or named lint rule, stops every command (exit 2, Pun
    // can't run). A set or a group skips the rules Pun can't run yet.
    let named_without_fixer: Vec<String> = (cfg.lint.rules.iter())
        .filter(|l| l.named && l.fixer.is_none())
        .map(|l| l.id.clone())
        .collect();
    let lint_fixers: Vec<(pun_config::Rule, bool, bool)> = (cfg.lint.rules.iter())
        .filter_map(|l| {
            let rule = pun_config::Rule {
                name: l.fixer?.to_string(),
                options: l.options.clone(),
            };
            Some((rule, l.fix, l.named))
        })
        .collect();
    let every: Vec<_> = (cfg.rules.iter().cloned())
        .chain(lint_fixers.iter().map(|(r, _, _)| r.clone()))
        .collect();
    pun_fixers::analyzers::active_preset::set(&cfg.preset);
    let not_ported = Pipeline::new(&every, &pun_fixers::all)?.not_ported;
    let unnamed = |name: &String| {
        lint_fixers
            .iter()
            .any(|(r, _, named)| r.name == *name && !named)
    };
    let unported: Vec<String> = (not_ported.iter())
        .filter(|n| !unnamed(n))
        .cloned()
        .chain(named_without_fixer)
        .collect();
    if !unported.is_empty() {
        return Err(unported_error(&a, &cfg, &unported).into());
    }
    let lint: Vec<(pun_config::Rule, bool)> = (lint_fixers.into_iter())
        .filter(|(r, _, _)| !not_ported.contains(&r.name))
        .map(|(r, fix, _)| (r, fix))
        .collect();
    // `--fix`: the lint rules it doesn't apply (`fix = "off"`) still run on the result, reported as remaining.
    let pending: Vec<_> = match stage {
        Stage::Lint(true) | Stage::Check(true) => (lint.iter())
            .filter(|(_, fix)| !fix)
            .map(|(r, _)| r.clone())
            .collect(),
        _ => vec![],
    };
    if let Stage::Lint(fix) | Stage::Check(fix) = stage {
        let lint = (lint.iter())
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
    if let Some(abort) = report::ensure_prettier(&root, blade_on) {
        return Err(abort.into());
    }
    let pending_engine = (!pending.is_empty())
        .then(|| {
            pipeline(&Config {
                rules: pending.clone(),
                ..cfg.clone()
            })
        })
        .transpose()?;
    let engine = pipeline(&cfg)?;
    if a.pun.explain {
        explain(&cfg, &engine);
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
                cache::Cache::load(p, sig)
            })
    };
    // Unreadable / unwritable files abort the whole run, like Pint (FileReader / Runner exceptions).
    // Besides the result: the content hash when the file is clean (for the cache), whether the cache said so, and
    // the pending rules' fixes (`--fix`).
    type Processed = (FileResult, Option<String>, bool, Option<FileResult>);
    let look = Look::stdout(a.color());
    // The progress line, for humans on a terminal (output.md §1); dropped before anything is printed.
    let human = a.format.is_none() && !a.quiet && !crate::agent::enabled();
    let spinner = human
        .then(|| Spinner::stderr(files.len(), &look, std::io::stderr().is_terminal()))
        .flatten();
    let process_file = |path: &PathBuf| -> Result<Processed> {
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
            line: 0,
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
        let outcome = match isolated(|| run_file(&engine, path, &code)) {
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
                r.line = crate::github::first_changed_line(&code, &new);
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
            (Some(e), Some(now)) => isolated(|| run_file(e, path, now))
                .ok()
                .transpose()?
                .and_then(Result::ok)
                .flatten(),
            _ => None,
        };
        let left = left.map(|(new, applied)| FileResult {
            diff: diff(result.as_deref().unwrap(), &new),
            line: crate::github::first_changed_line(result.as_deref().unwrap(), &new),
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
    let process = |path: &PathBuf| -> Result<Processed> {
        let r = process_file(path);
        if let Some(s) = &spinner {
            s.tick();
        }
        r
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
        // Pint's `--max-processes` (read with intval()) caps the threads, and with them the Prettier workers.
        let threads = a
            .max_processes
            .as_deref()
            .map(pun_tokens::php_compat::intval)
            .filter(|&n| n > 0)
            .map_or(0, |n| (n as usize).min(files.len().max(1)));
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()?
            .install(|| files.par_iter().map(process).collect())
    };
    drop(spinner);
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
        total: files.len(),
        dry_run,
        remaining: &remaining,
        verbose: a.verbose > 0,
        ansi: a.ansi,
        no_ansi: a.no_ansi,
        look,
        root: &root,
        elapsed: started.elapsed(),
        github: a.github_actions && human,
    };
    report::emit(
        &ctx,
        &results,
        a.format.as_deref(),
        a.quiet,
        a.output_to_file.as_deref(),
        a.output_format.as_deref(),
    )?;
    let failed = ((dry_run || a.repair) && results.iter().any(|r| r.status == Status::Fixed))
        || results.iter().any(FileResult::is_error)
        || !remaining.is_empty();
    // A fixer that threw or produced invalid code is Pun failing, not the file (exit 2).
    let crashed = (results.iter()).any(|r| matches!(r.status, Status::Exception | Status::Lint));

    eprint!("{}", engine.profile_report());
    Ok(if crashed { 2 } else { u8::from(failed) })
}

fn pipeline(cfg: &Config) -> Result<Pipeline> {
    pun_fixers::analyzers::active_preset::set(&cfg.preset);
    let mut p = Pipeline::new(&cfg.rules, &pun_fixers::all)?;
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
