//! Physical operands are shared; semantic return resolution remains per callee.
#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "the helpers print the failing case before the test asserts"
)]
use std::{
    borrow::Cow,
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, code::IssueCode, observations::CallScopeRequest,
    plugin::*, settings::Settings,
};
use mago_codex::{
    misc::GenericParent,
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            array::{TArray, key::ArrayKey, list::TList},
            scalar::{TScalar, int::TInteger, string::TStringLiteral},
        },
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_reporting::{Annotation, Issue};
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{cst::Expression, parser::parse_file};
use serde_json::Value;

const SOURCE: &[u8] = include_bytes!("fixtures/call-scope-observations/single-call-arguments.php");
const ORACLE: &str =
    include_str!("fixtures/call-scope-observations/single-call-argument-events.json");
const SELECTED: &str =
    include_str!("fixtures/call-scope-observations/single-call-arguments-expected.json");

#[derive(Clone, Default)]
struct Visits(Arc<Mutex<BTreeMap<(u32, u32), usize>>>);

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

fn describe(ty: &TUnion) -> String {
    let mut types: Vec<_> = ty
        .types
        .iter()
        .map(|atomic| match atomic {
            TAtomic::Mixed(mixed) if mixed.is_error() => "*ERROR*".to_owned(),
            TAtomic::Scalar(TScalar::ClassLikeString(
                mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString::Literal {
                    value,
                },
            )) => format!("'{value}'"),
            TAtomic::Scalar(TScalar::Integer(TInteger::Literal(value))) => value.to_string(),
            TAtomic::Scalar(TScalar::String(string))
                if matches!(string.literal, Some(TStringLiteral::Value(_))) =>
            {
                let Some(TStringLiteral::Value(value)) = string.literal else {
                    unreachable!()
                };
                format!("'{value}'")
            }
            TAtomic::GenericParameter(parameter) => match parameter.defining_entity {
                GenericParent::FunctionLike((class, method)) => format!(
                    "{} (method {}::{}(), parameter)",
                    parameter.parameter_name, class, method
                ),
                _ => atomic.get_id().to_string(),
            },
            TAtomic::Object(mago_codex::ttype::atomic::object::TObject::Named(named))
                if named.is_static && !named.is_this =>
            {
                format!("static({})", named.name)
            }
            TAtomic::Array(TArray::List(list)) if list.known_elements.is_some() => {
                format!(
                    "array{{{}}}",
                    list.known_elements
                        .as_ref()
                        .unwrap()
                        .values()
                        .map(|(_, ty)| describe(ty))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TAtomic::Array(TArray::Keyed(array)) if array.known_items.is_some() => {
                format!(
                    "array{{{}}}",
                    array
                        .known_items
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(key, (_, ty))| match key {
                            ArrayKey::Integer(_) => describe(ty),
                            ArrayKey::String(key) => format!("{}: {}", key, describe(ty)),
                            _ => panic!("unexpected fixture array key"),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            _ => atomic.get_id().to_string(),
        })
        .collect();
    types.sort_by_cached_key(|ty| ty.to_ascii_lowercase());
    types.join("|")
}

// Mago stores case-folded declaring-class identities; only template scope names
// need this display normalization. Literal string values remain case-sensitive.
fn oracle_type(ty: &str) -> String {
    let mut ty = ty.to_string();
    let mut offset = 0;
    while let Some(start) = ty[offset..].find(" (method ") {
        let start = offset + start + " (method ".len();
        let end = start + ty[start..].find("::").unwrap();
        ty.replace_range(start..end, &ty[start..end].to_ascii_lowercase());
        offset = end;
    }
    ty
}

fn analyze(
    native: bool,
    observe: bool,
    source: &'static [u8],
    selected: &str,
    registry: PluginRegistry,
) -> (
    Arc<File>,
    mago_analyzer::artifacts::AnalysisArtifacts,
    AnalysisResult,
    Visits,
) {
    let (file, artifacts, result, visits) =
        analyze_result(native, observe, source, selected, registry, false);
    (file, artifacts.unwrap(), result, visits)
}

fn analyze_result(
    native: bool,
    observe: bool,
    source: &'static [u8],
    selected: &str,
    mut registry: PluginRegistry,
    external: bool,
) -> (
    Arc<File>,
    Result<mago_analyzer::artifacts::AnalysisArtifacts, mago_analyzer::error::AnalysisError>,
    AnalysisResult,
    Visits,
) {
    let arena = LocalArena::new();
    let file = Arc::new(File::ephemeral(
        Cow::Borrowed(b"single-call-arguments.php"),
        Cow::Borrowed(source),
    ));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut settings = Settings {
        track_native_types: native,
        ..Settings::default()
    };
    if observe {
        let selected: Value = serde_json::from_str(selected).unwrap();
        for (position, call) in selected["calls"].as_object().unwrap() {
            let start = position.parse::<u32>().unwrap();
            settings.call_scope_observations.push(CallScopeRequest {
                call: Span::new(
                    file.id,
                    Position::new(start),
                    Position::new(start + call["expression"].as_str().unwrap().len() as u32),
                ),
                expressions: call["names"][0]["types"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|expression| {
                        Span::new(
                            file.id,
                            Position::new(expression["start"].as_u64().unwrap() as u32),
                            Position::new(expression["end"].as_u64().unwrap() as u32),
                        )
                    })
                    .collect(),
            });
        }
    }
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let visits = Visits::default();
    registry.register_expression_hook(visits.clone());
    let mut result = AnalysisResult::new(references);
    let session = mago_analyzer::external::ExternalAnalysisSession::from_files([Arc::clone(&file)]);
    let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
    let analyzer = if external {
        analyzer.with_external_analysis_session(&session)
    } else {
        analyzer
    };
    let artifacts = analyzer.analyze_with_artifacts(program, &mut result);
    (file, artifacts, result, visits)
}

#[test]
fn bound_this_unions_keep_lexical_classes_nested_closures_and_native_holders() {
    const SOURCE: &[u8] =
        include_bytes!("fixtures/call-scope-observations/single-call-bound-this.php");
    const SELECTED: &str =
        include_str!("fixtures/call-scope-observations/single-call-bound-this-expected.json");
    let expected: Value = serde_json::from_str(SELECTED).unwrap();
    for native in [false, true] {
        let (file, plain, plain_issues, plain_visits) =
            analyze(native, false, SOURCE, SELECTED, PluginRegistry::new());
        let (_, observed, observed_issues, observed_visits) =
            analyze(native, true, SOURCE, SELECTED, PluginRegistry::new());
        assert_eq!(plain_issues.issues, observed_issues.issues);
        assert_eq!(
            *plain_visits.0.lock().unwrap(),
            *observed_visits.0.lock().unwrap()
        );
        assert_eq!(plain.expression_types, observed.expression_types);
        let mut object_native_holders = 0;
        for event in expected["events"].as_object().unwrap().values() {
            let start = event["start"].as_u64().unwrap() as u32;
            let end = event["end"].as_u64().unwrap() as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(end));
            let visits = plain_visits.0.lock().unwrap();
            assert_eq!(
                visits.get(&(start, end)).copied().unwrap_or(0) as u64,
                event["visits"].as_u64().unwrap(),
                "{event}"
            );
            assert_eq!(
                plain.get_expression_type(&span).map(describe).as_deref(),
                event["full"].as_str(),
                "{event}"
            );
            let native_type = plain.get_native_expression_type(&span).map(describe);
            assert_eq!(
                native_type,
                observed.get_native_expression_type(&span).map(describe)
            );
            if native_type.is_some() {
                assert!(native);
                assert_eq!(native_type.as_deref(), event["native"].as_str(), "{event}");
                if native_type.as_deref() == Some("object") {
                    object_native_holders += 1;
                }
            }
            assert_eq!(
                plain.get_raw_expression_type(&span),
                observed.get_raw_expression_type(&span)
            );
            eprintln!(
                "bound-this native={native} {start}..{end} {} full={:?} native={native_type:?}",
                event["expression"],
                plain.get_expression_type(&span).map(describe)
            );
        }
        if native {
            assert!(
                object_native_holders >= 2,
                "both closure and arrow must retain the PHPDoc binding's native object holder"
            );
        }
        for (position, call) in expected["calls"].as_object().unwrap() {
            let start = position.parse::<u32>().unwrap();
            let span = Span::new(
                file.id,
                Position::new(start),
                Position::new(start + call["expression"].as_str().unwrap().len() as u32),
            );
            let answer = &observed.call_scope_observations[&span];
            assert_eq!(answer.len(), 1, "{call}");
            for expression in call["names"][0]["types"].as_array().unwrap() {
                let span = Span::new(
                    file.id,
                    Position::new(expression["start"].as_u64().unwrap() as u32),
                    Position::new(expression["end"].as_u64().unwrap() as u32),
                );
                assert_eq!(
                    answer[0].types.get(&span).map(describe).as_deref(),
                    expression["type"].as_str(),
                    "{call}"
                );
            }
        }
    }
}

#[test]
fn asymmetric_targets_share_physical_values_and_events_in_all_lanes() {
    assert_case(SOURCE, ORACLE, SELECTED, PluginRegistry::new);
}

#[test]
fn closure_contexts_are_combined_before_the_only_physical_body_walk() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-closures.php"),
        include_str!("fixtures/call-scope-observations/single-call-closure-events.json"),
        include_str!("fixtures/call-scope-observations/single-call-closures-expected.json"),
        PluginRegistry::new,
    );
}

#[test]
fn output_references_follow_the_one_physical_walk_reflection() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-references.php"),
        include_str!("fixtures/call-scope-observations/single-call-reference-events.json"),
        include_str!("fixtures/call-scope-observations/single-call-references-expected.json"),
        PluginRegistry::new,
    );
}

