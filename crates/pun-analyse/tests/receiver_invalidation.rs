//! PHPStan 2.3.1 receiver-local mutation and independent receiver controls.

use std::path::Path;

use pun_analyse::{Project, run};

#[test]
fn method_effects_invalidate_only_affected_mutable_receiver_properties() {
    check(
        "receivers.php",
        include_str!("fixtures/receiver-invalidation/expected.json"),
        false,
    );
}

#[test]
fn stub_purity_overrides_keep_their_explicit_effects() {
    check(
        "patched.php",
        include_str!("fixtures/receiver-invalidation/expected-patched.json"),
        true,
    );
}

fn check(input: &str, expected: &str, patched: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/receiver-invalidation");
    let expected: serde_json::Value = serde_json::from_str(expected).unwrap();
    let mut project = Project::new(vec![root.join(input)], Vec::new());
    project.php_version = 80500;
    if patched {
        project.scanned.push(root.join("scanned.php"));
        project.stubs.push(root.join("patch.stub"));
    }
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
    assert_eq!(serde_json::to_value(actual).unwrap(), expected);
}
