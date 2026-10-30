//! Native nullsafe reads are source-owned, passive observations of the selected walk.

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
            "native-nullsafe-transfer",
            "native-nullsafe-transfer",
            "passive native nullsafe evidence",
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
                .map(|ty| normalize(&ty.get_id().to_string())),
        ));
        Ok(())
    }
}

fn analyze(
    name: &'static str,
    source: &'static str,
    native: bool,
) -> (AnalysisArtifacts, Events, String) {
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(name.as_bytes()),
        Cow::Borrowed(source.as_bytes()),
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
    let before = format!("{codebase:?}");
    let found = Arc::new(Mutex::new(Vec::new()));
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observe(Arc::clone(&found)));
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    assert_eq!(before, format!("{codebase:?}"));
    let events = std::mem::take(&mut *found.lock().unwrap());
    (artifacts, events, format!("{:?}", result.issues))
}

// Display adaptation only: Mago prints constant integers as int(0).
fn normalize(value: &str) -> String {
    let mut parts: Vec<_> = value
        .split('|')
        .map(|part| {
            if part.parse::<i64>().is_ok() {
                format!("int({part})")
            } else {
                part.to_owned()
            }
        })
        .collect();
    parts.sort();
    parts.join("|")
}

#[test]
fn native_nullsafe_preserves_incoming_receivers_and_selected_results_without_replay() {
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/native-nullsafe-transfer/expected.json"
    ))
    .unwrap();
    let mut records = Vec::new();
    let mut failures = Vec::new();
    for (name, source) in [
        (
            "consumer.php",
            include_str!("fixtures/native-nullsafe-transfer/consumer.php"),
        ),
        (
            "properties.php",
            include_str!("fixtures/native-nullsafe-transfer/properties.php"),
        ),
        (
            "receiver-boundary.php",
            include_str!("fixtures/native-nullsafe-transfer/receiver-boundary.php"),
        ),
    ] {
        let (on, events, issues) = analyze(name, source, true);
        let (off, disabled, off_issues) = analyze(name, source, false);
        assert_eq!(issues, off_issues);
        assert_eq!(on.expression_types, off.expression_types);
        assert_eq!(on.symbol_references, off.symbol_references);
        assert_eq!(on.call_terminations, off.call_terminations);
        assert_eq!(on.inferred_return_types, off.inferred_return_types);
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
        let expected: Vec<_> = oracle["observations"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|record| record["file"] == name)
            .collect();
        let count = if name == "consumer.php" {
            // Four captured receivers, four property values, two isset and two ?? results.
            12
        } else {
            source
                .lines()
                .filter(|line| line.ends_with("// observe") || line.ends_with("// unavailable"))
                .count()
        };
        assert_eq!(expected.len(), count, "complete capture for {name}");
        for expected in expected {
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
                .then(|| normalize(expected["native"].as_str().unwrap()));
            let mut record = expected.clone();
            record["observed_native"] = json!(got.1);
            let span = mago_span::Span::new(
                mago_database::file::FileId::zero(),
                mago_span::Position::new(range.0),
                mago_span::Position::new(range.1),
            );
            record["observed_full"] = json!(
                on.get_expression_type(&span)
                    .map(|ty| ty.get_id().to_string())
            );
            records.push(record);
            if got.1 != want {
                failures.push(json!({"expected": expected, "got": got.1}));
            }
        }
        println!(
            "{name}: {} once-only expression visits; selected artifacts and provenance unchanged",
            ranges.len()
        );
    }
    println!(
        "complete observations: {}",
        serde_json::to_string_pretty(&records).unwrap()
    );
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
