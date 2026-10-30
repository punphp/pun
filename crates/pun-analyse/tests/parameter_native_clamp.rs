//! Parameter entry selects PHPDoc refinements within the independent native declaration.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
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
            "parameter-native-clamp",
            "parameter-native-clamp",
            "passive parameter entry evidence",
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

fn analyze(source: &'static str, native: bool) -> (AnalysisArtifacts, Events, String) {
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"types.php"),
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
    let events = Arc::new(Mutex::new(Vec::new()));
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observe(Arc::clone(&events)));
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    assert_eq!(before, format!("{codebase:?}"));
    let events = std::mem::take(&mut *events.lock().unwrap());
    (artifacts, events, format!("{:?}", result.issues))
}

fn normalize(value: &str) -> String {
    let mut parts: Vec<_> = value.split('|').collect();
    parts.sort();
    parts.join("|")
}

#[test]
fn parameter_selection_and_native_channels_match_complete_phpstan_records() {
    let source = include_str!("fixtures/parameter-native-clamp/types.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/parameter-native-clamp/expected.json"
    ))
    .unwrap();
    let (on, events, issues) = analyze(source, true);
    let (off, disabled, off_issues) = analyze(source, false);
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
    for &(start, end) in &ranges {
        *counts.entry((start, end)).or_insert(0) += 1;
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
    assert!(
        counts.values().all(|&count| count == 1),
        "replayed expression: {counts:?}"
    );
    assert!(disabled.iter().all(|(_, value)| value.is_none()));
    let expected = oracle["observations"].as_array().unwrap();
    assert_eq!(
        expected.len(),
        source
            .lines()
            .filter(|line| line.ends_with("// observe") || line.ends_with("// unavailable"))
            .count()
    );
    let mut records = Vec::new();
    let mut failures = Vec::new();
    for expected in expected {
        let range = (
            expected["start"].as_u64().unwrap() as u32,
            expected["end"].as_u64().unwrap() as u32,
        );
        assert_eq!(
            &source[range.0 as usize..range.1 as usize],
            expected["expression"].as_str().unwrap()
        );
        let got = events.iter().find(|(key, _)| *key == range).unwrap();
        let span = mago_span::Span::new(
            mago_database::file::FileId::zero(),
            mago_span::Position::new(range.0),
            mago_span::Position::new(range.1),
        );
        let full = normalize(&on.get_expression_type(&span).unwrap().get_id().to_string());
        let native = expected["supported"]
            .as_bool()
            .unwrap()
            .then(|| normalize(expected["native"].as_str().unwrap()));
        let mut record = expected.clone();
        record["observed_full"] = json!(full);
        record["observed_native"] = json!(got.1);
        if full != normalize(expected["full"].as_str().unwrap()) || got.1 != native {
            failures.push(record.clone());
        }
        records.push(record);
    }
    println!(
        "{} once-only expression visits; selected artifacts/provenance/issues/references/returns identical with native on/off",
        ranges.len()
    );
    println!(
        "complete observations: {}",
        serde_json::to_string_pretty(&records).unwrap()
    );
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parameter-native-clamp");
    for name in ["types.php", "channels.php"] {
        for level in [0, 4, 9] {
            let mut project = pun_analyse::Project::new(vec![root.join(name)], vec![]);
            project.level = level;
            project.php_version = 80500;
            let mut actual: Vec<_> = pun_analyse::run(&project)
                .unwrap()
                .into_iter()
                .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
                .collect();
            actual.sort();
            let actual = serde_json::to_value(actual).unwrap();
            let key = format!("{name}-{level}");
            println!(
                "complete diagnostics {key}: {}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            if actual != oracle["errors"][&key] {
                failures.push(
                    json!({"case": key, "actual": actual, "expected": oracle["errors"][&key]}),
                );
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
fn parameter_callable_and_template_entry_channels_remain_distinct() {
    use mago_codex::ttype::atomic::{TAtomic, callable::TCallable};

    let source = include_str!("fixtures/parameter-native-clamp/channels.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/parameter-native-clamp/expected.json"
    ))
    .unwrap();
    let (on, events, issues) = analyze(source, true);
    let (off, disabled, off_issues) = analyze(source, false);
    assert_eq!(issues, off_issues);
    assert_eq!(on.expression_types, off.expression_types);
    assert_eq!(on.symbol_references, off.symbol_references);
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
    assert!(counts.values().all(|&count| count == 1));
    assert!(disabled.iter().all(|(_, value)| value.is_none()));
    assert_eq!(oracle["channels"].as_array().unwrap().len(), 2);
    for expected in oracle["channels"].as_array().unwrap() {
        let range = (
            expected["start"].as_u64().unwrap() as u32,
            expected["end"].as_u64().unwrap() as u32,
        );
        let span = mago_span::Span::new(
            mago_database::file::FileId::zero(),
            mago_span::Position::new(range.0),
            mago_span::Position::new(range.1),
        );
        let ty = on.get_expression_type(&span).unwrap();
        let got = events.iter().find(|(key, _)| *key == range).unwrap();
        let channel = match ty.get_single() {
            TAtomic::GenericParameter(template) => {
                assert_eq!(got.1.as_deref(), expected["native"].as_str());
                json!({"name": template.parameter_name.to_string(), "bound": template.constraint.get_id().to_string()})
            }
            TAtomic::Callable(TCallable::Signature(signature)) => {
                // Dynamic callable invocation still invalidates native state before its callee.
                assert_eq!(got.1, None);
                json!({"parameters": signature.get_parameters().iter().map(|parameter| parameter.get_type_signature().unwrap().get_id().to_string()).collect::<Vec<_>>(),
                    "return": signature.return_type.as_ref().unwrap().get_id().to_string()})
            }
            other => panic!("lost parameter channel: {other:?}"),
        };
        let want = if expected.get("template").is_some() {
            &expected["template"]
        } else {
            &expected["callable"]
        };
        assert_eq!(&channel, want);
        println!(
            "parameter channel: {}",
            json!({"oracle": expected, "selected": ty.get_id().to_string(), "native": got.1, "channel": channel})
        );
    }
    println!(
        "{} once-only channel expression visits; native on/off isolation preserved",
        ranges.len()
    );
}
