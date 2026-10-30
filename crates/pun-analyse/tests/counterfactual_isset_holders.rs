//! Non-true isset invalidates a holder, rather than asserting that it is absent.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
use mago_codex::{populator::populate_codebase, scanner::scan_program, ttype::TType};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{cst::Expression, parser::parse_file};
use pun_analyse::{Project, run};
use serde_json::{Value, json};

const ORACLE: &str = include_str!("fixtures/counterfactual-isset-holders/expected.json");
const SOURCES: [(&str, &str); 2] = [
    (
        "scopes.php",
        include_str!("fixtures/counterfactual-isset-holders/scopes.php"),
    ),
    (
        "transfers.php",
        include_str!("fixtures/counterfactual-isset-holders/transfers.php"),
    ),
];

#[test]
fn non_true_isset_matches_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/counterfactual-isset-holders");
    let expected: Value = serde_json::from_str(ORACLE).unwrap();
    let mut failures = Vec::new();
    for (file, _) in SOURCES {
        for level in [1, 4] {
            let mut project = Project::new(vec![root.join(file)], Vec::new());
            project.php_version = 80500;
            project.level = level;
            project.check_maybe_undefined_variables = true;
            let mut got: Vec<_> = run(&project)
                .unwrap()
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
            let mut want = expected["fixtures"][file]["diagnostics"][level.to_string()]
                .as_array()
                .unwrap()
                .clone();
            got.sort_by_cached_key(Value::to_string);
            want.sort_by_cached_key(Value::to_string);
            println!(
                "holder-diagnostics={}",
                json!({"file": file, "level": level, "actual": got, "oracle": want}),
            );
            if got != want {
                failures.push(format!(
                    "{file}, level {level}: {} actual / {} expected complete records\nextra: {:?}\nmissing: {:?}",
                    got.len(),
                    want.len(),
                    got.iter().filter(|e| !want.contains(e)).collect::<Vec<_>>(),
                    want.iter().filter(|e| !got.contains(e)).collect::<Vec<_>>(),
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

type Reads = BTreeMap<u32, (String, String, Option<String>)>;
#[derive(Default)]
struct Events {
    reads: Reads,
    sequence: Vec<(u32, u32)>,
}
struct Observe {
    marked: Vec<u32>,
    found: Arc<Mutex<Events>>,
}
impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "counterfactual-isset-holders",
            "counterfactual-isset-holders",
            "passive holder invalidation evidence",
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
        if !self.marked.contains(&span.start.offset) {
            return Ok(());
        }
        let Expression::Variable(mago_syntax::cst::Variable::Direct(variable)) = expression else {
            panic!("marked read is not a direct variable");
        };
        let describe = |ty: &mago_codex::ttype::union::TUnion| {
            if ty.is_error() {
                "*ERROR*".to_owned()
            } else {
                ty.get_id().to_string()
            }
        };
        let certainty = context
            .block_context()
            .variable_certainty(mago_word::word(variable.name));
        let certainty = if certainty.is_defined() {
            "defined"
        } else if certainty.is_absent() {
            "absent"
        } else {
            "possible"
        };
        let selected = describe(
            context
                .get_expression_type(expression)
                .expect("completed selected read"),
        );
        let native = context.get_native_expression_type(expression).map(describe);
        assert!(
            found
                .reads
                .insert(span.start.offset, (certainty.to_owned(), selected, native))
                .is_none(),
            "marked read at {} evaluated twice",
            span.start.offset,
        );
        Ok(())
    }
}

#[test]
fn non_true_isset_preserves_source_reads_and_selected_walk_with_native_off_and_on() {
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let prelude = mago_prelude::Prelude::build();
    let mut failures = Vec::new();
    for (name, source) in SOURCES {
        let observations = oracle["fixtures"][name]["observations"].as_array().unwrap();
        let marked: Vec<_> = observations
            .iter()
            .map(|e| e["start"].as_u64().unwrap() as u32)
            .collect();
        let file = File::ephemeral(
            name.as_bytes().to_vec().into(),
            source.as_bytes().to_vec().into(),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
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
        let normalize = |value: &str| {
            value
                .parse::<i64>()
                .map_or_else(|_| value.to_owned(), |n| format!("int({n})"))
        };
        let mut previous = None;
        for native in [false, true] {
            let found = Arc::new(Mutex::new(Events::default()));
            let mut registry = PluginRegistry::with_library_providers();
            registry.register_expression_hook(Observe {
                marked: marked.clone(),
                found: Arc::clone(&found),
            });
            let settings = Settings {
                track_native_types: native,
                ..Settings::default()
            };
            let mut result = AnalysisResult::new(references.clone());
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
            let wanted: Reads = observations
                .iter()
                .map(|e| {
                    (
                        e["start"].as_u64().unwrap() as u32,
                        (
                            e["certainty"].as_str().unwrap().to_owned(),
                            normalize(e["selected"].as_str().unwrap()),
                            (native && e["native_supported"] == true)
                                .then(|| normalize(e["native"].as_str().unwrap())),
                        ),
                    )
                })
                .collect();
            let found = std::mem::take(&mut *found.lock().unwrap());
            println!(
                "holder-reads={}",
                json!({"file": name, "native": native, "actual": found.reads, "oracle": wanted}),
            );
            if found.reads != wanted {
                failures.push(format!(
                    "{name}, native={native}\nactual: {:?}\noracle: {wanted:?}",
                    found.reads
                ));
            }
            let unchanged = (
                artifacts.expression_types,
                artifacts.symbol_references,
                format!("{:?}", result.issues),
                found.sequence,
            );
            if let Some(previous) = &previous {
                assert_eq!(
                    &unchanged, previous,
                    "{name}: native tracking changed the selected walk"
                );
            } else {
                previous = Some(unchanged);
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
