//! Callable shape metadata must survive conversion without replacing narrowed or mutated values.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

#[test]
fn callable_array_descriptions_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callable-array-description");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/callable-array-description/expected.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for (case, expected) in oracle.as_object().unwrap() {
        let (file, level) = case.split_once(':').unwrap();
        if file == "native.php" {
            continue;
        }
        let level: u8 = level.parse().unwrap();
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = vec![root.join(file)];
        let mut direct = Project::new(vec![root.join(file)], vec![]);
        direct.level = level;
        let mut want: Vec<_> = expected
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                json!([
                    e["file"],
                    e["line"],
                    e["identifier"],
                    e["message"],
                    e.get("tip").unwrap_or(&Value::Null),
                    e["ignorable"]
                ])
            })
            .collect();
        want.sort_by_cached_key(Value::to_string);
        for (constructor, mut project) in [("new", direct), ("from_config", configured)] {
            project.php_version = 80500;
            let mut got: Vec<_> = run(&project)
                .unwrap()
                .into_iter()
                .map(|e| {
                    json!([
                        e.file.replace(root.to_str().unwrap(), "$ROOT"),
                        e.line,
                        e.identifier,
                        e.message,
                        e.tip,
                        e.can_be_ignored
                    ])
                })
                .collect();
            got.sort_by_cached_key(Value::to_string);
            println!(
                "{case}/{constructor}: {}",
                json!({"got": got, "want": want})
            );
            if got != want {
                failures.push(format!("{case}/{constructor}"));
            }
        }
    }
    assert!(failures.is_empty(), "mismatches: {}", failures.join(", "));
}

mod native {
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{cst::Expression, parser::parse_file};
    use serde_json::{Value, json};

    const SOURCE: &[u8] = include_bytes!("fixtures/callable-array-description/native.php");
    type Events = Vec<(u32, u32, Option<String>)>;
    struct Observe(Arc<Mutex<Events>>);
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "callable-description",
                "callable-description",
                "native isolation",
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
            e: &Expression<'_>,
            c: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            let native = c.get_native_expression_type(e).map(|t| {
                if t.is_array() {
                    let array = t.get_single_array().unwrap();
                    assert!(array.callable().is_none());
                    assert_eq!(
                        array.get_minimum_size(),
                        0,
                        "native must not regain the PHPDoc shape"
                    );
                    "array".to_owned()
                } else if let Some(value) = t.get_single_literal_int_value() {
                    value.to_string()
                } else {
                    assert!(t.is_mixed());
                    "mixed".to_owned()
                }
            });
            self.0
                .lock()
                .unwrap()
                .push((e.span().start.offset, e.span().end.offset, native));
            Ok(())
        }
    }

    #[test]
    fn callable_phpdoc_does_not_enter_raw_native_values_or_change_full_analysis() {
        let analyze = |native| {
            let arena = LocalArena::new();
            let file = File::ephemeral(Cow::Borrowed(b"native.php"), Cow::Borrowed(SOURCE));
            let program = parse_file(&arena, &file);
            assert!(!program.has_errors());
            let names = NameResolver::new(&arena).resolve(program);
            let settings = Settings {
                track_native_types: native,
                ..Settings::default()
            };
            let mut codebase = mago_prelude::Prelude::build().metadata;
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
            let events = Arc::new(Mutex::new(Vec::new()));
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(Observe(Arc::clone(&events)));
            let mut result = AnalysisResult::new(references);
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
            let reads = std::mem::take(&mut *events.lock().unwrap());
            (artifacts, reads, format!("{:?}", result.issues))
        };
        let (on, reads, issues) = analyze(true);
        let (off, disabled, off_issues) = analyze(false);
        assert_eq!(on.expression_types, off.expression_types);
        assert_eq!(on.symbol_references, off.symbol_references);
        assert_eq!(on.call_terminations, off.call_terminations);
        assert_eq!(on.if_true_assertions, off.if_true_assertions);
        assert_eq!(on.if_false_assertions, off.if_false_assertions);
        assert_eq!(on.inferred_return_types, off.inferred_return_types);
        assert_eq!(on.inferred_parameter_types, off.inferred_parameter_types);
        let writes = |a: &mago_analyzer::artifacts::AnalysisArtifacts| {
            a.variable_writes
                .iter()
                .map(|(span, writes)| ((span.start.offset, span.end.offset), format!("{writes:?}")))
                .collect::<BTreeMap<_, _>>()
        };
        assert_eq!(writes(&on), writes(&off));
        assert_eq!(issues, off_issues);
        assert_eq!(
            reads.iter().map(|r| (r.0, r.1)).collect::<Vec<_>>(),
            disabled.iter().map(|r| (r.0, r.1)).collect::<Vec<_>>()
        );
        assert!(disabled.iter().all(|r| r.2.is_none()));
        let mut counts = BTreeMap::new();
        for r in &reads {
            *counts.entry((r.0, r.1)).or_insert(0) += 1;
        }
        assert!(counts.values().all(|&count| count == 1));
        let oracle: Value = serde_json::from_str(include_str!(
            "fixtures/callable-array-description/expected.json"
        ))
        .unwrap();
        let source = std::str::from_utf8(SOURCE).unwrap();
        let mut observations = Vec::new();
        for expected in oracle["native.php:0"].as_array().unwrap() {
            let line = expected["line"].as_u64().unwrap() as usize;
            let text = source.lines().nth(line - 1).unwrap();
            let start = source
                .split_inclusive('\n')
                .take(line - 1)
                .map(str::len)
                .sum::<usize>()
                + text.find("$value").unwrap();
            let event = reads
                .iter()
                .find(|r| (r.0 as usize, r.1 as usize) == (start, start + 6))
                .unwrap();
            let want = expected["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .unwrap();
            if text.contains("// unavailable") {
                assert!(event.2.is_none());
            } else {
                assert_eq!(event.2.as_deref(), Some(want), "line {line}");
            }
            observations.push(json!({"line": line, "phpstan": want, "native": event.2}));
        }
        assert_eq!(observations.len(), 4);
        println!(
            "{}",
            json!({"observations": observations, "once_only_events": reads.len()})
        );
    }
}
