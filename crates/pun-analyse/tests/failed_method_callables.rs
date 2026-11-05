//! Pinned PHPStan 2.3.1 fallback values are bare Closure objects, not bottom or signatures.
use std::{
    borrow::Cow,
    path::Path,
    sync::{Arc, Mutex},
};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer,
    analysis_result::AnalysisResult,
    artifacts::AnalysisArtifacts,
    code::IssueCode,
    plugin::{ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginRegistry},
    settings::Settings,
};
use mago_codex::{
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            callable::{TCallable, TCallableSignature, parameter::TCallableParameter},
            iterable::TIterable,
            object::{TObject, named::TNamedObject},
            scalar::{TScalar, int::TInteger, string::TStringLiteral},
        },
        combine_union_types,
        comparator::{ComparisonResult, union_comparator},
        get_int, get_mixed, get_named_object, get_string,
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{cst::Expression, parser::parse_file};
use mago_word::word;
use pun_analyse::{Project, run};
use serde_json::{Value, json};

const SOURCE: &[u8] = include_bytes!("fixtures/failed-method-callables/cases.php");
const ORACLE: &str = include_str!("fixtures/failed-method-callables/expected.json");
// These are raw Analyzer controls. The existing ordinary-call bleeding-edge
// method.nameNotString observation gap is outside this producer port.
const SELECTORS: &[u8] = include_bytes!("fixtures/failed-method-callables/selectors.php");
const SELECTOR_ORACLE: &str =
    include_str!("fixtures/failed-method-callables/expected-selectors.json");
const COMPATIBILITY: &[u8] = include_bytes!("fixtures/failed-method-callables/compatibility.php");
const COMPATIBILITY_ORACLE: &str =
    include_str!("fixtures/failed-method-callables/expected-compatibility.json");
// A template-bound Closure retains the refinement that the direct PHPDoc
// Closure&callable spelling currently loses in Mago's type builder.
const COMPATIBILITY_CONTROLS: &[u8] =
    include_bytes!("fixtures/failed-method-callables/compatibility-controls.php");
const COMPATIBILITY_CONTROLS_ORACLE: &str =
    include_str!("fixtures/failed-method-callables/expected-compatibility-controls.json");
const RELATION_CONTROLS: &[u8] =
    include_bytes!("fixtures/failed-method-callables/relation-controls.php");
const RELATION_CONTROLS_ORACLE: &str =
    include_str!("fixtures/failed-method-callables/expected-relation-controls.json");

#[derive(Clone, Default)]
struct Visits(Arc<Mutex<Vec<(u32, u32)>>>);
impl ExpressionHook for Visits {
    fn is_observer(&self) -> bool {
        true
    }
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        let span = expression.span();
        self.0
            .lock()
            .unwrap()
            .push((span.start.offset, span.end.offset));
        Ok(ExpressionHookResult::Continue)
    }
}

fn describe(ty: &TUnion) -> Value {
    match ty.types.as_ref() {
        [TAtomic::Scalar(TScalar::Integer(TInteger::Literal(value)))] => {
            json!({"kind": "value", "type": value.to_string()})
        }
        [TAtomic::Scalar(TScalar::String(string))]
            if let Some(TStringLiteral::Value(value)) = string.literal =>
        {
            json!({"kind": "value", "type": format!("'{value}'")})
        }
        [TAtomic::Object(TObject::Named(object))]
            if object.name.as_bytes().eq_ignore_ascii_case(b"Closure") =>
        {
            assert!(object.type_parameters.is_none());
            assert!(object.intersection_types.is_none());
            json!({"kind": "bare-closure"})
        }
        [TAtomic::Callable(TCallable::Signature(signature))] => json!({
            "kind": "signature",
            "parameters": signature.get_parameters().iter().map(|parameter| parameter.get_type_signature().unwrap().get_id().to_string()).collect::<Vec<_>>(),
            "return": signature.get_return_type().unwrap().get_id().to_string(),
        }),
        _ => json!({"kind": "value", "type": ty.get_id().to_string()}),
    }
}

fn analyze(
    source: &'static [u8],
    native: bool,
    observe: bool,
) -> (
    AnalysisArtifacts,
    AnalysisResult,
    Visits,
    mago_database::file::FileId,
) {
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"cases.php"), Cow::Borrowed(source));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors(), "{:?}", program.errors);
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
    let visits = Visits::default();
    let mut registry = PluginRegistry::new();
    if observe {
        registry.register_expression_hook(visits.clone());
    }
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    (artifacts, result, visits, file.id)
}

