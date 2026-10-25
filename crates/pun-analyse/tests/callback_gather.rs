//! PHPStan 2.3.1 oracle output, through the public analyser (not a private inference helper).

use std::path::Path;

use pun_analyse::{Project, run, run_mago};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callback-gather");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    let mut want: Vec<_> = expected[file]
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
    // Compare every diagnostic, independent of the entry point's historical sorting policy.
    got.sort_by(|a, b| (a.0, &a.2, &a.1).cmp(&(b.0, &b.2, &b.1)));
    want.sort_by(|a, b| (a.0, &a.2, &a.1).cmp(&(b.0, &b.2, &b.1)));
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn raw_returns_and_contextual_body_types_are_separate() {
    check("functions.php");
}

#[test]
fn methods_named_arguments_and_receiver_context() {
    check("methods.php");
}

#[test]
fn builtin_callbacks_and_observers_keep_contextual_types() {
    check("controls.php");
}

#[test]
fn nested_gathers_use_phpstans_declared_signature_cutoff() {
    check("nested.php");
}

#[test]
fn bounded_template_inputs_keep_their_declared_constraints() {
    check("bounded.php");
}

#[test]
fn upstream_raw_inference_is_array_key_to_mixed_before_conversion() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/rules/data/Functions/data/bug-10612.php");
    let inferred = run_mago(&Project::new(vec![path], Vec::new()), None).unwrap();
    let outer = inferred.dumped.iter().find(|e| e.line == 39).unwrap();
    // PHPStan's array<mixed> is array-key -> mixed. Mago's TKeyedArray::get_id()
    // writes both domains, with a comma and space; this is not a Pun-rendered snapshot.
    assert_eq!(outer.type_id, "array<array-key, mixed>");
}

/// A one-shot plugin error in the speculative walk must propagate, not publish partial artifacts.
#[test]
fn speculative_errors_propagate_and_expression_observers_run_once() {
    use std::borrow::Cow;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use mago_allocator::LocalArena;
    use mago_analyzer::Analyzer;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::error::AnalysisError;
    use mago_analyzer::plugin::context::HookContext;
    use mago_analyzer::plugin::hook::{
        ExpressionHook, ExpressionHookResult, FunctionCallHook, HookResult,
    };
    use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
    use mago_analyzer::plugin::{PluginError, PluginRegistry};
    use mago_analyzer::settings::Settings;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::cst::{Call, Expression, FunctionCall};
    use mago_syntax::parser::parse_file;

    struct FailOnce(AtomicBool);
    impl Provider for FailOnce {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("callback-error", "callback-error", "one-shot failure");
            &META
        }
    }
    impl FunctionCallHook for FailOnce {
        fn before_function_call(
            &self,
            call: &FunctionCall<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            if matches!(call.function, Expression::Identifier(id) if id.value() == b"failGather")
                && self.0.swap(false, Ordering::SeqCst)
            {
                return Err(PluginError::Internal {
                    reason: "raw callback gather failed".into(),
                });
            }
            Ok(ExpressionHookResult::Continue)
        }
    }

    struct Observer(Arc<AtomicUsize>);
    impl Provider for Observer {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "callback-observer",
                "callback-observer",
                "counts body visits",
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
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if let Expression::Call(Call::Function(call)) = expression
                && matches!(call.function, Expression::Identifier(id) if id.value() == b"failGather")
            {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        }
    }

    let file = File::new(
        Cow::Borrowed(b"callback-error.php"),
        FileType::Host,
        None,
        Cow::Borrowed(include_bytes!("fixtures/callback-gather/error.php")),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let version = PHPVersion::new(8, 5, 11);
    let mut codebase = scan_program(&arena, &file, program, &names, version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut registry = PluginRegistry::new();
    registry.register_function_call_hook(FailOnce(AtomicBool::new(true)));
    let observed = Arc::new(AtomicUsize::new(0));
    registry.register_expression_hook(Observer(Arc::clone(&observed)));
    let analyzer = Analyzer::new(
        &arena,
        &file,
        &names,
        &codebase,
        &registry,
        Settings::new(version),
    );
    let mut result = AnalysisResult::new(SymbolReferences::new());
    assert!(matches!(
        analyzer.analyze_with_artifacts(program, &mut result),
        Err(AnalysisError::Plugin(PluginError::Internal { reason })) if reason == "raw callback gather failed"
    ));
    let mut result = AnalysisResult::new(SymbolReferences::new());
    analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    assert_eq!(
        observed.load(Ordering::SeqCst),
        1,
        "only the contextual walk publishes observer events"
    );
}

#[test]
fn upstream_bug_10612() {
    upstream_case(0);
}

#[test]
fn upstream_bug_10612_at_level_five() {
    upstream_case(5);
}

#[test]
fn upstream_bug_10612_at_level_nine() {
    upstream_case(9);
}

fn upstream_case(level: u8) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let root = if level == 0 {
        fixtures.join("rules")
    } else {
        fixtures.join(format!("levels/{level}"))
    };
    let oracle: Value = serde_json::from_slice(
        &std::fs::read(root.join("Functions/CallToFunctionParametersRuleTest.json")).unwrap(),
    )
    .unwrap();
    let case = oracle["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["method"] == "testBug10612")
        .unwrap();
    let mut project = Project::new(
        vec![fixtures.join("rules/data/Functions/data/bug-10612.php")],
        Vec::new(),
    );
    project.level = level;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| (e.line, e.identifier, e.message))
        .collect();
    let want: Vec<_> = case["errors"]
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
    assert_eq!(got, want);
}
