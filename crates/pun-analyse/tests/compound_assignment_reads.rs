//! Prepared reads survive applyWrite without replaying expressions or semantic hooks.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts, plugin::*,
    settings::Settings,
};
use mago_codex::{
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{TType, union::TUnion},
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Span};
use mago_syntax::{
    cst::{Call, Expression},
    parser::parse_file,
    walker::MutWalker,
};
use serde_json::{Value, json};

const CODE: &[u8] = include_bytes!("fixtures/compound-assignment-reads/inputs.php");
const ORIGINAL: &[u8] =
    include_bytes!("fixtures/rules/data/Arrays/data/offset-access-assignop-nullsafe.php");
const EXPECTED: &str = include_str!("fixtures/compound-assignment-reads/expected.json");

/// (level, file, line, identifier) records whose message differs for a reason outside these reads.
const GAPS: &[(i64, &str, i64, &str)] = &[
    // `$c ? [$v] : []`: PHPStan's TypeCombinator keeps `array{}|array{int}`, Mago's combiner joins the two constant
    // arrays into `array{0?: int}`, so offsetAccess.notFound describes the holder differently
    (9, "inputs.php", 118, "offsetAccess.notFound"),
];

fn gap(level: i64, record: &Value) -> bool {
    GAPS.iter().any(|(l, file, line, id)| {
        *l == level && record[0] == *file && record[1] == *line && record[2] == *id
    })
}

#[test]
fn compound_read_diagnostics_match_phpstan_for_implemented_rules() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value = serde_json::from_str(EXPECTED).unwrap();
    for level in [3, 5, 9] {
        let mut project = pun_analyse::Project::new(
            vec![
                root.join("compound-assignment-reads/inputs.php"),
                root.join("rules/data/Arrays/data/offset-access-assignop-nullsafe.php"),
            ],
            vec![],
        );
        project.level = level;
        project.php_version = 80500;
        project.options.set(pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, true);
        let mut got: Vec<_> = pun_analyse::run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                (
                    std::path::Path::new(&error.file)
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
        got.sort();
        let level = i64::from(level);
        let got: Vec<Value> = serde_json::from_value::<Vec<Value>>(serde_json::to_value(got).unwrap())
            .unwrap()
            .into_iter()
            .filter(|record| !gap(level, record))
            .collect();
        // Keep the complete capture. As in levels.rs, compare every record for an implemented rule.
        let want: Vec<_> = expected["diagnostics"][&level.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .filter(|record| pun_analyse::reports(record[2].as_str().unwrap()) && !gap(level, record))
            .cloned()
            .collect();
        assert_eq!(
            json!(got),
            json!(want),
            "level {level}"
        );
    }
}

#[derive(Default)]
struct Targets(BTreeMap<Span, Span>);
impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Targets {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, (): &mut ()) {
        if let Expression::Assignment(assignment) = expression {
            self.0.insert(assignment.rhs.span(), assignment.lhs.span());
        }
    }
}

#[test]
fn unavailable_prepared_reads_are_not_replaced_by_the_write_fallback() {
    struct UntypedRoot;
    impl Provider for UntypedRoot {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("untyped-root", "untyped-root", "no result");
            &META
        }
    }
    impl ExpressionHook for UntypedRoot {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            Ok(
                if matches!(expression, Expression::Call(Call::Function(call))
                if matches!(call.function, Expression::Identifier(name) if name.value() == b"source"))
                {
                    ExpressionHookResult::Skip
                } else {
                    ExpressionHookResult::Continue
                },
            )
        }
    }
    let code = b"<?php function source(): array { return []; } function value(): int { return 1; } function test(): void { source()['b'] += value(); }";
    let file = File::ephemeral(b"untyped.php".as_slice().into(), code.as_slice().into());
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings::default();
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut registry = PluginRegistry::with_library_providers();
    registry.register_expression_hook(UntypedRoot);
    let events = Arc::<Mutex<Events>>::default();
    registry.register_expression_hook(Probe {
        targets: BTreeMap::new(),
        events: Arc::clone(&events),
    });
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let source = std::str::from_utf8(code).unwrap();
    let start = source.rfind("source()").unwrap() as u32;
    for len in [8, 13] {
        let span = Span::new(file.id, start.into(), (start + len).into());
        assert!(artifacts.get_expression_result(&span).is_none());
    }
}

#[derive(Default, Debug)]
struct Events {
    before: BTreeMap<Span, usize>,
    after: BTreeMap<Span, usize>,
    reads: BTreeMap<Span, Value>,
    writes: BTreeMap<Span, String>,
}

struct Probe {
    targets: BTreeMap<Span, Span>,
    events: Arc<Mutex<Events>>,
}
impl Provider for Probe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("prepared-reads", "prepared-reads", "prewrite handoff");
        &META
    }
}
impl ExpressionHook for Probe {
    fn is_observer(&self) -> bool {
        true
    }

    fn before_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        let mut events = self.events.lock().unwrap();
        *events.before.entry(expression.span()).or_default() += 1;
        if let Some(target) = self.targets.get(&expression.span()) {
            events.reads.insert(
                *target,
                json!({
                    "value": context.get_expression_type(target).map(describe),
                    "raw": context.get_raw_expression_type(target).map(describe),
                    "native": context.get_native_expression_type(target).map(describe),
                }),
            );
        }
        Ok(ExpressionHookResult::Continue)
    }

    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let mut events = self.events.lock().unwrap();
        *events.after.entry(expression.span()).or_default() += 1;
        if let Expression::Assignment(assignment) = expression
            && let Some(array) = context.get_variable_type(b"$array")
        {
            events.writes.insert(assignment.lhs.span(), describe(array));
        }
        Ok(())
    }
}

