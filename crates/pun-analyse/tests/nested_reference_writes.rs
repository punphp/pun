//! Nested reference writes are checked against PHPStan 2.3.1, not comparison workarounds.
use std::{
    borrow::Cow,
    collections::BTreeMap,
    path::Path,
    sync::{Arc, Mutex},
};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer,
    analysis_result::AnalysisResult,
    observations::CallScopeRequest,
    plugin::create_registry_with_plugins,
    plugin::{ExpressionHook, HookContext, HookResult},
    settings::Settings,
};
use mago_codex::{
    populator::populate_codebase,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            array::{TArray, key::ArrayKey},
            scalar::{TScalar, int::TInteger, string::TStringLiteral},
        },
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{cst::Expression, parser::parse_file};
use pun_analyse::{Project, run};
use serde_json::Value;

// Exercise the frontend's read-reference policy through the public hook API too.
struct ReadInputReferences;
impl ExpressionHook for ReadInputReferences {
    fn before_arguments(
        &self,
        invocation: &mago_analyzer::plugin::InvocationInfo<'_, '_, '_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(
            invocation.function_name().as_str(),
            "array_push" | "array_unshift"
        ) {
            let argument = invocation.get_argument(0, &[b"array"]).unwrap();
            context
                .block_context_mut()
                .variable_state
                .non_creating_references
                .insert(argument.span());
        }
        Ok(())
    }
}

#[test]
fn original_bug3387_matches_the_complete_frozen_public_case() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value =
        serde_json::from_slice(
            &std::fs::read(root.join(
                "levels/4/Comparison/NumberComparisonOperatorsConstantConditionRuleTest.json",
            ))
            .unwrap(),
        )
        .unwrap();
    let expected = expected["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == "testBug3387")
        .unwrap();
    let path = root.join("rules/data/Comparison/data/bug-3387.php");
    let mut project = Project::new(vec![path], Vec::new());
    project.level = 4;
    let actual: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message, error.tip))
        .collect();
    let mut expected: Vec<_> = expected["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
            )
        })
        .collect();
    expected.sort_by(|a, b| (a.0, &a.2).cmp(&(b.0, &b.2)));
    assert_eq!(actual, expected);
}

// Only presentation differs: recursively retain every array key, optional flag,
// generic bound and union member while sorting unordered type alternatives.
fn canonical(text: &str) -> String {
    fn split(text: &str, separator: u8) -> Vec<&str> {
        let (mut start, mut depth, mut quoted) = (0, 0, false);
        let mut parts = Vec::new();
        for (index, byte) in text.bytes().enumerate() {
            if byte == b'\'' {
                quoted = !quoted;
            }
            if quoted {
                continue;
            }
            match byte {
                b'{' | b'<' | b'(' => depth += 1,
                b'}' | b'>' | b')' => depth -= 1,
                byte if byte == separator && depth == 0 => {
                    parts.push(text[start..index].trim());
                    start = index + 1;
                }
                _ => {}
            }
        }
        parts.push(text[start..].trim());
        parts
    }
    let union = split(text, b'|');
    if union.len() > 1 {
        let mut parts: Vec<_> = union.into_iter().map(canonical).collect();
        parts.sort();
        return parts.join("|");
    }
    if let Some(inner) = text
        .strip_prefix("array{")
        .and_then(|s| s.strip_suffix('}'))
    {
        let mut fields: Vec<_> = split(inner, b',')
            .into_iter()
            .map(|field| {
                field.split_once(": ").map_or_else(
                    || canonical(field),
                    |(key, value)| format!("{key}: {}", canonical(value)),
                )
            })
            .collect();
        fields.sort();
        return format!("array{{{}}}", fields.join(", "));
    }
    if let Some((name, inner)) = text.split_once('<')
        && text.ends_with('>')
    {
        return format!(
            "{name}<{}>",
            split(&inner[..inner.len() - 1], b',')
                .into_iter()
                .map(canonical)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    text.to_owned()
}

fn describe(ty: &TUnion) -> String {
    canonical(
        &ty.types
            .iter()
            .map(|atomic| match atomic {
                TAtomic::Scalar(TScalar::Integer(TInteger::Literal(value))) => value.to_string(),
                TAtomic::Scalar(TScalar::String(value))
                    if matches!(value.literal, Some(TStringLiteral::Value(_))) =>
                {
                    let Some(TStringLiteral::Value(value)) = value.literal else {
                        unreachable!()
                    };
                    format!("'{value}'")
                }
                TAtomic::Array(array) if array.is_empty() => "array{}".to_owned(),
                TAtomic::Array(TArray::List(list)) if list.known_elements.is_some() => {
                    let mut fields: Vec<_> = list
                        .known_elements
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(index, (optional, value))| {
                            format!(
                                "{index}{}: {}",
                                if *optional { "?" } else { "" },
                                describe(value)
                            )
                        })
                        .collect();
                    if !list.element_type.is_never() {
                        fields.push(format!("...<{}>", describe(&list.element_type)));
                    }
                    format!("array{{{}}}", fields.join(", "))
                }
                TAtomic::Array(TArray::List(list)) => format!(
                    "{}list{}",
                    if list.non_empty { "non-empty-" } else { "" },
                    if list.element_type.is_mixed() {
                        String::new()
                    } else {
                        format!("<{}>", describe(&list.element_type))
                    }
                ),
                TAtomic::Array(TArray::Keyed(array)) if array.known_items.is_some() => {
                    let mut fields: Vec<_> = array
                        .known_items
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(key, (optional, value))| {
                            let key = match key {
                                ArrayKey::String(key) => key.to_string(),
                                ArrayKey::Integer(key) => key.to_string(),
                                _ => panic!("unexpected symbolic array key"),
                            };
                            format!(
                                "{key}{}: {}",
                                if *optional { "?" } else { "" },
                                describe(value)
                            )
                        })
                        .collect();
                    if let Some((key, value)) = &array.parameters {
                        fields.push(format!("...<{}, {}>", describe(key), describe(value)));
                    }
                    format!("array{{{}}}", fields.join(", "))
                }
                TAtomic::Array(TArray::Keyed(array)) => {
                    let parameters = array
                        .parameters
                        .as_ref()
                        .map(|(key, value)| format!("<{}, {}>", describe(key), describe(value)))
                        .unwrap_or_default();
                    format!(
                        "{}array{parameters}",
                        if array.non_empty { "non-empty-" } else { "" }
                    )
                }
                _ => atomic.get_id().to_string(),
            })
            .collect::<Vec<_>>()
            .join("|"),
    )
}

