//! Branches of the attribute rules that PHPStan's own rule fixtures miss: `#[Attribute]` flags given by constants and
//! expressions, `#[NoDiscard]` read from PHPDoc and on the methods PHPStan types void, built-in attribute classes, and
//! the checks gated on PHP versions below 8.5. Expected: PHPStan's run on tests/fixtures/attributes for each
//! phpstan*.neon there, restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>, Option<bool>);

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/attributes");
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    // New oracle exports retain ignorability too; keep the original fixtures' expectations unchanged.
    let ignorability = expected["errors"].as_array().unwrap().iter().any(|e| e.get("ignorable").is_some());
    let mut got: Vec<Key> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip, ignorability.then_some(e.can_be_ignored))
        })
        .collect();
    got.sort();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| reports(e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool(),
            )
        })
        .collect();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn php_8_5() {
    check("phpstan");
}

/// Attributes on global constants, `#[Override]` on properties and `#[Deprecated]` on traits need PHP 8.5.
#[test]
fn php_8_4() {
    check("phpstan-php84");
}

#[test]
fn constructor_contracts_and_attribute_groups() {
    check("phpstan-contracts");
}

/// Attributes do not produce argument.namedNotSupported even when the configured PHP is older than 8.0.
#[test]
fn attribute_named_arguments_are_not_version_gated() {
    check("phpstan-contracts-php74");
}

#[test]
fn constructor_checks_follow_the_configured_level() {
    check("phpstan-contracts-level1");
    check("phpstan-contracts-level5");
}
