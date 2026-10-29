//! ForeachHandler's constant-entry conditional holders through both public analyzers.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program, ttype::TType,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use serde_json::Value;

fn expected() -> Value {
    serde_json::from_str(include_str!(
        "fixtures/foreach_conditional_holders/expected.json"
    ))
    .unwrap()
}

fn diagnostics(case: &str) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foreach_conditional_holders");
    let file = if case == "bug13054" {
        root.join("../rules/data/Methods/data/bug-13054.php")
    } else {
        root.join(format!("{case}.php"))
    };
    let oracle = expected();
    let mut failures = Vec::new();
    for level in [0, 5] {
        let mut project = pun_analyse::Project::new(vec![file.clone()], vec![]);
        project.level = level;
        project.php_version = 80500;
        let mut actual: Vec<_> = pun_analyse::run(&project)
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
        actual.sort();
        let actual = serde_json::to_value(actual).unwrap();
        let key = format!("{case}-{level}");
        let want = &oracle[&key]["errors"];
        eprintln!("{key}: {}", serde_json::to_string(&actual).unwrap());
        if &actual != want {
            failures.push(serde_json::json!({"case": key, "expected": want, "actual": actual}));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn bug13054_matches_complete_diagnostics() {
    diagnostics("bug13054");
}

#[test]
fn generic_union_overwrites_and_references_match_complete_diagnostics() {
    diagnostics("calls");
}

// These probes are scalar string unions; adapt display syntax only, retaining every alternative.
fn scalar_id(description: &str) -> String {
    let mut types: Vec<_> = description
        .split('|')
        .map(|part| {
            if part.starts_with('\'') {
                format!("string({part})")
            } else {
                part.to_owned()
            }
        })
        .collect();
    types.sort();
    types.join("|")
}

#[test]
fn constant_entry_observations_are_independent_of_native_tracking() {
    let source = include_bytes!("fixtures/foreach_conditional_holders/scopes.php");
    let oracle = expected();
    let mut failures = Vec::new();
    for native in [false, true] {
        let arena = LocalArena::new();
        let file = File::ephemeral(
            Cow::Borrowed(b"scopes.php"),
            Cow::Borrowed(source.as_slice()),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let mut settings = Settings::default();
        settings.track_native_types = native;
        let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let registry = PluginRegistry::default();
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
        let mut result = AnalysisResult::new(references);
        let artifacts = analyzer
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        for observation in oracle["scopes-0"]["observations"].as_array().unwrap() {
            let start = observation["start"].as_u64().unwrap() as u32;
            let end = observation["end"].as_u64().unwrap() as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(end));
            let actual = artifacts.get_expression_type(&span).map(|ty| {
                let mut parts: Vec<_> = ty
                    .types
                    .iter()
                    .map(|atomic| atomic.get_id().to_string())
                    .collect();
                parts.sort();
                parts.join("|")
            });
            let wanted = scalar_id(observation["type"].as_str().unwrap());
            if actual.as_deref() != Some(wanted.as_str()) {
                failures.push(serde_json::json!({"native": native, "observation": observation, "actual": actual}));
            }
            // The partial native API deliberately invalidates loop state before entering foreach.
            // Turning it on must neither fabricate native answers nor change ordinary holder inference.
            assert!(
                artifacts.get_native_expression_type(&span).is_none(),
                "{observation}"
            );
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
