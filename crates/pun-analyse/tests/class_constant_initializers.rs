//! Canonical class initializer values and lexical scopes, from actual PHPStan 2.3.1 runs.

use std::path::Path;

use pun_analyse::{analyse, reports};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

fn check(name: &str) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/class-constant-initializers");
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
                    error["message"].as_str().unwrap().to_owned(),
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
    got.sort();
    want.sort();
    assert_eq!(got, want, "{name}");
}

#[test]
fn canonical_class_values_keep_unions_and_declaring_scopes() {
    check("values");
}

#[test]
fn exact_and_late_bound_values_do_not_replace_declared_types() {
    check("declared");
    check("declared-level2");
}

#[test]
fn inherited_interfaces_attributes_and_metadata_only_roots() {
    check("scopes");
}

#[test]
fn wildcard_signatures_and_class_string_reads_share_canonical_metadata() {
    check("metadata-roots");
}

#[test]
fn integer_bitmasks_keep_all_initializer_alternatives() {
    check("bitmasks");
}

#[test]
fn missing_scopes_and_existing_self_cycle_guards_do_not_invent_values() {
    check("safety");
}

#[test]
fn existing_trait_array_inputs_and_required_receivers_keep_their_contracts() {
    check("neighbors");
}

#[test]
fn unsupported_scanner_inputs_retain_the_existing_bounds() {
    use mago_allocator::LocalArena;
    use mago_codex::{
        metadata::CodebaseMetadata, scanner::inference::infer_initializer_with_codebase,
        ttype::get_int,
    };
    use mago_database::file::File;
    use mago_names::{resolver::NameResolver, scope::NamespaceScope};
    use mago_syntax::{cst::Statement, parser::parse_file};
    use std::collections::HashMap;

    // Mago bounds invalid/unknown integer bitwise inputs by int. This preserves its existing
    // scanner contract, not PHPStan's error reporting or additional initializer grammar.
    for (source, expected) in [
        ("missing() | 1", Some(get_int())),
        ("1 << -1", Some(get_int())),
        ("1 >> -1", Some(get_int())),
        ("$holder::FLAG", None),
    ] {
        let file = File::ephemeral(
            b"bounds.php".as_slice().into(),
            format!("<?php {source};").into_bytes().into(),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let Statement::Expression(statement) = &program.statements[1] else {
            panic!("expression")
        };
        assert_eq!(
            infer_initializer_with_codebase(
                &file,
                &names,
                &NamespaceScope::new(None),
                statement.expression,
                &CodebaseMetadata::new(),
                None,
                &HashMap::new(),
            ),
            expected,
            "{source}"
        );
    }
}

#[test]
fn scanner_class_dependencies_do_not_require_runtime_expression_overlays() {
    use mago_allocator::LocalArena;
    use mago_codex::{
        populator::populate_codebase,
        reference::SymbolReferences,
        scanner::{inference::infer_initializer_with_codebase, scan_program},
    };
    use mago_database::file::File;
    use mago_names::{resolver::NameResolver, scope::NamespaceScope};
    use mago_php_version::PHPVersion;
    use mago_syntax::{cst::Statement, parser::parse_file};
    use mago_word::word;
    use std::collections::HashMap;

    let file = File::ephemeral(
        b"holders.php".as_slice().into(),
        b"<?php
        class Base { const ZERO = 0; const BOTH = true ? 1 : 4; const UNKNOWN = unsupported(); }
        class Child extends Base {}
    "
        .as_slice()
        .into(),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut metadata = scan_program(&arena, &file, program, &names, PHPVersion::new(8, 4, 0));
    populate_codebase(
        &mut metadata,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    for (source, expected) in [
        ("self::ZERO ?: 1", vec![1]),
        ("parent::ZERO ?? 1", vec![0, 1]),
        ("Base::BOTH", vec![1, 4]),
        ("Child::BOTH", vec![1, 4]),
    ] {
        let file = File::ephemeral(
            b"read.php".as_slice().into(),
            format!("<?php {source};").into_bytes().into(),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let Statement::Expression(statement) = &program.statements[1] else {
            panic!("expression")
        };
        let value = infer_initializer_with_codebase(
            &file,
            &names,
            &NamespaceScope::new(None),
            statement.expression,
            &metadata,
            Some(word("Child")),
            &HashMap::new(),
        )
        .unwrap();
        let mut values: Vec<_> = value
            .types
            .iter()
            .map(|value| value.get_literal_int_value().unwrap())
            .collect();
        values.sort_unstable();
        assert_eq!(values, expected, "{source}");
    }
    let file = File::ephemeral(
        b"unresolved.php".as_slice().into(),
        b"<?php Base::UNKNOWN;".as_slice().into(),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let Statement::Expression(statement) = &program.statements[1] else {
        panic!("expression")
    };
    assert!(
        infer_initializer_with_codebase(
            &file,
            &names,
            &NamespaceScope::new(None),
            statement.expression,
            &metadata,
            None,
            &HashMap::new()
        )
        .unwrap()
        .is_mixed()
    );
}

#[test]
fn memoized_property_reads_keep_all_class_constant_editor_targets() {
    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, artifacts::editor::MemberKind,
        plugin::PluginRegistry, settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    let source = include_bytes!("fixtures/class-constant-initializers/safety.php");
    let file = File::ephemeral(b"safety.php".as_slice().into(), source.as_slice().into());
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let version = PHPVersion::new(8, 4, 0);
    let mut metadata = scan_program(&arena, &file, program, &names, version);
    populate_codebase(
        &mut metadata,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let registry = PluginRegistry::default();
    let analyzer = Analyzer::new(
        &arena,
        &file,
        &names,
        &metadata,
        &registry,
        Settings::new(version),
    )
    .with_editor_artifacts();
    let mut result = AnalysisResult::new(SymbolReferences::new());
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let editor = artifacts.editor_data(source, &metadata).unwrap();
    let targets: Vec<_> = editor
        .members
        .iter()
        .filter(|reference| {
            reference.kind == MemberKind::ClassConstant && reference.name.as_bytes() == b"ITEM"
        })
        .collect();
    assert_eq!(
        targets.len(),
        4,
        "both self and static reads, including skipped memoized receivers"
    );
    let declaration = source
        .windows(6)
        .position(|text| text == b"ITEM =")
        .unwrap() as u32;
    for reference in targets {
        assert_eq!(
            reference
                .declaration
                .map(|span| (span.start.offset, span.end.offset)),
            Some((declaration, declaration + 4))
        );
        assert!(!reference.uncertain);
        assert!(reference.static_access);
    }
}

#[test]
fn serialized_prelude_metadata_preserves_raw_unions_and_declarations() {
    use mago_allocator::LocalArena;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::{get_int, get_literal_int};
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_prelude::Prelude;
    use mago_syntax::parser::parse_file;

    let file = File::ephemeral(
        b"unions.php".as_slice().into(),
        b"<?php
        class Values {
            /** @var int */ const int FLAGS = true ? 1 : 4;
            const COPY = self::FLAGS;
            const RECURSIVE = self::RECURSIVE;
        }
        enum State: int {
            const YES = 1;
            case Active = self::YES;
        }
    "
        .as_slice()
        .into(),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut prelude = Prelude {
        metadata: scan_program(&arena, &file, program, &names, PHPVersion::new(8, 4, 0)),
        ..Default::default()
    };
    populate_codebase(
        &mut prelude.metadata,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let prelude = Prelude::decode(&prelude.encode().unwrap()).unwrap();
    let flags = prelude
        .metadata
        .get_class_constant(b"Values", b"FLAGS")
        .unwrap();
    let raw = flags.inferred_type.as_ref().unwrap();
    assert_eq!(raw.types.len(), 2);
    assert!(raw.types.contains(get_literal_int(1).get_single()));
    assert!(raw.types.contains(get_literal_int(4).get_single()));
    assert_eq!(
        flags.type_declaration.as_ref().unwrap().type_union,
        get_int()
    );
    assert_eq!(flags.type_metadata.as_ref().unwrap().type_union, get_int());
    assert!(flags.type_metadata.as_ref().unwrap().from_docblock);
    assert_eq!(flags.value_type(true), get_int());
    let copy = prelude
        .metadata
        .get_class_constant_type(b"Values", b"COPY")
        .unwrap();
    assert_eq!(copy.types.len(), 2);
    assert!(copy.types.contains(get_literal_int(1).get_single()));
    assert!(copy.types.contains(get_literal_int(4).get_single()));
    assert!(
        prelude
            .metadata
            .get_class_constant(b"Values", b"RECURSIVE")
            .unwrap()
            .inferred_type
            .as_ref()
            .unwrap()
            .is_never()
    );
    assert_eq!(
        prelude
            .metadata
            .get_enum_case(b"State", b"Active")
            .unwrap()
            .value_type
            .as_ref(),
        Some(get_literal_int(1).get_single())
    );
}
