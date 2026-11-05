//! Named function failures preserve ErrorType without replacing resolved returns or argument writes.

use std::path::Path;

use pun_analyse::{Project, analyse_project};
use serde_json::Value;

#[test]
fn failed_named_function_results_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-functions");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    for level in [0, 2, 10] {
        let mut project = Project::new(
            vec![root.join("matrix.php"), root.join("guards.php")],
            Vec::new(),
        );
        project.php_version = 80500;
        project.level = level;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            level >= 1,
        );
        let mut actual: Vec<_> = analyse_project(&project)
            .unwrap()
            .errors
            .into_iter()
            .map(|error| {
                (
                    Path::new(&error.file)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    error.line,
                    error.identifier,
                    error.message,
                    error.tip,
                    error.can_be_ignored,
                )
            })
            .collect();
        actual.sort();
        assert_eq!(
            serde_json::to_value(actual).unwrap(),
            expected[level.to_string()],
            "PHPStan level {level}"
        );
    }
}
