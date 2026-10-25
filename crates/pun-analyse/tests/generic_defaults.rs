//! New-object template defaults: exact PHPStan 2.3.1 diagnostics through the public API.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::artifacts::AnalysisArtifacts;
use mago_analyzer::plugin::{
    ExpressionHook, HookContext, HookResult, PluginRegistry, Provider, ProviderMeta,
};
use mago_analyzer::settings::Settings;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::Expression;
use mago_syntax::parser::parse_file;
use pun_analyse::{Project, run, run_mago};
use serde_json::Value;

fn check(file: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generic-defaults");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/generic-defaults/expected-phpstan.json"
    ))
    .unwrap();
    let mut project = Project::new(vec![root.join(file)], Vec::new());
    project.level = level;
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message, e.tip))
        .collect();
    let mut want: Vec<_> = oracle[file][level.to_string()]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{file}, level {level}");
}

#[test]
fn constructor_defaults() {
    check("defaults.php", 0);
    check("defaults.php", 5);
}

#[test]
fn empty_array_arguments_infer_never_without_erasing_shape_templates() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generic-defaults");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/generic-defaults/empty-arrays.json")).unwrap();
    let mut want: Vec<_> = expected["files"]
        .as_object()
        .unwrap()
        .values()
        .next()
        .unwrap()["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    let mut got: Vec<_> = run(&Project::new(
        vec![root.join("empty-arrays.php")],
        Vec::new(),
    ))
    .unwrap()
    .into_iter()
    .map(|e| (e.line, e.identifier, e.message))
    .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn bounds_spl_and_weakmap_without_defaults() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generic-defaults");
    let result = run_mago(
        &Project::new(vec![root.join("controls.php")], Vec::new()),
        None,
    )
    .unwrap();
    let got: Vec<_> = result
        .dumped
        .into_iter()
        .map(|d| (d.line, d.type_id))
        .collect();
    // Compatibility with Mago 1.59's unchanged no-default branches, not PHPStan bound parity.
    // The pinned codex renders Placeholder as `_`; instantiation keeps SPL's special `never`.
    assert_eq!(
        got,
        [
            (
                14,
                "GenericDefaultsControls\\BoundWithoutDefault<_>".to_string()
            ),
            (
                15,
                "GenericDefaultsControls\\EmptyWithoutDefault<_>".to_string()
            ),
            (16, "SplObjectStorage<never, never>".to_string()),
            (17, "WeakMap<_, _>".to_string()),
        ]
    );
}

#[test]
fn inferred_mixed_is_not_an_absent_inference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generic-defaults");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/generic-defaults/expected-phpstan.json"
    ))
    .unwrap();
    let result = run_mago(
        &Project::new(vec![root.join("inferred-mixed.php")], Vec::new()),
        None,
    )
    .unwrap();
    let got: Vec<_> = result
        .dumped
        .into_iter()
        .map(|d| (d.line, d.type_id))
        .collect();
    let want: Vec<_> = oracle["inferred-mixed.php"]["0"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| {
            (
                d["line"].as_u64().unwrap() as u32,
                d["message"]
                    .as_str()
                    .unwrap()
                    .strip_prefix("Dumped type: ")
                    .unwrap()
                    .to_string(),
            )
        })
        .collect();
    // Read the public raw type: Pun's existing display drops implicit mixed in a list.
    assert_eq!(got, want);
}

type Visits = BTreeMap<(u32, u32), usize>;

struct Observer(Arc<Mutex<Visits>>);
impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "generic-defaults",
            "generic-defaults",
            "counts the authoritative walk",
        );
        &META
    }
}
impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }

    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expression.span();
        *self
            .0
            .lock()
            .unwrap()
            .entry((span.start.offset, span.end.offset))
            .or_default() += 1;
        if matches!(expression, Expression::Instantiation(_)) {
            assert!(context.get_native_expression_type(expression).is_none());
        }
        Ok(())
    }
}

fn analyze(code: &[u8], native: bool) -> (AnalysisArtifacts, Visits, String) {
    let file = File::new(
        Cow::Owned(format!("{}.php", Observer::meta().id).into_bytes()),
        FileType::Host,
        None,
        Cow::Owned(code.to_vec()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut settings = Settings::default();
    assert!(!settings.track_native_types);
    settings.track_native_types = native;
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let visits = Arc::<Mutex<Visits>>::default();
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observer(Arc::clone(&visits)));
    let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
    let mut result = AnalysisResult::new(references);
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let visits = std::mem::take(&mut *visits.lock().unwrap());
    (artifacts, visits, format!("{:?}", result.issues))
}

#[test]
fn defaults_keep_one_observer_walk_and_native_off_full_artifacts() {
    for code in [
        include_bytes!("fixtures/generic-defaults/defaults.php").as_slice(),
        include_bytes!("fixtures/parent-defaults/constants.php").as_slice(),
    ] {
        let (enabled, visits, issues) = analyze(code, true);
        let (disabled, off_visits, off_issues) = analyze(code, false);
        assert_eq!(issues, off_issues);
        assert_eq!(visits, off_visits);
        assert!(!visits.is_empty());
        assert!(visits.values().all(|count| *count == 1));
        assert_eq!(enabled.expression_types, disabled.expression_types);
        assert_eq!(enabled.if_true_assertions, disabled.if_true_assertions);
        assert_eq!(enabled.if_false_assertions, disabled.if_false_assertions);
        assert_eq!(
            enabled.true_branch_only_assertions,
            disabled.true_branch_only_assertions
        );
        assert_eq!(
            enabled.inferred_return_types,
            disabled.inferred_return_types
        );
        assert_eq!(
            enabled.inferred_yield_key_types,
            disabled.inferred_yield_key_types
        );
        assert_eq!(
            enabled.inferred_yield_value_types,
            disabled.inferred_yield_value_types
        );
        assert_eq!(enabled.symbol_references, disabled.symbol_references);
        assert_eq!(
            enabled.fully_matched_switch_offsets,
            disabled.fully_matched_switch_offsets
        );
        assert_eq!(
            enabled.inferred_parameter_types,
            disabled.inferred_parameter_types
        );
        assert_eq!(
            enabled.method_initialized_properties,
            disabled.method_initialized_properties
        );
        assert_eq!(
            enabled.method_calls_this_methods,
            disabled.method_calls_this_methods
        );
        assert_eq!(
            enabled.method_calls_parent_constructor,
            disabled.method_calls_parent_constructor
        );
        assert_eq!(
            enabled.method_calls_parent_initializer,
            disabled.method_calls_parent_initializer
        );
        assert_eq!(
            enabled.resolved_method_calls,
            disabled.resolved_method_calls
        );
        assert!(enabled.loop_scope.is_none() && disabled.loop_scope.is_none());
        assert!(enabled.case_scopes.is_empty() && disabled.case_scopes.is_empty());
        assert!(enabled.closure_bind_scope.is_none() && disabled.closure_bind_scope.is_none());
    }
}