#[test]
fn combined_parameter_contracts_preserve_names_tails_out_and_closure_this() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-combined.php"),
        include_str!("fixtures/call-scope-observations/single-call-combined-events.json"),
        include_str!("fixtures/call-scope-observations/single-call-combined-expected.json"),
        PluginRegistry::new,
    );
}

#[test]
fn a_later_variadic_target_folds_the_earlier_targets_fixed_tail() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-reverse-tail.php"),
        include_str!("fixtures/call-scope-observations/single-call-reverse-tail-events.json"),
        include_str!("fixtures/call-scope-observations/single-call-reverse-tail-expected.json"),
        PluginRegistry::new,
    );
}

#[test]
fn distinct_template_names_do_not_discard_another_targets_inferred_bounds() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-distinct-templates.php"),
        include_str!("fixtures/call-scope-observations/single-call-distinct-templates-events.json"),
        include_str!(
            "fixtures/call-scope-observations/single-call-distinct-templates-expected.json"
        ),
        PluginRegistry::new,
    );
}

#[test]
fn debugging_targets_reuse_the_prepass_instead_of_repeating_its_operands() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-debug-targets.php"),
        include_str!("fixtures/call-scope-observations/single-call-debug-targets-events.json"),
        r#"{"calls":{}}"#,
        PluginRegistry::new,
    );
}

