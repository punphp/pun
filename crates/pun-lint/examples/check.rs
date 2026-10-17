//! `check <path>...`: run every rule in check mode (no file is written) over PHP files, on all cores, for
//! scripts/bench.py's lint mode. A path is a file or a directory (walked like `fix_dir`: no `vendor/`,
//! `node_modules/`, dot directories or Blade templates). PHP 8.4, like `fix_dir`.
//! Prints `{"files": .., "flagged": .., "problems": .., "refused": ..}`, where `flagged` counts the files with a
//! problem (comparable with Rector's changed files) and `refused` the files that do not parse.
//! `check --rules` prints the rule IDs, one per line.
//!
//! [`READERS`] threads walk the directories and read the files; one thread per core checks the sources they read.
//! The readers share one stack of the directories still to list and the files still to read, so checking starts with
//! the first file read. Opening a file is the largest single cost (about 11 us on macOS, ten times a `stat`), and
//! opens on more than four threads at once slow each other down: eight threads open laravel-framework's files slower
//! than four. Reading on a few threads of their own keeps the opens from contending while every core parses.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};

use pun_lint::{Linter, rules};

/// The threads that walk and read. Fewer leave the checkers waiting for sources; more make the opens contend
/// (laravel-framework, 8 cores: 2 readers 46 ms, 3 readers 43 ms, 4 readers 44 ms, 5 readers 47 ms).
const READERS: usize = 3;

enum Item {
    Dir(PathBuf),
    File(PathBuf),
}

/// The directories still to list and the files still to read, and how many readers are listing a directory (and
/// may still add items).
struct Queue {
    state: Mutex<(Vec<Item>, usize)>,
    changed: Condvar,
}

impl Queue {
    /// The next item, or `None` once the stack is empty and no directory is being listed.
    fn take(&self) -> Option<Item> {
        let mut state = self.state.lock().unwrap();
        loop {
            if let Some(item) = state.0.pop() {
                if matches!(item, Item::Dir(_)) {
                    state.1 += 1;
                }
                return Some(item);
            }
            if state.1 == 0 {
                return None;
            }
            state = self.changed.wait(state).unwrap();
        }
    }

    /// Add what a directory held and mark that directory listed.
    fn done_dir(&self, found: Vec<Item>) {
        let mut state = self.state.lock().unwrap();
        state.0.extend(found);
        state.1 -= 1;
        drop(state);
        self.changed.notify_all();
    }
}

/// The sources read and not checked yet, and how many readers are still running.
struct Sources {
    state: Mutex<(Vec<(PathBuf, String)>, usize)>,
    changed: Condvar,
}

impl Sources {
    /// The next source, or `None` once every reader has finished and every source was taken.
    fn take(&self) -> Option<(PathBuf, String)> {
        let mut state = self.state.lock().unwrap();
        loop {
            if let Some(item) = state.0.pop() {
                return Some(item);
            }
            if state.1 == 0 {
                return None;
            }
            state = self.changed.wait(state).unwrap();
        }
    }

    fn put(&self, path: PathBuf, source: String) {
        self.state.lock().unwrap().0.push((path, source));
        self.changed.notify_one();
    }

    fn reader_done(&self) {
        self.state.lock().unwrap().1 -= 1;
        self.changed.notify_all();
    }
}

/// The PHP files and the subdirectories to walk in `dir`. The entry's `file_type` comes with the directory listing,
/// so only a symlink costs a `stat` (to follow it, as `Path::is_dir` does).
fn list(dir: &Path) -> Vec<Item> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if kind.is_dir() || (kind.is_symlink() && entry.path().is_dir()) {
            if name != "vendor" && name != "node_modules" && !name.starts_with('.') {
                found.push(Item::Dir(entry.path()));
            }
        } else if name.ends_with(".php") && !name.ends_with(".blade.php") {
            found.push(Item::File(entry.path()));
        }
    }
    found
}

fn main() {
    let linter = Linter::new(rules::all(), 80400);
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--rules") {
        for id in linter.rule_ids() {
            println!("{id}");
        }
        return;
    }
    let roots = args
        .iter()
        .rev()
        .map(|arg| {
            let path = PathBuf::from(arg);
            if path.is_file() {
                Item::File(path)
            } else {
                Item::Dir(path)
            }
        })
        .collect();
    let items = Queue {
        state: Mutex::new((roots, 0)),
        changed: Condvar::new(),
    };
    let sources = Sources {
        state: Mutex::new((Vec::new(), READERS)),
        changed: Condvar::new(),
    };
    let (files, flagged, problems, refused) = (
        AtomicUsize::new(0),
        AtomicUsize::new(0),
        AtomicUsize::new(0),
        AtomicUsize::new(0),
    );
    let checkers = std::thread::available_parallelism().map_or(1, |n| n.get());
    std::thread::scope(|s| {
        for _ in 0..READERS {
            s.spawn(|| {
                while let Some(item) = items.take() {
                    match item {
                        Item::Dir(dir) => items.done_dir(list(&dir)),
                        Item::File(path) => {
                            files.fetch_add(1, Ordering::Relaxed);
                            if let Ok(source) = std::fs::read_to_string(&path) {
                                sources.put(path, source);
                            }
                        }
                    }
                }
                sources.reader_done();
            });
        }
        for _ in 0..checkers {
            s.spawn(|| {
                while let Some((path, source)) = sources.take() {
                    match linter.check(&path, &source) {
                        Ok(found) if !found.is_empty() => {
                            flagged.fetch_add(1, Ordering::Relaxed);
                            problems.fetch_add(found.len(), Ordering::Relaxed);
                        }
                        Ok(_) => {}
                        Err(_) => {
                            refused.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            });
        }
    });
    println!(
        "{{\"files\": {}, \"flagged\": {}, \"problems\": {}, \"refused\": {}}}",
        files.into_inner(),
        flagged.into_inner(),
        problems.into_inner(),
        refused.into_inner()
    );
}
