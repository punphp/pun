//! IfHandler's declaration effects, compared with every public PHPStan diagnostic field.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn conditional_declaration_effects_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/branch-declaration-effects");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 2, 4, 9] {
        let mut project = Project::new(
            vec![root.join("branches.php"), root.join("dependencies.php")],
            vec![],
        );
        project.level = level;
        project.php_version = 80500;
        project.check_maybe_undefined_variables = level > 0;
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
        let want = expected[level.to_string()].as_array().unwrap();
        let got = actual.as_array().unwrap();
        if got != want {
            failures.push(json!({
                "level": level,
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
