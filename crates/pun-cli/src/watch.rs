//! `pun format --watch`: a normal run, then a run over each burst of changed files that a normal run would
//! pick (same discovery, config and exclusions, re-read every time), with the usual output.
//! Stops when the project directory goes away or the parent process exits.

use crate::run::{LintOpts, Stage, format, load_config};
use crate::{FormatArgs, root};
use anyhow::Result;
use notify::{EventKind, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use xxhash_rust::xxh3::xxh3_64;

/// Editors save in bursts (truncate + write, temp file + rename): wait for this much quiet before running.
const DEBOUNCE: Duration = Duration::from_millis(100);

pub(crate) fn run(mut a: FormatArgs) -> Result<u8> {
    a.pun.watch = false;
    let started_by = parent();
    let root = root()?;
    report(&a, format(a.clone(), Stage::Format, LintOpts::default()));
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&root, RecursiveMode::Recursive)?;
    // Re-runs format exactly the changed files; git selections only pick the first run's files.
    let rerun = FormatArgs {
        dirty: false,
        diff: None,
        pun: crate::PunArgs {
            staged: false,
            since: None,
            ..a.pun.clone()
        },
        ..a.clone()
    };
    // The content each file had when Pun last finished with it: events that leave it so (Pun's own writes, which
    // FSEvents may report after the first run) are no change.
    let hash = |p: &PathBuf| std::fs::read(p).ok().map(|c| xxh3_64(&c));
    let mut done: HashMap<PathBuf, u64> = load_config(&a, &root)
        .and_then(|cfg| pun_workspace::discover(&cfg, &a.paths))
        .unwrap_or_default()
        .into_iter()
        .filter_map(|f| Some((f.clone(), hash(&f)?)))
        .collect();
    loop {
        // Checks for a dead parent (the editor or shell that started the watch) at least every second: the watch ends
        // with it. Reads (Linux inotify's open/close, Pun's own reads of pun.toml and the tree included) wake it
        // without starting a run.
        let first = loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                _ if parent() != started_by => return Ok(0),
                Ok(Ok(e)) if matches!(e.kind, EventKind::Access(_)) => {}
                Ok(e) => break e,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(0),
            }
        };
        let mut changed = HashSet::new();
        // Reads (Linux inotify's open/close) and watcher errors change nothing.
        let mut add = |e: notify::Result<notify::Event>| {
            let events = e
                .into_iter()
                .filter(|e| !matches!(e.kind, EventKind::Access(_)));
            changed.extend(events.flat_map(|e| e.paths))
        };
        add(first);
        while let Ok(e) = rx.recv_timeout(DEBOUNCE) {
            add(e);
        }
        if !root.exists() {
            return Ok(0);
        }
        let files =
            match load_config(&a, &root).and_then(|cfg| pun_workspace::discover(&cfg, &a.paths)) {
                Ok(f) => f,
                Err(e) => {
                    report(&a, Err(e));
                    continue;
                }
            };
        let targets: Vec<PathBuf> = files
            .into_iter()
            .filter(|f| changed.contains(f) && done.get(f).is_none_or(|&h| hash(f) != Some(h)))
            .collect();
        if targets.is_empty() {
            continue;
        }
        report(
            &rerun,
            format(
                FormatArgs {
                    paths: targets.clone(),
                    ..rerun.clone()
                },
                Stage::Format,
                LintOpts::default(),
            ),
        );
        for t in targets {
            if let Some(h) = hash(&t) {
                done.insert(t, h);
            }
        }
    }
}

/// A failed run (unreadable file, broken pun.toml) is reported as `main` reports it, cause chain included, and the
/// watch goes on.
fn report(a: &FormatArgs, r: Result<u8>) {
    if let Err(e) = r {
        crate::report::fail(e, &a.failure());
    }
}

/// Reparenting (to init or a subreaper) is how a Unix process sees its parent die.
#[cfg(unix)]
fn parent() -> u32 {
    std::os::unix::process::parent_id()
}

/// ponytail: no parent check on Windows (no stdlib parent id); stop the watch with Ctrl-C there.
#[cfg(not(unix))]
fn parent() -> u32 {
    0
}
