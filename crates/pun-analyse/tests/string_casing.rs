//! Fresh PHPStan 2.3.1 ASCII casing intersections and asymmetric joins.

use std::path::Path;

use pun_analyse::{Project, run};

fn check(file: &str, expected_file: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/string-casing");
    let expected: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join(expected_file)).unwrap()).unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            serde_json::json!([
                file,
                e.line,
                e.identifier,
                e.message,
                e.tip,
                e.can_be_ignored
            ])
        })
        .collect();
    got.sort_by_key(|e| (e[1].as_i64().unwrap(), e[3].as_str().unwrap().to_owned()));
    assert_eq!(serde_json::Value::Array(got), expected);
}

#[test]
fn simultaneous_casing_survives_production_inference() {
    check("matrix.php", "expected.json", 7);
}

#[test]
fn nonempty_concatenation_refines_truthiness_without_strengthening_controls() {
    check("concat.php", "concat-expected.json", 0);
}
