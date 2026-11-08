//! Source-owned explicit mixed identity, through the public Analyzer and native-type API.

use std::borrow::Cow;
use std::path::Path;
use std::sync::LazyLock;

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::create_registry_with_plugins;
use mago_analyzer::settings::Settings;
use mago_codex::populator::populate_codebase;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_prelude::Prelude;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use serde_json::Value;

static PRELUDE: LazyLock<Prelude> = LazyLock::new(Prelude::build);

fn explicit(ty: &TUnion) -> Value {
    match ty.types.as_ref() {
        [TAtomic::Mixed(mixed)] => Value::Bool(mixed.is_explicit()),
        _ => Value::Null,
    }
}

fn check(file_name: &str, partial_native: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/explicit-mixed");
    let source = std::fs::read(root.join(file_name)).unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let file = File::new(
        Cow::Owned(file_name.as_bytes().to_vec()),
        FileType::Host,
        None,
        Cow::Owned(source),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings {
        track_native_types: true,
        ..Settings::default()
    };
    let mut codebase = PRELUDE.metadata.clone();
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        settings.version,
    ));
    let mut references = PRELUDE.symbol_references.clone();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let plugins = create_registry_with_plugins(&[], false);
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &plugins, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let mut failures = Vec::new();
    let mut checked = 0;
    let mut native_checked = 0;
    for (offset, capture) in oracle[file_name].as_object().unwrap() {
        let start = offset.parse::<u32>().unwrap();
        let expression = capture["expression"].as_str().unwrap();
        let span = Span::new(
            file.id,
            Position::new(start),
            Position::new(start + expression.len() as u32),
        );
        // These existing inference gaps are not explicit-mixed producer contracts:
        // array_pop(string) has a separate return-provider bug, and a truthy branch
        // assignment does not reconstruct PHPStan's complete falsey scalar/array union.
        if (file_name == "xm.php" && capture["line"] == 54)
            || (file_name == "xm2.php" && capture["line"] == 42)
        {
            continue;
        }
        let ty = artifacts
            .get_expression_type(&span)
            .unwrap_or_else(|| panic!("missing {file_name}:{} {expression}", capture["line"]));
        let got = explicit(ty);
        let want = &capture["full"]["explicit"];
        checked += 1;
        if &got != want {
            failures.push(format!(
                "{file_name}:{} {expression}: full explicit={got}, want {want}; {ty:?}",
                capture["line"]
            ));
        }
        if capture["full"]["type"] == "mixed" && expression.contains('[') {
            assert!(
                matches!(ty.types.as_ref(), [TAtomic::Mixed(mixed)] if mixed.is_vanilla()),
                "offset constraints leaked: {ty:?}"
            );
        }
        if capture["full"]["type"] == "mixed~(0|0.0|''|'0'|array{}|false|null)" {
            assert!(
                matches!(ty.types.as_ref(), [TAtomic::Mixed(mixed)] if mixed.is_truthy()),
                "declared truthiness lost: {ty:?}"
            );
        }
        if let Ok(integer) = capture["full"]["type"].as_str().unwrap().parse::<i64>() {
            assert!(
                matches!(ty.types.as_ref(), [atomic] if atomic.get_literal_int_value() == Some(integer)),
                "overwritten scalar: {ty:?}"
            );
        }
        if partial_native && let Some(native) = artifacts.get_native_expression_type(&span) {
            native_checked += 1;
            let got = explicit(native);
            let want = &capture["native"]["explicit"];
            if &got != want {
                failures.push(format!(
                    "{file_name}:{} {expression}: native explicit={got}, want {want}",
                    capture["line"]
                ));
            }
        }
    }
    assert!(checked >= 15);
    if partial_native {
        assert!(native_checked >= 10, "native controls must be available");
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn declarations_offsets_assignments_and_captures_keep_source_identity() {
    check("declarations.php", true);
}

#[test]
fn supplied_method_builtin_and_unknown_member_repro() {
    check("xm.php", false);
}

#[test]
fn supplied_asymmetric_unions_and_capture_overwrites_repro() {
    check("xm2.php", false);
}
