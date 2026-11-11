//! `analyse <project dir> <phpstan.neon>`: Pun's level-0 analysis as JSON, for scripts/analyse-corpus.py.
//! Paths are relative to the project dir.
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a command-line tool reports on stdout and stderr"
)]

use std::path::Path;

use pun_phpstan::config::{Options, resolve};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);
    let options = Options {
        config: Some(args[2].clone()),
        ..Options::for_project(&args[1])
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let errors = pun_analyse::analyse(root, &config).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{}/", root.display());
    let errors: Vec<_> = errors
        .iter()
        .map(|e| {
            serde_json::json!({
                "file": e.file.strip_prefix(&prefix).unwrap_or(&e.file),
                "line": e.line,
                "identifier": e.identifier,
                "message": e.message,
                "tip": e.tip,
            })
        })
        .collect();
    println!(
        "{}",
        serde_json::json!({ "rules": pun_analyse::IDENTIFIERS, "errors": errors })
    );
}
