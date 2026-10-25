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
use pun_phpstan::types::{
    ConstArray, Level, NeverKind, Tri, Type, accepts, intersect, is_super_type_of, mixed, union,
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

fn phpstan_types() -> BTreeMap<&'static str, Type> {
    let mut types = BTreeMap::from([
        ("mixed", mixed()),
        ("error", Type::Error),
        ("zero", Type::ConstInt(0)),
        ("one", Type::ConstInt(1)),
        ("thirteen", Type::ConstInt(13)),
        ("null", Type::Null),
        ("int", Type::Int),
        ("string", Type::String),
        ("literal", Type::ConstString("x".into())),
        ("empty", Type::ConstArray(Box::new(ConstArray::empty()))),
        ("never", Type::Never(NeverKind::Implicit)),
        ("explicit-never", Type::Never(NeverKind::Explicit)),
        ("range", Type::int_range(Some(-1), Some(1))),
    ]);
    for (name, excluded) in [
        ("not-zero", Type::ConstInt(0)),
        ("not-one", Type::ConstInt(1)),
        ("not-null", Type::Null),
        ("not-int", Type::Int),
        ("not-string", Type::String),
        ("not-range", types["range"].clone()),
        ("not-empty", types["empty"].clone()),
        ("not-zero-null", union(vec![Type::ConstInt(0), Type::Null])),
        (
            "not-zero-empty",
            union(vec![Type::ConstInt(0), types["empty"].clone()]),
        ),
    ] {
        types.insert(name, Type::mixed_subtracted(false, excluded));
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
fn pun_mixed_exclusions_describe_compare_and_accept_like_phpstan() {
    let types = phpstan_types();
    let expected = oracle();
    let mut failures = Vec::new();
    for (label, want) in expected["types"].as_object().unwrap() {
        let parts: Vec<_> = label.split(':').collect();
        let result = match parts.as_slice() {
            ["union", a, b] => union(vec![types[a].clone(), types[b].clone()]),
            ["intersect", a, b] => intersect(vec![types[a].clone(), types[b].clone()]),
            [a] => types[a].clone(),
            _ => continue, // Subtraction and unset originate in codex, not the conversion layer.
        };
        for (level, key) in [
            (Level::Precise, "precise"),
            (Level::Value, "value"),
            (Level::TypeOnly, "typeOnly"),
        ] {
            let got = canonical(&result.describe(level));
            if got != canonical(want[key].as_str().unwrap()) {
                failures.push(format!("{label} {key}: {got}, expected {}", want[key]));
            }
        }
    }
    for (label, want) in expected["relations"].as_object().unwrap() {
        let (a, b) = label.split_once(':').unwrap();
        for (key, got) in [
            ("super", is_super_type_of(&types[a], &types[b])),
            ("accepts", accepts(&types[a], &types[b], true)),
        ] {
            let got = match got {
                Tri::Yes => "yes",
                Tri::Maybe => "maybe",
                Tri::No => "no",
            };
            if got != want[key].as_str().unwrap() {
                failures.push(format!("{label} {key}: {got}, expected {}", want[key]));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn mixed_exclusions_keep_public_traversal_cast_and_acceptance_diagnostics() {
    use pun_phpstan::types::{Cast, StrictMixed, accepts_reasons, cast_is_error, to_phpdoc};
    let types = phpstan_types();
    let excluded = &types["not-zero"];
    let expected = oracle();
    let want = &expected["utilities"];
    for (cast, label) in [
        (Cast::String, "toString"),
        (Cast::Integer, "toInteger"),
        (Cast::BitwiseNot, "toBitwiseNotType"),
    ] {
        assert_eq!(
            cast_is_error(excluded, cast, StrictMixed::default()),
            want[label].as_bool().unwrap()
        );
    }
    assert_eq!(to_phpdoc(excluded), want["phpdoc"]);
    assert_eq!(excluded.is_cloneable().yes(), want["cloneable"]);
    assert_eq!(excluded.is_implicit_mixed(), want["implicit"]);
    assert_eq!(
        Type::mixed_subtracted(true, Type::ConstInt(0)).is_implicit_mixed(),
        want["explicit"]
    );
    assert_eq!(
        Type::array(excluded.clone(), Type::Int).describe(Level::Precise),
        want["array-int"]
    );
    assert_eq!(
        Type::array(excluded.clone(), mixed()).describe(Level::Precise),
        want["array-mixed"]
    );
    assert_eq!(
        cast_is_error(
            excluded,
            Cast::Integer,
            StrictMixed {
                implicit: true,
                explicit: false
            }
        ),
        want["strict-toInteger"]
    );
    assert_eq!(
        serde_json::to_value(accepts_reasons(
            excluded,
            &Type::ConstInt(0),
            true,
            StrictMixed::default()
        ))
        .unwrap(),
        want["accepts-reasons"]
    );
    assert_eq!(
        serde_json::to_value(accepts_reasons(
            &Type::ConstInt(0),
            excluded,
            true,
            StrictMixed::default()
        ))
        .unwrap(),
        want["reasons"]
    );
    let shifted = excluded.traverse(&|t| {
        if *t == Type::ConstInt(0) {
            Type::ConstInt(1)
        } else {
            t.clone()
        }
    });
    assert_eq!(shifted.describe(Level::Precise), want["traverse"]);
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
