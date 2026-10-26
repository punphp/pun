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
    check("phpstan-contracts-php74-level5");
}

#[test]
fn constructor_checks_follow_the_configured_level() {
    check("phpstan-contracts-level1");
    check("phpstan-contracts-level5");
}

#[test]
fn explicit_attribute_aliases_keep_the_written_name() {
    check("phpstan-input-aliases");
}

#[test]
fn parenthesized_attribute_flags_keep_the_inferred_value() {
    check("phpstan-input-flags");
}

#[test]
fn control_flow_attribute_flags_follow_initializer_types_not_runtime_branches() {
    check("phpstan-control-flow");
}

/// Check the scanner's public metadata too: the rule must not be repairing missing initializer types.
#[test]
fn scanner_control_flow_flags_match_the_attribute_oracle() {
    use std::borrow::Cow;
    use std::collections::BTreeSet;

    use mago_allocator::LocalArena;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::expander::{TypeExpansionOptions, expand_union};
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    let source = include_str!("fixtures/attributes/control-flow.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/attributes/expected-phpstan-control-flow.json"
    ))
    .unwrap();
    let invalid: BTreeSet<_> = oracle["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|error| error["identifier"] == "attribute.notAttribute")
        .map(|error| {
            error["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Class ")
                .unwrap()
                .strip_suffix(" is not an Attribute class.")
                .unwrap()
        })
        .collect();
    let file = File::new(
        Cow::Borrowed(b"control-flow.php"),
        FileType::Host,
        None,
        Cow::Borrowed(source.as_bytes()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::new(8, 4, 0));
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let mut checked = 0;
    for class in codebase.class_likes.values() {
        let Some(attribute) = class
            .attributes
            .iter()
            .find(|attribute| attribute.name.as_bytes() == b"Attribute")
        else {
            continue;
        };
        let flags = attribute.arguments[0]
            .value_type
            .clone()
            .and_then(|mut flags| {
                expand_union(&codebase, &mut flags, &TypeExpansionOptions::default());
                flags.get_single_literal_int_value()
            });
        let name = String::from_utf8_lossy(class.original_name.as_bytes());
        assert_eq!(flags.is_some(), !invalid.contains(name.as_ref()), "{name}");
        checked += 1;
    }
    assert_eq!(checked, source.matches("#[\\Attribute(").count());
}
