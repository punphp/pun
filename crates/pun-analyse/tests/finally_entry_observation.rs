//! Finally inputs are actual branch ends, throw points, and abrupt-exit scopes.

use std::borrow::Cow;
use std::collections::BTreeMap;

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::settings::Settings;
use mago_codex::ttype::TType;
use mago_codex::{populator::populate_codebase, scanner::scan_program};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::Span;
use mago_syntax::parser::parse_file;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const INPUT: &[u8] = include_bytes!("fixtures/finally-entry-observation/input.php");

fn canonical(value: &str) -> String {
    let mut members: Vec<_> = value
        .split('|')
        .map(|v| {
            if v.parse::<i64>().is_ok() {
                format!("int({v})")
            } else {
                v.to_ascii_lowercase()
            }
        })
        .collect();
    members.sort();
    members.join("|")
}

#[test]
fn actual_finally_entries_and_continuations_match_complete_phpstan_records() {
    let expected: Value = serde_json::from_slice(include_bytes!(
        "fixtures/finally-entry-observation/phpstan-2.3.1.json"
    ))
    .unwrap();
    assert_eq!(
        Sha256::digest(INPUT)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>(),
        expected["input_sha256"]
    );
    let mut failures = Vec::new();
    for native in [false, true] {
        let arena = LocalArena::new();
        let file = File::ephemeral(Cow::Borrowed(b"input.php"), Cow::Borrowed(INPUT));
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let settings = Settings {
            track_native_types: native,
            ..Default::default()
        };
        let prelude = mago_prelude::Prelude::build();
        let mut codebase = prelude.metadata;
        codebase.extend(scan_program(
            &arena,
            &file,
            program,
            &names,
            settings.version,
        ));
        let mut references = prelude.symbol_references;
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let plugins = PluginRegistry::new();
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &plugins, settings);
        let mut result = AnalysisResult::new(references);
        let artifacts = analyzer
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        let mut actual = BTreeMap::new();
        let mut want = BTreeMap::new();
        for (name, record) in expected["records"].as_object().unwrap() {
            let span = Span::dummy(
                record["start"].as_u64().unwrap() as u32,
                record["end"].as_u64().unwrap() as u32,
            );
            actual.insert(
                name,
                artifacts
                    .get_expression_type(&span)
                    .map(|ty| canonical(&ty.get_id().to_string())),
            );
            want.insert(name, Some(canonical(record["full"].as_str().unwrap())));
            assert!(
                artifacts.get_native_expression_type(&span).is_none(),
                "native {name}"
            );
        }
        println!(
            "finally-entry-records={}",
            json!({"native":native,"actual":actual,"expected":want})
        );
        if actual != want {
            failures.push(json!({"native":native,"actual":actual,"expected":want}));
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}

#[test]
fn default_frontend_preserves_complete_ordinary_diagnostics() {
    let expected: Value = serde_json::from_slice(include_bytes!(
        "fixtures/finally-entry-observation/phpstan-2.3.1.json"
    ))
    .unwrap();
    let mut project = pun_analyse::Project::new(
        vec![
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/finally-entry-observation/input.php"),
        ],
        Vec::new(),
    );
    project.php_version = 80500;
    let result = pun_analyse::analyse_project(&project).unwrap();
    let actual: Vec<_> = result
        .errors
        .into_iter()
        .map(|e| json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
        .collect();
    assert_eq!(json!(actual), expected["errors"]);
    assert!(result.general.is_empty(), "{:?}", result.general);
}
