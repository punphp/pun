//! Native property reflection and straight-line value transfers through the public Analyzer.

use std::borrow::Cow;

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::artifacts::AnalysisArtifacts;
use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::settings::Settings;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use serde_json::Value;

fn analyze(source: &[u8], native: bool) -> (AnalysisArtifacts, String) {
    let arena = LocalArena::new();
    let file = File::new(
        Cow::Borrowed(b"native-properties.php"),
        FileType::Host,
        None,
        Cow::Owned(source.to_vec()),
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
    let before = format!("{codebase:?}");
    let registry = PluginRegistry::new();
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    assert_eq!(
        before,
        format!("{codebase:?}"),
        "native evaluation must not mutate declarations"
    );
    (artifacts, format!("{:?}", result.issues))
}

fn identity(ty: &TUnion) -> Value {
    match ty.types.as_ref() {
        [TAtomic::Mixed(mixed)] => Value::Bool(mixed.is_explicit()),
        _ => Value::Null,
    }
}

#[test]
fn reflected_properties_copies_offsets_methods_and_captures_match_both_phpstan_versions() {
    check_fixture(
        include_bytes!("fixtures/explicit-mixed/native-properties.php"),
        include_str!("fixtures/explicit-mixed/expected-native-properties.json"),
        include_str!("fixtures/explicit-mixed/expected-native-properties-227.json"),
        "native-properties.php",
        47,
    );
}

#[test]
fn ordinary_and_constructed_receivers_use_their_own_native_property_and_method_reflection() {
    check_fixture(
        include_bytes!("fixtures/explicit-mixed/native-receiver-controls.php"),
        include_str!("fixtures/explicit-mixed/expected-native-receiver-controls.json"),
        include_str!("fixtures/explicit-mixed/expected-native-receiver-controls-227.json"),
        "native-receiver-controls.php",
        7,
    );
}

fn check_fixture(source: &[u8], expected: &str, other: &str, file: &str, count: usize) {
    let expected: Value = serde_json::from_str(expected).unwrap();
    assert_eq!(expected, serde_json::from_str::<Value>(other).unwrap());
    let (enabled, issues) = analyze(source, true);
    let (disabled, off_issues) = analyze(source, false);
    assert_eq!(issues, off_issues);
    assert_eq!(enabled.expression_types, disabled.expression_types);
    assert_eq!(
        enabled.inferred_return_types,
        disabled.inferred_return_types
    );
    let mut failures = Vec::new();
    let captures = expected[file].as_object().unwrap();
    for (offset, capture) in captures {
        let start = offset.parse::<u32>().unwrap();
        let expression = capture["expression"].as_str().unwrap();
        let span = Span::new(
            mago_database::file::FileId::zero(),
            Position::new(start),
            Position::new(start + expression.len() as u32),
        );
        let full = enabled.get_expression_type(&span).unwrap();
        assert_eq!(
            identity(full),
            capture["full"]["explicit"],
            "full line {} {expression}",
            capture["line"]
        );
        assert_eq!(full.get_id().as_str(), capture["full"]["type"].as_str());
        let Some(native) = enabled.get_native_expression_type(&span) else {
            failures.push(format!(
                "line {} {expression}: native observation unavailable",
                capture["line"]
            ));
            continue;
        };
        assert_eq!(
            identity(native),
            capture["native"]["explicit"],
            "native line {} {expression}: {native:?}",
            capture["line"]
        );
        assert_eq!(native.get_id().as_str(), capture["native"]["type"].as_str());
        assert!(disabled.get_native_expression_type(&span).is_none());
    }
    assert_eq!(captures.len(), count);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn expression_span(source: &str, expression: &str) -> Span {
    let start = source.rfind(expression).unwrap() as u32;
    Span::new(
        mago_database::file::FileId::zero(),
        Position::new(start),
        Position::new(start + expression.len() as u32),
    )
}

#[test]
fn unsupported_native_receivers_members_and_reference_writes_stay_unavailable() {
    for expression in [
        "$this->$selector",
        "$this->$selector()",
        "$this->magic",
        "$this->annotated",
        "$this->missing",
        "$this->missing()",
        "$this->items",
        "$this->hooked",
        "$this->staticProp",
        "$this->arrayReturn()",
        "$this->required(1)",
        "$this->required()",
        "$this->reference()",
        "$this->prop[$flag]",
        "new Constructor()",
        "new Generic()",
        "new Unknown()",
        "new $selector()",
        "new Simple(1)",
        "$this->prop = 1",
        "$copy = 1",
        "$alias = 1",
        "$callable",
    ] {
        let source = format!("<?php
class Constructor {{ public function __construct() {{}} }}
/** @template T */ class Generic {{}}
class Simple {{}}
/**
 * @property mixed $magic
 * @property int $annotated
 */
class NativeBoundary {{
    public mixed $prop;
    public mixed $annotated;
    public array $items;
    public mixed $hooked {{ get => null; }}
    public static mixed $staticProp;
    public function arrayReturn(): array {{ return []; }}
    public function required(int $value): mixed {{ return $value; }}
    public function &reference(): mixed {{ return $this->prop; }}
    public function example(string $selector, bool $flag): void {{
        $copy = $this->prop;
        $alias =& $copy;
        $callable = fn () => $this->prop;
        $this->prop; // before
        {expression}; // unsupported
        $this->unavailable; // after
    }}
}}
");
        let (enabled, issues) = analyze(source.as_bytes(), true);
        let (disabled, off_issues) = analyze(source.as_bytes(), false);
        assert_eq!(issues, off_issues, "{expression}");
        assert_eq!(enabled.expression_types, disabled.expression_types, "{expression}");
        let before = expression_span(&source, "$this->prop; // before");
        let before = Span::new(before.file_id, before.start, Position::new(before.start.offset + 11));
        assert_eq!(identity(enabled.get_native_expression_type(&before).unwrap()), Value::Bool(false));
        for unavailable in [expression, "$this->unavailable"] {
            let span = expression_span(&source, unavailable);
            assert!(enabled.get_native_expression_type(&span).is_none(), "{expression}: {unavailable}");
        }
    }
}

#[test]
fn closure_creation_keeps_only_supported_outer_native_continuations() {
    for (body, reference, available) in [
        ("$copy;", "", true),
        ("unknown();", "", true),
        ("$copy;", "&", true),
        ("unknown();", "&", true),
        ("$copy = 1;", "&", true),
    ] {
        let source = format!("<?php class CaptureBoundary {{
    public mixed $prop;
    public function example(): void {{
        $copy = $this->prop;
        $callable = function () use ({reference}$copy) {{ {body} }};
        $this->prop;
    }}
}}");
        let (enabled, issues) = analyze(source.as_bytes(), true);
        let (disabled, off_issues) = analyze(source.as_bytes(), false);
        assert_eq!(issues, off_issues);
        assert_eq!(enabled.expression_types, disabled.expression_types);
        let native = enabled.get_native_expression_type(&expression_span(&source, "$this->prop"));
        assert_eq!(native.is_some(), available, "{reference} {body}");
        if let Some(native) = native {
            assert_eq!(identity(native), Value::Bool(false));
        }
    }
}
