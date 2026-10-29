//! Live virtual property holders, independently captured from PHPStan 2.3.1.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::plugin::*;
use mago_analyzer::property_initialization::{
    PropertyInitializationCertainty, PropertyInitializationRequest,
};
use mago_analyzer::settings::Settings;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts};
use mago_codex::{populator::populate_codebase, scanner::scan_program};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{cst::Expression, parser::parse_file};
use mago_word::word;
use serde_json::Value;

#[derive(Clone, Default)]
struct Visits(Arc<Mutex<BTreeMap<(u32, u32), usize>>>);
impl Provider for Visits {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "initialization-visits",
            "initialization-visits",
            "live expression visits",
        );
        &META
    }
}
impl ExpressionHook for Visits {
    fn is_observer(&self) -> bool {
        true
    }
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        let span = expression.span();
        *self
            .0
            .lock()
            .unwrap()
            .entry((span.start.offset, span.end.offset))
            .or_default() += 1;
        Ok(ExpressionHookResult::Continue)
    }
}

fn check(source: &[u8], expected: &str) {
    let records: Vec<Value> = serde_json::from_str(expected).unwrap();
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"property-initialization.php"),
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
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut requests: Vec<_> = records
        .iter()
        .map(|record| PropertyInitializationRequest {
            expression: Span::new(
                file.id,
                Position::new(record["start"].as_u64().unwrap() as u32),
                Position::new(record["end"].as_u64().unwrap() as u32),
            ),
            property: word(record["property"].as_str().unwrap()),
        })
        .collect();
    let absent_operand = Span::new(file.id, Position::new(0), Position::new(0));
    requests.push(PropertyInitializationRequest {
        expression: absent_operand,
        property: word("value"),
    });
    for flow in [false, true] {
        let mut baseline = None;
        for native in [false, true] {
            let mut native_baseline = None;
            for enabled in [false, true] {
                let visits = Visits::default();
                let mut registry = PluginRegistry::new();
                registry.register_expression_hook(visits.clone());
                let settings = Settings {
                    track_native_types: native,
                    track_variable_flow: flow,
                    property_initialization_observations: if enabled {
                        requests.clone()
                    } else {
                        Vec::new()
                    },
                    ..Settings::default()
                };
                let mut result = AnalysisResult::new(references.clone());
                let artifacts =
                    Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
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
                if let Some(baseline) = &baseline {
                    assert_eq!(
                        &invariant, baseline,
                        "tracking/native must not change types, diagnostics or real visit counts"
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
                        "all partial native answers, including unavailable ones, must be unchanged"
                    );
                } else {
                    assert_eq!(native_answers.values().any(Option::is_some), native);
                    native_baseline = Some(native_answers);
                }
                if !enabled {
                    assert!(artifacts.property_initialization_observations.is_empty());
                    continue;
                }
                let mut differences = Vec::new();
                for (record, request) in records.iter().zip(&requests) {
                    let got = artifacts
                        .property_initialization_observations
                        .get(&request.expression)
                        .and_then(|markers| markers.get(&request.property));
                    // Deliberate activation boundary, not a PHPStan expectation: direct
                    // helper replay has its own 50-line contract. Never substitute Mago's
                    // transitive runtime initializer summary for that missing transfer.
                    if record["class"] == "DirectHelper" {
                        assert_eq!(got, Some(&PropertyInitializationCertainty::No));
                        continue;
                    }
                    if got.map(|certainty| format!("{certainty:?}"))
                        != record["certainty"].as_str().map(str::to_owned)
                    {
                        differences.push(format!(
                            "{}::{} line {} {}: {got:?}, want {}",
                            record["class"],
                            record["method"],
                            record["line"],
                            record["expression"],
                            record["certainty"]
                        ));
                    }
                }
                assert!(differences.is_empty(), "{}", differences.join("\n"));
                assert!(
                    !artifacts
                        .property_initialization_observations
                        .contains_key(&absent_operand)
                );
                // The public response is owned, Send + Sync, without a runtime/arena or worker dependency.
                fn owned_transport<T: Send + Sync + 'static>(value: T) -> T {
                    std::thread::spawn(move || value).join().unwrap()
                }
                assert_eq!(
                    owned_transport(artifacts.property_initialization_observations.clone()),
                    artifacts.property_initialization_observations
                );
                // Ordinary straight-line operands are observed exactly once; loop fixed points may
                // already visit other operands more than once, equally with tracking off and on.
                for record in records.iter().filter(|record| {
                    matches!(
                        record["class"].as_str(),
                        Some("EarlyReturn" | "AssignedReturn" | "OrderedMethods")
                    )
                }) {
                    let key = (
                        record["start"].as_u64().unwrap() as u32,
                        record["end"].as_u64().unwrap() as u32,
                    );
                    assert_eq!(visits.0.lock().unwrap().get(&key), Some(&1), "{record}");
                }
                let call = b"propertyInitializationValue()";
                if let Some(start) = source
                    .windows(call.len())
                    .rposition(|window| window == call)
                {
                    let span = (start as u32, (start + call.len()) as u32);
                    assert_eq!(
                        visits.0.lock().unwrap().get(&span),
                        Some(&1),
                        "constructor call must not be replayed for certainty"
                    );
                }
                check_legacy_control(&artifacts, &records);
            }
        }
    }
}

fn check_legacy_control(artifacts: &AnalysisArtifacts, records: &[Value]) {
    if records
        .iter()
        .any(|record| record["class"] == "EarlyReturn")
    {
        // This existing readonly/runtime summary is NOT the virtual marker: it includes the
        // later assignment even though an earlier constructor path returned uninitialized.
        assert!(
            artifacts.method_initialized_properties[&(word("earlyreturn"), word("__construct"))]
                .contains(&word("$value"))
        );
    }
}

#[test]
fn constructor_exits_promotions_serialization_inheritance_and_virtual_values_match_phpstan() {
    check(
        include_bytes!("fixtures/property-initialization/property-initialization-original.php"),
        include_str!(
            "fixtures/property-initialization/property-initialization-original-expected.json"
        ),
    );
}

#[test]
fn abrupt_loop_finally_hook_and_creation_edges_match_phpstan() {
    check(
        include_bytes!("fixtures/property-initialization/property-initialization-edges.php"),
        include_str!(
            "fixtures/property-initialization/property-initialization-edges-expected.json"
        ),
    );
}

#[test]
fn raw_exit_controls_nested_joins_and_single_evaluation_match_phpstan() {
    check(
        include_bytes!("fixtures/property-initialization/property-initialization-controls.php"),
        include_str!(
            "fixtures/property-initialization/property-initialization-controls-expected.json"
        ),
    );
}

#[test]
fn live_branch_loop_try_alias_nested_and_ordered_method_scopes_match_phpstan() {
    check(
        include_bytes!("fixtures/property-initialization/property-initialization-flow.php"),
        include_str!("fixtures/property-initialization/property-initialization-flow-expected.json"),
    );
}
