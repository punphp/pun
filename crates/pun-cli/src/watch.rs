//! `pun format --watch`: a normal run, then a run over each burst of changed files that a normal run would
//! pick (same discovery, config and exclusions, re-read every time), with the usual Pint output.
//! Stops when the project directory goes away.

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
    a.watch = false;
    let root = root()?;
    report(format(a.clone(), false));
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&root, RecursiveMode::Recursive)?;
    // Re-runs format exactly the changed files; git selections only pick the first run's files.
    let rerun = FormatArgs { dirty: false, diff: None, staged: false, since: None, ..a.clone() };
    // The content each file had when Pun last finished with it: events that leave it so (Pun's own writes) are no change.
    let mut done: HashMap<PathBuf, u64> = HashMap::new();
    let hash = |p: &PathBuf| std::fs::read(p).ok().map(|c| xxh3_64(&c));
    while let Ok(first) = rx.recv() {
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
    Ok(0)
}

/// A failed run (unreadable file, broken pint.json) is reported like `main` does, and the watch goes on.
fn report(r: Result<u8>) {
    if let Err(e) = r {
        eprintln!("\n  {e}\n");
    }
}
