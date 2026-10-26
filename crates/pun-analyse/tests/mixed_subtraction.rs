//! Public type contracts captured from pinned PHPStan 2.3.1, independent of unset's producers.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::mixed::TMixed;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::comparator::{ComparisonResult, union_comparator};
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    add_union_type, add_union_type_preserving_array_shapes, combine_union_types,
    combine_union_types_preserving_array_shapes, get_empty_keyed_array, get_error, get_int,
    get_literal_int, get_literal_string, get_mixed, get_never, get_null, get_string,
    intersect_union_types,
};
use serde_json::Value;

fn oracle() -> Value {
    serde_json::from_str(include_str!("fixtures/mixed-subtraction/types.json")).unwrap()
}

fn codex_types() -> BTreeMap<&'static str, TUnion> {
    let codebase = CodebaseMetadata::new();
    let mut types = BTreeMap::from([
        ("mixed", get_mixed()),
        ("error", get_error()),
        ("zero", get_literal_int(0)),
        ("one", get_literal_int(1)),
        ("thirteen", get_literal_int(13)),
        ("null", get_null()),
        ("int", get_int()),
        ("string", get_string()),
        ("literal", get_literal_string(mago_word::word("x"))),
        ("empty", get_empty_keyed_array()),
        ("never", get_never()),
        ("explicit-never", get_never()),
        (
            "range",
            TUnion::from_atomic(TAtomic::Scalar(TScalar::Integer(TInteger::Range(-1, 1)))),
        ),
    ]);
    for (name, excluded) in [
        ("not-zero", get_literal_int(0)),
        ("not-one", get_literal_int(1)),
        ("not-null", get_null()),
        ("not-int", get_int()),
        ("not-string", get_string()),
        ("not-range", types["range"].clone()),
        ("not-empty", get_empty_keyed_array()),
        (
            "not-zero-null",
            TUnion::from_vec(vec![
                TAtomic::Null,
                TAtomic::Scalar(TScalar::Integer(TInteger::Literal(0))),
            ]),
        ),
        (
            "not-zero-empty",
            TUnion::from_vec(vec![
                get_empty_keyed_array().get_single().clone(),
                get_literal_int(0).get_single().clone(),
            ]),
        ),
    ] {
        types.insert(name, TMixed::new().subtract(&excluded, &codebase));
    }
    types
}

// Only spelling differs: Mago's scalar IDs, integer aliases and explicit never.
fn codex_spelling(ty: &TUnion) -> String {
    let mut parts: Vec<_> = ty
        .types
        .iter()
        .map(|atomic| match atomic {
            TAtomic::Mixed(m) if m.is_error() => "*ERROR*".into(),
            TAtomic::Mixed(m) => match m.get_subtracted_type() {
                Some(excluded) => {
                    let s = codex_spelling(excluded);
                    if excluded.types.len() > 1 {
                        format!("mixed~({s})")
                    } else {
                        format!("mixed~{s}")
                    }
                }
                None => "mixed".into(),
            },
            TAtomic::Never => "*NEVER*".into(),
            TAtomic::Array(a) if a.is_empty() => "array{}".into(),
            TAtomic::Scalar(TScalar::Integer(TInteger::Literal(i))) => i.to_string(),
            TAtomic::Scalar(TScalar::String(s)) if s.get_known_literal_value().is_some() => {
                format!(
                    "'{}'",
                    String::from_utf8_lossy(s.get_known_literal_value().unwrap())
                )
            }
            _ => atomic
                .get_id()
                .to_string()
                .replace("non-positive-int", "int<min, 0>")
                .replace("non-negative-int", "int<0, max>")
                .replace("positive-int", "int<1, max>")
                .replace("negative-int", "int<min, -1>"),
        })
        .collect();
    parts.sort();
    parts.join("|")
}