#[derive(Clone, Default)]
struct CallFlow(Arc<Mutex<BTreeMap<(u32, u32), bool>>>);

impl ExpressionHook for CallFlow {
    fn is_observer(&self) -> bool {
        true
    }

    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(expression, Expression::Call(_)) {
            let span = expression.span();
            self.0.lock().unwrap().insert(
                (span.start.offset, span.end.offset),
                context.block_context().flags.has_returned(),
            );
        }
        Ok(())
    }
}

#[test]
fn invalid_dynamic_calls_are_errors_without_making_implicit_never_terminate() {
    const SOURCE: &[u8] =
        include_bytes!("fixtures/call-scope-observations/single-call-invalid-dynamic.php");
    const ORACLE: &str =
        include_str!("fixtures/call-scope-observations/single-call-invalid-dynamic-events.json");
    assert_case(SOURCE, ORACLE, r#"{"calls":{}}"#, PluginRegistry::new);
    let expected: Value = serde_json::from_str(ORACLE).unwrap();
    for native in [false, true] {
        for observe in [false, true] {
            let flow = CallFlow::default();
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(flow.clone());
            let (file, artifacts, result, _) =
                analyze(native, observe, SOURCE, r#"{"calls":{}}"#, registry);
            let invalid_start = SOURCE
                .windows(b"$invalid(++$i)".len())
                .position(|bytes| bytes == b"$invalid(++$i)")
                .unwrap() as u32;
            let invalid: Vec<_> = result
                .issues
                .iter()
                .filter(|issue| issue.code.as_deref() == Some(IssueCode::InvalidCallable.as_str()))
                .map(|issue| {
                    issue
                        .annotations
                        .iter()
                        .find(|annotation| annotation.kind.is_primary())
                        .unwrap()
                        .span
                        .start
                        .offset
                })
                .collect();
            assert_eq!(invalid, vec![invalid_start]);
            for (start, record) in
                expected.as_object().unwrap().iter().filter(|(_, record)| {
                    record["type"] == "*ERROR*" && record["expression"] != "$i"
                })
            {
                let start = start.parse().unwrap();
                let end = record["end"].as_u64().unwrap() as u32;
                let span = Span::new(file.id, Position::new(start), Position::new(end));
                assert!(
                    artifacts.get_expression_type(&span).unwrap().is_error(),
                    "{record}"
                );
                assert!(
                    artifacts.get_raw_expression_type(&span).unwrap().is_error(),
                    "{record}"
                );
                assert!(
                    !artifacts
                        .call_terminations
                        .get(&(start, end))
                        .copied()
                        .unwrap_or(false)
                );
                assert_eq!(
                    flow.0.lock().unwrap()[&(start, end)],
                    record["expression"] == "stop()()"
                );
                assert!(artifacts.get_native_expression_type(&span).is_none());
            }
        }
    }
}

#[test]
fn dynamic_name_returns_select_on_before_scope_not_the_post_operand_scope() {
    assert_case(
        include_bytes!("fixtures/call-scope-observations/single-call-before-scope.php"),
        include_str!("fixtures/call-scope-observations/single-call-before-scope-events.json"),
        include_str!("fixtures/call-scope-observations/single-call-before-scope-expected.json"),
        PluginRegistry::new,
    );
}

struct AsymmetricReturns;
impl Provider for AsymmetricReturns {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "single-call-returns",
            "single-call-returns",
            "oracle return extension",
        );
        &META
    }
}
impl MethodReturnTypeProvider for AsymmetricReturns {
    fn targets() -> &'static [MethodTarget] {
        const TARGETS: &[MethodTarget] = &[
            MethodTarget::exact(b"SingleCallTargets", b"foo"),
            MethodTarget::exact(b"SingleCallTargets", b"bar"),
        ];
        TARGETS
    }
    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let value = context
            .get_expression_type(invocation.get_argument(0, &[b"input"])?)?
            .clone();
        Some(if method.eq_ignore_ascii_case(b"foo") {
            value
        } else {
            TUnion::from_atomic(TAtomic::Array(TArray::List(TList::from_known_elements(
                BTreeMap::from([(0, (false, value))]),
            ))))
        })
    }
}

