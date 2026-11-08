//! The bounded getPlusType mixed/mixed branch, through the public Analyzer.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts,
    observations::CallScopeRequest, plugin::PluginRegistry, settings::Settings,
};
use mago_codex::{
    metadata::ttype::TypeMetadata,
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{TAtomic, array::TArray, mixed::TMixed},
        get_empty_keyed_array, get_error, get_float, get_int, get_keyed_array, get_mixed,
        get_mixed_iterable, get_never, get_null, get_string,
        union::TUnion,
    },
};
use mago_database::file::{File, FileId};
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;
use mago_word::{empty_word, word};
use pun_analyse::{Project, run};
use serde_json::{Value, json};

fn oracle() -> Value {
    serde_json::from_str(include_str!("fixtures/mixed-addition/expected.json")).unwrap()
}

fn input(label: &str) -> TUnion {
    let array = || get_keyed_array(get_mixed(), get_mixed());
    let excluded = match label {
        "mixed" => return get_mixed(),
        "explicit" => {
            return TUnion::from_atomic(TAtomic::Mixed(TMixed::new().with_is_explicit(true)));
        }
        "error" => return get_error(),
        "int" => return get_int(),
        "float" => return get_float(),
        "empty-array" => return get_empty_keyed_array(),
        "never" => return get_never(),
        "not-array" => array(),
        "not-iterable" => get_mixed_iterable(),
        "not-array-null" => {
            let mut types = array().types.into_owned();
            types.extend(get_null().types.into_owned());
            TUnion::from_vec(types)
        }
        "not-empty-array" => get_empty_keyed_array(),
        "not-int-array" => get_keyed_array(get_int(), get_mixed()),
        "not-nonempty-array" => {
            let TAtomic::Array(TArray::Keyed(array)) = array().get_single().clone() else {
                unreachable!()
            };
            TUnion::from_atomic(TAtomic::Array(TArray::Keyed(array.with_non_empty(true))))
        }
        "not-string" => get_string(),
        _ => panic!("unknown oracle input {label}"),
    };
    TUnion::from_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(excluded)))
}

fn span(source: &str, expression: &str) -> Span {
    let start = source.find(expression).unwrap();
    Span::new(
        FileId::new(b"addition.php"),
        Position::new(start as u32),
        Position::new((start + expression.len()) as u32),
    )
}

fn analyze(
    source: &str,
    left: TUnion,
    right: TUnion,
    native: bool,
    observations: bool,
) -> (AnalysisArtifacts, AnalysisResult) {
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"addition.php"),
        Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
    // Exercise the analyzer independently of guard producers and PHPDoc's type-string grammar.
    // These are public reflection inputs, mirroring the oracle getPlusType callback's types.
    let function = codebase
        .function_likes
        .get_mut(&(empty_word(), word("example")))
        .unwrap();
    for (parameter, ty) in function.parameters.iter_mut().zip([left, right]) {
        parameter.type_metadata = Some(TypeMetadata::new(ty, parameter.span));
    }
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut settings = Settings {
        track_native_types: native,
        ..Default::default()
    };
    if observations {
        settings.call_scope_observations.push(CallScopeRequest {
            call: span(
                source,
                "$sink->take(($leftCopy = $left) + ($rightCopy = $right))",
            ),
            expressions: vec![span(source, "($leftCopy = $left) + ($rightCopy = $right)")],
        });
    }
    let registry = PluginRegistry::new();
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    (artifacts, result)
}

// Adapt only presentation: PHPStan wraps benevolent unions and spells the unconstrained
// array as `array`; Mago's canonical ID displays its explicit key/value parameters.
fn description(ty: &TUnion) -> String {
    let mut members: Vec<_> = ty
        .types
        .iter()
        .map(|atomic| match atomic {
            TAtomic::Array(TArray::Keyed(array))
                if array.known_items.is_none()
                    && array
                        .parameters
                        .as_ref()
                        .is_some_and(|(key, value)| key.is_array_key() && value.is_mixed()) =>
            {
                "array".to_string()
            }
            TAtomic::Array(array) if array.is_empty() => "array{}".to_string(),
            TAtomic::Never => "*NEVER*".to_string(),
            _ => atomic.get_id().to_string(),
        })
        .collect();
    members.sort();
    let result = members.join("|");
    if ty.ignore_nullable_issues() && ty.ignore_falsable_issues() {
        format!("({result})")
    } else {
        result
    }
}

