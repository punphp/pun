//! Continuation availability is independent of a call's native return contract.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts, plugin::*,
    settings::Settings,
};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program, ttype::TType,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{cst::Expression, parser::parse_file};
use serde_json::{Value, json};

type Events = Vec<((u32, u32), Option<String>)>;
struct Observe(Arc<Mutex<Events>>);
impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "native-call-continuation",
            "native-call-continuation",
            "passive native evidence",
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
        self.0.lock().unwrap().push((
            (span.start.offset, span.end.offset),
            context
                .get_native_expression_type(expression)
                .map(|ty| ty.get_id().to_string()),
        ));
        Ok(())
    }
}

fn analyze(native: bool) -> (AnalysisArtifacts, Events, String) {
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"calls.php"),
        Cow::Borrowed(include_bytes!(
            "fixtures/native-call-continuation/calls.php"
        )),
    );
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
    let found = Arc::new(Mutex::new(Vec::new()));
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observe(Arc::clone(&found)));
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let events = std::mem::take(&mut *found.lock().unwrap());
    (artifacts, events, format!("{:?}", result.issues))
}

fn normalized(value: &str) -> String {
    if value.parse::<i64>().is_ok() {
        format!("int({value})")
    } else if value.starts_with('\'') {
        format!("string({value})")
    } else {
        value.to_owned()
    }
}

#[test]
fn calls_keep_only_proven_continuations_without_fabricating_native_return_values() {
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/native-call-continuation/expected.json"
    ))
    .unwrap();
    let source = include_str!("fixtures/native-call-continuation/calls.php");
    assert_eq!(
        oracle["observations"].as_array().unwrap().len(),
        source
            .lines()
            .filter(|line| line.ends_with("// observe") || line.ends_with("// unavailable"))
            .count(),
        "every marked public probe needs an independent oracle record",
    );
    let (on, events, issues) = analyze(true);
    let (off, disabled, off_issues) = analyze(false);
    assert_eq!(issues, off_issues);
    assert_eq!(on.expression_types, off.expression_types);
    assert_eq!(on.symbol_references, off.symbol_references);
    assert_eq!(on.call_terminations, off.call_terminations);
    let ranges: Vec<_> = events.iter().map(|(range, _)| *range).collect();
    assert_eq!(
        ranges,
        disabled.iter().map(|(range, _)| *range).collect::<Vec<_>>()
    );
    let mut counts = BTreeMap::new();
    for range in &ranges {
        *counts.entry(range).or_insert(0) += 1;
    }
    assert!(
        counts.values().all(|&count| count == 1),
        "replayed expression: {counts:?}"
    );
    assert!(disabled.iter().all(|(_, value)| value.is_none()));
    for &(start, end) in &ranges {
        let span = mago_span::Span::new(
            mago_database::file::FileId::zero(),
            mago_span::Position::new(start),
            mago_span::Position::new(end),
        );
        let full = |artifacts: &AnalysisArtifacts| {
            artifacts.get_expression_result(&span).map(|result| {
                (
                    result.get_raw_type().clone(),
                    result.is_eager(),
                    result.has_value_override(),
                )
            })
        };
        assert_eq!(
            full(&on),
            full(&off),
            "selected provenance at {start}..{end}"
        );
    }
    let mut failures = Vec::new();
    let mut records = Vec::new();
    for expected in oracle["observations"].as_array().unwrap() {
        let range = (
            expected["start"].as_u64().unwrap() as u32,
            expected["end"].as_u64().unwrap() as u32,
        );
        assert_eq!(
            &source[range.0 as usize..range.1 as usize],
            expected["expression"].as_str().unwrap()
        );
        let got = events
            .iter()
            .find(|(key, _)| *key == range)
            .unwrap_or_else(|| panic!("unvisited: {expected}"));
        let want = expected["supported"]
            .as_bool()
            .unwrap()
            .then(|| normalized(expected["native"].as_str().unwrap()));
        let mut record = expected.clone();
        record["observed_native"] = json!(got.1);
        records.push(record);
        if got.1 != want {
            failures.push(json!({"expected":expected,"got":got.1}));
        }
    }
    println!(
        "complete observations: {}",
        serde_json::to_string_pretty(&records).unwrap()
    );
    println!(
        "{} once-only expression visits; selected artifacts and provenance unchanged",
        ranges.len()
    );
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
