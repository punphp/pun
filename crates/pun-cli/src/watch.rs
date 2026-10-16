//! `pun format --watch`: a normal run, then a run over each burst of changed files that a normal run would
//! pick (same discovery, config and exclusions, re-read every time), with the usual Pint output.
//! Stops when the project directory goes away or the parent process exits.

use crate::{FormatArgs, format, load_config, root};
use anyhow::Result;
use notify::{EventKind, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use xxhash_rust::xxh3::xxh3_64;

/// Editors save in bursts (truncate + write, temp file + rename): wait for this much quiet before running.
const DEBOUNCE: Duration = Duration::from_millis(100);

pub fn run(mut a: FormatArgs) -> Result<u8> {
    a.pun.watch = false;
    let started_by = parent();
    let root = root()?;
    report(format(a.clone(), false));
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&root, RecursiveMode::Recursive)?;
    // Re-runs format exactly the changed files; git selections only pick the first run's files.
    let rerun = FormatArgs { dirty: false, diff: None, pun: crate::PunArgs { staged: false, since: None, ..a.pun.clone() }, ..a.clone() };
    // The content each file had when Pun last finished with it: events that leave it so (Pun's own writes) are no change.
    let mut done: HashMap<PathBuf, u64> = HashMap::new();
    let hash = |p: &PathBuf| std::fs::read(p).ok().map(|c| xxh3_64(&c));
    loop {
        // Polls for a dead parent (the editor or shell that started the watch): the watch ends with it.
        let first = loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(e) => break e,
                Err(mpsc::RecvTimeoutError::Timeout) if parent() == started_by => {}
                Err(_) => return Ok(0),
            }
        };
        let mut changed = HashSet::new();
        let mut add = |e: notify::Result<notify::Event>| match e {
            Ok(e) if !matches!(e.kind, EventKind::Access(_)) => changed.extend(e.paths),
            _ => {}
        };
        add(first);
        while let Ok(e) = rx.recv_timeout(DEBOUNCE) {
            add(e);
        }
        if !root.exists() {
            return Ok(0);
        }
        let files = match load_config(&a, false, &root).and_then(|cfg| pun_workspace::discover(&cfg, &a.paths)) {
            Ok(f) => f,
            Err(e) => {
                report(Err(e));
                continue;
            }
        };
        let targets: Vec<PathBuf> = files.into_iter().filter(|f| changed.contains(f) && done.get(f).is_none_or(|&h| hash(f) != Some(h))).collect();
        if targets.is_empty() {
            continue;
        }
        report(format(FormatArgs { paths: targets.clone(), ..rerun.clone() }, false));
        for t in targets {
            if let Some(h) = hash(&t) {
                done.insert(t, h);
            }
        }
    }
}

/// A failed run (unreadable file, broken pint.json) is reported like `main` does, and the watch goes on.
fn report(r: Result<u8>) {
    if let Err(e) = r {
        eprintln!("\n  {e}\n");
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
