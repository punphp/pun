//! Repeated expression specifications from real PHPUnit assertion metadata, through public Mago hooks.

use std::borrow::Cow;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program, ttype::TType,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{cst::Expression, parser::parse_file};
use serde_json::{Value, json};

const INPUT: &str = include_str!("fixtures/repeated-assertion-expressions/input.php");

struct Observe(Arc<Mutex<Vec<Value>>>);
impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "repeated-assertion-expressions",
            "repeated-assertion-expressions",
            "original full and native observations",
        );
        &META
    }
}
impl ExpressionHook for Observe {
    fn is_observer(&self) -> bool {
        true
    }

    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expression.span();
        if !INPUT.as_bytes()[span.end.offset as usize..].starts_with(b"; // observe") {
            return Ok(());
        }
        self.0.lock().unwrap().push(json!({
            "line": INPUT[..span.start.offset as usize].bytes().filter(|byte| *byte == b'\n').count() + 1,
            "start": span.start.offset,
            "end": span.end.offset,
            "expression": &INPUT[span.start.offset as usize..span.end.offset as usize],
            "full": context.get_expression_type(expression).map(|ty| ty.get_id().to_string()),
            "observed_native": context.get_native_expression_type(expression).map(|ty| ty.get_id().to_string()),
        }));
        Ok(())
    }
}

fn phpunit_sources(path: &Path, out: &mut Vec<(String, String)>) {
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            phpunit_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "php") {
            out.push((
                path.to_string_lossy().into_owned(),
                std::fs::read_to_string(&path).unwrap(),
            ));
        }
    }
}

fn analyze(dead: bool, native: bool) -> Vec<Value> {
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"input.php"), Cow::Borrowed(INPUT.as_bytes()));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings {
        version: mago_php_version::PHPVersion::new(8, 4, 0),
        analyze_dead_code: dead,
        track_native_types: native,
        ..Settings::default()
    };
    let mut codebase = mago_prelude::Prelude::build().metadata;
    let mut declarations = Vec::new();
    phpunit_sources(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../pun-lint/tests/phpunit-stubs"),
        &mut declarations,
    );
    for (path, source) in &declarations {
        let declaration = File::ephemeral(
            Cow::Owned(path.as_bytes().to_vec()),
            Cow::Owned(source.as_bytes().to_vec()),
        );
        let syntax = parse_file(&arena, &declaration);
        let resolved = NameResolver::new(&arena).resolve(syntax);
        codebase.extend(scan_program(
            &arena,
            &declaration,
            syntax,
            &resolved,
            settings.version,
        ));
    }
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        settings.version,
    ));
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let observations = Arc::new(Mutex::new(Vec::new()));
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observe(Arc::clone(&observations)));
    let mut result = AnalysisResult::new(references);
    Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    std::mem::take(&mut observations.lock().unwrap())
}

fn oracle_records(capture: &str) -> Vec<Value> {
    capture
        .lines()
        .map(|line| {
            let mut record: Value = serde_json::from_str(line).unwrap();
            if record["full"] == "*NEVER*" {
                record["full"] = json!("never");
            }
            // Mago's partial native lane does not publish these method-call observations.
            // This is an availability boundary, not permission to select the native type.
            record.as_object_mut().unwrap().remove("native");
            record["observed_native"] = Value::Null;
            record
        })
        .collect()
}

#[test]
fn complete_rector_scopes_with_dead_tails() {
    let expected = oracle_records(include_str!(
        "fixtures/repeated-assertion-expressions/rector-types.jsonl"
    ));
    assert_eq!(
        expected.len(),
        INPUT
            .lines()
            .filter(|line| line.ends_with("// observe"))
            .count()
    );
    for native in [false, true] {
        let actual = analyze(true, native);
        println!(
            "repeated-assertion-complete={}",
            json!({"native_enabled":native,"actual":actual,"rector":expected})
        );
        assert_eq!(actual, expected);
    }
}

#[test]
fn default_termination_and_native_availability_are_independent() {
    let expected = oracle_records(include_str!(
        "fixtures/repeated-assertion-expressions/phpstan-2.3.1-types.jsonl"
    ));
    let without_native = analyze(false, false);
    let with_native = analyze(false, true);
    let dead = analyze(true, false);
    assert_eq!(
        without_native.len(),
        dead.len() - 4,
        "only the fail and return tails are omitted"
    );
    assert_eq!(without_native, expected);
    assert_eq!(with_native, expected);
    let offsets: Vec<_> = with_native
        .iter()
        .map(|record| record["start"].as_u64().unwrap())
        .collect();
    let mut unique = offsets.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(
        offsets.len(),
        unique.len(),
        "observers visit each original expression once"
    );
}
