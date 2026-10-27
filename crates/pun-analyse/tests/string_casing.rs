//! Fresh PHPStan 2.3.1 ASCII casing intersections and asymmetric joins.

use std::path::Path;

use pun_analyse::{Project, run};

#[test]
fn simultaneous_casing_survives_production_inference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/string-casing");
    let expected: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut project = Project::new(vec![root.join("matrix.php")], Vec::new());
    project.level = 7;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            serde_json::json!([
                "matrix.php",
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
