//! `dump <path>...`: every problem `check` finds and the source `fix` writes, for every PHP file under the paths
//! (walked like `check`), in path order. Two builds of pun-lint give the same output when they lint alike, so a
//! performance change compares its dump with the one before it.

use std::path::{Path, PathBuf};

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
    let mut files = Vec::new();
    for path in std::env::args().skip(1) {
        php_files(Path::new(&path), &mut files);
    }
    files.sort();
    for path in &files {
        let Ok(source) = std::fs::read_to_string(path) else {
            continue;
        };
        println!("== {}", path.display());
        println!("check {:?}", linter.check(path, &source));
        match linter.fix(path, &source) {
            Ok(fixed) if fixed.source != source => println!("fix {fixed:?}"),
            Ok(fixed) => println!("fix unchanged, remaining {:?}", fixed.remaining),
            Err(e) => println!("fix error {e:?}"),
        }
    }
}