#[test]
fn each_callee_keeps_its_real_semantic_provider_after_shared_operands() {
    assert_case(
        SOURCE,
        include_str!("fixtures/call-scope-observations/single-call-provider-events.json"),
        SELECTED,
        || {
            let mut registry = PluginRegistry::new();
            registry.register_method_provider(AsymmetricReturns);
            registry
        },
    );
}

struct BoundaryIssues;
impl ExpressionHook for BoundaryIssues {
    fn is_observer(&self) -> bool {
        true
    }

    fn before_arguments(
        &self,
        invocation: &InvocationInfo<'_, '_, '_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        context.report(
            IssueCode::InvalidArgument,
            Issue::error("physical argument boundary")
                .with_annotation(Annotation::primary(invocation.span())),
        );
        Ok(())
    }

    fn before_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        if matches!(expression, Expression::UnaryPrefix(_)) {
            context.report(
                IssueCode::InvalidArgument,
                Issue::error("physical operand")
                    .with_annotation(Annotation::primary(expression.span())),
            );
        }
        Ok(ExpressionHookResult::Continue)
    }
}

#[test]
fn physical_hook_and_operand_issues_survive_once_without_callee_replays() {
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let mut expected: Vec<_> = oracle
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(start, record)| {
            let expression = record["expression"].as_str().unwrap();
            let message = if expression.starts_with("$target->") {
                "physical argument boundary"
            } else if expression.starts_with("++") {
                "physical operand"
            } else {
                return None;
            };
            Some((
                start.parse::<u32>().unwrap(),
                record["end"].as_u64().unwrap() as u32,
                message,
            ))
        })
        .collect();
    expected.sort_unstable();
    assert_eq!(expected.len(), 6);
    for native in [false, true] {
        for observe in [false, true] {
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(BoundaryIssues);
            let (_, _, result, _) = analyze(native, observe, SOURCE, SELECTED, registry);
            let mut actual: Vec<_> = result
                .issues
                .iter()
                .filter(|issue| issue.message.starts_with("physical "))
                .map(|issue| {
                    let primary = issue
                        .annotations
                        .iter()
                        .find(|annotation| annotation.kind.is_primary())
                        .unwrap();
                    (
                        primary.span.start.offset,
                        primary.span.end.offset,
                        issue.message.as_ref(),
                    )
                })
                .collect();
            actual.sort_unstable();
            assert_eq!(actual, expected, "native={native} observe={observe}");
        }
    }
}

