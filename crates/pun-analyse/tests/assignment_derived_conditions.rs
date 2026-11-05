//! Pure assignment RHS types inherit PHPStan's existing conditional holders.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn derived_assignment_conditions_match_complete_phpstan_records() {
    check(&[
        "arrays", "guards", "calls", "limit", "named", "effects", "retained",
    ]);
}

#[test]
fn implicit_string_calls_preserve_phpstan_impurity() {
    check(&["strings"]);
}

fn check(cases: &[&str]) {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/assignment-derived-conditions");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for case in cases {
        for level in [0, 4, 5] {
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
                failures.push(serde_json::json!({
                    "case": name,
                    "expected": want.len(),
                    "actual": got.len(),
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
fn retained_assignment_shapes_do_not_change_live_or_native_artifacts() {
    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, observations::CallScopeRequest, plugin::*,
        settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::{HasSpan, Position, Span};
    use mago_syntax::{cst::Expression, parser::parse_file};
    use std::{
        borrow::Cow,
        collections::BTreeMap,
        sync::{Arc, Mutex},
    };

    #[derive(Clone, Default)]
    struct Visits(Arc<Mutex<BTreeMap<Span, usize>>>);
    impl Provider for Visits {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("assignment-visits", "assignment-visits", "live expressions");
            &META
        }
    }
    impl ExpressionHook for Visits {
        fn is_observer(&self) -> bool {
            true
        }
        fn after_expression(
            &self,
            expression: &Expression<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            *self.0.lock().unwrap().entry(expression.span()).or_default() += 1;
            Ok(())
        }
    }

    let code = include_bytes!("fixtures/assignment-derived-conditions/retained.php");
    let text = std::str::from_utf8(code).unwrap();
    let mut baseline = None;
    for native in [false, true] {
        for observe in [false, true] {
            let arena = LocalArena::new();
            let file = File::ephemeral(
                Cow::Borrowed(b"retained.php"),
                Cow::Borrowed(code.as_slice()),
            );
            let program = parse_file(&arena, &file);
            assert!(!program.has_errors());
            let names = NameResolver::new(&arena).resolve(program);
            let span = |start: usize, length: usize| {
                Span::new(
                    file.id,
                    Position::new(start as u32),
                    Position::new((start + length) as u32),
                )
            };
            let call_start = text.find("$object->$method(").unwrap();
            let call = span(call_start, "$object->$method(...$arguments)".len());
            let argument = span(
                call_start + "$object->$method(...".len(),
                "$arguments".len(),
            );
            let key = span(text.find("[$key =>").unwrap() + 1, "$key".len());
            let mut settings = Settings {
                track_native_types: native,
                ..Settings::default()
            };
            if observe {
                settings.call_scope_observations.push(CallScopeRequest {
                    call,
                    expressions: vec![argument],
                });
            }
            let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
            let mut references = SymbolReferences::new();
            populate_codebase(
                &mut codebase,
                &mut references,
                Default::default(),
                Default::default(),
            );
            let mut result = AnalysisResult::new(references);
            let visits = Visits::default();
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(visits.clone());
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
            assert_eq!(
                visits.0.lock().unwrap()[&key],
                1,
                "pure RHS pricing published another live visit"
            );
            if observe {
                let scopes = &artifacts.call_scope_observations[&call];
                assert_eq!(scopes.len(), 2);
                for (name, key) in [("first", "left"), ("second", "right")] {
                    let scope = scopes
                        .iter()
                        .find(|scope| scope.name.as_str() == Some(name))
                        .unwrap();
                    assert!(scope.filtered);
                    // PHPStan's complete retained.php dumps specify these exact closed arrays.
                    use mago_codex::ttype::atomic::{
                        TAtomic,
                        array::{TArray, key::ArrayKey},
                    };
                    let TAtomic::Array(TArray::Keyed(array)) = scope.types[&argument].get_single()
                    else {
                        panic!("{name}: expected a single closed keyed array");
                    };
                    assert!(array.parameters.is_none());
                    let items = array.known_items.as_ref().unwrap();
                    assert_eq!(items.len(), 1);
                    let (optional, value) = &items[&ArrayKey::String(mago_word::word(key))];
                    assert!(!optional);
                    assert_eq!(value.get_single_literal_int_value(), Some(1));
                }
            }
            if let Some(full) = &baseline {
                assert_eq!(
                    &artifacts.expression_types, full,
                    "native {native}, observe {observe}"
                );
            } else {
                baseline = Some(artifacts.expression_types.clone());
            }
        }
    }
}
