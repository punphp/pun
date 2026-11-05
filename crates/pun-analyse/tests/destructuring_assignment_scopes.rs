//! Expression-assignment scope transfers use structural identities and real scope lifetimes.

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
fn destructuring_assignment_scopes_match_complete_phpstan_records() {
    let inputs: Value = serde_json::from_str(include_str!(
        "fixtures/destructuring-assignment-scopes/inputs.json"
    ))
    .unwrap();
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/destructuring-assignment-scopes/expected.json"
    ))
    .unwrap();
    let root = std::env::temp_dir().join(format!(
        "pun-destructuring-assignment-scopes-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).unwrap();
    let mut files = Vec::new();
    for (name, source) in inputs.as_object().unwrap() {
        let file = root.join(format!("{name}.php"));
        std::fs::write(&file, source.as_str().unwrap()).unwrap();
        files.push(file);
    }
    let mut failures = Vec::new();
    for (mode, level, maybe) in [
        ("level0", 0, false),
        ("default", 1, true),
        ("quiet", 1, false),
    ] {
        let mut project = Project::new(files.clone(), Vec::new());
        project.php_version = 80500;
        project.level = level;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            maybe,
        );
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
        actual.sort_by_cached_key(Value::to_string);
        let mut expected = oracle["diagnostics"][mode].as_array().unwrap().clone();
        expected.sort_by_cached_key(Value::to_string);
        println!(
            "destructuring-scope-records={}",
            json!({"mode": mode, "actual": actual, "oracle": expected, "general": result.general})
        );
        if actual != expected || !result.general.is_empty() {
            failures.push(mode);
        }
    }
    std::fs::remove_dir_all(root).unwrap();
    assert!(failures.is_empty(), "{failures:?}");
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

fn scalar(ty: &TUnion) -> String {
    if let Some(value) = ty.get_single_literal_int_value() {
        return value.to_string();
    }
    assert!(ty.is_int(), "unexpected observation: {ty:?}");
    "int".into()
}

impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "destructuring-assignment-scopes",
            "destructuring-assignment-scopes",
            "passive key and target observations",
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
        if self.marks.contains(&span.start.offset) && matches!(expression, Expression::Variable(_))
        {
            let selected = scalar(context.get_expression_type(expression).unwrap());
            let native = context.get_native_expression_type(expression).map(scalar);
            assert!(
                found
                    .reads
                    .insert(span.start.offset, (selected, native))
                    .is_none()
            );
        }
        Ok(())
    }
}

#[test]
fn destructuring_key_and_target_observations_are_once_only_and_native_independent() {
    let inputs: Value = serde_json::from_str(include_str!(
        "fixtures/destructuring-assignment-scopes/inputs.json"
    ))
    .unwrap();
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/destructuring-assignment-scopes/expected.json"
    ))
    .unwrap();
    let observations = oracle["observations"].as_array().unwrap();
    let source = inputs["observations"].as_str().unwrap();
    let file = File::ephemeral(
        b"observations.php".as_slice().into(),
        source.as_bytes().to_vec().into(),
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
            "destructuring-scope-observations={}",
            json!({"native":native,"actual":found.reads,"oracle":observations,"sequence":found.sequence})
        );
        assert_eq!(found.reads.len(), observations.len());
        assert_eq!(
            found.sequence.iter().collect::<BTreeSet<_>>().len(),
            found.sequence.len(),
            "expression visited more than once"
        );
        let mut order = Vec::new();
        for row in observations {
            let start = row["start"].as_u64().unwrap() as u32;
            let (selected, native_type) = &found.reads[&start];
            assert_eq!(selected, row["selected"].as_str().unwrap());
            if native {
                if let Some(value) = native_type {
                    assert_eq!(value, row["native"].as_str().unwrap());
                }
                if row["selected"] == "int" {
                    assert_eq!(native_type.as_deref(), Some("int"));
                }
            } else {
                assert!(native_type.is_none());
            }
            order.push(
                found
                    .sequence
                    .iter()
                    .position(|&(s, _)| s == start)
                    .unwrap(),
            );
        }
        assert!(
            order.windows(2).all(|p| p[0] < p[1]),
            "RHS and key order differs from PHPStan"
        );
        let target = (source.find("=> $a[]").unwrap() + 3) as u32;
        let target = found
            .sequence
            .iter()
            .position(|&(s, _)| s == target)
            .unwrap();
        assert!(
            order[1] < target && target < order[2],
            "target must follow its key"
        );
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