fn canonical(description: &str) -> String {
    if let Some(inner) = description
        .strip_prefix("mixed~(")
        .and_then(|s| s.strip_suffix(')'))
    {
        let mut parts: Vec<_> = inner.split('|').collect();
        parts.sort();
        format!("mixed~({})", parts.join("|"))
    } else {
        let mut parts: Vec<_> = description.split('|').collect();
        parts.sort();
        parts.join("|")
    }
}

#[test]
fn shared_mixed_subtraction_union_intersection_and_serde_match_phpstan() {
    let codebase = CodebaseMetadata::new();
    let types = codex_types();
    let expected = oracle();
    let mut failures = Vec::new();
    for (label, want) in expected["types"].as_object().unwrap() {
        let parts: Vec<_> = label.split(':').collect();
        let result = match parts.as_slice() {
            ["union", a, b] => {
                let (a, b) = (&types[a], &types[b]);
                let union = combine_union_types(a, b, &codebase, Default::default());
                for combined in [
                    add_union_type(a.clone(), b, &codebase, Default::default()),
                    add_union_type_preserving_array_shapes(
                        a.clone(),
                        b,
                        &codebase,
                        Default::default(),
                    ),
                    combine_union_types_preserving_array_shapes(
                        a,
                        b,
                        &codebase,
                        Default::default(),
                    ),
                ] {
                    assert_eq!(combined, union, "{label}");
                }
                union
            }
            ["intersect", a, b] => intersect_union_types(&types[a], &types[b], &codebase).unwrap(),
            ["unset", a] => {
                let TAtomic::Mixed(m) = types[a].get_single() else {
                    unreachable!()
                };
                TUnion::from_atomic(TAtomic::Mixed(m.unset_offset(&codebase)))
            }
            ["subtract", a, b] => {
                let TAtomic::Mixed(m) = types[a].get_single() else {
                    unreachable!()
                };
                m.subtract(&types[b], &codebase)
            }
            [a] => types[a].clone(),
            _ => unreachable!(),
        };
        let got = canonical(&codex_spelling(&result));
        if got != canonical(want["precise"].as_str().unwrap()) {
            failures.push(format!("{label}: {got}, expected {}", want["precise"]));
        }
        assert_eq!(
            result.is_error(),
            want["error"].as_bool().unwrap(),
            "{label}"
        );
        let restored: TUnion =
            serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
        assert_eq!(result, restored, "{label}");
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));

    for (label, expected) in expected["relations"].as_object().unwrap() {
        let (a, b) = label.split_once(':').unwrap();
        if a.starts_with("not-") {
            let contained = union_comparator::is_contained_by(
                &codebase,
                &types[b],
                &types[a],
                false,
                false,
                false,
                &mut ComparisonResult::new(),
            );
            assert_eq!(contained, expected["super"] == "yes", "{label}");
        }
    }
    let plain = get_mixed();
    let excluded = &types["not-zero"];
    assert_eq!(HashSet::from([plain.clone(), excluded.clone()]).len(), 2);
    assert_eq!(BTreeSet::from([plain, excluded.clone()]).len(), 2);
    let mut legacy = serde_json::to_value(TMixed::new()).unwrap();
    legacy.as_object_mut().unwrap().remove("subtracted_type");
    assert_eq!(
        serde_json::from_value::<TMixed>(legacy).unwrap(),
        TMixed::new()
    );
    assert_ne!(excluded.get_id(), get_mixed().get_id());
    let nonnull = TUnion::from_atomic(TAtomic::Mixed(TMixed::new().with_is_non_null(true)));
    for (got, label) in [
        (
            combine_union_types(&types["not-null"], &nonnull, &codebase, Default::default()),
            "legacy-nonnull-union",
        ),
        (
            intersect_union_types(&excluded, &nonnull, &codebase).unwrap(),
            "legacy-nonnull-intersect",
        ),
        (
            TMixed::new()
                .with_is_non_null(true)
                .subtract(&get_literal_int(0), &codebase),
            "legacy-nonnull-subtract",
        ),
        (
            combine_union_types(
                &excluded,
                &TUnion::from_atomic(TAtomic::Mixed(TMixed::truthy())),
                &codebase,
                Default::default(),
            ),
            "legacy-truthy-union",
        ),
    ] {
        assert_eq!(
            canonical(&codex_spelling(&got)),
            canonical(expected["utilities"][label].as_str().unwrap()),
            "{label}"
        );
    }
    assert_eq!(
        union_comparator::is_contained_by(
            &codebase,
            &nonnull,
            &types["not-null"],
            false,
            false,
            false,
            &mut ComparisonResult::new(),
        ),
        expected["utilities"]["legacy-nonnull-contained"]
            .as_bool()
            .unwrap()
    );
    for (got, label) in [
        (excluded.to_non_nullable(), "not-zero-null"),
        (types["not-null"].clone().as_nullable(), "mixed"),
        (types["not-zero-null"].clone().as_nullable(), "not-zero"),
        (
            combine_union_types(
                excluded,
                &TUnion::from_atomic(TAtomic::Mixed(TMixed::isset_from_loop())),
                &codebase,
                Default::default(),
            ),
            "mixed",
        ),
    ] {
        assert_eq!(
            canonical(&codex_spelling(&got)),
            canonical(expected["types"][label]["precise"].as_str().unwrap())
        );
    }
}