#[derive(Clone)]
struct StopOperand {
    start: u32,
    attempts: Arc<AtomicUsize>,
    fail_on: usize,
}

impl ExpressionHook for StopOperand {
    fn after_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if expression.span().start.offset == self.start
            && matches!(expression, Expression::UnaryPrefix(_))
            && self.attempts.fetch_add(1, Ordering::SeqCst) + 1 == self.fail_on
        {
            return Err(PluginError::Internal {
                reason: "operand failed".into(),
            });
        }
        Ok(())
    }
}

#[test]
fn operand_and_before_scope_query_errors_propagate_without_another_live_walk() {
    let expected: Value = serde_json::from_str(ORACLE).unwrap();
    let first = expected
        .as_object()
        .unwrap()
        .values()
        .find(|record| record["expression"] == "++$i")
        .unwrap();
    let start = SOURCE
        .windows(b"++$i".len())
        .position(|bytes| bytes == b"++$i")
        .unwrap() as u32;
    let end = first["end"].as_u64().unwrap() as u32;
    for native in [false, true] {
        for observe in [false, true] {
            for speculative_only in [false, true] {
                let mut registry = PluginRegistry::new();
                registry.register_expression_hook(StopOperand {
                    start,
                    attempts: Arc::default(),
                    fail_on: if speculative_only { 2 } else { 1 },
                });
                let (_, outcome, _, visits) =
                    analyze_result(native, observe, SOURCE, SELECTED, registry, false);
                assert!(
                    matches!(outcome, Err(mago_analyzer::error::AnalysisError::Plugin(PluginError::Internal { reason })) if reason == "operand failed")
                );
                let visits = visits.0.lock().unwrap();
                assert_eq!(visits.get(&(start, end)), Some(&1));
                let later = expected
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter(|(position, record)| {
                        record["expression"] == "$i" && position.parse::<u32>().unwrap() > end
                    })
                    .min_by_key(|(position, _)| position.parse::<u32>().unwrap())
                    .unwrap();
                assert_eq!(
                    visits
                        .get(&(
                            later.0.parse().unwrap(),
                            later.1["end"].as_u64().unwrap() as u32
                        ))
                        .copied()
                        .unwrap_or(0),
                    usize::from(speculative_only)
                );
            }
        }
    }
}

