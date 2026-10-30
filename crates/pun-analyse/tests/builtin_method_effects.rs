//! Canonical builtin effects remain separate from source PHPDoc and unknown user effects.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_codex::metadata::function_like::{FunctionLikeMetadata, ReflectedSideEffects};
use mago_codex::scanner::scan_program;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;
use mago_word::word;
use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};

#[test]
fn builtin_method_effects_match_complete_phpstan_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/builtin-method-effect-conditions");
    let mut expected: Vec<Value> = serde_json::from_str(include_str!(
        "fixtures/builtin-method-effect-conditions/expected.json"
    ))
    .unwrap();
    let mut project = Project::new(vec![root.join("input.php")], Vec::new());
    project.php_version = 80500;
    project.level = 4;
    let analysis = analyse_project(&project).unwrap();
    let mut actual: Vec<_> = analysis
        .errors
        .into_iter()
        .map(|error| {
            json!([
                Path::new(&error.file)
                    .file_name()
                    .unwrap()
                    .to_string_lossy(),
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored
            ])
        })
        .collect();
    actual.sort_by_cached_key(Value::to_string);
    expected.sort_by_cached_key(Value::to_string);
    println!(
        "builtin-method-effects={}",
        json!({"actual": actual, "oracle": expected})
    );
    assert_eq!(actual, expected);
    assert!(analysis.general.is_empty(), "{:?}", analysis.general);
}

#[test]
fn reflected_effects_round_trip_without_replacing_source_phpdoc() {
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"purity.php"),
        Cow::Borrowed(b"<?php /** @phpstan-pure */ function original(): int { return 1; }"),
    );
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let metadata = scan_program(
        &arena,
        &file,
        program,
        &names,
        mago_php_version::PHPVersion::new(8, 5, 0),
    );
    let original = &metadata.function_likes[&(word(b""), word(b"original"))];
    assert_eq!(original.reflected_side_effects, None);
    assert_eq!(original.declared_purity(), Some(true));
    let mut legacy = serde_json::to_value(original).unwrap();
    legacy
        .as_object_mut()
        .unwrap()
        .remove("reflected_side_effects");
    let restored: FunctionLikeMetadata = serde_json::from_value(legacy).unwrap();
    assert_eq!(restored.reflected_side_effects, None);
    assert_eq!(restored.declared_purity(), Some(true));
    for (effect, serialized) in [
        (ReflectedSideEffects::Yes, "Yes"),
        (ReflectedSideEffects::No, "No"),
        (ReflectedSideEffects::Maybe, "Maybe"),
    ] {
        let mut projected = original.clone();
        projected.reflected_side_effects = Some(effect);
        let encoded = serde_json::to_value(&projected).unwrap();
        assert_eq!(encoded["reflected_side_effects"], serialized);
        let decoded: FunctionLikeMetadata = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded.reflected_side_effects, Some(effect));
        assert_eq!(decoded.declared_purity(), Some(true));
    }
    let prelude = mago_prelude::Prelude::build();
    assert_eq!(
        prelude.metadata.function_likes[&(word(b"spldoublylinkedlist"), word(b"shift"))]
            .reflected_side_effects,
        None
    );
}