#[test]
fn analyzer_copies_joins_and_offset_access_preserve_only_the_correct_exclusions() {
    use mago_allocator::LocalArena;
    use mago_analyzer::plugin::{
        ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginRegistry, Provider,
        ProviderMeta,
    };
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, settings::Settings};
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{
        cst::{Call, Expression},
        parser::parse_file,
    };
    use std::borrow::Cow;
    use std::sync::{Arc, Mutex};

    const FLOW: &[u8] = include_bytes!("fixtures/mixed-subtraction/flow.php");
    fn line(expression: &Expression<'_>) -> u32 {
        1 + FLOW[..expression.span().start.offset as usize]
            .iter()
            .filter(|b| **b == b'\n')
            .count() as u32
    }
    struct Source(Arc<Mutex<BTreeMap<u32, String>>>);
    impl Provider for Source {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "mixed-subtraction",
                "mixed-subtraction",
                "narrowed mixed source",
            );
            &META
        }
    }
    impl ExpressionHook for Source {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            if matches!(expression, Expression::Variable(_)) && line(expression) == 8 {
                return Ok(ExpressionHookResult::SkipWithType(
                    TMixed::new().subtract(&get_literal_int(0), &CodebaseMetadata::new()),
                ));
            }
            Ok(ExpressionHookResult::Continue)
        }
        fn after_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if let Expression::Call(Call::Function(call)) = expression
                && matches!(call.function, Expression::Identifier(id) if id.value() == b"\\PHPStan\\dumpType")
            {
                let argument = call.argument_list.arguments.iter().next().unwrap().value();
                self.0.lock().unwrap().insert(
                    line(expression),
                    codex_spelling(context.get_expression_type(argument).unwrap()),
                );
            }
            Ok(())
        }
    }
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/mixed-subtraction/flow.json")).unwrap();
    let want: BTreeMap<u32, String> = expected["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| {
            (
                m["line"].as_u64().unwrap() as u32,
                m["message"]
                    .as_str()
                    .unwrap()
                    .strip_prefix("Dumped type: ")
                    .unwrap()
                    .into(),
            )
        })
        .collect();
    for native in [false, true] {
        let file = File::new(
            Cow::Borrowed(b"mixed-subtraction.php"),
            FileType::Host,
            None,
            Cow::Borrowed(FLOW),
        );
        let arena = LocalArena::new();
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
        let got = Arc::new(Mutex::new(BTreeMap::new()));
        let mut plugins = PluginRegistry::new();
        plugins.register_expression_hook(Source(Arc::clone(&got)));
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &plugins, settings);
        let mut result = AnalysisResult::new(references);
        analyzer
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        assert_eq!(*got.lock().unwrap(), want, "native={native}");
    }
}

