//! Canonical initializer holders differ from ordinary class-constant expression flow.

use std::path::Path;

use pun_analyse::analyse;
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn finite_class_strings_keep_all_canonical_alternatives() {
    assert_oracle("finite");
}

#[test]
fn object_holders_select_raw_or_effective_declarations() {
    assert_oracle("policies");
}

#[test]
fn forward_scanned_and_attribute_holders_share_initializer_resolution() {
    assert_oracle("scanned");
}

#[test]
fn heterogeneous_holders_keep_reflection_fallback_order() {
    assert_oracle("fallback");
}

#[test]
fn exact_holders_keep_first_mixed_or_error_identity_through_aliases() {
    assert_oracle("order");
}

#[test]
fn nested_raw_values_enums_and_enclosing_inference_keep_canonical_children() {
    assert_oracle("edges");
}

fn assert_oracle(group: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/initializer-holders")
        .join(group);
    let dir = root.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{dir}/phpstan.neon")),
        ..Options::for_project(&dir)
    })
    .unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    assert_eq!(oracle["errors"], serde_json::json!([]));
    let mut want = Vec::new();
    for (file, body) in oracle["files"].as_object().unwrap() {
        for error in body["messages"].as_array().unwrap() {
            want.push((
                file.clone(),
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["ignorable"].as_bool().unwrap(),
            ));
        }
    }
    let mut got: Vec<_> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                error
                    .file
                    .strip_prefix(&format!("{dir}/"))
                    .unwrap()
                    .to_owned(),
                error.line,
                error.identifier,
                error.message,
                error.can_be_ignored,
            )
        })
        .collect();
    got.sort();
    want.sort();
    let missing: Vec<_> = want.iter().filter(|e| !got.contains(e)).collect();
    let extra: Vec<_> = got.iter().filter(|e| !want.contains(e)).collect();
    assert_eq!(got, want, "missing {missing:#?}\nextra {extra:#?}");
}
