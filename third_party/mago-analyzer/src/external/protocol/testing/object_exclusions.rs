use super::*;

use mago_allocator::LocalArena;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_extension::{WorkerCommand, WorkerPool, WorkerPoolOptions};
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;
use std::num::NonZeroUsize;
use std::sync::Mutex;

use crate::external::{ExternalAnalyzer, ExternalAnalyzerHandle};
use crate::plugin::PluginRegistry;

struct Warnings(Arc<Mutex<Vec<String>>>);

impl tracing::Subscriber for Warnings {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        *metadata.level() <= tracing::Level::WARN
    }

    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}

    fn event(&self, event: &tracing::Event<'_>) {
        struct Fields(String);
        impl tracing::field::Visit for Fields {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                use std::fmt::Write;
                write!(&mut self.0, "{field}={value:?}; ").unwrap();
            }
        }
        let mut fields = Fields(String::new());
        event.record(&mut fields);
        self.0.lock().unwrap().push(fields.0);
    }
}

fn analyze(
    mode: &str,
    source: &'static [u8],
) -> Result<(TUnion, CodebaseMetadata, Vec<String>), crate::error::AnalysisError> {
    let registration = registration_response()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let command = WorkerCommand::new("php").with_arguments([
        "-r".to_string(),
        include_str!("object_exclusions.php")
            .strip_prefix("<?php\n")
            .unwrap()
            .to_string(),
        registration,
        mode.to_string(),
    ]);
    let pool = Arc::new(
        WorkerPool::spawn(
            command,
            NonZeroUsize::new(1).unwrap(),
            WorkerPoolOptions::default(),
        )
        .unwrap(),
    );
    let external = ExternalAnalyzer::initialize([pool], PHPVersion::PHP84, &[], false).unwrap();
    let mut registry = PluginRegistry::default();
    registry.set_external_analyzer(Arc::new(ExternalAnalyzerHandle::ready(external)));
    let arena = LocalArena::new();
    let file = Arc::new(File::ephemeral(
        Cow::Borrowed(b"object-exclusion-roundtrip.php"),
        Cow::Borrowed(source),
    ));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP84);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let session = ExternalAnalysisSession::from_files([Arc::clone(&file)]);
    let analyzer = crate::Analyzer::new(
        &arena,
        &file,
        &names,
        &codebase,
        &registry,
        Default::default(),
    )
    .with_external_analysis_session(&session);
    let mut result = crate::analysis_result::AnalysisResult::new(references);
    let warnings = Arc::new(Mutex::new(Vec::new()));
    let artifacts = tracing::subscriber::with_default(Warnings(Arc::clone(&warnings)), || {
        analyzer.analyze_with_artifacts(program, &mut result)
    })?;
    let marker = b"$roundtrip;";
    let start = source
        .windows(marker.len())
        .rposition(|part| part == marker)
        .unwrap() as u32;
    let span = mago_span::Span::new(
        file.id,
        mago_span::Position::new(start),
        mago_span::Position::new(start + marker.len() as u32 - 1),
    );
    let returned = artifacts
        .get_expression_type(&span)
        .expect("returned provider value")
        .clone();
    let warnings = warnings.lock().unwrap().clone();
    Ok((returned, codebase, warnings))
}

const FLOW: &[u8] = b"<?php
interface A {} interface B {} interface C {}
function demo_service($value) {}
function inspect($query): void {
    if ($query instanceof A) { return; }
    if ($query instanceof C) {
        $roundtrip = demo_service($query);
        $roundtrip;
    }
}";

const SEEDED: &[u8] = b"<?php
interface A {} interface B {} interface C {}
function demo_service($value = null) {}
$seed = demo_service();
$roundtrip = demo_service($seed);
$roundtrip;";

