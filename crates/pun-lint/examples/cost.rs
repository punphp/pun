//! Cost per file of the lint engine:
//! `cargo run --release -p punphp-lint --example cost -- <dir>...`
//! Times, per PHP file (not Blade, not vendor/ or node_modules/), the parse alone (a linter with no rules), `check`
//! with every rule, and `fix`. Prints totals and the mean per file. Single-threaded.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use pun_lint::{Linter, rules};

fn php_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
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
    let mut files = Vec::new();
    for dir in std::env::args().skip(1) {
        php_files(Path::new(&dir), &mut files);
    }
    let parse_only = Linter::new(Vec::new(), 80400);
    let linter = Linter::new(rules::all(), 80400);
    let (mut bytes, mut parse, mut check, mut fix) =
        (0usize, Duration::ZERO, Duration::ZERO, Duration::ZERO);
    let (mut problems, mut changed, mut rounds, mut skipped) = (0usize, 0usize, 0usize, 0usize);
    for path in &files {
        let Ok(source) = std::fs::read_to_string(path) else {
            continue;
        };
        let t = Instant::now();
        if parse_only.check(path, &source).is_err() {
            skipped += 1;
            continue;
        }
        parse += t.elapsed();
        bytes += source.len();
        let t = Instant::now();
        problems += linter.check(path, &source).unwrap().len();
        check += t.elapsed();
        let t = Instant::now();
        let fixed = linter.fix(path, &source).unwrap();
        fix += t.elapsed();
        rounds += fixed.rounds;
        changed += usize::from(fixed.source != source);
    }
    let n = (files.len() - skipped).max(1) as u32;
    println!(
        "files {} (skipped, syntax errors: {skipped}), {:.1} MB",
        files.len(),
        bytes as f64 / 1e6
    );
    println!("problems {problems}, files changed by fix {changed}, fix rounds {rounds}");
    for (what, d) in [
        ("parse", parse),
        ("check (parse + 2 rules)", check),
        ("fix (rounds until settled)", fix),
    ] {
        println!(
            "{what:28} total {:8.1} ms   per file {:7.1} us",
            d.as_secs_f64() * 1e3,
            (d / n).as_secs_f64() * 1e6
        );
    }
}
