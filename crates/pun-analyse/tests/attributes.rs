//! Branches of the attribute rules that PHPStan's own rule fixtures miss: `#[Attribute]` flags given by constants and
//! expressions, `#[NoDiscard]` read from PHPDoc and on the methods PHPStan types void, built-in attribute classes, and
//! the checks gated on PHP versions below 8.5. Expected: PHPStan's run on tests/fixtures/attributes for each
//! phpstan*.neon there, restricted to the identifiers Pun reports.

use std::path::Path;

use pun_analyse::{IDENTIFIERS, analyse};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

type Key = (String, u32, String, String, Option<String>);

fn check(neon: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/attributes");
    let dir = root.to_string_lossy().into_owned();
    let options = Options {
        config: Some(format!("{dir}/{neon}.neon")),
        ..Options::for_project(&dir)
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let prefix = format!("{dir}/");
    let mut got: Vec<Key> = analyse(&root, &config)
        .unwrap()
        .into_iter()
        .map(|e| {
            let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
            (file, e.line, e.identifier, e.message, e.tip)
        })
        .collect();
    got.sort();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(format!("expected-{neon}.json"))).unwrap())
            .unwrap();
    let mut want: Vec<Key> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| IDENTIFIERS.contains(&e["identifier"].as_str().unwrap_or("")))
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_u64().unwrap() as u32,
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
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