#[test]
fn analyzer_worker_roundtrip_retains_a_real_narrowed_object_exclusion() {
    let (returned, codebase, warnings) = analyze("echo", FLOW).unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    // Actual PHPStan 2.3.1: object-intersections/subtraction.json line 18 and
    // type-api.json's C-not-A describe this value as C~A&C, not C or A&C.
    let object = returned.get_single_named_object().unwrap();
    assert_eq!(object.name, word("C"));
    assert!(
        object.subtracted_type.is_some(),
        "provider returned {}, not PHPStan's C~A&C",
        returned.get_id()
    );
    let excluded = object.subtracted_type.as_ref().unwrap();
    // Codex retains its primary conjunct; PHPStan sorts the public description.
    let mut parts: Vec<_> = excluded
        .get_id()
        .to_string()
        .split('&')
        .map(str::to_owned)
        .collect();
    parts.sort();
    assert_eq!(parts, ["A", "C"]);
    assert!(object.intersection_types.is_none());

    // PHPStan's retained type-api capture: not-A|C-not-A is mixed~A&C.
    // Dropping the payload (C) or making it positive (A&C) instead produces mixed.
    let named = |name| TAtomic::Object(TObject::Named(TNamedObject::new(word(name))));
    let without_a = TUnion::from_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(
        TUnion::from_atomic(named("A")),
    )));
    let mut positive = named("A");
    positive.add_intersection_type(named("C"));
    for (value, negative) in [
        (returned, true),
        (TUnion::from_atomic(named("C")), false),
        (TUnion::from_atomic(positive), false),
    ] {
        let combined = mago_codex::ttype::combine_union_types(
            &without_a,
            &value,
            &codebase,
            Default::default(),
        );
        let TAtomic::Mixed(mixed) = combined.get_single() else {
            panic!("expected mixed");
        };
        assert_eq!(mixed.get_subtracted_type().is_some(), negative);
        if let Some(excluded) = mixed.get_subtracted_type() {
            let mut parts: Vec<_> = excluded
                .get_id()
                .to_string()
                .split('&')
                .map(str::to_owned)
                .collect();
            parts.sort();
            assert_eq!(parts, ["A", "C"]);
        }
    }
}

#[test]
fn analyzer_worker_roundtrip_keeps_ordinary_union_and_nested_negative_payloads_distinct() {
    let (ordinary, _, warnings) = analyze("ordinary", SEEDED).unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(ordinary.get_id().as_str(), Some("C"));
    assert!(
        ordinary
            .get_single_named_object()
            .unwrap()
            .subtracted_type
            .is_none()
    );

    let (union, _, warnings) = analyze("union", SEEDED).unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    let union = union.get_single_named_object().unwrap();
    let excluded = union
        .subtracted_type
        .as_ref()
        .expect("nonempty union exclusion");
    assert_eq!(excluded.get_id().as_str(), Some("(A&C)|(B&C)"));
    assert_eq!(excluded.types.len(), 2);
    assert!(union.intersection_types.is_none());

    let (nested, _, warnings) = analyze("nested", SEEDED).unwrap();
    assert!(warnings.is_empty(), "{warnings:?}");
    let inner = nested
        .get_single_named_object()
        .unwrap()
        .subtracted_type
        .as_ref()
        .expect("outer exclusion")
        .get_single_named_object()
        .unwrap();
    assert_eq!(inner.name, word("C"));
    assert_eq!(
        inner
            .subtracted_type
            .as_ref()
            .expect("nested exclusion")
            .get_id()
            .as_str(),
        Some("A&C")
    );
}

#[test]
fn analyzer_rejects_exclusion_payloads_beyond_existing_complete_type_limits() {
    let source = b"<?php function demo_service() {} $roundtrip = demo_service(); $roundtrip;";
    for (mode, message) in [
        ("deep", "maximum nesting depth"),
        ("many", "count 65537 exceeds limit 65536"),
        ("empty", "contains no atomic types"),
    ] {
        let (returned, _, warnings) = analyze(mode, source).unwrap();
        assert!(returned.is_mixed(), "{mode}: {}", returned.get_id());
        assert_eq!(warnings.len(), 1, "{mode}: {warnings:?}");
        assert!(warnings[0].contains(message), "{mode}: {warnings:?}");
        assert!(warnings[0].contains("using native analysis fallback"));
    }
}
