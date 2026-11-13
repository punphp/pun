//! Built-in functions' return types from PHPStan 2.3.5 (fixtures/builtin-returns/phpstan.neon): min()/max(), the
//! functionMap's integer ranges and non-empty strings, and array_column() with a union of constant keys.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn ranges_accessories_and_columns() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/builtin-returns");
    let mut project = Project::new(vec![root.join("types.php")], Vec::new());
    project.level = 5;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.message))
        .collect();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let want: Vec<_> = oracle["files"]["types.php"]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["message"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(want.len(), 23);
    assert_eq!(got, want);
}
