//! Conditional expression type holders, observed through the public runner.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn conditional_types_match_phpstan() {
    assert_fixture("correlations.php", "expected.json");
}

#[test]
fn conditional_guard_boundaries_match_phpstan() {
    assert_fixture("guards.php", "expected-guards.json");
}

#[test]
fn conditional_invalidation_boundaries_match_phpstan() {
    assert_fixture("invalidation.php", "expected-invalidation.json");
}

fn assert_fixture(fixture: &str, capture: &str) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/conditional-type-correlation");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(capture)).unwrap()).unwrap();
    let mut failures = Vec::new();
    for mode in ["default", "quiet", "foreach-off", "loop-off"] {
        let config = resolve(&Options {
            config: Some(
                root.join(format!("{mode}.neon"))
                    .to_string_lossy()
                    .into_owned(),
            ),
            ..Options::for_project(&root.to_string_lossy())
        })
        .unwrap();
        let mut project = Project::from_config(&root, &config.parameters);
        project.analysed = vec![root.join(fixture)];
        let mut actual: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                (
                    Path::new(&error.file)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    error.line,
                    error.identifier,
                    error.message,
                    error.tip,
                    error.can_be_ignored,
                )
            })
            .collect();
        actual.sort();
        let actual = serde_json::to_value(actual).unwrap();
        if actual != expected[mode] {
            let want = expected[mode].as_array().unwrap();
            let got = actual.as_array().unwrap();
            failures.push(serde_json::json!({
                "mode": mode,
                "expected": want.len(),
                "actual": got.len(),
                "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