#[derive(Clone, Default)]
struct Visits(Arc<Mutex<BTreeMap<(u32, u32), usize>>>);
impl ExpressionHook for Visits {
    fn is_observer(&self) -> bool {
        true
    }
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<mago_analyzer::plugin::ExpressionHookResult> {
        let span = expression.span();
        *self
            .0
            .lock()
            .unwrap()
            .entry((span.start.offset, span.end.offset))
            .or_default() += 1;
        Ok(mago_analyzer::plugin::ExpressionHookResult::Continue)
    }
}

#[test]
fn nested_reference_contract_preserves_values_results_and_observations() {
    let source = include_bytes!("fixtures/nested-reference-writes/contract.php");
    let oracle: Value = serde_json::from_slice(include_bytes!(
        "fixtures/nested-reference-writes/contract-types.json"
    ))
    .unwrap();
    let mut failures = Vec::new();
    for native in [false, true] {
        for observe in [false, true] {
            let arena = LocalArena::new();
            let file = Arc::new(File::ephemeral(
                Cow::Borrowed(b"contract.php"),
                Cow::Borrowed(source),
            ));
            let program = parse_file(&arena, &file);
            assert!(!program.has_errors());
            let names = NameResolver::new(&arena).resolve(program);
            let prelude = mago_prelude::Prelude::build();
            let mut codebase = prelude.metadata;
            codebase.extend(scan_program(
                &arena,
                &file,
                program,
                &names,
                Default::default(),
            ));
            let mut references = prelude.symbol_references;
            populate_codebase(
                &mut codebase,
                &mut references,
                Default::default(),
                Default::default(),
            );
            let span_of = |entry: &Value| {
                Span::new(
                    file.id,
                    Position::new(entry["start"].as_u64().unwrap() as u32),
                    Position::new(entry["end"].as_u64().unwrap() as u32),
                )
            };
            let mut settings = Settings {
                track_native_types: native,
                ..Settings::default()
            };
            if observe {
                for call in oracle.as_object().unwrap().values().filter(|entry| {
                    entry["expression"]
                        .as_str()
                        .unwrap()
                        .starts_with("$probe->record(")
                }) {
                    let span = span_of(call);
                    let expressions = oracle
                        .as_object()
                        .unwrap()
                        .values()
                        .map(&span_of)
                        .filter(|expression| {
                            expression.start.offset > span.start.offset
                                && expression.end.offset < span.end.offset
                        })
                        .collect();
                    settings.call_scope_observations.push(CallScopeRequest {
                        call: span,
                        expressions,
                    });
                }
            }
            let requests = settings.call_scope_observations.clone();
            let visits = Visits::default();
            let mut registry = create_registry_with_plugins(&[], false);
            registry.register_expression_hook(visits.clone());
            registry.register_expression_hook(ReadInputReferences);
            let mut result = AnalysisResult::new(references);
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
            for (position, expected) in oracle.as_object().unwrap() {
                let span = span_of(expected);
                let full = artifacts.get_expression_type(&span).map(describe);
                let raw = artifacts.get_raw_expression_type(&span).map(describe);
                let keep_void = artifacts.get_keep_void_expression_type(&span).map(describe);
                let native_type = artifacts.get_native_expression_type(&span).map(describe);
                let expected_type = canonical(expected["full"].as_str().unwrap());
                let expected_raw = canonical(expected["keep_void"].as_str().unwrap());
                eprintln!(
                    "native={native} observe={observe} {position} {} full={full:?} raw={raw:?} keep_void={keep_void:?} native={native_type:?}; PHPStan {expected_type} raw={expected_raw}",
                    expected["expression"]
                );
                if full.as_deref() != Some(expected_type.as_str())
                    || raw.as_deref() != Some(expected_raw.as_str())
                    || keep_void.as_deref() != Some(expected_raw.as_str())
                {
                    failures.push(format!("native={native} observe={observe} {position} {} full={full:?}, raw={raw:?}, keep_void={keep_void:?}; PHPStan {expected_type}, {expected_raw}", expected["expression"]));
                }
                let end = span.end.offset as usize;
                let line_end = source[end..]
                    .iter()
                    .position(|byte| *byte == b'\n')
                    .map_or(source.len(), |offset| end + offset);
                // The marker may sit inside the expression (`identity($copy /* probe-native */)`) or after it.
                let native_available = native
                    && String::from_utf8_lossy(&source[span.start.offset as usize..line_end])
                        .contains("probe-native");
                let expected_native =
                    native_available.then(|| canonical(expected["native"].as_str().unwrap()));
                if native_type != expected_native {
                    failures.push(format!("native availability {native}/{observe} {position}: {native_type:?} != {expected_native:?}"));
                }
            }
            assert_eq!(artifacts.call_scope_observations.len(), requests.len());
            for request in requests {
                let call = request.call;
                let scopes = artifacts
                    .call_scope_observations
                    .get(&call)
                    .expect("requested call");
                assert_eq!(scopes.len(), 1);
                assert!(!scopes[0].filtered);
                for span in request.expressions {
                    let key = format!("{}:{}", span.start.offset, span.end.offset);
                    let expected = &oracle[&key];
                    let actual = scopes[0].types.get(&span).map(describe);
                    let expected_type = canonical(expected["full"].as_str().unwrap());
                    if actual.as_deref() != Some(expected_type.as_str()) {
                        failures.push(format!(
                            "selected {call:?} {key}: {actual:?} != {}",
                            expected["full"]
                        ));
                    }
                }
            }
            let source_text = String::from_utf8_lossy(source);
            for (start, text) in source_text.match_indices("replace($array[keyName()])") {
                let key_start = start + text.find("keyName()").unwrap();
                let counts = visits.0.lock().unwrap();
                assert_eq!(
                    counts
                        .get(&(key_start as u32, (key_start + "keyName()".len()) as u32))
                        .expect("key call reached"),
                    counts
                        .get(&(start as u32, (start + text.len()) as u32))
                        .expect("reference call reached"),
                    "a virtual write must not walk the key again"
                );
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn asymmetric_reference_controls_match_complete_public_diagnostics() {
    let expected: Value = serde_json::from_slice(include_bytes!(
        "fixtures/nested-reference-writes/contract-diagnostics.json"
    ))
    .unwrap();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/nested-reference-writes/contract.php");
    let mut project = Project::new(vec![path], Vec::new());
    project.level = 4;
    let actual: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message, error.tip))
        .collect();
    let mut expected: Vec<_> = expected["files"]
        .as_object()
        .unwrap()
        .values()
        .flat_map(|file| file["messages"].as_array().unwrap())
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
            )
        })
        .collect();
    expected.sort_by(|left, right| (left.0, &left.2).cmp(&(right.0, &right.2)));
    assert_eq!(actual, expected);
}