#[test]
fn public_analyzer_mixed_addition_matches_phpstan_in_all_observation_modes() {
    let source = "<?php
class Sink { public function take(mixed $value): void {} }
function example(mixed $left, mixed $right): void {
    $sink = new Sink();
    $sink->take(($leftCopy = $left) + ($rightCopy = $right));
    $leftCopy;
    $rightCopy;
}
";
    let binary = span(source, "($leftCopy = $left) + ($rightCopy = $right)");
    for row in oracle()["plus"].as_array().unwrap() {
        let left = input(row["left"].as_str().unwrap());
        let right = input(row["right"].as_str().unwrap());
        for native in [false, true] {
            for observations in [false, true] {
                let (artifacts, result) =
                    analyze(source, left.clone(), right.clone(), native, observations);
                let ty = artifacts.get_expression_type(&binary).unwrap();
                assert_eq!(
                    description(ty),
                    row["type"],
                    "{row}, native={native}, observations={observations}"
                );
                assert_eq!(
                    ty.ignore_nullable_issues() && ty.ignore_falsable_issues(),
                    row["benevolent"]
                );
                // The native lane cannot price mixed arithmetic, and must not borrow full types.
                assert!(artifacts.get_native_expression_type(&binary).is_none());
                if !left.is_never() && !right.is_never() {
                    // Both assignments must really execute, even when the result is benevolent.
                    for (expression, expected) in [("$leftCopy;", &left), ("$rightCopy;", &right)] {
                        let mut read = span(source, expression);
                        read.end.offset -= 1;
                        assert_eq!(artifacts.get_expression_type(&read).unwrap(), expected);
                    }
                    let codes: Vec<_> = result
                        .issues
                        .iter()
                        .filter_map(|issue| issue.code.as_deref())
                        .collect();
                    assert!(!codes.contains(&"mixed-operand"), "{row}: {codes:?}");
                }
                if observations && !left.is_never() && !right.is_never() {
                    let scopes = artifacts
                        .call_scope_observations
                        .get(&span(
                            source,
                            "$sink->take(($leftCopy = $left) + ($rightCopy = $right))",
                        ))
                        .unwrap();
                    assert_eq!(
                        description(scopes[0].types.get(&binary).unwrap()),
                        row["type"]
                    );
                } else if !observations {
                    assert!(artifacts.call_scope_observations.is_empty());
                }
            }
        }
    }
}

#[test]
fn public_analyzer_benevolent_addition_has_no_mixed_operand_or_return_cascade() {
    let source = "<?php function example(mixed $left, mixed $right): array|float|int { return $left + $right; }";
    for row in oracle()["plus"].as_array().unwrap() {
        if row["benevolent"] != true {
            continue;
        }
        let (artifacts, result) = analyze(
            source,
            input(row["left"].as_str().unwrap()),
            input(row["right"].as_str().unwrap()),
            false,
            false,
        );
        // Every member captured from PHPStan is accepted by this declared return union.
        assert!(result.issues.is_empty(), "{row}: {:?}", result.issues);
        assert_eq!(
            description(
                artifacts
                    .get_expression_type(&span(source, "$left + $right"))
                    .unwrap()
            ),
            row["type"]
        );
    }
}

#[test]
fn frontend_addition_preserves_complete_phpstan_diagnostics_including_level_ten() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mixed-addition");
    for level in [0, 8, 9, 10] {
        let mut project = Project::new(vec![root.join("frontend.php")], vec![]);
        project.level = level;
        project.php_version = 80500;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                json!([
                    Path::new(&error.file)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    error.line,
                    error.identifier,
                    error.message,
                    error.tip,
                    error.can_be_ignored,
                ])
            })
            .collect();
        got.sort_by_key(Value::to_string);
        let mut expected = oracle()["diagnostics"][level.to_string()]
            .as_array()
            .unwrap()
            .clone();
        expected.sort_by_key(Value::to_string);
        assert_eq!(got, expected, "level {level}");
    }
}

#[test]
fn mixed_subtraction_and_multiplication_are_benevolent_numbers() {
    // PHPStan 2.3.1 dumps `(float|int)` for `mixed - mixed` and `mixed * mixed` (resolveCommonMath()).
    let mut number = TUnion::from_vec(vec![
        get_int().types[0].clone(),
        get_float().types[0].clone(),
    ]);
    number.set_ignore_nullable_issues(true);
    number.set_ignore_falsable_issues(true);
    for operator in ["-", "*"] {
        let source = format!(
            "<?php function example(mixed $left, mixed $right): mixed {{ return $left {operator} $right; }}"
        );
        let (artifacts, result) = analyze(&source, get_mixed(), get_mixed(), false, false);
        let binary = span(&source, &format!("$left {operator} $right"));
        assert_eq!(artifacts.get_expression_type(&binary).unwrap(), &number);
        // This pre-existing native diagnostic remains outside the bounded plus port.
        assert_eq!(
            result
                .issues
                .iter()
                .filter(|issue| issue.code.as_deref() == Some("mixed-operand"))
                .count(),
            2
        );
    }
}