#[test]
fn negative_guards_produce_mixed_exclusions_without_an_injection_hook() {
    guard_oracle("guards.php", include_str!("fixtures/mixed-subtraction/guards.json"));
}

#[test]
fn explicit_mixed_and_unset_array_domains_match_the_shared_phpstan_api() {
    use std::sync::Arc;
    use mago_codex::ttype::atomic::array::{TArray, keyed::TKeyedArray, list::TList};

    let codebase = CodebaseMetadata::new();
    let explicit = TMixed::new().with_is_explicit(true);
    let explicit_union = TUnion::from_atomic(TAtomic::Mixed(explicit.clone()));
    let types = BTreeMap::from([
        ("implicit", get_mixed()),
        ("explicit", explicit_union.clone()),
        ("excluded", explicit.subtract(&get_int(), &codebase)),
        ("int", get_int()),
    ]);
    let mut got = serde_json::Map::new();
    for (name, array) in [
        ("array", TArray::Keyed(TKeyedArray::new_with_parameters(
            Arc::new(explicit_union.clone()), Arc::new(explicit_union.clone()),
        ))),
        ("list", TArray::List(TList::new(Arc::new(explicit_union)))),
    ] {
        let excluded = TMixed::new().subtract(&TUnion::from_atomic(TAtomic::Array(array)), &codebase);
        let TAtomic::Mixed(mixed) = excluded.get_single() else { panic!("not mixed") };
        let unset = mixed.unset_offset(&codebase);
        let TAtomic::Array(array) = unset.get_subtracted_type().unwrap().get_single() else { panic!("not array") };
        let item = mago_codex::ttype::get_array_value_parameter(array, &codebase);
        let TAtomic::Mixed(item) = item.get_single() else { panic!("not mixed") };
        got.insert(format!("unset:{name}"), serde_json::json!({
            "nonempty": array.is_non_empty(), "item-explicit": item.is_explicit(),
        }));
    }
    for a in ["implicit", "explicit", "excluded"] {
        for (b, other) in &types {
            for (operation, result) in [
                ("union", combine_union_types(&types[a], other, &codebase, Default::default())),
                ("intersect", intersect_union_types(&types[a], other, &codebase).unwrap()),
            ] {
                let metadata = match result.types.as_ref() {
                    [TAtomic::Mixed(mixed)] => Value::Bool(mixed.is_explicit()),
                    _ => Value::Null,
                };
                got.insert(format!("{operation}:{a}:{b}"), metadata);
                let restored: TUnion = serde_json::from_value(serde_json::to_value(&result).unwrap()).unwrap();
                assert_eq!(restored, result);
            }
        }
    }
    let TAtomic::Mixed(excluded) = types["excluded"].get_single() else { panic!("not mixed") };
    got.insert("subtract-explicit".into(), Value::Bool(excluded.is_explicit()));
    got.insert("unset-explicit".into(), Value::Bool(excluded.unset_offset(&codebase).is_explicit()));
    let expected: Value = serde_json::from_str(include_str!("fixtures/mixed-subtraction/guard-types.json")).unwrap();
    assert_eq!(Value::Object(got), expected);
    let mut legacy = serde_json::to_value(TMixed::new()).unwrap();
    legacy.as_object_mut().unwrap().remove("is_explicit");
    assert_eq!(serde_json::from_value::<TMixed>(legacy).unwrap(), TMixed::new());
}

#[test]
fn array_guards_preserve_explicit_parameters_copies_joins_and_falsey_intersections() {
    guard_oracle("array-guards.php", include_str!("fixtures/mixed-subtraction/array-guards.json"));
}

