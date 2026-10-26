//! Canonical trait initializers, from fresh actual PHPStan 2.3.1 public analyses.

use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

fn check(name: &str) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/trait-constant-initializers");
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
            let identifier = error["identifier"].as_str().unwrap();
            if reports(identifier) {
                want.push((
                    file.clone(),
                    error["line"].as_i64().unwrap(),
                    identifier.to_owned(),
                    error["message"]
                        .as_str()
                        .unwrap()
                        .replace(&dir, "<fixtures>"),
                    error["tip"].as_str().map(str::to_owned),
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
                error.message.replace(&dir, "<fixtures>"),
                error.tip,
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

#[test]
fn reflected_values_and_conflicts_use_consumer_scopes_and_whole_unions() {
    check("level0");
}

#[test]
fn declarations_and_trait_attributes_keep_ordinary_consumer_checks() {
    check("edges");
}

#[test]
fn trait_dependent_attribute_flags_retain_exact_integer_requirements() {
    check("flags");
}

#[test]
fn parent_selection_shadowed_dependencies_and_ordered_shapes() {
    check("selection");
}

#[test]
fn cross_file_magic_constants_distinguish_reflection_from_declaration_scope() {
    check("magic");
    check("magic-paths");
}