#[test]
fn producer_post_write_and_continuation_match_fresh_phpstan() {
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let mut baseline = None;
    for native in [false, true] {
        for observe in [false, true] {
            let (artifacts, result, visits, file) = analyze(SOURCE, native, observe);
            let mut failures = Vec::new();
            let mut native_values = 0;
            for row in oracle["types"].as_array().unwrap() {
                let start = row["start"].as_u64().unwrap() as u32;
                let end = row["end"].as_u64().unwrap() as u32;
                let span = Span::new(file, Position::new(start), Position::new(end));
                assert_eq!(
                    &SOURCE[start as usize..end as usize],
                    row["expression"].as_str().unwrap().as_bytes()
                );
                let actual = artifacts
                    .get_expression_result(&span)
                    .map(|result| describe(result.get_type()));
                if actual.as_ref() != Some(&row["full"]) {
                    failures.push(json!({"expression": row["expression"], "start": start, "actual": actual, "expected": row["full"]}));
                }
                if let Some(ty) = artifacts.get_native_expression_type(&span) {
                    assert!(native);
                    native_values += 1;
                    assert_eq!(describe(ty), row["native"], "native {}", row["expression"]);
                }
                if !native
                    || row["full"]["kind"] == "bare-closure"
                    || row["full"]["kind"] == "signature"
                {
                    assert!(
                        artifacts.get_native_expression_type(&span).is_none(),
                        "partial native API must not invent callable support"
                    );
                }
            }
            assert_eq!(
                native_values > 0,
                native,
                "supported literal native reads must remain available"
            );
            assert!(
                failures.is_empty(),
                "native={native}, observe={observe}: {}",
                serde_json::to_string_pretty(&failures).unwrap()
            );
            if observe {
                let visits = visits.0.lock().unwrap();
                let mut positions = Vec::new();
                for expression in [
                    "($receiver = new CallableTarget())",
                    "($name = 123)",
                    "$receiver->$name(...)",
                    "$receiver->{$name}(...)",
                ] {
                    let start = std::str::from_utf8(SOURCE)
                        .unwrap()
                        .find(expression)
                        .unwrap() as u32;
                    let span = (start, start + expression.len() as u32);
                    assert_eq!(
                        visits.iter().filter(|&&visited| visited == span).count(),
                        1,
                        "{expression}"
                    );
                    positions.push(visits.iter().position(|&visited| visited == span).unwrap());
                }
                assert!(
                    positions[0] < positions[1],
                    "receiver must run before selector"
                );
            }
            let state = (artifacts.expression_types, result.issues);
            if let Some(previous) = &baseline {
                assert_eq!(
                    previous, &state,
                    "native/observation must not change full analysis"
                );
            }
            baseline = Some(state);
        }
    }
}

#[test]
fn complete_diagnostics_preserve_receiver_errors_without_dynamic_selector_validation() {
    for (file, oracle) in [
        ("cases.php", ORACLE),
        ("compatibility.php", COMPATIBILITY_ORACLE),
    ] {
        check_complete_diagnostics(file, oracle);
    }
}

fn check_complete_diagnostics(file: &str, oracle: &str) {
    let oracle: Value = serde_json::from_str(oracle).unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-method-callables");
    for level in [0, 5, 9] {
        for bleeding in [false, true] {
            let mut project = Project::new(vec![root.join(file)], vec![]);
            project.level = level;
            project.php_version = 80500;
            project.check_this_only = Some(false);
            project.options.set(pun_analyse::ProjectOptions::BLEEDING_EDGE, bleeding);
            project.options.set(pun_analyse::ProjectOptions::CHECK_NON_STRINGABLE_DYNAMIC_ACCESS, bleeding);
            let mut actual: Vec<_> = run(&project)
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
            actual.sort_by_key(Value::to_string);
            let key = format!("{level}{}", if bleeding { "-bleeding" } else { "" });
            assert_eq!(
                json!(actual),
                oracle["errors"][key],
                "{file}, level {level}, bleeding={bleeding}"
            );
        }
    }
}

