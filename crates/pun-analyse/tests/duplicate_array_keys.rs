//! Public rule expectations from PHPStan 2.3.1, including its upstream union-key fixtures.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(project: Project, errors: &Value) {
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let mut want: Vec<_> = errors
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap_or(true),
            )
        })
        .collect();
    want.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert_eq!(
        got,
        want,
        "{:?}; PHP {}; bleeding edge {}",
        project.analysed,
        project.php_version,
        project
            .options
            .contains(pun_analyse::ProjectOptions::BLEEDING_EDGE)
    );
}

#[test]
fn upstream_duplicate_keys_and_array_regressions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("Arrays/DuplicateKeysInLiteralArraysRuleTest.json")).unwrap(),
    )
    .unwrap();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["files"][0].as_str().unwrap();
        check(
            Project::new(vec![root.join("data").join(file)], Vec::new()),
            &case["errors"],
        );
    }
}

#[test]
fn inferred_keys_and_pending_union_promotion() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    check(
        Project::new(vec![root.join("keys.php")], Vec::new()),
        &expected["files"]["keys.php"]["messages"],
    );
}

#[test]
fn float_casts_constants_and_overridable_receivers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-scalars.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("scalars.php")], Vec::new());
    project.php_version = 80400;
    check(project, &expected["files"]["scalars.php"]["messages"]);
}

#[test]
fn compound_constant_initializers_use_the_declaring_scope() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-initializers.json")).unwrap())
            .unwrap();
    let mut project = Project::new(
        vec![root.join("initializers.php")],
        vec![root.join("initializers-source.php")],
    );
    project.php_version = 80400;
    check(project, &expected["files"]["initializers.php"]["messages"]);
}

#[test]
fn platform_integer_copies_preserve_bitness_and_actual_narrowing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-platform.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("platform.php")], Vec::new());
    project.php_version = 80400;
    check(project, &expected["files"]["platform.php"]["messages"]);
}

#[test]
fn full_arithmetic_and_constant_resolution_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-full-constants.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("full-constants.php")], Vec::new());
    project.php_version = 80400;
    check(
        project,
        &expected["files"]["full-constants.php"]["messages"],
    );
}

#[test]
fn wide_integer_literals_and_copies_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-wide-integers.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("wide-integers.php")], Vec::new());
    project.php_version = 80400;
    check(project, &oracle["errors"]);
}

#[test]
fn wide_integer_cst_preserves_php_parser_values_bytes_and_spans() {
    use mago_allocator::LocalArena;
    use mago_database::file::File;
    use mago_span::HasSpan;
    use mago_syntax::cst::{Expression, Literal};
    use mago_syntax::parser::parse_file;
    use mago_syntax::walker::MutWalker;
    use serde_json::json;

    struct Literals(Vec<Value>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Literals {
        fn walk_in_expression(&mut self, expr: &'ast Expression<'arena>, _: &mut ()) {
            let Expression::Literal(literal) = expr else {
                return;
            };
            let (kind, raw, value) = match literal {
                Literal::Integer(i) => ("Scalar_Int", i.raw, i.value.unwrap().to_string()),
                Literal::Float(f) => (
                    "Scalar_Float",
                    f.raw,
                    format!("{:016x}", f.value.0.to_bits()),
                ),
                _ => return,
            };
            self.0.push(json!({
                "kind": kind,
                "raw": std::str::from_utf8(raw).unwrap(),
                "value": value,
                "start": literal.span().start.offset,
                "end": literal.span().end.offset,
            }));
        }
    }
    let source = include_bytes!("fixtures/duplicate-array-keys/wide-integers.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/duplicate-array-keys/expected-wide-integers.json"
    ))
    .unwrap();
    let file = File::ephemeral(
        b"wide-integers.php".as_slice().into(),
        source.as_slice().into(),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let mut literals = Literals(Vec::new());
    literals.walk_program(program, &mut ());
    assert_eq!(json!(literals.0), oracle["literals"]);
}

fn unpacking(config: &str, php_version: u32, bleeding_edge: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-{config}.json"))).unwrap(),
    )
    .unwrap();
    let mut project = Project::new(vec![root.join("unpacking.php")], Vec::new());
    project.php_version = php_version;
    project
        .options
        .set(pun_analyse::ProjectOptions::BLEEDING_EDGE, bleeding_edge);
    check(project, &expected["files"]["unpacking.php"]["messages"]);
}

#[test]
fn unpacking_preserves_default_unknown_sealedness() {
    unpacking("unpacking", 80400, false);
}

#[test]
fn unpacking_sealed_shapes_with_bleeding_edge() {
    unpacking("unpacking-bleeding", 80400, true);
}

#[test]
fn unpacking_before_php81_renumbers_string_keys() {
    unpacking("unpacking-php80", 80000, true);
}

