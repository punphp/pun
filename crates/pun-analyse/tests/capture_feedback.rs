//! PHPStan's by-reference feedback uses the selected full holder in both lanes,
//! not the final body's narrower or unavailable native scope.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program, ttype::TType,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{cst::Expression, parser::parse_file};
use serde_json::Value;

type Reads = BTreeMap<u32, (u32, String, Option<String>, usize)>;
struct Observer(Arc<Mutex<Reads>>, Vec<u32>);
impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "capture-feedback",
            "capture-feedback",
            "capture scope feedback",
        );
        &META
    }
}
impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }

    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expression.span();
        if !matches!(expression, Expression::Variable(_)) || !self.1.contains(&span.start.offset) {
            return Ok(());
        }
        let full = context
            .get_expression_type(expression)
            .unwrap()
            .get_id()
            .to_string();
        let native = context
            .get_native_expression_type(expression)
            .map(|ty| ty.get_id().to_string());
        let mut reads = self.0.lock().unwrap();
        let count = reads.get(&span.start.offset).map_or(0, |read| read.3);
        reads.insert(
            span.start.offset,
            (span.end.offset, full, native, count + 1),
        );
        Ok(())
    }
}

// Adapt scalar wrappers and the constant shape printer's key quoting/order only.
fn display(value: &str) -> String {
    let value = value
        .replace("string('", "'")
        .replace("')", "'")
        .replace("int(", "")
        .replace(')', "");
    let mut members: Vec<_> = value
        .split('|')
        .map(|member| {
            if let Some(shape) = member
                .strip_prefix("array{")
                .and_then(|shape| shape.strip_suffix('}'))
            {
                let mut items: Vec<_> = shape
                    .split(", ")
                    .map(|item| {
                        item.split_once(": ").map_or_else(
                            || item.to_owned(),
                            |(key, value)| format!("{}: {value}", key.trim_matches('\'')),
                        )
                    })
                    .collect();
                items.sort();
                format!("array{{{}}}", items.join(", "))
            } else {
                member.to_owned()
            }
        })
        .collect();
    members.sort();
    members.join("|")
}

#[test]
fn readonly_capture_feedback_matches_asymmetric_phpstan_full_and_native_scopes() {
    let source = include_bytes!("fixtures/parameter-conditions/feedback.php");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/parameter-conditions/feedback-types.json"
    ))
    .unwrap();
    let expected = expected.as_object().unwrap();
    let targets: Vec<_> = expected
        .values()
        .map(|read| read["start"].as_u64().unwrap() as u32)
        .collect();
    let mut previous = None;
    let mut failures = Vec::new();
    for native in [false, true] {
        let arena = LocalArena::new();
        let file = File::ephemeral(
            Cow::Borrowed(b"feedback.php"),
            Cow::Borrowed(source.as_slice()),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let mut settings = Settings::default();
        settings.track_native_types = native;
        let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let observed = Arc::new(Mutex::new(Reads::default()));
        let mut registry = PluginRegistry::new();
        registry.register_expression_hook(Observer(Arc::clone(&observed), targets.clone()));
        let mut result = AnalysisResult::new(references);
        Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        let reads = observed.lock().unwrap();
        assert_eq!(reads.len(), expected.len());
        for (label, want) in expected {
            let start = want["start"].as_u64().unwrap() as u32;
            let (end, full, actual_native, _) = &reads[&start];
            assert_eq!(*end as u64, want["end"].as_u64().unwrap(), "{label}");
            // The existing partial native API rejects array formal scopes and array
            // expressions. A closure's opaque body cannot poison the parent's lane.
            let available = native
                && !label.ends_with("number")
                && !label.ends_with("shape")
                && label != "inside-opaque";
            let want_native = available.then(|| display(want["native"].as_str().unwrap()));
            let full = display(full);
            let actual_native = actual_native.as_deref().map(display);
            if full != display(want["full"].as_str().unwrap()) || actual_native != want_native {
                failures.push(serde_json::json!({
                    "native_tracking": native, "label": label,
                    "expected_full": want["full"], "actual_full": full,
                    "expected_native": want_native, "actual_native": actual_native,
                }));
            }
        }
        let full_walk: Vec<_> = reads
            .iter()
            .map(|(start, (end, full, _, count))| (*start, *end, full.clone(), *count))
            .collect();
        if let Some(previous) = &previous {
            assert_eq!(&full_walk, previous);
        }
        previous = Some(full_walk);
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
