//! `check <path>...`: run every rule in check mode (no file is written) over PHP files, on all cores, for
//! scripts/bench.py's lint mode. A path is a file or a directory (walked like `fix_dir`: no `vendor/`,
//! `node_modules/`, dot directories or Blade templates). PHP 8.4, like `fix_dir`.
//! Prints `{"files": .., "flagged": .., "problems": .., "refused": ..}`, where `flagged` counts the files with a
//! problem (comparable with Rector's changed files) and `refused` the files that do not parse.
//! `check --rules` prints the rule IDs, one per line.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use pun_lint::{Linter, rules};

fn php_files(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_file() {
        out.push(path.to_path_buf());
        return;
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if name != "vendor" && name != "node_modules" && !name.starts_with('.') {
                php_files(&path, out);
            }
        } else if name.ends_with(".php") && !name.ends_with(".blade.php") {
            out.push(path);
        }
    }
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
    let mut files = Vec::new();
    for path in &args {
        php_files(Path::new(path), &mut files);
    }
    let (flagged, problems, refused) = (
        AtomicUsize::new(0),
        AtomicUsize::new(0),
        AtomicUsize::new(0),
    );
    let next = AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                while let Some(path) = files.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let Ok(source) = std::fs::read_to_string(path) else {
                        continue;
                    };
                    match linter.check(path, &source) {
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
        files.len(),
        flagged.into_inner(),
        problems.into_inner(),
        refused.into_inner()
    );
}
