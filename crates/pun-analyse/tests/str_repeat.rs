//! Public inference expectations captured independently from PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};

fn check(file: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/str-repeat");
    let expected: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let expected: Vec<_> = expected
        .as_array()
        .unwrap()
        .iter()
        .filter(|error| error[0] == file)
        .cloned()
        .collect();
    assert_ne!(expected, [] as [serde_json::Value; 0]);
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    project
        .options
        .set(pun_analyse::ProjectOptions::SWITCH_CONDITION, true);
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            serde_json::json!([
                file,
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored
            ])
        })
        .collect();
    got.sort_by_key(|error| {
        (
            error[1].as_i64().unwrap(),
            error[2].as_str().unwrap().to_owned(),
            error[3].as_str().unwrap().to_owned(),
        )
    });
    assert_eq!(got, expected, "{file}");
}

#[test]
fn finite_products_zero_negative_and_flow() {
    check("finite.php", 0);
}

#[test]
fn aggregate_byte_budget_boundaries() {
    check("boundaries.php", 0);
}

#[test]
fn accessory_fallback_preserves_independent_guarantees() {
    check("accessories.php", 0);
}

#[test]
fn literal_bytes_do_not_collapse_invalid_utf8_values() {
    check("bytes.php", 0);
}

#[test]
fn byte_conversion_preserves_ascii_comparison_guarantees() {
    check("byte-comparisons.php", 4);
}

#[test]
fn dispatch_respects_the_existing_extension_boundary() {
    check("dispatch.php", 0);
}

#[test]
fn templates_and_class_strings_follow_their_concrete_parents() {
    check("templates.php", 0);
}
