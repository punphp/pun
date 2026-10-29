//! CalledMethodProcessor's public marker answers, captured from PHPStan 2.3.1.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::plugin::*;
use mago_analyzer::property_initialization::PropertyInitializationRequest;
use mago_analyzer::settings::Settings;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult};
use mago_codex::{populator::populate_codebase, scanner::scan_program};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{
    cst::{Expression, Statement},
    parser::parse_file,
};
use mago_word::word;
use serde_json::Value;

#[derive(Clone, Default)]
struct CalledMethodVisits(Arc<Mutex<BTreeMap<(Span, &'static str), usize>>>);
impl Provider for CalledMethodVisits {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "called-method-visits",
            "called-method-visits",
            "real visits",
        );
        &META
    }
    fn is_observer(&self) -> bool {
        true
    }
}
impl ExpressionHook for CalledMethodVisits {
    fn is_observer(&self) -> bool {
        true
    }
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        *self
            .0
            .lock()
            .unwrap()
            .entry((expression.span(), "expression"))
            .or_default() += 1;
        Ok(ExpressionHookResult::Continue)
    }
}
impl StatementHook for CalledMethodVisits {
    fn before_statement(
        &self,
        statement: &Statement<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<HookAction> {
        *self
            .0
            .lock()
            .unwrap()
            .entry((statement.span(), "statement"))
            .or_default() += 1;
        Ok(HookAction::Continue)
    }
}

fn check_called_methods(source: &[u8], capture: &str, scanned: Option<&[u8]>) {
    let records: Vec<Value> = serde_json::from_str(capture).unwrap();
    assert!(!records.is_empty());
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Owned(format!("{}.php", CalledMethodVisits::meta().id).into_bytes()),
        Cow::Owned(source.to_vec()),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata;
    let mut references = prelude.symbol_references;
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        Settings::default().version,
    ));
    let mut absent_requests = Vec::new();
    if let Some(scanned) = scanned {
        let stub = File::ephemeral(
            Cow::Borrowed(b"called-methods-scan-only.php"),
            Cow::Owned(scanned.to_vec()),
        );
        let parsed = parse_file(&arena, &stub);
        assert!(!parsed.has_errors());
        let resolved = NameResolver::new(&arena).resolve(parsed);
        codebase.extend(scan_program(
            &arena,
            &stub,
            parsed,
            &resolved,
            Settings::default().version,
        ));
        let operand = b"$this->value";
        absent_requests.extend(scanned.windows(operand.len()).enumerate().filter_map(
            |(start, value)| {
                (value == operand).then(|| PropertyInitializationRequest {
                    expression: Span::new(
                        stub.id,
                        Position::new(start as u32),
                        Position::new((start + operand.len()) as u32),
                    ),
                    property: word("value"),
                })
            },
        ));
    }
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let requests: Vec<_> = records
        .iter()
        .map(|record| {
            let start = record["start"].as_u64().unwrap() as u32;
            let end = record["end"].as_u64().unwrap() as u32;
            assert_eq!(
                &source[start as usize..end as usize],
                record["expression"].as_str().unwrap().as_bytes()
            );
            assert_eq!(
                file.line_number(start) + 1,
                record["line"].as_u64().unwrap() as u32
            );
            PropertyInitializationRequest {
                expression: Span::new(file.id, Position::new(start), Position::new(end)),
                property: word(record["property"].as_str().unwrap()),
            }
        })
        .collect();
    let mut baseline = None;
    for flow in [false, true] {
        for native in [false, true] {
            let mut native_baseline = None;
            for enabled in [false, true] {
                let visits = CalledMethodVisits::default();
                let mut registry = PluginRegistry::new();
                registry.register_expression_hook(visits.clone());
                registry.register_statement_hook(visits.clone());
                let mut observations = requests.clone();
                observations.extend(absent_requests.iter().copied());
                let mut result = AnalysisResult::new(references.clone());
                let artifacts = Analyzer::new(
                    &arena,
                    &file,
                    &names,
                    &codebase,
                    &registry,
                    Settings {
                        track_native_types: native,
                        track_variable_flow: flow,
                        property_initialization_observations: if enabled {
                            observations
                        } else {
                            Vec::new()
                        },
                        ..Settings::default()
                    },
                )
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
                let mut issues: Vec<_> = result
                    .issues
                    .iter()
                    .map(|issue| format!("{issue:?}"))
                    .collect();
                issues.sort();
                let invariant = (
                    artifacts.expression_types.clone(),
                    issues,
                    visits.0.lock().unwrap().clone(),
                );
                if let Some(before) = &baseline {
                    assert_eq!(
                        &invariant, before,
                        "helper gathering leaked types, diagnostics or observer visits"
                    );
                } else {
                    baseline = Some(invariant);
                }
                let native_answers: BTreeMap<_, _> = artifacts
                    .expression_types
                    .keys()
                    .map(|&(start, end)| {
                        let span = Span::new(file.id, Position::new(start), Position::new(end));
                        (
                            (start, end),
                            artifacts.get_native_expression_type(&span).cloned(),
                        )
                    })
                    .collect();
                if let Some(before) = &native_baseline {
                    assert_eq!(
                        &native_answers, before,
                        "helper gathering leaked native holder types"
                    );
                } else {
                    assert_eq!(native_answers.values().any(Option::is_some), native);
                    native_baseline = Some(native_answers);
                }
                if !enabled {
                    assert!(artifacts.property_initialization_observations.is_empty());
                    continue;
                }
                let got: Vec<_> = records
                    .iter()
                    .zip(&requests)
                    .map(|(record, request)| {
                        let mut record = record.clone();
                        record["certainty"] = artifacts
                            .property_initialization_observations
                            .get(&request.expression)
                            .and_then(|markers| markers.get(&request.property))
                            .map(|certainty| Value::String(format!("{certainty:?}")))
                            .unwrap_or(Value::Null);
                        record
                    })
                    .collect();
                assert_eq!(got, records, "complete ordered PHPStan records");
                assert_eq!(
                    artifacts.property_initialization_observations.len(),
                    requests.len()
                );
                for request in &absent_requests {
                    assert!(
                        !artifacts
                            .property_initialization_observations
                            .contains_key(&request.expression)
                    );
                }
            }
        }
    }
}

#[test]
fn called_method_eligibility_raw_returns_live_ends_and_size_limits_match_phpstan() {
    check_called_methods(
        include_bytes!("fixtures/property-initialization/called-methods.php"),
        include_str!("fixtures/property-initialization/called-methods-expected.json"),
        None,
    );
}

#[test]
fn called_methods_do_not_walk_scanned_only_declarations() {
    check_called_methods(
        include_bytes!("fixtures/property-initialization/called-methods-scan-controls.php"),
        include_str!("fixtures/property-initialization/called-methods-scan-controls-expected.json"),
        Some(include_bytes!(
            "fixtures/property-initialization/called-methods-scan-only.php"
        )),
    );
}

#[test]
fn called_method_recursive_constructor_discarded_returns_and_final_branches_match_phpstan() {
    check_called_methods(
        include_bytes!("fixtures/property-initialization/called-methods-end-controls.php"),
        include_str!("fixtures/property-initialization/called-methods-end-controls-expected.json"),
        None,
    );
}

#[test]
fn called_method_compound_reflections_match_phpstan() {
    check_called_methods(
        include_bytes!("fixtures/property-initialization-bridge/called-methods-compound.php"),
        include_str!(
            "fixtures/property-initialization-bridge/called-methods-compound-markers.json"
        ),
        None,
    );
}
