//! Raw initializer objects and ordinary constructor inference follow different PHPStan contracts.

use std::path::Path;

use pun_analyse::analyse;
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn new_initializers_keep_raw_object_identity_and_local_constructor_inference() {
    assert_oracle("phpstan.neon", "expected-phpstan.json");
}

#[test]
fn raw_object_readers_keep_declarations_member_defaults_and_cached_receivers_distinct() {
    assert_oracle("phpstan-readers.neon", "expected-readers.json");
}

fn assert_oracle(config: &str, capture: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/initializer-objects");
    let dir = root.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{dir}/{config}")),
        ..Options::for_project(&dir)
    })
    .unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(capture)).unwrap()).unwrap();
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