#[test]
fn inspection_and_confirmation_propagate_operand_failures() {
    for name in ["inspect", "confirm"] {
        let source = format!(
            "<?php namespace Mago {{ function {name}(...$values): void {{}} }} namespace {{ $i = 0; \\Mago\\{name}(++$i, 'int'); }}"
        );
        let source: &'static [u8] = Box::leak(source.into_bytes().into_boxed_slice());
        let start = source
            .windows(b"++$i".len())
            .position(|bytes| bytes == b"++$i")
            .unwrap() as u32;
        for native in [false, true] {
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(StopOperand {
                start,
                attempts: Arc::default(),
                fail_on: 1,
            });
            let (_, outcome, _, visits) =
                analyze_result(native, false, source, r#"{"calls":{}}"#, registry, false);
            assert!(
                matches!(outcome, Err(mago_analyzer::error::AnalysisError::Plugin(PluginError::Internal { reason })) if reason == "operand failed")
            );
            assert_eq!(visits.0.lock().unwrap().get(&(start, start + 4)), Some(&1));
        }
    }
}

fn signature_worker() -> PluginRegistry {
    use mago_analyzer::external::{
        ExternalAnalyzer, ExternalAnalyzerHandle,
        protocol::{ANALYZER_PROTOCOL_MAJOR, ANALYZER_PROTOCOL_MINOR},
    };
    use mago_extension::{WorkerCommand, WorkerPool, WorkerPoolOptions};
    use std::num::NonZeroUsize;
    let worker =
        include_str!("../../../third_party/mago-analyzer/tests/failed-method-provider.php")
            .replace(
                "pack('nnnn', 2, 1, $kind, 0)",
                &format!(
                    "pack('nnnn', {ANALYZER_PROTOCOL_MAJOR}, {ANALYZER_PROTOCOL_MINOR}, $kind, 0)"
                ),
            )
            // Exercise effective signatures for declared targets as well as
            // otherwise absent magic methods: callable signature | override.
            .replace("pack('NnCN', 1, 0, 1, 2)", "pack('NnCN', 1, 0, 3, 2)");
    let pool = WorkerPool::spawn(
        WorkerCommand::new("php")
            .with_argument("-r")
            .with_argument(worker.strip_prefix("<?php").unwrap()),
        NonZeroUsize::new(1).unwrap(),
        WorkerPoolOptions::default(),
    )
    .unwrap();
    let external = ExternalAnalyzer::initialize(
        [Arc::new(pool)],
        mago_php_version::PHPVersion::PHP85,
        &[],
        false,
    )
    .unwrap();
    let mut registry = PluginRegistry::new();
    registry.set_external_analyzer(Arc::new(ExternalAnalyzerHandle::ready(external)));
    registry.prepare_external_analyzer().unwrap();
    registry
}

#[test]
fn external_signatures_and_magic_fallbacks_share_the_real_operand_once() {
    const SOURCE: &[u8] = br"<?php
class ProviderTarget {
    public function provided($input): int { return 1; }
    public function __call(string $name, array $arguments): string { return 'magic'; }
}
class OtherTarget {
    public function provided($input): string { return 'other'; }
    public function __call(string $name, array $arguments): bool { return false; }
}
function exercise(ProviderTarget|OtherTarget $target): void {
    $i = 0;
    $target->provided(++$i);
    $i;
    $target->magic(++$i);
    $i;
    $target->withoutProvider(++$i);
    $i;
}
";
    for native in [false, true] {
        let (file, outcome, result, visits) = analyze_result(
            native,
            false,
            SOURCE,
            r#"{"calls":{}}"#,
            signature_worker(),
            true,
        );
        let artifacts = outcome.unwrap();
        for (call, expected) in [
            ("$target->provided(++$i)", "73"),
            // UnionType::getMethod() keeps the members that have the method, and a call no member has a method
            // for is an ErrorType: `__call()` alone gives PHPStan no method reflection.
            ("$target->magic(++$i)", "73"),
            ("$target->withoutProvider(++$i)", "*ERROR*"),
        ] {
            let start = SOURCE
                .windows(call.len())
                .position(|bytes| bytes == call.as_bytes())
                .unwrap() as u32;
            let span = Span::new(
                file.id,
                Position::new(start),
                Position::new(start + call.len() as u32),
            );
            assert_eq!(
                artifacts
                    .get_expression_type(&span)
                    .map(describe)
                    .as_deref(),
                Some(expected),
                "{call}"
            );
        }
        let mut expected_errors = Vec::new();
        for (index, (start, _)) in SOURCE
            .windows(4)
            .enumerate()
            .filter(|(_, bytes)| *bytes == b"++$i")
            .enumerate()
        {
            let start = start as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(start + 4));
            assert_eq!(visits.0.lock().unwrap().get(&(start, start + 4)), Some(&1));
            assert_eq!(
                artifacts.get_expression_type(&span).map(describe),
                Some((index + 1).to_string())
            );
            // The worker's zero-parameter signature applies to both provided()
            // targets and only ProviderTarget::magic(), never either fallback.
            expected_errors.extend(std::iter::repeat_n(start, [2, 1, 0][index]));
        }
        let mut actual_errors: Vec<_> = result
            .issues
            .iter()
            .filter(|issue| issue.code.as_deref() == Some(IssueCode::TooManyArguments.as_str()))
            .map(|issue| {
                issue
                    .annotations
                    .iter()
                    .find(|annotation| annotation.kind.is_primary())
                    .unwrap()
                    .span
                    .start
                    .offset
            })
            .collect();
        actual_errors.sort_unstable();
        assert_eq!(actual_errors, expected_errors);
    }
}

