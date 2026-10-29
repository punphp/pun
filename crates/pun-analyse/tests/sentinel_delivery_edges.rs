//! Source-owned array metadata and selected call dependencies, captured with PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn public_values_keep_initializer_order_and_selected_scope_dependencies() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sentinel-delivery-edges");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    for level in [0, 4, 5] {
        let mut project = Project::new(vec![root.join("values.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored,]))
            .collect();
        got.sort_by_key(Value::to_string);
        assert_eq!(
            got,
            oracle["errors"][level.to_string()]
                .as_array()
                .unwrap()
                .clone(),
            "level {level}"
        );
    }
}
