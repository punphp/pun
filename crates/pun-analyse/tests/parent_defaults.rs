//! Parent-relative template defaults, through PHPStan's public diagnostics and Mago's public type API.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn parent_constants_and_class_names_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parent-defaults");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parent-defaults/expected-phpstan.json"
    ))
    .unwrap();
    for level in [0, 5] {
        let mut project = Project::new(vec![root.join("constants.php")], Vec::new());
        project.level = level;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| (e.line, e.identifier, e.message, e.tip))
            .collect();
        let mut want: Vec<_> = expected[level.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e["line"].as_i64().unwrap(),
                    e["identifier"].as_str().unwrap().to_string(),
                    e["message"].as_str().unwrap().to_string(),
                    e["tip"].as_str().map(str::to_string),
                )
            })
            .collect();
        got.sort();
        want.sort();
        assert_eq!(got, want, "level {level}");
    }
}
