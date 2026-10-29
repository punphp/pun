//! Canonical initializer values retain each source alternative's array order (PHPStan 2.3.1).

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn conditional_and_union_initializers_keep_source_owned_order_without_selecting_a_branch() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/initializer-array-order");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    for level in [0, 4] {
        let mut project = Project::new(vec![root.join("values.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
            .collect();
        got.sort_by_key(Value::to_string);
        assert_eq!(
            got,
            *oracle["errors"][level.to_string()].as_array().unwrap(),
            "level {level}"
        );
    }
}