#[test]
fn mixed_metadata_restoration_keeps_both_explicit_origins_and_concrete_narrowings() {
    guard_oracle("metadata-restoration.php", include_str!("fixtures/mixed-subtraction/metadata-restoration.json"));
}

#[test]
fn guarded_mixed_receivers_follow_explicit_implicit_and_template_level_thresholds() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/mixed-subtraction");
    let oracle: Value = serde_json::from_str(include_str!("fixtures/mixed-subtraction/level-receivers.json")).unwrap();
    for level in [2, 8, 9, 10] {
        let mut project = pun_analyse::Project::new(vec![root.join("level-receivers.php")], Vec::new());
        project.php_version = 80400;
        project.level = level;
        let got: Vec<_> = pun_analyse::run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| matches!(e.identifier.as_str(), "classConstant.nonObject" | "method.nonObject" | "property.nonObject"))
            .map(|e| serde_json::json!([e.line, e.identifier, e.message, e.tip, e.can_be_ignored]))
            .collect();
        assert_eq!(serde_json::json!(got), oracle[level.to_string()], "level {level}");
    }
}

#[test]
fn call_assertions_require_declared_contracts_not_guessed_predicates() {
    guard_oracle("call-assertions.php", include_str!("fixtures/mixed-subtraction/call-assertions.json"));
}

#[test]
fn inferred_parent_predicates_do_not_become_declared_child_contracts() {
    guard_oracle("inherited-assertions.php", include_str!("fixtures/mixed-subtraction/inherited-assertions.json"));
}

