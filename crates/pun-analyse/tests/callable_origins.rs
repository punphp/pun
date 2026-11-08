//! PHPStan 2.3.1's callable signatures survive only along their actual declaration source edges.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-origins");
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-level-{level}.json"))).unwrap(),
    )
    .unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    project.php_version = 80500;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = oracle[file]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_ne!(
        want,
        [] as [(
            i64,
            std::string::String,
            std::string::String,
            std::option::Option<std::string::String>,
            bool
        ); 0]
    );
    assert_eq!(got, want, "{file}, level {level}");
}

#[test]
fn declared_signatures_names_unions_and_reference_positions() {
    for level in [2, 7] {
        check("signatures.php", level);
    }
}

#[test]
fn copies_reassignments_literals_and_reference_escapes_do_not_borrow_stale_signatures() {
    check("assignments.php", 2);
}
