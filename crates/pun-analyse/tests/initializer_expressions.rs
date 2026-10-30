//! Initializer reflection is not ordinary expression flow (actual PHPStan 2.3.1 captures).

use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn unsupported_initializer_reflection_is_mixed_not_a_missing_source_value() {
    let name = "fallback";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/initializer-expressions");
    let dir = root.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{dir}/{name}.neon")),
        ..Options::for_project(&dir)
    })
    .unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{name}.json"))).unwrap())
            .unwrap();
    assert_eq!(oracle["errors"], serde_json::json!([]));
    let mut want = Vec::new();
    for (file, body) in oracle["files"].as_object().unwrap() {
        for error in body["messages"].as_array().unwrap() {
            if reports(error["identifier"].as_str().unwrap()) {
                want.push((
                    file.clone(),
                    error["line"].as_i64().unwrap(),
                    error["identifier"].as_str().unwrap().to_owned(),
                    error["message"].as_str().unwrap().to_owned(),
                    error["ignorable"].as_bool().unwrap(),
                ));
            }
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
    assert_eq!(got, want, "{name}:\nmissing {missing:#?}\nextra {extra:#?}");
}
