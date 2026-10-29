//! PHPStan retains a declared static signature after diagnosing an interface call.

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
use pun_analyse::{Project, run};
use serde_json::{Value, json};

const ORACLE: &str = include_str!("fixtures/interface-static/expected.json");

#[test]
fn interface_calls_preserve_complete_phpstan_diagnostics() {
    check_diagnostics(&["direct.php", "neighbors.php"]);
}

#[test]
fn the_original_interface_call_case_retains_both_complete_primary_diagnostics() {
    check_diagnostics(&["direct.php"]);
}

fn check_diagnostics(files: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/interface-static");
    let expected: Value = serde_json::from_str(ORACLE).unwrap();
    let mut failures = Vec::new();
    for level in [0, 5, 9] {
        let mut project = Project::new(files.iter().map(|name| root.join(name)).collect(), vec![]);
        project.level = level;
        project.php_version = 80500;
        project.check_this_only = Some(false);
        let mut actual: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                json!([
                    Path::new(&error.file)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    error.line,
                    error.identifier,
                    error.message,
                    error.tip,
                    error.can_be_ignored,
                ])
            })
            .collect();
        actual.sort_by_key(Value::to_string);
        let wanted: Vec<_> = expected["errors"][level.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| files.contains(&row[0].as_str().unwrap()))
            .cloned()
            .collect();
        if actual != wanted {
            failures.push(json!({"level": level, "actual": actual, "expected": wanted}));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn direct_interface_calls_keep_their_declared_results_after_the_primary_error() {
    check_types(
        "direct.php",
        include_bytes!("fixtures/interface-static/direct.php"),
    );
}

#[test]
fn class_string_concrete_and_unknown_methods_keep_their_distinct_results() {
    check_types(
        "neighbors.php",
        include_bytes!("fixtures/interface-static/neighbors.php"),
    );
}

fn check_types(name: &'static str, source: &'static [u8]) {
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let expected = oracle["types"][name].as_array().unwrap();
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(name.as_bytes()), Cow::Borrowed(source));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Settings::default().version);
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let registry = PluginRegistry::new();
    let mut baseline = None;
    for native in [false, true] {
        let mut result = AnalysisResult::new(SymbolReferences::new());
        let artifacts = Analyzer::new(
            &arena,
            &file,
            &names,
            &codebase,
            &registry,
            Settings {
                track_native_types: native,
                ..Settings::default()
            },
        )
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
        let mut values = Vec::new();
        for row in expected {
            let start = row["start"].as_u64().unwrap() as u32;
            let end = row["end"].as_u64().unwrap() as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(end));
            assert_eq!(
                &source[start as usize..end as usize],
                row["expression"].as_str().unwrap().as_bytes()
            );
            let expression = artifacts.get_expression_result(&span).unwrap();
            let ty = expression.get_type();
            let raw = expression.get_raw_type();
            values.push(json!({
                "start": start, "end": end, "expression": row["expression"],
                "type": if ty.is_error() { "*ERROR*".to_owned() } else { ty.get_id().to_string() },
                "error": ty.is_error(),
                "raw_type": if raw.is_error() { "*ERROR*".to_owned() } else { raw.get_id().to_string() },
                "raw_error": raw.is_error(),
            }));
        }
        assert_eq!(&values, expected, "{name}, value, native={native}");
        let state = (artifacts.expression_types, result.issues);
        if let Some(previous) = &baseline {
            assert_eq!(
                previous, &state,
                "native tracking must not change full analysis"
            );
        }
        baseline = Some(state);
    }
}
