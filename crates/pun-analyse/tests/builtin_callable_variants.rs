//! Source variants of builtin first-class callables, through both public Project constructors.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn source_variants_retain_required_parameters_and_configured_version() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/builtin-callable-variants");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/builtin-callable-variants/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for version in [80400, 80500] {
        for level in [4, 9] {
            let mut direct = Project::new(vec![root.join("callbacks.php")], Vec::new());
            direct.php_version = version;
            direct.level = level;
            let mut parameters = pun_phpstan::Array::new();
            parameters.set(
                pun_phpstan::Key::from("level"),
                pun_phpstan::Value::Int(level.into()),
            );
            parameters.set(
                pun_phpstan::Key::from("phpVersion"),
                pun_phpstan::Value::Int(version.into()),
            );
            let mut configured = Project::from_config(&root, &parameters);
            configured.analysed = direct.analysed.clone();
            for (constructor, project) in [("new", direct), ("from_config", configured)] {
                let mut actual: Vec<_> = run(&project)
                    .unwrap()
                    .into_iter()
                    .map(|error| {
                        json!([
                            Path::new(&error.file)
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap(),
                            error.line,
                            error.identifier,
                            error.message,
                            error.tip,
                            error.can_be_ignored
                        ])
                    })
                    .collect();
                actual.sort_by_key(Value::to_string);
                let expected = &oracle["errors"][format!("{version}/{level}")];
                if actual != *expected.as_array().unwrap() {
                    failures.push(
                        json!({"version": version, "level": level, "constructor": constructor,
                        "expected": expected, "actual": actual}),
                    );
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}