fn assert_case(
    source: &'static [u8],
    oracle: &str,
    selected: &str,
    registry: fn() -> PluginRegistry,
) {
    let oracle: Value = serde_json::from_str(oracle).unwrap();
    let mut failures = Vec::new();
    for native in [false, true] {
        let (file, plain, plain_result, plain_visits) =
            analyze(native, false, source, selected, registry());
        let (_, observed, observed_result, observed_visits) =
            analyze(native, true, source, selected, registry());
        assert_eq!(
            *plain_visits.0.lock().unwrap(),
            *observed_visits.0.lock().unwrap()
        );
        assert_eq!(plain_result.issues, observed_result.issues);
        assert_eq!(plain.expression_types, observed.expression_types);
        for (start, record) in oracle.as_object().unwrap() {
            // A write-only assignment target has no physical read in either analyser.
            if (record["expression"] == "$i" || record["expression"] == "$value")
                && record["type"] == "*ERROR*"
            {
                continue;
            }
            let start = start.parse::<u32>().unwrap();
            let end = record["end"].as_u64().unwrap() as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(end));
            for (observe, artifacts, visits) in [
                (false, &plain, &plain_visits),
                (true, &observed, &observed_visits),
            ] {
                let visits = visits.0.lock().unwrap();
                let count = visits.get(&(start, end)).copied().unwrap_or(0);
                if count as u64 != record["visits"].as_u64().unwrap() {
                    failures.push(format!(
                        "native={native} observe={observe} {start} {}: visits {count}, expected {}",
                        record["expression"], record["visits"]
                    ));
                }
                let actual = artifacts.get_expression_type(&span).map(describe);
                let want = oracle_type(record["type"].as_str().unwrap());
                eprintln!(
                    "physical native={native} observe={observe} {start}..{end} {}: visits={count} type={actual:?}",
                    record["expression"]
                );
                if actual.as_deref() != Some(want.as_str()) {
                    failures.push(format!("native={native} observe={observe} {start} {}: type {actual:?}, expected {want}", record["expression"]));
                }
            }
            assert_eq!(
                plain.get_raw_expression_type(&span),
                observed.get_raw_expression_type(&span)
            );
            assert_eq!(
                plain.get_native_expression_type(&span),
                observed.get_native_expression_type(&span)
            );
            if let Some(previous) = plain.get_expression_result(&span) {
                let current = observed.get_expression_result(&span).unwrap();
                assert_eq!(previous.is_eager(), current.is_eager());
                assert_eq!(previous.has_value_override(), current.has_value_override());
            }
        }
        let selected: Value = serde_json::from_str(selected).unwrap();
        for (position, call) in selected["calls"].as_object().unwrap() {
            let start = position.parse::<u32>().unwrap();
            let span = Span::new(
                file.id,
                Position::new(start),
                Position::new(start + call["expression"].as_str().unwrap().len() as u32),
            );
            for name in call["names"].as_array().unwrap() {
                let actual = observed.call_scope_observations[&span]
                    .iter()
                    .find(|actual| actual.name.as_str() == name["name"].as_str())
                    .unwrap();
                for expression in name["types"].as_array().unwrap().iter().filter(|e| {
                    let expression = e["expression"].as_str().unwrap();
                    expression.contains("$i")
                        || (expression == "$value" && e["type"] != "*ERROR*")
                        || expression == "$this->read()"
                }) {
                    let read = Span::new(
                        file.id,
                        Position::new(expression["start"].as_u64().unwrap() as u32),
                        Position::new(expression["end"].as_u64().unwrap() as u32),
                    );
                    let ty = actual.types.get(&read).map(describe);
                    let want = oracle_type(expression["type"].as_str().unwrap());
                    eprintln!(
                        "selected native={native} call={start} name={} {}: type={ty:?}",
                        name["name"], expression["expression"]
                    );
                    if ty.as_deref() != Some(want.as_str()) {
                        failures.push(format!(
                            "native={native} selected {} {}: {ty:?}, expected {}",
                            name["name"], expression["expression"], expression["type"]
                        ));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
