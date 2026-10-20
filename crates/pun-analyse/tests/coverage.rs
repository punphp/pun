//! Branches PHPStan's own rule fixtures miss: each directory in tests/fixtures/coverage holds PHP files and one or
//! more phpstan*.neon, and expected-<neon>.json is PHPStan's run on it (`scripts/export-analyse-expected.py <dir>
//! [neon]`), restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

/// PHPStan 2.3 errors Pun does not report yet (case directory, file, line, identifier); one Pun reports fails the
/// test as extra.
const GAPS: &[(&str, &str, i64, &str)] = &[
    // UnusedConstructorParametersRule follows VariableWritesNode dataflow: a constructor that never ends (`for (;;)`)
    // reads no parameter
    ("strict-rules", "construct-global.php", 61, "constructor.unusedParameter"),
];

fn check(dir: &Path, neon: &str) -> Result<(), String> {
    let root = dir.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{root}/{neon}.neon")),
        ..Options::for_project(&root)
    };
    let config = resolve(&options).map_err(|e| e.to_string())?;
    let prefix = format!("{root}/");
    let mut got: Vec<Key> = analyse(dir, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(dir.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["identifier"].as_str().is_some_and(reports))
        .filter(|e| {
            !GAPS.iter().any(|(case, file, line, id)| {
                dir.ends_with(case)
                    && e["file"] == *file
                    && e["line"] == *line
                    && e["identifier"] == *id
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
    if got == want {
        return Ok(());
    }
    let missing: Vec<_> = want.iter().filter(|k| !got.contains(k)).collect();
    let extra: Vec<_> = got.iter().filter(|k| !want.contains(k)).collect();
    Err(format!("missing {missing:#?}\nextra {extra:#?}"))
}

#[test]
fn coverage_fixtures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/coverage");
    let mut cases: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    cases.sort();
    let mut failures = Vec::new();
    for dir in cases {
        let mut neons: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .filter_map(|e| {
                let name = e.file_name().into_string().unwrap();
                name.strip_suffix(".neon").map(str::to_string)
            })
            .collect();
        neons.sort();
        for neon in neons {
            if let Err(e) = check(&dir, &neon) {
                failures.push(format!("{}/{neon}: {e}", dir.display()));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// A circular class hierarchy stops PHPStan with "Reflection error: Circular reference to class" (no file errors to
/// compare); the PHPDoc inheritance walks must still end.
#[test]
fn circular_hierarchy_ends() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cycle");
    let mut project = pun_analyse::Project::new(vec![dir.join("cycle.php")], Vec::new());
    project.level = 6;
    assert!(pun_analyse::run(&project).is_ok());
}
