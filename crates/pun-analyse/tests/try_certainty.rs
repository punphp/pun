//! PHPStan 2.3.5's try/catch certainty: impossible catches do not add an incoming scope.
//! Expectations are the standalone oracle's JSON from fixtures/try-certainty/phpstan.neon.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn only_reachable_catches_contribute_variable_certainty() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/try-certainty");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    for (file, expected_file) in [
        ("transfers.php", "transfers.php"),
        ("../rules/data/Exceptions/data/bug-4806.php", "bug-4806.php"),
    ] {
        let mut project = Project::new(vec![root.join(file)], Vec::new());
        project.level = 4;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            true,
        );
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|error| error.identifier.starts_with("variable."))
            .map(|error| (error.line, error.identifier, error.message))
            .collect();
        let mut want: Vec<_> = oracle[expected_file]
            .as_array()
            .unwrap()
            .iter()
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
        assert_eq!(got, want, "{file}");
    }
}
