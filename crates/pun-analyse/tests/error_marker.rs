//! PHPStan ErrorType identity through the public Analyzer and codex type APIs.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::{
    ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginRegistry, Provider,
    ProviderMeta,
};
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::mixed::TMixed;
use mago_codex::ttype::atomic::mixed::truthiness::TMixedTruthiness;
use mago_codex::ttype::combiner::{CombinerOptions, combine};
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    add_union_type, add_union_type_preserving_array_shapes, combine_owned_union_types_rc,
    combine_union_types, combine_union_types_preserving_array_shapes, combine_union_types_rc,
    get_error, get_int, get_literal_int, get_mixed, get_truthy_mixed, intersect_union_types,
};
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::{Call, Expression};
use mago_syntax::parser::parse_file;
use serde_json::Value;

type Dumps = Arc<Mutex<BTreeMap<u32, (String, bool)>>>;

struct ErrorSource(Dumps);

impl Provider for ErrorSource {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("error-marker", "error-marker", "error source");
        &META
    }
}

impl ExpressionHook for ErrorSource {
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        if let Expression::Call(Call::Function(call)) = expression
            && matches!(call.function, Expression::Identifier(id) if id.value() == b"error_value")
        {
            // The undefined-read and unset producers belong to separate patches.
            return Ok(ExpressionHookResult::SkipWithType(get_error()));
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
            let line = 1 + FLOW[..expression.span().start.offset as usize]
                .iter()
                .filter(|byte| **byte == b'\n')
                .count() as u32;
            let ty = context.get_expression_type(argument).unwrap();
            self.0
                .lock()
                .unwrap()
                .insert(line, (type_id(ty), ty.is_error()));
        }
        Ok(())
    }
}

const FLOW: &[u8] = include_bytes!("fixtures/error-marker/flow.php");

#[test]
fn analyzer_keeps_error_identity_without_infecting_offset_reads_or_overwrites() {
    for native in [false, true] {
        let file = File::new(
            Cow::Owned(format!("{}.php", ErrorSource::meta().id).into_bytes()),
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
        let dumps = Dumps::default();
        let mut plugins = PluginRegistry::new();
        plugins.register_expression_hook(ErrorSource(Arc::clone(&dumps)));
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &plugins, settings);
        let mut result = AnalysisResult::new(references);
        analyzer
            .analyze_with_artifacts(program, &mut result)
            .unwrap();

        let oracle: Value =
            serde_json::from_str(include_str!("fixtures/error-marker/flow.json")).unwrap();
        let want: BTreeMap<_, _> = oracle["messages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|dump| {
                let phpstan = dump["message"]
                    .as_str()
                    .unwrap()
                    .strip_prefix("Dumped type: ")
                    .unwrap();
                (
                    dump["line"].as_u64().unwrap() as u32,
                    (mago_id(phpstan), phpstan == "*ERROR*"),
                )
            })
            .collect();
        assert_eq!(*dumps.lock().unwrap(), want, "native={native}");
    }
}

// Only adapt display syntax. Marker identity and constraints remain assertions.
fn mago_id(phpstan: &str) -> String {
    match phpstan {
        "mixed~null" => "nonnull".to_string(),
        "mixed~(0|0.0|''|'0'|array{}|false|null)" => "truthy-mixed".to_string(),
        "0|0.0|''|'0'|array{}|false|null" => "falsy-mixed".to_string(),
        integer if integer.parse::<i64>().is_ok() => format!("int({integer})"),
        other => other.to_string(),
    }
}

fn type_id(ty: &TUnion) -> String {
    // Mago's empty flag tracks availability; both mixed forms have the same falsey values.
    if ty.is_falsy_mixed() {
        "falsy-mixed".to_string()
    } else {
        ty.get_id().to_string()
    }
}

fn check_type(label: &str, ty: &TUnion) {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/error-marker/types.json")).unwrap();
    let want = &oracle[label];
    assert!(!want.is_null(), "missing PHPStan oracle for {label}");
    assert_eq!(ty.is_error(), want["error"].as_bool().unwrap(), "{label}");
    assert_eq!(
        matches!(ty.types.as_ref(), [TAtomic::Mixed(mixed)] if mixed.is_vanilla()),
        want["unconstrained"].as_bool().unwrap(),
        "{label}"
    );
    let falsey = want["falsey"].as_bool().unwrap();
    assert_eq!(ty.is_falsy_mixed(), falsey, "{label}");
    // PHPStan expands falsey values into a concrete union. Mago retains constrained mixed.
    if !falsey {
        assert_eq!(ty.is_mixed(), want["mixed"].as_bool().unwrap(), "{label}");
    }
    assert_eq!(
        type_id(ty),
        mago_id(want["type"].as_str().unwrap()),
        "{label}"
    );
}

