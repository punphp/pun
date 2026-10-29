//! The level-2 unspecified-declaration `void.pure` branch, not the complete Pure*Rule mappings.

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn declaration_effects_match_the_oracle_without_claiming_unknown_producers() {
    assert!(pun_analyse::reports("void.pure"));
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/declaration-purity");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/declaration-purity/expected.json")).unwrap();
    for level in [1, 2, 4, 10] {
        let mut project = Project::new(
            ["matrix", "supported", "boundaries"]
                .map(|name| root.join(format!("{name}.php")))
                .to_vec(),
            Vec::new(),
        );
        project.php_version = 80500;
        project.level = level;
        project.check_maybe_undefined_variables = true;
        let errors = analyse_project(&project).unwrap().errors;
        let actual = errors.into_iter()
            .map(|error| json!({
                "file": std::path::Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                "line": error.line, "identifier": error.identifier, "message": error.message,
                "tip": error.tip, "ignorable": error.can_be_ignored,
            })).collect::<Vec<_>>();
        let selected = |errors: &[Value]| {
            errors
                .iter()
                .filter(|error| {
                    (error["identifier"] == "void.pure"
                        && !error["file"]
                            .as_str()
                            .unwrap()
                            .starts_with("boundaries.php"))
                        || error["identifier"] == "ignore.unmatchedIdentifier"
                        || (error["file"] == "matrix.php"
                            && ["assign.invalidExpr", "variable.undefined"]
                                .contains(&error["identifier"].as_str().unwrap()))
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        let oracle = expected[level.to_string()].as_array().unwrap();
        // Unsupported oracle positives may become supported as their real producers land. Reject
        // false positives everywhere, but do not turn those known omissions into permanent negatives.
        for error in actual.iter().filter(|e| e["identifier"] == "void.pure") {
            assert!(oracle.contains(error), "level {level}: unexpected {error}");
        }
        assert_eq!(
            selected(&actual),
            selected(oracle),
            "level {level}: supported declaration contracts"
        );
    }
}

#[test]
fn constructor_declaration_effects_match_independent_phpstan_records() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/declaration-purity");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/declaration-purity/constructors.json"
    ))
    .unwrap();
    for level in [1, 2, 4, 10] {
        let mut project = Project::new(vec![root.join("constructors.php")], Vec::new());
        project.php_version = 80500;
        project.level = level;
        let actual: Vec<_> = analyse_project(&project)
            .unwrap()
            .errors
            .into_iter()
            .filter(|error| error.identifier == "void.pure")
            .map(|error| {
                json!({
                    "file": std::path::Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                    "line": error.line,
                    "identifier": error.identifier, "message": error.message,
                    "tip": error.tip, "ignorable": error.can_be_ignored,
                })
            })
            .collect();
        let all_oracle: Vec<_> = expected[level.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .filter(|error| error["identifier"] == "void.pure")
            .cloned()
            .collect();
        for error in &actual {
            assert!(
                all_oracle.contains(error),
                "level {level}: unexpected {error}"
            );
        }
        // Preserve the existing producer's unsupported-call boundary; the complete capture retains
        // those positives, and the preceding assertion rejects false positives even there.
        let supported_lines = include_str!("fixtures/declaration-purity/constructors.php")
            .lines()
            .position(|line| line.starts_with("// Unsupported argument policies"))
            .unwrap() as u64;
        let supported = |records: Vec<Value>| {
            records
                .into_iter()
                .filter(|error| error["line"].as_u64().unwrap() <= supported_lines)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            supported(actual),
            supported(all_oracle),
            "level {level}: constructor effects"
        );
    }
}
