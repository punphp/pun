//! Dynamic callable references retain level-zero argument scopes and once-only source reads.

use pun_analyse::{Project, analyse_project};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
use mago_codex::{populator::populate_codebase, scanner::scan_program, ttype::union::TUnion};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{cst::Expression, parser::parse_file};

#[test]
fn dynamic_callable_references_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dynamic-callable-references");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/dynamic-callable-references/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for (mode, level, maybe) in [
        ("level0", 0, false),
        ("default", 1, true),
        ("quiet", 1, false),
    ] {
        let mut project = Project::new(
            vec![root.join("calls.php"), root.join("observations.php")],
            Vec::new(),
        );
        project.php_version = 80500;
        project.level = level;
        project.options.set(pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, maybe);
        let result = analyse_project(&project).unwrap();
        let mut actual: Vec<_> = result
            .errors
            .into_iter()
            .map(|e| {
                json!([
                    Path::new(&e.file).file_name().unwrap().to_str().unwrap(),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored,
                ])
            })
            .collect();
        let mut expected = oracle["diagnostics"][mode].as_array().unwrap().clone();
        actual.sort_by_cached_key(Value::to_string);
        expected.sort_by_cached_key(Value::to_string);
        println!(
            "dynamic-callable-records={}",
            json!({"mode":mode,"actual":actual,"oracle":expected,"general":result.general})
        );
        if actual != expected || !result.general.is_empty() {
            failures.push(mode);
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

// These source observations are independent of Pun's PHPStan conversion and the argument-rule consumer.
fn scalar(ty: &TUnion) -> String {
    assert!(!ty.is_error(), "an invalid read must not become mixed");
    if let Some(value) = ty.get_single_literal_int_value() {
        return value.to_string();
    }
    if let Some(value) = ty.get_single_literal_string_value() {
        return format!("'{}'", std::str::from_utf8(value).unwrap());
    }
    assert!(ty.is_mixed(), "unexpected scalar observation: {ty:?}");
    "mixed".to_owned()
}

#[derive(Default)]
struct Events {
    reads: BTreeMap<u32, (String, Option<String>)>,
    sequence: Vec<(u32, u32)>,
}

struct Observe {
    marks: BTreeSet<u32>,
    found: Arc<Mutex<Events>>,
}

impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "dynamic-callable-references",
            "dynamic-callable-references",
            "passive dynamic callable argument reads",
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
        let mut found = self.found.lock().unwrap();
        found.sequence.push((span.start.offset, span.end.offset));
        if self.marks.contains(&span.start.offset) {
            let selected = scalar(context.get_expression_type(expression).unwrap());
            let native = context.get_native_expression_type(expression).map(scalar);
            assert!(
                found
                    .reads
                    .insert(span.start.offset, (selected, native))
                    .is_none(),
                "argument evaluated more than once"
            );
        }
        Ok(())
    }
}

#[test]
fn dynamic_callable_source_reads_are_once_only_and_native_independent() {
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/dynamic-callable-references/expected.json"
    ))
    .unwrap();
    let observations = oracle["observations"].as_array().unwrap();
    let file = File::ephemeral(
        b"observations.php".as_slice().into(),
        include_bytes!("fixtures/dynamic-callable-references/observations.php")
            .to_vec()
            .into(),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata.clone();
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        Settings::default().version,
    ));
    let mut references = prelude.symbol_references.clone();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut previous = None;
    for native in [false, true] {
        let found = Arc::new(Mutex::new(Events::default()));
        let mut registry = PluginRegistry::with_library_providers();
        registry.register_expression_hook(Observe {
            marks: observations
                .iter()
                .map(|row| row["start"].as_u64().unwrap() as u32)
                .collect(),
            found: Arc::clone(&found),
        });
        let mut result = AnalysisResult::new(references.clone());
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
        let found = std::mem::take(&mut *found.lock().unwrap());
        println!(
            "dynamic-callable-observations={}",
            json!({"native":native,"actual":found.reads,"oracle":observations})
        );
        assert_eq!(found.reads.len(), observations.len());
        assert_eq!(
            found.sequence.iter().collect::<BTreeSet<_>>().len(),
            found.sequence.len(),
            "expression evaluated more than once"
        );
        for row in observations {
            let (selected, native_type) = &found.reads[&(row["start"].as_u64().unwrap() as u32)];
            assert_eq!(selected, row["selected"].as_str().unwrap());
            if !native {
                assert!(native_type.is_none());
            } else if let Some(native_type) = native_type {
                // The native lane is partial; a closure assignment currently invalidates it.
                assert_eq!(native_type, row["native"].as_str().unwrap());
            }
        }
        let unchanged = (
            artifacts.expression_types,
            artifacts.symbol_references,
            format!("{:?}", result.issues),
            found.sequence,
        );
        if let Some(previous) = &previous {
            assert_eq!(&unchanged, previous);
        } else {
            previous = Some(unchanged);
        }
    }
}
