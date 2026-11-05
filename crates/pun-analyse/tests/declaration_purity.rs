//! Declaration effects and the builtin-parent purity override boundary, not complete Pure*Rule mappings.

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn generic_array_declaration_effects_match_complete_phpstan_records() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/generic-array-declaration-effects");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/generic-array-declaration-effects/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for (file, levels) in [
        ("arrays", [0, 4, 10].as_slice()),
        ("object-bounds", [0, 4, 10].as_slice()),
        // Effect eligibility is independent of level 10's strict mixed-offset legality producer.
        ("mixed-bounds", [0, 4].as_slice()),
    ] {
        for &level in levels {
            let mut project = Project::new(vec![root.join(format!("{file}.php"))], Vec::new());
            project.php_version = 80500;
            project.level = level;
            project.options.set(
                pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
                level > 0,
            );
            let analysis = analyse_project(&project).unwrap();
            assert!(analysis.general.is_empty(), "{:?}", analysis.general);
            let mut actual: Vec<_> = analysis
                .errors
                .into_iter()
                .map(|error| {
                    json!([
                        std::path::Path::new(&error.file)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                        error.line,
                        error.identifier,
                        error.message,
                        error.tip,
                        error.can_be_ignored,
                    ])
                })
                .collect();
            actual.sort_by_cached_key(Value::to_string);
            let mut want = expected[file][level.to_string()]
                .as_array()
                .unwrap()
                .clone();
            want.sort_by_cached_key(Value::to_string);
            if actual != want {
                failures.push(json!({
                    "file": file, "level": level,
                    "missing": want.iter().filter(|record| !actual.contains(record)).collect::<Vec<_>>(),
                    "extra": actual.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                }));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn unset_declaration_effects_match_complete_phpstan_records() {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/declaration-purity");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/declaration-purity/unset.json")).unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 2, 4, 5, 8, 10] {
        let mut project = Project::new(vec![root.join("unset.php")], Vec::new());
        project.php_version = 80500;
        project.level = level;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            level > 0,
        );
        let mut actual: Vec<_> = analyse_project(&project)
            .unwrap()
            .errors
            .into_iter()
            .map(|error| {
                json!({
                    "file": std::path::Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                    "line": error.line, "identifier": error.identifier, "message": error.message,
                    "tip": error.tip, "ignorable": error.can_be_ignored,
                })
            })
            .collect();
        actual.sort_by_key(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["message"].as_str().unwrap().to_owned(),
            )
        });
        let want = expected[level.to_string()].as_array().unwrap();
        if &actual != want {
            failures.push(json!({
                "level": level,
                "missing": want.iter().filter(|record| !actual.contains(record)).collect::<Vec<_>>(),
                "extra": actual.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

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
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            true,
        );
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

#[test]
fn builtin_purity_overrides_match_complete_phpstan_records_through_project() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/builtin-purity-override");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/builtin-purity-override/expected.json"
    ))
    .unwrap();
    for mode in ["enabled", "disabled", "no-overrides"] {
        let mut project = Project::new(vec![root.join("contracts.php")], Vec::new());
        project.php_version = 80500;
        project.level = 3;
        project.options.set(
            pun_analyse::ProjectOptions::REPORT_METHOD_PURITY_OVERRIDE,
            mode != "disabled",
        );
        project.method_rules.overriding_method = mode != "no-overrides";
        let mut actual: Vec<_> = analyse_project(&project)
            .unwrap()
            .errors
            .into_iter()
            .map(|error| {
                json!({
                    "file": std::path::Path::new(&error.file).file_name().unwrap().to_str().unwrap(),
                    "line": error.line, "identifier": error.identifier, "message": error.message,
                    "tip": error.tip, "ignorable": error.can_be_ignored,
                })
            })
            .collect();
        actual.sort_by_key(|error| {
            (
                error["file"].as_str().unwrap().to_string(),
                error["line"].as_i64().unwrap(),
                error["message"].as_str().unwrap().to_string(),
            )
        });
        assert_eq!(actual, *expected[mode].as_array().unwrap(), "{mode}");
    }
}
