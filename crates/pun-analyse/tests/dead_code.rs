//! Branches of the dead-code rules that PHPStan's own rule fixtures miss: NotAnalysedTraitRule (it stays quiet when
//! only files are analysed, as the level fixtures are), the bleedingEdge toggles (reportPreciseLineForUnusedFunction-
//! Parameter, unusedLabel) and the private members of traits. Expected: PHPStan's run on tests/fixtures/dead_code for
//! each phpstan*.neon there, restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, analyse};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

/// PHPStan 2.3 errors Pun does not report yet (file, line, identifier); one Pun reports fails the test as extra.
const GAPS: &[(&str, i64, &str)] = &[
    // a `for` loop whose body always exits checks its condition once, on the initial values (`$i = 0; $i < 1`)
    ("src/conditions.php", 609, "smaller.alwaysTrue"),
];

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dead_code");
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let mut got: Vec<Key> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            // PHPStan's JSON names a trait's file without its ` (in context of …)`
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file);
            let file = file
                .split(" (in context of ")
                .next()
                .unwrap_or(file)
                .to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| IDENTIFIERS.contains(&e["identifier"].as_str().unwrap_or("")))
        .filter(|e| {
            !GAPS.iter().any(|(file, line, id)| {
                e["file"] == *file && e["line"] == *line && e["identifier"] == *id
            })
        })
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn default_toggles() {
    check("phpstan");
}

#[test]
fn bleeding_edge_toggles() {
    check("phpstan-bleeding");
}
