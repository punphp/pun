//! IssetCheck and coalescing use PHPStan's refined string-to-array-key semantics.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn refined_offset_keys_match_complete_phpstan_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refined-isset-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for case in ["bug-4671", "keys"] {
        for level in [0, 1, 4, 5, 9] {
            let mut project = Project::new(vec![root.join(format!("{case}.php"))], vec![]);
            project.level = level;
            project.php_version = 80500;
            project.options.set(
                pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
                level > 0,
            );
            let mut actual: Vec<_> = run(&project)
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
            let name = format!("{case}-{level}");
            if actual != expected[&name] {
                let want = expected[&name].as_array().unwrap();
                let got = actual.as_array().unwrap();
                failures.push(json!({
                    "case": name,
                    "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                    "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                }));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn coalesce_produces_expression_holders_without_replaying_indexes() {
    use mago_allocator::LocalArena;
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::{HasSpan, Span};
    use mago_syntax::{
        cst::{Call, Expression},
        parser::parse_file,
    };
    use std::{
        borrow::Cow,
        collections::BTreeMap,
        sync::{Arc, Mutex},
    };

    #[derive(Clone)]
    struct Observe {
        expected: Value,
        snapshots: Arc<Mutex<Vec<Value>>>,
        visits: Arc<Mutex<BTreeMap<Span, usize>>>,
        source: &'static str,
    }
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("offset-holders", "offset-holders", "live scope");
            &META
        }
    }
    impl ExpressionHook for Observe {
        fn is_observer(&self) -> bool {
            true
        }

        fn before_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            *self
                .visits
                .lock()
                .unwrap()
                .entry(expression.span())
                .or_default() += 1;
            // Prepared targets publish completed reads without running ordinary
            // after-expression hooks. The synthetic operator consumes that read.
            if let Expression::Binary(binary) = expression {
                let tail = &self.source[binary.lhs.span().end.offset as usize..];
                if tail
                    .lines()
                    .next()
                    .is_some_and(|line| line.contains("// input:"))
                {
                    let line = self.source[..binary.lhs.span().start.offset as usize]
                        .bytes()
                        .filter(|b| *b == b'\n')
                        .count()
                        + 1;
                    self.snapshots.lock().unwrap().push(json!({
                        "line": line,
                        "error": context.artifacts().get_expression_type(binary.lhs).unwrap().is_error(),
                    }));
                }
            }
            if let Expression::Call(Call::Function(_)) = expression {
                let at = expression.span().start.offset as usize;
                if self.source[at..].starts_with("observe()") {
                    let line = self.source[..at].bytes().filter(|b| *b == b'\n').count() + 1;
                    let expected = self
                        .expected
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|record| record["line"] == line)
                        .unwrap();
                    let block = context.block_context();
                    let reads: Vec<_> = expected["reads"].as_array().unwrap().iter().map(|read| {
                        let text = read["expression"].as_str().unwrap();
                        let local = block.locals.get(&mago_word::word(text));
                        let mut record = json!({"expression": text, "certainty": if local.is_some() { "Yes" } else { "No" }});
                        if read.get("value").is_some() {
                            record["value"] = json!(local.unwrap().get_single_literal_int_value().unwrap().to_string());
                        }
                        if read.get("error").is_some() {
                            record["error"] = json!(local.is_some_and(|value| value.is_error()));
                        }
                        record
                    }).collect();
                    self.snapshots
                        .lock()
                        .unwrap()
                        .push(json!({"line": line, "reads": reads}));
                }
            }
            Ok(ExpressionHookResult::Continue)
        }
    }

    for (source, expected, indexes) in [
        (
            include_str!("fixtures/refined-isset-keys/producer.php"),
            serde_json::from_str::<Value>(include_str!(
                "fixtures/refined-isset-keys/producer.json"
            ))
            .unwrap(),
            &["$i++", "$j++"][..],
        ),
        (
            include_str!("fixtures/refined-isset-keys/prepared.php"),
            serde_json::from_str::<Value>(include_str!(
                "fixtures/refined-isset-keys/prepared.json"
            ))
            .unwrap()["producer"]
                .clone(),
            &["$index++"][..],
        ),
        (
            include_str!("fixtures/refined-isset-keys/written.php"),
            serde_json::from_str::<Value>(include_str!("fixtures/refined-isset-keys/written.json"))
                .unwrap()["producer"]
                .clone(),
            &["$index++"][..],
        ),
    ] {
        let mut baseline = None;
        for native in [false, true] {
            let arena = LocalArena::new();
            let file = File::ephemeral(
                Cow::Borrowed(b"producer.php"),
                Cow::Borrowed(source.as_bytes()),
            );
            let parsed = parse_file(&arena, &file);
            assert!(!parsed.has_errors());
            let names = NameResolver::new(&arena).resolve(parsed);
            let settings = Settings {
                track_native_types: native,
                ..Settings::default()
            };
            let mut codebase = scan_program(&arena, &file, parsed, &names, settings.version);
            let mut references = SymbolReferences::new();
            populate_codebase(
                &mut codebase,
                &mut references,
                Default::default(),
                Default::default(),
            );
            let observer = Observe {
                expected: expected.clone(),
                snapshots: Default::default(),
                visits: Default::default(),
                source,
            };
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(observer.clone());
            let mut result = AnalysisResult::new(references);
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(parsed, &mut result)
                .unwrap();
            assert_eq!(
                json!(*observer.snapshots.lock().unwrap()),
                expected,
                "native {native}"
            );
            for index in indexes {
                let start = source.find(index).unwrap() as u32;
                let visits: usize = observer
                    .visits
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|(span, _)| {
                        span.start.offset == start && span.end.offset == start + index.len() as u32
                    })
                    .map(|(_, count)| count)
                    .sum();
                assert_eq!(visits, 1, "index {index}, native {native}");
            }
            if let Some((full, visits)) = &baseline {
                assert_eq!(
                    &artifacts.expression_types, full,
                    "native tracking changed full artifacts"
                );
                assert_eq!(
                    &*observer.visits.lock().unwrap(),
                    visits,
                    "native tracking changed expression events"
                );
            } else {
                baseline = Some((
                    artifacts.expression_types,
                    observer.visits.lock().unwrap().clone(),
                ));
            }
        }
    }
}