#[test]
fn ordinary_selectors_still_validate_and_actual_partial_application_keeps_its_own_fallback() {
    for native in [false, true] {
        let (_, result, _, _) = analyze(SOURCE, native, false);
        let mut codes: Vec<_> = result
            .issues
            .iter()
            .map(|issue| issue.code.as_deref().unwrap())
            .collect();
        codes.sort_unstable();
        let mut expected = [
            IssueCode::InvalidMethodAccess.as_str(),
            IssueCode::InvalidMethodAccess.as_str(),
            IssueCode::MethodAccessOnNull.as_str(),
            // Both unknown-signature invocations return mixed. Raw Mago warns
            // on those assignments; Pun's PHPStan-facing consumer omits it.
            IssueCode::MixedAssignment.as_str(),
            IssueCode::MixedAssignment.as_str(),
            IssueCode::NonExistentMethod.as_str(),
        ];
        expected.sort_unstable();
        assert_eq!(
            codes, expected,
            "receiver/method errors and mixed-result assignment warnings remain, never dynamic-name errors: {:?}",
            result.issues
        );
        let (artifacts, result, _, file) = analyze(SELECTORS, native, false);
        assert_eq!(
            result
                .issues
                .iter()
                .filter(|issue| issue.code.as_deref()
                    == Some(IssueCode::InvalidMemberSelector.as_str()))
                .count(),
            1,
            "ordinary calls still reject integer selectors"
        );
        // Properties retain their existing integer-to-string coercion, then report
        // the missing property through Mago's ordinary property resolver.
        assert!(result.issues.iter().any(|issue| issue.code.as_deref() == Some(IssueCode::NonExistentProperty.as_str())), "{:?}", result.issues);
        let oracle: Value = serde_json::from_str(SELECTOR_ORACLE).unwrap();
        for row in oracle["types"].as_array().unwrap() {
            let span = Span::new(
                file,
                Position::new(row["start"].as_u64().unwrap() as u32),
                Position::new(row["end"].as_u64().unwrap() as u32),
            );
            assert_eq!(
                describe(artifacts.get_expression_type(&span).unwrap()),
                row["full"],
                "property-name coercion must not overwrite the selector's own integer value"
            );
        }
        let source = include_bytes!("fixtures/failed-method-callables/partial.php");
        let (artifacts, result, _, file) = analyze(source, native, false);
        assert_eq!(
            result
                .issues
                .iter()
                .filter(|issue| issue.code.as_deref()
                    == Some(IssueCode::InvalidMemberSelector.as_str()))
                .count(),
            1
        );
        for (expression, expected) in [
            (
                "$receiver->known(?, 1)",
                json!({"kind": "signature", "parameters": ["string"], "return": "int"}),
            ),
            (
                "$receiver->missing(?)",
                json!({"kind": "value", "type": "never"}),
            ),
            (
                "$receiver->{$name}(?)",
                json!({"kind": "value", "type": "never"}),
            ),
        ] {
            let start = std::str::from_utf8(source)
                .unwrap()
                .find(expression)
                .unwrap() as u32;
            let span = Span::new(
                file,
                Position::new(start),
                Position::new(start + expression.len() as u32),
            );
            assert_eq!(
                describe(artifacts.get_expression_type(&span).unwrap()),
                expected,
                "{expression}"
            );
        }
    }
}