#[test]
fn public_union_paths_keep_phpstan_operand_order_and_concrete_absorption() {
    let codebase = CodebaseMetadata::new();
    let error = get_error();
    let mixed = get_mixed();
    let integer = get_literal_int(2);
    let nonnull = TUnion::from_atomic(TAtomic::Mixed(TMixed::new().with_is_non_null(true)));
    for (label, left, right) in [
        ("error|error", &error, &error),
        ("error|integer", &error, &integer),
        ("integer|error", &integer, &error),
        ("error|13", &error, &get_literal_int(13)),
        ("13|error", &get_literal_int(13), &error),
        ("error|mixed", &error, &mixed),
        ("mixed|error", &mixed, &error),
        ("error|nonnull", &error, &nonnull),
        ("nonnull|error", &nonnull, &error),
        ("mixed", &mixed, &nonnull),
    ] {
        let options = CombinerOptions::default();
        for combined in [
            combine_union_types(left, right, &codebase, options),
            combine_union_types_preserving_array_shapes(left, right, &codebase, options),
            add_union_type(left.clone(), right, &codebase, options),
            add_union_type_preserving_array_shapes(left.clone(), right, &codebase, options),
            TUnion::from_vec(combine(
                [left.types.to_vec(), right.types.to_vec()].concat(),
                &codebase,
                options,
            )),
        ] {
            check_type(label, &combined);
        }

        let left = Rc::new(left.clone());
        let right = Rc::new(right.clone());
        check_type(
            label,
            &combine_union_types_rc(&left, &right, &codebase, options),
        );
        check_type(
            label,
            &combine_owned_union_types_rc(Rc::clone(&left), &right, &codebase, options),
        );
        check_type(
            label,
            &combine_owned_union_types_rc(left, &right, &codebase, options),
        );
    }

    for (label, types) in [
        ("integer|error|mixed", [&integer, &error, &mixed]),
        ("integer|mixed|error", [&integer, &mixed, &error]),
    ] {
        check_type(
            label,
            &TUnion::from_vec(combine(
                types.into_iter().flat_map(|ty| ty.types.to_vec()).collect(),
                &codebase,
                CombinerOptions::default(),
            )),
        );
    }

    let mut optional = error;
    optional.set_possibly_undefined(true, Some(true));
    let joined = combine_union_types(&integer, &optional, &codebase, CombinerOptions::default());
    assert!(joined.is_error());
    assert!(joined.possibly_undefined());
    assert!(joined.possibly_undefined_from_try());
}

#[test]
fn public_intersections_apply_constraints_instead_of_infecting_the_result() {
    let codebase = CodebaseMetadata::new();
    let error = get_error();
    let mixed = get_mixed();
    let integer = get_literal_int(2);
    let nonnull = TUnion::from_atomic(TAtomic::Mixed(TMixed::new().with_is_non_null(true)));
    let truthy = get_truthy_mixed();
    let falsey = TUnion::from_atomic(TAtomic::Mixed(
        TMixed::new().with_truthiness(TMixedTruthiness::Falsy),
    ));
    for (label, left, right) in [
        ("error&error", &error, &error),
        ("error&mixed", &error, &mixed),
        ("mixed&error", &mixed, &error),
        ("error&integer", &error, &integer),
        ("integer&error", &integer, &error),
        ("integer&int", &integer, &get_int()),
        ("int&integer", &get_int(), &integer),
        ("error&nonnull", &error, &nonnull),
        ("nonnull&error", &nonnull, &error),
        ("error&truthy", &error, &truthy),
        ("truthy&error", &truthy, &error),
        ("error&falsey", &error, &falsey),
        ("falsey&error", &falsey, &error),
    ] {
        check_type(
            label,
            &intersect_union_types(left, right, &codebase).unwrap(),
        );
    }
    // Narrowing subtracts excluded values. Unlike explicit intersection, ErrorType overrides it.
    check_type("error-null", &error.to_non_nullable());
    check_type("error-falsey", &error.to_truthy());
    check_type(
        "error&falsey",
        &TUnion::from_atomic(TAtomic::Mixed(TMixed::error().as_empty())),
    );
    check_type(
        "error&falsey",
        &TUnion::from_atomic(TAtomic::Mixed(
            TMixed::error().with_truthiness(TMixedTruthiness::Falsy),
        )),
    );
    // No-op updates also keep identity.
    let unchanged = TMixed::error()
        .with_is_non_null(false)
        .with_truthiness(TMixedTruthiness::Undetermined);
    check_type("error", &TUnion::from_atomic(TAtomic::Mixed(unchanged)));
}

#[test]
fn public_identity_and_serde_keep_errors_distinct_from_ordinary_mixed() {
    let error = get_error();
    let mixed = get_mixed();
    assert_eq!(HashSet::from([error.clone(), mixed.clone()]).len(), 2);
    assert_eq!(BTreeSet::from([error.clone(), mixed.clone()]).len(), 2);
    for (label, ty) in [("error", error), ("mixed", mixed)] {
        let restored = serde_json::from_value::<TUnion>(serde_json::to_value(ty).unwrap()).unwrap();
        check_type(label, &restored);
    }

    let mut legacy = serde_json::to_value(TMixed::new()).unwrap();
    legacy.as_object_mut().unwrap().remove("is_error");
    let restored: TMixed = serde_json::from_value(legacy).unwrap();
    assert!(!restored.is_error());
    check_type("mixed", &TUnion::from_atomic(TAtomic::Mixed(restored)));
}