#[test]
fn sequential_offset_history_matches_complete_phpstan_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refined-isset-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("history.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for case in ["sequential", "history", "producer"] {
        for level in [0, 1, 4, 5, 9] {
            let mut project = Project::new(vec![root.join(format!("{case}.php"))], vec![]);
            project.level = level;
            project.php_version = 80500;
            project.options.set(
                pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
                level > 0,
            );
            let mut actual: Vec<_> = run(&project)
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
            let name = format!("{case}-{level}");
            if actual != expected[&name] {
                let want = expected[&name].as_array().unwrap();
                let got = actual.as_array().unwrap();
                failures.push(json!({
                    "case": name,
                    "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                    "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                }));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn array_keys_and_raw_string_offsets_remain_asymmetric() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refined-isset-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("asymmetric.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 4, 5, 9] {
        let mut project = Project::new(vec![root.join("asymmetric.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            level > 0,
        );
        let mut actual: Vec<_> = run(&project)
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
        let want = expected[level.to_string()].as_array().unwrap();
        let got = actual.as_array().unwrap();
        if got != want {
            failures.push(json!({
                "level": level,
                "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn prepared_offset_existence_is_independent_of_failed_read_values() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refined-isset-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("prepared.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 4, 5, 9] {
        let mut project = Project::new(vec![root.join("prepared.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            level > 0,
        );
        let mut actual: Vec<_> = run(&project)
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
        let want = expected["diagnostics"][level.to_string()]
            .as_array()
            .unwrap();
        let got = actual.as_array().unwrap();
        if got != want {
            failures.push(json!({
                "level": level,
                "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn written_offset_holders_do_not_require_array_receivers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refined-isset-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("written.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 4, 5, 9] {
        let mut project = Project::new(vec![root.join("written.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            level > 0,
        );
        let mut actual: Vec<_> = run(&project)
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
        let want = expected["diagnostics"][level.to_string()]
            .as_array()
            .unwrap();
        let got = actual.as_array().unwrap();
        if got != want {
            failures.push(json!({
                "level": level,
                "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn finite_array_keys_select_only_their_own_values() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/refined-isset-keys");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("finite.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for level in [0, 1, 4, 5, 9] {
        let mut project = Project::new(vec![root.join("finite.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            level > 0,
        );
        let mut actual: Vec<_> = run(&project)
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
        let want = expected[level.to_string()].as_array().unwrap();
        let got = actual.as_array().unwrap();
        if got != want {
            failures.push(json!({
                "level": level,
                "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
