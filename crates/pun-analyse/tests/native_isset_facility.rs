//! Position-owned native Isset/coalesce transfers, through the public Analyzer.

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
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{cst::Expression, parser::parse_file};
use serde_json::{Value, json};

type Events = BTreeMap<(u32, u32, &'static str), Option<String>>;
struct Observe(Arc<Mutex<(Events, Vec<(u32, u32, &'static str)>)>>);
impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("native-isset", "native-isset", "passive native evidence");
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
        let kind = match expression {
            Expression::Binary(_) => "binary",
            Expression::Assignment(_) => "assignment",
            _ => "other",
        };
        let key = (span.start.offset, span.end.offset, kind);
        let native = context.get_native_expression_type(expression).map(|ty| {
            if ty.is_error() {
                "*ERROR*".to_owned()
            } else {
                normalize(&ty.get_id().to_string())
            }
        });
        let mut found = self.0.lock().unwrap();
        found.0.insert(key, native);
        found.1.push(key);
        Ok(())
    }
}

fn analyze(
    native: bool,
) -> (
    AnalysisArtifacts,
    Events,
    Vec<(u32, u32, &'static str)>,
    String,
) {
    let arena = LocalArena::new();
    let file = File::new(
        Cow::Borrowed(b"transfers.php"),
        FileType::Host,
        None,
        Cow::Borrowed(include_bytes!(
            "fixtures/native-isset-facility/transfers.php"
        )),
    );
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
    let found = Arc::new(Mutex::new((Events::new(), Vec::new())));
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observe(Arc::clone(&found)));
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let (events, sequence) = std::mem::take(&mut *found.lock().unwrap());
    (artifacts, events, sequence, format!("{:?}", result.issues))
}

fn split_top(value: &str, delimiter: char) -> Vec<&str> {
    let mut depth = 0;
    let mut quoted = false;
    let mut start = 0;
    let mut parts = Vec::new();
    for (offset, character) in value.char_indices() {
        match character {
            '\'' => quoted = !quoted,
            '(' | '{' | '<' if !quoted => depth += 1,
            ')' | '}' | '>' if !quoted => depth -= 1,
            _ => {}
        }
        if character == delimiter && !quoted && depth == 0 {
            parts.push(value[start..offset].trim());
            start = offset + character.len_utf8();
        }
    }
    parts.push(value[start..].trim());
    parts
}

// Display adaptation only: PHPStan literal syntax versus Mago's tagged scalars.
fn normalize(value: &str) -> String {
    if matches!(value, "array" | "array<array-key, mixed>") {
        return "array".to_owned();
    }
    let parts = split_top(value, '|');
    if parts.len() > 1 {
        let mut parts: Vec<_> = parts.into_iter().map(normalize).collect();
        parts.sort();
        parts.join("|")
    } else if let Some(shape) = value
        .strip_prefix("array{")
        .or_else(|| value.strip_prefix("list{"))
        .and_then(|value| value.strip_suffix('}'))
    {
        let mut items: Vec<_> = split_top(shape, ',')
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let parts = split_top(item, ':');
                let (key, value) = if parts.len() == 2 {
                    (parts[0].trim_matches('\'').to_owned(), parts[1])
                } else {
                    (index.to_string(), item)
                };
                format!("{key}:{}", normalize(value))
            })
            .collect();
        items.sort();
        format!("array{{{}}}", items.join(","))
    } else if value.starts_with('\'') {
        format!("string({value})")
    } else if value.parse::<i64>().is_ok() {
        format!("int({value})")
    } else {
        value
            .replace("mixed~(null)", "mixed~null")
            .replace("*NEVER*", "never")
    }
}

#[test]
fn live_native_isset_transfers_match_phpstan_without_changing_selected_evaluation() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/native-isset-facility/expected.json")).unwrap();
    let (on, events, sequence, issues) = analyze(true);
    let (off, disabled, off_sequence, off_issues) = analyze(false);
    assert_eq!(issues, off_issues);
    assert_eq!(on.expression_types, off.expression_types);
    assert_eq!(on.symbol_references, off.symbol_references);
    assert_eq!(on.call_terminations, off.call_terminations);
    assert_eq!(sequence, off_sequence);
    let mut counts = BTreeMap::new();
    for key in &sequence {
        *counts.entry(key).or_insert(0) += 1;
    }
    assert!(
        counts.values().all(|&count| count == 1),
        "replayed expression: {counts:?}"
    );
    assert!(disabled.values().all(Option::is_none));
    let mut failures = Vec::new();
    for &(start, end, _) in &sequence {
        let span = mago_span::Span::new(
            mago_database::file::FileId::zero(),
            mago_span::Position::new(start),
            mago_span::Position::new(end),
        );
        let full_result = |artifacts: &AnalysisArtifacts| {
            artifacts.get_expression_result(&span).map(|result| {
                (
                    result.get_raw_type().clone(),
                    result.is_eager(),
                    result.has_value_override(),
                )
            })
        };
        if full_result(&on) != full_result(&off) {
            failures.push(json!({"provenance_start":start,"end":end,
                "on":format!("{:?}",full_result(&on)),
                "off":format!("{:?}",full_result(&off))}));
        }
    }
    for expected in oracle["observations"].as_array().unwrap() {
        let start = expected["start"].as_u64().unwrap() as u32;
        let end = expected["end"].as_u64().unwrap() as u32;
        let key = sequence
            .iter()
            .rev()
            .find(|(a, b, _)| (*a, *b) == (start, end));
        let got = if let Some(key) = key {
            events[key].clone()
        } else {
            assert_eq!(
                expected["carried"], true,
                "missing expression visit: {expected}"
            );
            let span = mago_span::Span::new(
                mago_database::file::FileId::zero(),
                mago_span::Position::new(start),
                mago_span::Position::new(end),
            );
            on.get_native_expression_type(&span)
                .map(|ty| normalize(&ty.get_id().to_string()))
        };
        let want = expected["supported"]
            .as_bool()
            .unwrap()
            .then(|| normalize(expected["native"].as_str().unwrap()));
        if got != want {
            failures.push(json!({"expected":expected, "got":got}));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
    let observations = oracle["observations"].as_array().unwrap();
    println!(
        "PHPStan comparisons: {} supported, {} unavailable, {} carried operands; {} once-only expression events with identical full provenance and selected artifacts",
        observations
            .iter()
            .filter(|row| row["supported"] == true)
            .count(),
        observations
            .iter()
            .filter(|row| row["supported"] == false)
            .count(),
        observations
            .iter()
            .filter(|row| row["carried"] == true)
            .count(),
        sequence.len(),
    );
}