fn guard_oracle(file: &str, oracle: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/mixed-subtraction");
    let mut project = pun_analyse::Project::new(vec![root.join(file)], Vec::new());
    project.php_version = 80400;
    let mut got: Vec<_> = pun_analyse::run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
        .collect();
    let oracle: Value = serde_json::from_str(oracle).unwrap();
    let mut want: Vec<_> = oracle["files"][file]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (
            e["line"].as_i64().unwrap(),
            e["identifier"].as_str().unwrap().to_string(),
            e["message"].as_str().unwrap().to_string(),
            e["tip"].as_str().map(str::to_string),
            e["ignorable"].as_bool().unwrap(),
        ))
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn negative_guard_exclusions_are_independent_of_native_tracking() {
    use std::borrow::Cow;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::plugin::{
        ExpressionHook, HookContext, HookResult, PluginRegistry, Provider, ProviderMeta,
    };
    use mago_analyzer::{Analyzer, settings::Settings};
    use mago_codex::{populator::populate_codebase, scanner::scan_program};
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{cst::{Call, Expression}, parser::parse_file};

    const SOURCE: &[u8] = include_bytes!("fixtures/mixed-subtraction/guards.php");
    const ARRAYS: &[u8] = include_bytes!("fixtures/mixed-subtraction/array-guards.php");
    const CALLS: &[u8] = include_bytes!("fixtures/mixed-subtraction/call-assertions.php");
    const INHERITED: &[u8] = include_bytes!("fixtures/mixed-subtraction/inherited-assertions.php");
    struct Observe(Arc<Mutex<BTreeMap<u32, TUnion>>>, &'static [u8]);
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "negative-mixed-guards", "negative-mixed-guards", "passive guard observations",
            );
            &META
        }
    }
    impl ExpressionHook for Observe {
        fn is_observer(&self) -> bool { true }

        fn after_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if let Expression::Call(Call::Function(call)) = expression
                && matches!(call.function, Expression::Identifier(id) if id.value() == b"\\PHPStan\\dumpType")
            {
                let argument = call.argument_list.arguments.iter().next().unwrap().value();
                let line = 1 + self.1[..expression.span().start.offset as usize]
                    .iter().filter(|b| **b == b'\n').count() as u32;
                self.0.lock().unwrap().insert(
                    line, context.get_expression_type(argument).unwrap().clone(),
                );
            }
            Ok(())
        }
    }
    let prelude = mago_prelude::Prelude::build();
    let mut baseline = None;
    for (source, native) in [
        (SOURCE, false), (SOURCE, true), (ARRAYS, false), (ARRAYS, true), (CALLS, false), (CALLS, true),
        (INHERITED, false), (INHERITED, true),
    ] {
        if !native { baseline = None; }
        let arena = LocalArena::new();
        let file = File::ephemeral(Cow::Borrowed(b"negative-mixed-guards.php"), Cow::Borrowed(source));
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let settings = Settings { track_native_types: native, ..Settings::default() };
        let mut codebase = prelude.metadata.clone();
        codebase.extend(scan_program(&arena, &file, program, &names, settings.version));
        let mut references = prelude.symbol_references.clone();
        populate_codebase(&mut codebase, &mut references, Default::default(), Default::default());
        if source == CALLS {
            // Keep body inference available to callback providers; only call-site policy changes.
            for name in ["guessedstring", "guessednotint", "guessednull", "guessedreference"] {
                let name = format!("mixedcallassertions\\{name}");
                assert!(codebase.get_function(name.as_bytes()).unwrap().assertions_inferred);
            }
            for name in ["declaredtrue", "declaredfalse", "requirestring"] {
                let name = format!("mixedcallassertions\\{name}");
                assert!(!codebase.get_function(name.as_bytes()).unwrap().assertions_inferred);
            }
        }
        let observed = Arc::new(Mutex::new(BTreeMap::new()));
        let mut registry = PluginRegistry::with_library_providers();
        registry.register_expression_hook(Observe(Arc::clone(&observed), source));
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
        let mut result = AnalysisResult::new(references);
        let artifacts = analyzer.analyze_with_artifacts(program, &mut result).unwrap();
        let observed = std::mem::take(&mut *observed.lock().unwrap());
        if source == SOURCE {
            // Actual PHPStan dumps mixed~int at line 8 and ErrorType after every failed-read guard.
            let TAtomic::Mixed(not_int) = observed[&8].get_single() else { panic!("not mixed") };
            assert_eq!(not_int.get_subtracted_type(), Some(&get_int()));
            for line in [125, 128, 131] {
                assert!(observed[&line].is_error(), "line {line}: {:?}", observed[&line]);
            }
        } else if source == ARRAYS {
            // PHPStan's array<mixed, mixed> after the join and repeated guard has explicit parameters.
            let TAtomic::Array(mago_codex::ttype::atomic::array::TArray::Keyed(array)) = observed[&20].get_single()
                else { panic!("not a keyed array: {:?}", observed[&20]) };
            let (key, value) = array.parameters.as_ref().unwrap();
            for parameter in [key, value] {
                let TAtomic::Mixed(mixed) = parameter.get_single() else { panic!("not mixed") };
                assert!(mixed.is_explicit());
            }
        } else {
            let (file, oracle) = if source == CALLS {
                ("call-assertions.php", include_str!("fixtures/mixed-subtraction/call-assertions.json"))
            } else {
                ("inherited-assertions.php", include_str!("fixtures/mixed-subtraction/inherited-assertions.json"))
            };
            let oracle: Value = serde_json::from_str(oracle).unwrap();
            let expected: BTreeMap<_, _> = oracle["files"][file]["messages"].as_array().unwrap().iter()
                .map(|record| (record["line"].as_u64().unwrap() as u32,
                    record["message"].as_str().unwrap().strip_prefix("Dumped type: ").unwrap().to_string()))
                .collect();
            let actual: BTreeMap<_, _> = observed.iter().map(|(line, ty)| (*line, codex_spelling(ty))).collect();
            assert_eq!(actual, expected);
        }
        let state = (observed, artifacts.expression_types, format!("{:?}", result.issues));
        if let Some(baseline) = &baseline {
            assert_eq!(&state, baseline, "native tracking changed the full analysis");
        } else {
            baseline = Some(state);
        }
    }
}