#[test]
fn opaque_closure_compatibility_preserves_signatures_purity_and_object_boundaries() {
    for (source, capture) in [
        (COMPATIBILITY, COMPATIBILITY_ORACLE),
        (COMPATIBILITY_CONTROLS, COMPATIBILITY_CONTROLS_ORACLE),
    ] {
        let oracle: Value = serde_json::from_str(capture).unwrap();
        let mut baseline = None;
        for native in [false, true] {
            for observe in [false, true] {
                let (artifacts, result, _, file) = analyze(source, native, observe);
                for row in oracle["types"].as_array().unwrap() {
                    let start = row["start"].as_u64().unwrap() as u32;
                    let end = row["end"].as_u64().unwrap() as u32;
                    assert_eq!(
                        &source[start as usize..end as usize],
                        row["expression"].as_str().unwrap().as_bytes()
                    );
                    let span = Span::new(file, Position::new(start), Position::new(end));
                    assert_eq!(
                        describe(artifacts.get_expression_type(&span).unwrap()),
                        row["full"],
                        "{}",
                        row["expression"]
                    );
                    if let Some(ty) = artifacts.get_native_expression_type(&span) {
                        assert!(native);
                        assert_eq!(describe(ty), row["native"]);
                    }
                }
                // Mago distinguishes definite and possible argument errors. Both
                // correspond to PHPStan's argument.type; no other issue is dropped.
                let mut actual: Vec<_> = result
                    .issues
                    .iter()
                    .map(|issue| {
                        let code = issue.code.as_deref().unwrap();
                        assert!(
                            code == IssueCode::InvalidArgument.as_str()
                                || code == IssueCode::PossiblyInvalidArgument.as_str(),
                            "unexpected raw issue: {issue:?}"
                        );
                        let primary = issue
                            .annotations
                            .iter()
                            .find(|annotation| annotation.kind.is_primary())
                            .unwrap();
                        let line = source[..primary.span.start.offset as usize]
                            .iter()
                            .filter(|&&byte| byte == b'\n')
                            .count()
                            + 1;
                        json!([line, "argument.type"])
                    })
                    .collect();
                let mut expected: Vec<_> = oracle["errors"]["9"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|row| json!([row[1], row[2]]))
                    .collect();
                actual.sort_by_key(Value::to_string);
                expected.sort_by_key(Value::to_string);
                assert_eq!(actual, expected, "native={native}, observe={observe}");
                let state = (artifacts.expression_types, result.issues);
                if let Some(previous) = &baseline {
                    assert_eq!(previous, &state);
                }
                baseline = Some(state);
            }
        }
    }
}

