//! `check <path>...`: run every rule in check mode (no file is written) over PHP files, on all cores, for
//! scripts/bench.py's lint mode. A path is a file or a directory (walked like `fix_dir`: no `vendor/`,
//! `node_modules/`, dot directories or Blade templates). PHP 8.4, like `fix_dir`.
//! Prints `{"files": .., "flagged": .., "problems": .., "refused": ..}`, where `flagged` counts the files with a
//! problem (comparable with Rector's changed files) and `refused` the files that do not parse.
//! `check --rules` prints the rule IDs, one per line.
//!
//! The walk and the checks share the threads: one stack holds the directories still to read and the files still to
//! check, so the first files are checked while the rest of the tree is being listed (like `ignore`'s parallel walker).
//! Listing every file on one thread before checking any took about a quarter of laravel-framework's run.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};

use pun_lint::{Linter, rules};

enum Item {
    Dir(PathBuf),
    File(PathBuf),
}

/// The items not taken yet, and how many threads are reading a directory (and may still add items).
struct Queue {
    state: Mutex<(Vec<Item>, usize)>,
    changed: Condvar,
}

impl Queue {
    /// The next item, or `None` once the stack is empty and no directory is being read.
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

    /// Add what a directory held and mark that directory read.
    fn done_dir(&self, found: Vec<Item>) {
        let mut state = self.state.lock().unwrap();
        state.0.extend(found);
        state.1 -= 1;
        drop(state);
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
    let queue = Queue {
        state: Mutex::new((roots, 0)),
        changed: Condvar::new(),
    };
    let (files, flagged, problems, refused) = (
        AtomicUsize::new(0),
        AtomicUsize::new(0),
        AtomicUsize::new(0),
        AtomicUsize::new(0),
    );
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                while let Some(item) = queue.take() {
                    let path = match item {
                        Item::Dir(dir) => {
                            queue.done_dir(list(&dir));
                            continue;
                        }
                        Item::File(path) => path,
                    };
                    files.fetch_add(1, Ordering::Relaxed);
                    let Ok(source) = std::fs::read_to_string(&path) else {
                        continue;
                    };
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
