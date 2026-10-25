//! UnsetRule's public contract from PHPStan 2.3.1's rule fixtures and actual oracle output.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(project: Project, expected: &Value) -> Result<(), String> {
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = expected
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap_or(true),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    if got == want {
        Ok(())
    } else {
        Err(format!(
            "{:?}; PHP {}\nactual: {got:?}\nexpected: {want:?}",
            project.analysed, project.php_version
        ))
    }
}

#[test]
fn upstream_unset_rule() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("Variables/UnsetRuleTest.json")).unwrap())
            .unwrap();
    let mut failures = Vec::new();
    for case in expected["cases"].as_array().unwrap() {
        let paths = |key: &str| {
            case[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| root.join("data").join(p.as_str().unwrap()))
                .collect()
        };
        if let Err(error) = check(Project::new(paths("files"), paths("scan")), &case["errors"]) {
            failures.push(format!("{}: {error}", case["method"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

#[test]
fn readonly_array_access_offset_keeps_constructor_arguments() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("Properties/MissingReadOnlyPropertyAssignRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = expected["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == "testBug13856")
        .unwrap();
    let file = root.join("data").join(case["files"][0].as_str().unwrap());
    let mut project = Project::new(vec![file], Vec::new());
    project.php_version = 80400;
    check(project, &case["errors"]).unwrap();
}

fn oracle(name: &str, expected_name: &str, php_version: u32) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unset");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(expected_name)).unwrap()).unwrap();
    let messages = expected["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"]
        .clone();
    let mut project = Project::new(vec![root.join(name)], Vec::new());
    project.php_version = php_version;
    check(project, &messages).unwrap();
}

#[test]
fn recursive_offsets_and_entry_certainty() {
    oracle("offsets.php", "offsets.json", 80400);
}

#[test]
fn native_properties_and_reflection_controls() {
    oracle("properties.php", "properties.json", 80400);
}

#[test]
fn key_casts_and_unsealed_offsets() {
    oracle("keys.php", "keys.json", 80400);
}

#[test]
fn unset_transfers_keep_incoming_metadata_and_invalidate_mutations() {
    oracle("transfers.php", "transfers.json", 80400);
}

#[test]
fn template_array_intersections_preserve_only_surviving_identities() {
    oracle("template-arrays.php", "template-arrays.json", 80400);
}

#[test]
fn generic_key_subtraction_matches_phpstan() {
    oracle("generic-keys.php", "generic-keys.json", 80400);
}

#[test]
fn generic_key_and_value_identities_match_phpstan() {
    oracle(
        "generic-key-identities.php",
        "generic-key-identities.json",
        80400,
    );
}

#[test]
fn virtual_unset_write_back_matches_phpstan() {
    oracle("write-back.php", "write-back.json", 80400);
}

#[test]
fn generic_parent_write_back_matches_phpstan() {
    oracle("generic-parents.php", "generic-parents.json", 80400);
}

#[test]
fn entry_reads_preserve_compatible_evaluation_and_single_index_effects() {
    oracle("entry-reads.php", "entry-reads.json", 80400);
}

#[test]
fn possibly_hooked_properties_depend_on_php_version() {
    oracle("version.php", "version.json", 80400);
    oracle("version.php", "version-82.json", 80200);
}
