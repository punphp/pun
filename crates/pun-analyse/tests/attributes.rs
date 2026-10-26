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

#[test]
fn global_initializer_reflection_is_distinct_from_file_local_flow() {
    check("phpstan-global-initializers");
}

#[test]
fn global_initializer_arrays_and_filtered_unions_keep_canonical_values() {
    check("phpstan-global-initializer-arrays");
}

#[test]
fn defined_global_initializer_reflection_and_local_flow_stay_distinct() {
    check("phpstan-global-initializer-defines");
}

#[test]
fn global_initializer_dependencies_resolve_before_filtering() {
    check("phpstan-global-initializer-dependencies");
    scanner_flags(
        include_str!("fixtures/attributes/global-initializer-dependencies.php"),
        include_str!("fixtures/attributes/expected-phpstan-global-initializer-dependencies.json"),
        true,
    );
}

/// Check the scanner's public metadata too: the rule must not be repairing missing initializer types.
#[test]
fn scanner_control_flow_flags_match_the_attribute_oracle() {
    scanner_flags(
        include_str!("fixtures/attributes/control-flow.php"),
        include_str!("fixtures/attributes/expected-phpstan-control-flow.json"),
        false,
    );
}

fn scanner_flags(source: &'static str, oracle: &str, resolved_globals: bool) {
    use std::borrow::Cow;
    use std::collections::{BTreeSet, HashMap};

    use mago_allocator::LocalArena;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::inference::infer_initializer;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::expander::{TypeExpansionOptions, expand_union};
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_names::scope::NamespaceScope;
    use mago_php_version::PHPVersion;
    use mago_span::{HasSpan, Span};
    use mago_syntax::cst::Expression;
    use mago_syntax::parser::parse_file;
    use mago_syntax::walker::MutWalker;

    let oracle: Value = serde_json::from_str(oracle).unwrap();
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
    struct Expressions<'ast, 'arena>(HashMap<Span, &'ast Expression<'arena>>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Expressions<'ast, 'arena> {
        fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
            self.0.insert(expression.span(), expression);
        }
    }
    let mut expressions = Expressions(HashMap::new());
    expressions.walk_program(program, &mut ());
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
        let argument = &attribute.arguments[0];
        let flags = if resolved_globals {
            infer_initializer(
                &file, &names, &NamespaceScope::global(),
                expressions.0[&argument.value_span.unwrap()], &codebase.constants, &HashMap::new(),
            )
        } else {
            argument.value_type.clone()
        }.and_then(|mut flags| {
                expand_union(&codebase, &mut flags, &TypeExpansionOptions::default());
                flags.get_single_literal_int_value()
            });
        let name = String::from_utf8_lossy(class.original_name.as_bytes());
        assert_eq!(flags.is_some(), !invalid.contains(name.as_ref()), "{name}");
        checked += 1;
    }
    assert_eq!(checked, source.matches("#[\\Attribute(").count());
}