#[test]
fn quoted_shapes_reach_duplicate_keys_through_scanning_and_inline_annotations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-quoted-shapes.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("quoted-shapes.php")], Vec::new());
    project.php_version = 80400;
    project
        .options
        .set(pun_analyse::ProjectOptions::BLEEDING_EDGE, true);
    check(project, &oracle["errors"]);
}

#[test]
fn raw_phpdoc_ingestion_preserves_binary_values_and_error_offsets() {
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/duplicate-array-keys/expected-raw-doc.json"
    ))
    .unwrap();
    let source: Vec<u8> = oracle["source_hex"]
        .as_str()
        .unwrap()
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    let root = std::env::temp_dir().join(format!("pun-raw-doc-test-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let file = root.join("raw.php");
    std::fs::write(&file, source).unwrap();
    // Remove the scratch source even when the diagnostic assertion fails.
    let result = std::panic::catch_unwind(|| {
        for level in [0, 2] {
            let mut project = Project::new(vec![file.clone()], Vec::new());
            project.level = level;
            project
                .options
                .set(pun_analyse::ProjectOptions::BLEEDING_EDGE, true);
            check(project, &oracle["levels"][level.to_string()]);
        }
    });
    std::fs::remove_dir_all(&root).unwrap();
    if let Err(error) = result {
        std::panic::resume_unwind(error);
    }
}

#[test]
fn scanned_quoted_shape_keys_and_overwrites_match_phpstan_hex_values() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::array::TArray;
    use mago_codex::ttype::atomic::array::key::ArrayKey;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_syntax::parser::parse_file;
    use serde_json::json;

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate-array-keys");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-quoted-shapes.json")).unwrap())
            .unwrap();
    let file = File::ephemeral(
        Cow::Borrowed(b"quoted-shapes.php"),
        Cow::Owned(std::fs::read(root.join("quoted-shapes.php")).unwrap()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let codebase = scan_program(&arena, &file, program, &names, Default::default());
    let hex = |bytes: &[u8]| {
        bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    for (name, expected) in oracle["shapes"].as_object().unwrap() {
        let function = codebase
            .get_function(format!("QuotedShapeKeys\\{name}").as_bytes())
            .unwrap();
        let ty = &function.parameters[0]
            .type_metadata
            .as_ref()
            .unwrap()
            .type_union;
        let [TAtomic::Array(TArray::Keyed(shape))] = ty.types.as_ref() else {
            panic!("expected one keyed shape for {name}: {ty:?}");
        };
        let mut entries: Vec<_> = shape
            .known_items
            .as_ref()
            .unwrap()
            .iter()
            .map(|(key, (_, value))| {
                let key = match key {
                    ArrayKey::Integer(value) => json!(["int", value]),
                    ArrayKey::String(value) => json!(["string", hex(value.as_bytes())]),
                    _ => panic!("unexpected shape key: {key:?}"),
                };
                let value = value.get_single_literal_string_value().unwrap();
                json!([key, hex(value)])
            })
            .collect();
        entries.sort_by_key(Value::to_string);
        let mut expected = expected.as_array().unwrap().clone();
        expected.sort_by_key(Value::to_string);
        assert_eq!(entries, expected, "{name}");
    }
}

#[test]
fn bare_shape_spellings_and_object_properties_keep_their_existing_values() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::array::TArray;
    use mago_codex::ttype::atomic::object::TObject;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_syntax::parser::parse_file;

    // The parser's bare-float extension is not a PHPStan shape-key grammar claim.
    // Object property names retain their prior undecoded interior; only array semantics change.
    let source = br#"<?php
/** @param array{1.5: int, +1.5: int, -1.5: int, name: int} $shape */
function bare(array $shape): void {}
/** @param object{"\x61": int} $shape */
function objectShape(object $shape): void {}
"#;
    let file = File::ephemeral(Cow::Borrowed(b"bare.php"), Cow::Borrowed(source));
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let codebase = scan_program(&arena, &file, program, &names, Default::default());
    let bare = codebase.get_function(b"bare").unwrap();
    let bare = &bare.parameters[0]
        .type_metadata
        .as_ref()
        .unwrap()
        .type_union;
    let [TAtomic::Array(TArray::Keyed(shape))] = bare.types.as_ref() else {
        panic!("expected keyed shape: {bare:?}");
    };
    let keys: Vec<_> = shape
        .known_items
        .as_ref()
        .unwrap()
        .keys()
        .map(|key| key.get_string().unwrap())
        .collect();
    assert_eq!(keys, [b"+1.5".as_slice(), b"-1.5", b"1.5", b"name"]);

    let object = codebase.get_function(b"objectshape").unwrap();
    let object = &object.parameters[0]
        .type_metadata
        .as_ref()
        .unwrap()
        .type_union;
    let [TAtomic::Object(TObject::WithProperties(object))] = object.types.as_ref() else {
        panic!("expected object shape: {object:?}");
    };
    let keys: Vec<_> = object
        .known_properties
        .keys()
        .map(|key| key.as_bytes())
        .collect();
    assert_eq!(keys, [br"\x61".as_slice()]);
}
