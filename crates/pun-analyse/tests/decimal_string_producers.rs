//! Decimal-string producers and the native lane, against PHPStan 2.3.1.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program, ttype::TType,
};
use mago_database::file::{File, FileId, FileType};
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn decimal_string_producers_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/decimal-string-producers");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    for level in [0, 4, 9] {
        let mut project = Project::new(vec![root.join("producers.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| {
                json!([
                    Path::new(&e.file).file_name().unwrap().to_str().unwrap(),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored
                ])
            })
            .collect();
        got.sort_by_key(Value::to_string);
        let mut want = expected[format!("producers-{level}")]
            .as_array()
            .unwrap()
            .clone();
        want.sort_by_key(Value::to_string);
        assert_eq!(got, want, "level {level}");
    }
}

#[test]
fn native_integer_strings_do_not_use_phpdoc_refinements_or_change_full_results() {
    let source = include_bytes!("fixtures/decimal-string-producers/native.php");
    let expected: Value = serde_json::from_str(include_str!(
        "fixtures/decimal-string-producers/expected.json"
    ))
    .unwrap();
    let file = File::new(
        Cow::Borrowed(b"native.php"),
        FileType::Host,
        None,
        Cow::Borrowed(source),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let registry = PluginRegistry::new();
    let analyze = |track_native_types| {
        let settings = Settings {
            track_native_types,
            ..Default::default()
        };
        let mut result = AnalysisResult::new(SymbolReferences::new());
        let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        (artifacts, result)
    };
    let (enabled, on_result) = analyze(true);
    let (disabled, off_result) = analyze(false);
    assert_eq!(enabled.expression_types, disabled.expression_types);
    assert_eq!(
        format!("{:?}", on_result.issues),
        format!("{:?}", off_result.issues)
    );
    let mut offset = 0;
    let mut got = Vec::new();
    for (line, text) in std::str::from_utf8(source)
        .unwrap()
        .split_inclusive('\n')
        .enumerate()
    {
        if let Some((expression, _)) = text.split_once("// native") {
            let expression = expression.trim_end().strip_suffix(';').unwrap();
            let span = Span::new(
                FileId::zero(),
                Position::new((offset + 4) as u32),
                Position::new((offset + expression.len()) as u32),
            );
            // Adapt only each atomic literal's display syntax and PHPStan's union order.
            let mut members: Vec<_> = enabled
                .get_native_expression_type(&span)
                .unwrap()
                .types
                .iter()
                .map(|atomic| {
                    let id = atomic.get_id().to_string();
                    id.strip_prefix("string(")
                        .and_then(|s| s.strip_suffix(')'))
                        .unwrap_or(&id)
                        .to_string()
                })
                .collect();
            members.sort();
            let actual = members.join("|");
            got.push(json!([
                "native.php",
                line + 1,
                "phpstan.dumpNativeType",
                format!("Dumped type: {actual}"),
                null,
                false
            ]));
            assert!(disabled.get_native_expression_type(&span).is_none());
        }
        offset += text.len();
    }
    assert_eq!(got, expected["native-0"].as_array().unwrap().clone());
}

#[test]
fn decimal_string_metadata_and_intersections_match_the_public_type_api() {
    use mago_codex::ttype::atomic::{
        TAtomic,
        scalar::{TScalar, string::TString},
    };
    use mago_codex::ttype::comparator::{ComparisonResult, atomic_comparator};
    use pun_phpstan::{
        resolve::resolve_str,
        types::{Level, Type},
    };

    let expected: Value =
        serde_json::from_str(include_str!("fixtures/decimal-string-producers/types.json")).unwrap();
    let rows = expected["types"].as_array().unwrap();
    let mut code = String::from("<?php\n");
    for (i, row) in rows.iter().enumerate() {
        code.push_str(&format!(
            "/** @param {} $value */\nfunction slot{i}($value): void {{}}\n",
            row["input"].as_str().unwrap()
        ));
    }
    let file = File::ephemeral(
        Cow::Borrowed(b"metadata.php"),
        Cow::Owned(code.into_bytes()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let description = |string: &TString| {
        string
            .get_known_literal_value()
            .map_or_else(
                || resolve_str(&string.get_id().to_string(), None),
                |value| Type::ConstString(value.to_vec()),
            )
            .describe(Level::Precise)
    };
    let mut strings = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let function = codebase
            .get_function(format!("slot{i}").as_bytes())
            .unwrap();
        let ty = &function.parameters[0]
            .get_type_metadata()
            .unwrap()
            .type_union;
        let TAtomic::Scalar(TScalar::String(string)) = ty.get_single() else {
            panic!("{}: {ty:?}", row["input"]);
        };
        assert_eq!(
            description(string),
            row["describe"].as_str().unwrap(),
            "{}",
            row["input"]
        );
        assert_eq!(
            json!({
                "decimal": string.is_decimal_integer().map_or("Maybe", |yes| if yes { "Yes" } else { "No" }),
                "numeric": string.is_known_numeric(), "nonempty": string.is_non_empty(),
                "lower": string.is_lowercase(), "upper": string.is_uppercase(),
            }),
            json!({"decimal":row["decimal"], "numeric":row["numeric"], "nonempty":row["nonempty"],
            "lower":row["lower"], "upper":row["upper"]}),
            "{}",
            row["input"]
        );
        strings.push(*string);
    }
    for row in expected["intersections"].as_array().unwrap() {
        let left = &strings[row[0].as_u64().unwrap() as usize];
        let right = &strings[row[1].as_u64().unwrap() as usize];
        assert_eq!(
            left.intersect(right)
                .map_or_else(|| "*NEVER*".into(), |s| description(&s)),
            row[2].as_str().unwrap(),
            "{row}"
        );
    }
    for row in expected["contains"].as_array().unwrap() {
        let container =
            TAtomic::Scalar(TScalar::String(strings[row[0].as_u64().unwrap() as usize]));
        let input = TAtomic::Scalar(TScalar::String(strings[row[1].as_u64().unwrap() as usize]));
        assert_eq!(
            atomic_comparator::is_contained_by(
                &codebase,
                &input,
                &container,
                true,
                &mut ComparisonResult::new()
            ),
            row[2].as_bool().unwrap(),
            "{row}"
        );
    }
    // Assertion refinement retains the inverse decimal accessory under an independent numeric guarantee.
    assert_eq!(
        description(&strings[2].as_numeric(false)),
        rows[6]["describe"].as_str().unwrap()
    );
}
