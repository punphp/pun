//! Native intersections through public scanning, before Pun's panic recovery can mask their source.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::TType;
use mago_codex::ttype::union::TUnion;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;
use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn native_signature_metadata_matches_phpstan_without_masked_source_recovery() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native-intersections");
    let source = std::fs::read(root.join("types.php")).unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let file = File::ephemeral(Cow::Borrowed(b"types.php"), Cow::Owned(source.clone()));
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    // No catch_unwind or masking: the old empty-union assertion must fail this public control.
    let mut codebase = mago_prelude::Prelude::build().metadata;
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        Default::default(),
    ));
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    for (name, expected) in oracle["types"].as_object().unwrap() {
        let function = codebase.get_function(name.as_bytes()).unwrap();
        for (slot, metadata) in [
            (
                "parameter",
                function.parameters[0]
                    .type_declaration_metadata
                    .as_ref()
                    .unwrap(),
            ),
            (
                "return",
                function.return_type_declaration_metadata.as_ref().unwrap(),
            ),
        ] {
            let expected_type = expected[slot]["type"]
                .as_str()
                .unwrap()
                .replace("*NEVER*", "never");
            // as_nullable retains redundant never. Compare semantic types without mutating metadata.
            let normalized = TUnion::from_vec(metadata.type_union.types.to_vec());
            assert_eq!(
                normalized.get_id().to_string(),
                expected_type,
                "{name} {slot}"
            );
            assert!(!metadata.from_docblock, "{name} {slot}");
            assert!(!metadata.inferred, "{name} {slot}");
            let hint =
                &source[metadata.span.start.offset as usize..metadata.span.end.offset as usize];
            assert_eq!(
                hint,
                expected[slot]["hint"].as_str().unwrap().as_bytes(),
                "{name} {slot}"
            );
        }
    }
}

#[test]
fn public_diagnostics_keep_impossible_signatures_and_valid_neighbors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native-intersections");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut got: Vec<_> = run(&Project::new(vec![root.join("types.php")], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|error| {
            serde_json::json!([
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored
            ])
        })
        .collect();
    got.sort_by_key(Value::to_string);
    let mut expected = oracle["errors"].as_array().unwrap().clone();
    expected.sort_by_key(Value::to_string);
    assert_eq!(got, expected);
}
