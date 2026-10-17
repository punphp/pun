//! Fix every PHP file under the given directories in place with both sample rules (PHP 8.4), for comparing with
//! `rector process` on a copy of a corpus project (context/reports/lint-spike.md). Prints each changed file.

use std::path::{Path, PathBuf};

use pun_lint::{Linter, rules};

fn php_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
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
    let mut files = Vec::new();
    for dir in std::env::args().skip(1) {
        php_files(Path::new(&dir), &mut files);
    }
    let linter = Linter::new(rules::all(), 80400);
    for path in files {
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        match linter.fix(&path, &source) {
            Ok(fixed) if fixed.source != source => {
                std::fs::write(&path, &fixed.source).unwrap();
                println!("{}", path.display());
            }
            Ok(_) => {}
            Err(e) => eprintln!("{}: {e}", path.display()),
        }
    }
}