#[test]
fn opaque_closure_acceptance_is_not_a_subtype_proof() {
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/failed-method-callables/expected-relations.json"
    ))
    .unwrap();
    let arena = LocalArena::new();
    let file = File::ephemeral(
        Cow::Borrowed(b"relations.php"),
        Cow::Borrowed(RELATION_CONTROLS),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Settings::default().version);
    populate_codebase(
        &mut codebase,
        &mut SymbolReferences::new(),
        Default::default(),
        Default::default(),
    );
    let opaque = get_named_object(word("Closure"), None);
    let signature = |pure, closure, parameter: Option<TUnion>, return_type| {
        TCallableSignature::new(pure, closure)
            .with_parameters(
                parameter
                    .map(|ty| TCallableParameter::new(Some(Arc::new(ty)), false, false, false))
                    .into_iter()
                    .collect(),
            )
            .with_return_type(Some(Arc::new(return_type)))
    };
    for (name, signature) in [
        ("callable", TCallableSignature::mixed(false)),
        (
            "callable(): mixed",
            signature(false, false, None, get_mixed()),
        ),
        (
            "callable(int): string",
            signature(false, false, Some(get_int()), get_string()),
        ),
        (
            "Closure(): mixed",
            signature(false, true, None, get_mixed()),
        ),
        (
            "Closure(int): string",
            signature(false, true, Some(get_int()), get_string()),
        ),
        (
            "pure-Closure(): int",
            signature(true, true, None, get_int()),
        ),
        (
            "pure-callable(): int",
            signature(true, false, None, get_int()),
        ),
    ] {
        let target = TUnion::from_atomic(TAtomic::Callable(TCallable::Signature(signature)));
        let mut result = ComparisonResult::new();
        let accepted = union_comparator::is_contained_by(
            &codebase,
            &opaque,
            &target,
            false,
            false,
            false,
            &mut result,
        );
        assert_eq!(
            accepted,
            oracle["relations"][name]["accepts"] == "Yes",
            "{name}"
        );
        assert_eq!(
            accepted && !result.is_only_accepted,
            oracle["relations"][name]["supertype"] == "Yes",
            "assignment acceptance must not erase a callable constraint: {name}"
        );
        if !name.contains("Closure") {
            let joined = combine_union_types(&opaque, &target, &codebase, Default::default());
            assert_eq!(
                joined.types.len() == 2,
                oracle["relations"][name]["union"]
                    .as_str()
                    .unwrap()
                    .contains('|'),
                "an accepted but unproven subtype must survive a union: {name}"
            );
        }
        // One definite alternative is sufficient, in either order. An earlier
        // merely accepted callable must not contaminate the nominal match.
        for reverse in [false, true] {
            let mut alternatives = target.types.to_vec();
            alternatives.extend_from_slice(&opaque.types);
            if reverse {
                alternatives.reverse();
            }
            let mut result = ComparisonResult::new();
            assert!(union_comparator::is_contained_by(
                &codebase,
                &opaque,
                &TUnion::new(Cow::Owned(alternatives)),
                false,
                false,
                false,
                &mut result,
            ));
            assert!(
                !result.is_only_accepted,
                "definite nominal alternative: {name}"
            );
        }
    }
    let typed = TUnion::from_atomic(TAtomic::Callable(TCallable::Signature(signature(
        false,
        false,
        Some(get_int()),
        get_string(),
    ))));
    let contravariant = |argument| {
        TUnion::from_atomic(TAtomic::Object(TObject::Named(
            TNamedObject::new_with_type_parameters(word("RelationContra"), Some(vec![argument])),
        )))
    };
    let mut result = ComparisonResult::new();
    let accepted = union_comparator::is_contained_by(
        &codebase,
        &contravariant(typed.clone()),
        &contravariant(opaque.clone()),
        false,
        false,
        false,
        &mut result,
    );
    assert!(
        accepted,
        "preserve the existing contravariant assignment relation"
    );
    assert_eq!(
        accepted && !result.is_only_accepted,
        oracle["contravariant_supertype"] == "Yes"
    );
    let mut alternatives = opaque.types.to_vec();
    alternatives.extend_from_slice(&get_int().types);
    let known = TUnion::from_atomic(TAtomic::Callable(TCallable::Signature(signature(
        false,
        true,
        Some(get_int()),
        get_string(),
    ))));
    let mut result = ComparisonResult::new();
    let accepted = union_comparator::is_contained_by(
        &codebase,
        &contravariant(TUnion::new(Cow::Owned(alternatives))),
        &contravariant(known.clone()),
        false,
        false,
        false,
        &mut result,
    );
    assert!(accepted);
    assert_eq!(
        accepted && !result.is_only_accepted,
        oracle["contravariant_union_supertype"] == "Yes",
        "a failed forward comparison must not contaminate the successful reverse relation"
    );
    let invariant = |argument| {
        TUnion::from_atomic(TAtomic::Object(TObject::Named(
            TNamedObject::new_with_type_parameters(word("RelationInvariant"), Some(vec![argument])),
        )))
    };
    let iterable = |value| {
        TUnion::from_atomic(TAtomic::Iterable(TIterable::new(
            Arc::new(get_int()),
            Arc::new(value),
        )))
    };
    for (name, input, target) in [
        ("invariant", invariant(known), invariant(opaque.clone())),
        (
            "iterable",
            iterable(opaque.clone()),
            iterable(typed.clone()),
        ),
    ] {
        let mut result = ComparisonResult::new();
        let accepted = union_comparator::is_contained_by(
            &codebase,
            &input,
            &target,
            false,
            false,
            false,
            &mut result,
        );
        assert!(
            accepted,
            "preserve the existing nested assignment relation: {name}"
        );
        assert_eq!(
            accepted && !result.is_only_accepted,
            oracle[format!("{name}_supertype")] == "Yes",
            "{name}"
        );
    }

    // Inference variables are definite matches under the existing relation,
    // even when another alternative only accepts the opaque Closure.
    let mut alternatives = typed.types.to_vec();
    alternatives.push(TAtomic::Variable(word("Callback")));
    let mut result = ComparisonResult::new();
    assert!(union_comparator::is_contained_by(
        &codebase,
        &opaque,
        &TUnion::new(Cow::Owned(alternatives)),
        false,
        false,
        false,
        &mut result,
    ));
    assert!(!result.is_only_accepted);
    assert_eq!(result.type_variable_lower_bounds.len(), 1);
    assert_eq!(result.type_variable_lower_bounds[0].0, word("Callback"));
}

#[test]
fn a_known_closure_projection_keeps_its_signature_and_argument_diagnostic() {
    let oracle: Value = serde_json::from_str(RELATION_CONTROLS_ORACLE).unwrap();
    for native in [false, true] {
        let (artifacts, _, _, file) = analyze(RELATION_CONTROLS, native, false);
        for row in oracle["types"].as_array().unwrap() {
            let span = Span::new(
                file,
                Position::new(row["start"].as_u64().unwrap() as u32),
                Position::new(row["end"].as_u64().unwrap() as u32),
            );
            assert_eq!(
                describe(artifacts.get_expression_type(&span).unwrap()),
                row["full"]
            );
        }
    }
    check_complete_diagnostics("relation-controls.php", RELATION_CONTROLS_ORACLE);
}
