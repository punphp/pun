//! Coalescing diagnostics use source-input scopes, not synthetic writes or post-assignment artifacts.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn coalesce_inputs_match_phpstan() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(fixtures.join("coalesce-inputs/expected-phpstan.json")).unwrap(),
    )
    .unwrap();
    for (level, path) in [
        (
            1,
            "rules/data/Variables/data/null-coalesce-assign-right-side-scope.php",
        ),
        (
            4,
            "rules/data/Variables/data/bug-nullCoalesceMultiDimLoop.php",
        ),
        (4, "rules/data/Variables/data/bug-15046.php"),
        (1, "coalesce-inputs/inputs.php"),
        (4, "coalesce-inputs/inputs.php"),
    ] {
        let file = fixtures.join(path);
        let mut project = Project::new(vec![file.clone()], Vec::new());
        project.level = level;
        project.php_version = 80400;
        project.options.set(pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, true);
        let relevant = |identifier: &str| {
            identifier.starts_with("nullCoalesce.")
                || matches!(identifier, "variable.undefined" | "phpstan.dumpType")
        };
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|error| relevant(&error.identifier))
            .map(|error| (error.line, error.identifier, error.message))
            .collect();
        let mut want: Vec<_> = expected[level.to_string()]
            [file.file_name().unwrap().to_str().unwrap()]["messages"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|error| relevant(error["identifier"].as_str().unwrap()))
            .map(|error| {
                (
                    error["line"].as_i64().unwrap(),
                    error["identifier"].as_str().unwrap().to_string(),
                    error["message"].as_str().unwrap().to_string(),
                )
            })
            .collect();
        got.sort();
        want.sort();
        assert_eq!(got, want, "level {level}, {path}");
    }
}