fn describe(ty: &TUnion) -> String {
    if ty.is_error() {
        "*ERROR*".into()
    } else {
        ty.get_id().to_string().replace(' ', "")
    }
}

fn analyze(code: &[u8], native: bool, observe: bool) -> (File, AnalysisArtifacts, Events, String) {
    let file = File::ephemeral(b"compound.php".as_slice().into(), code.to_vec().into());
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings {
        track_native_types: native,
        ..Settings::default()
    };
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let events = Arc::<Mutex<Events>>::default();
    let mut registry = PluginRegistry::with_library_providers();
    if observe {
        let mut targets = Targets::default();
        targets.walk_program(program, &mut ());
        registry.register_expression_hook(Probe {
            targets: targets.0,
            events: Arc::clone(&events),
        });
    }
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let events = std::mem::take(&mut *events.lock().unwrap());
    (file, artifacts, events, format!("{:?}", result.issues))
}

#[test]
fn completed_target_reads_match_the_real_phpstan_prepare_target_handoff() {
    let expected: Value = serde_json::from_str(EXPECTED).unwrap();
    for (name, code) in [
        ("inputs.php", CODE),
        ("offset-access-assignop-nullsafe.php", ORIGINAL),
    ] {
        for native in [false, true] {
            let (file, artifacts, events, _) = analyze(code, native, true);
            let mut actual = Vec::new();
            let mut want = Vec::new();
            for record in expected["reads"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r[0] == name)
            {
                let span = Span::new(
                    file.id,
                    (record[1].as_u64().unwrap() as u32).into(),
                    (record[2].as_u64().unwrap() as u32).into(),
                );
                let result = artifacts.get_expression_result(&span).unwrap();
                let before_write = &events.reads[&span];
                assert_eq!(json!(describe(result.get_raw_type())), before_write["raw"]);
                assert_eq!(json!(describe(result.get_type())), before_write["value"]);
                assert!(
                    !events.before.contains_key(&span),
                    "no second semantic visit to a composed target"
                );
                assert!(
                    !events.after.contains_key(&span),
                    "no fabricated target after hook"
                );
                actual.push(json!([
                    name,
                    span.start.offset,
                    span.end.offset,
                    describe(result.get_type()),
                    result.get_type().is_error()
                ]));
                want.push(record.clone());
                let native_record = expected["native_reads"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|read| {
                        read[0] == record[0] && read[1] == record[1] && read[2] == record[2]
                    })
                    .unwrap();
                // Only ??= has a supported native prepare-target transfer. Other
                // compound operators keep their explicit unavailability boundary.
                let native_want = if native && native_record[4].as_bool().unwrap() {
                    native_record[3].clone()
                } else {
                    Value::Null
                };
                assert_eq!(
                    json!(result.get_native_type().map(describe)),
                    native_want,
                    "{name} {span:?}: native target read"
                );
                assert_eq!(
                    json!(result.get_raw_native_type().map(describe)),
                    native_want
                );
                assert_eq!(before_write["native"], native_want);
                println!(
                    "{name} {span:?}: RHS entry {:?}; completed {}; written receiver {:?}",
                    events.reads.get(&span),
                    describe(result.get_type()),
                    events.writes.get(&span),
                );
                if name == "offset-access-assignop-nullsafe.php" {
                    assert!(
                        events.writes[&span].contains("mixed"),
                        "the written scope must not be rolled back"
                    );
                }
            }
            assert_eq!(actual, want, "{name}, native={native}");
        }
    }
}

#[test]
fn prepared_read_observers_do_not_replay_operands_or_change_full_results() {
    for native in [false, true] {
        let (file, plain, _, plain_issues) = analyze(CODE, native, false);
        let (_, observed, events, observed_issues) = analyze(CODE, native, true);
        assert_eq!(plain.expression_types, observed.expression_types);
        assert_eq!(plain_issues, observed_issues);
        for &(start, end) in plain.expression_types.keys() {
            let span = Span::new(file.id, start.into(), end.into());
            let plain = plain.get_expression_result(&span).unwrap();
            let observed = observed.get_expression_result(&span).unwrap();
            assert_eq!(plain.get_raw_type(), observed.get_raw_type());
            assert_eq!(plain.get_native_type(), observed.get_native_type());
            assert_eq!(plain.get_raw_native_type(), observed.get_raw_native_type());
            assert_eq!(plain.get_keep_void_type(), observed.get_keep_void_type());
            assert_eq!(plain.is_eager(), observed.is_eager());
            assert_eq!(plain.has_value_override(), observed.has_value_override());
        }
        let source = std::str::from_utf8(CODE).unwrap();
        for needle in ["key()]", "value();"] {
            let start = source.find(needle).unwrap() as u32;
            let length = needle.find(')').unwrap() as u32 + 1;
            for visits in [&events.before, &events.after] {
                assert_eq!(
                    visits
                        .iter()
                        .find(|(span, _)| {
                            span.start.offset == start && span.end.offset == start + length
                        })
                        .map(|(_, count)| *count),
                    Some(1),
                    "{needle}"
                );
            }
        }
    }
}
