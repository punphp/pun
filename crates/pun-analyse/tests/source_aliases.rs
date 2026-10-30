//! Source locators do not execute class_alias(). Expected use-site diagnostics are the retained PHPStan 2.3.1
//! CLI captures from the source-alias investigation, not the scanner's current aliases or bootstrap behaviour.

use std::path::Path;

use pun_analyse::analyse;
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

fn check(mode: &str, capture: &str, uses: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/source-aliases");
    let dir = root.to_string_lossy().into_owned();
    let config = resolve(&Options {
        config: Some(format!("{dir}/phpstan-{mode}.neon")),
        ..Options::for_project(&dir)
    })
    .unwrap();
    let prefix = format!("{dir}/");
    // Declaration-call argument checks are a separate contract. Compare every diagnostic at the shared lookup
    // consumers, including instantiation, static methods/constants, signatures and native attribute names.
    let mut got: Vec<_> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .filter_map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file);
            uses.contains(&file).then(|| {
                (
                    file.to_string(),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored,
                )
            })
        })
        .collect();
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-{capture}.json"))).unwrap(),
    )
    .unwrap();
    assert!(expected["general"].as_array().unwrap().is_empty());
    let mut want: Vec<_> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{mode}");
}

#[test]
fn source_aliases_do_not_create_classes() {
    check("minimal", "minimal", &["minimal.php"]);
}

#[test]
fn analysed_alias_calls_do_not_register_global_identities() {
    check("analysed", "source", &["uses.php", "false-use.php"]);
}

#[test]
fn reversing_analysed_files_does_not_execute_alias_calls() {
    check("reversed", "source", &["uses.php", "false-use.php"]);
}

#[test]
fn scanned_files_do_not_register_global_identities() {
    check("scanned", "source", &["uses.php", "false-use.php"]);
}

#[test]
fn scanned_directories_do_not_register_global_identities() {
    check("directory", "source", &["uses.php", "false-use.php"]);
}

#[test]
fn dead_dormant_guarded_and_shadowed_calls_do_not_register_aliases() {
    check("edge-source", "edge", &["edge-uses.php"]);
    check("edge-scanned", "edge", &["edge-uses.php"]);
}
