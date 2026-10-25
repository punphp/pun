//! DefinedVariableRule and CompactVariablesRule through the public analyser. The expectations are PHPStan
//! 2.3.1's exported level-0 diagnostics, not Mago's native undefined-variable issues.
//! The custom oracle runs use PHP's `-d phpstan.restarted=1 -d register_argc_argv=0|1` and PHPStan's `--debug`:
//! the turbo restart otherwise drops the INI setting that supplies cliArgumentsVariablesRegistered.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn variable_contexts_and_configuration_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("variables.json")).unwrap()).unwrap();
    let mut failures = Vec::new();
    for (name, level, cli, maybe, non_stringable) in [
        ("level0", 0, true, false, false),
        ("edge", 2, false, true, true),
        ("maybe", 0, true, true, false),
        ("quiet", 2, false, false, true),
    ] {
        let mut params = pun_phpstan::Array::new();
        params.set("level".into(), pun_phpstan::Value::Int(level));
        params.set(
            "cliArgumentsVariablesRegistered".into(),
            pun_phpstan::Value::Bool(cli),
        );
        params.set(
            "checkMaybeUndefinedVariables".into(),
            pun_phpstan::Value::Bool(maybe),
        );
        let mut project = Project::from_config(&root, &params);
        project.analysed = vec![root.join("variables.php")];
        project.check_non_stringable_dynamic_access = non_stringable;
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| e.identifier.starts_with("variable."))
            .map(|e| (e.line, e.identifier, e.message))
            .collect();
        let mut want: Vec<_> = expected[name]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e[0].as_i64().unwrap(),
                    e[1].as_str().unwrap().to_string(),
                    e[2].as_str().unwrap().to_string(),
                )
            })
            .collect();
        got.sort();
        want.sort();
        if got != want {
            failures.push(format!("{name}\n got {got:?}\nwant {want:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn engine_exception_reporting_preserves_certainty_and_switch_errors_propagate() {
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::Analyzer;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::error::AnalysisError;
    use mago_analyzer::plugin::{
        ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginError, PluginRegistry,
        Provider, ProviderMeta,
    };
    use mago_analyzer::settings::Settings;
    use mago_codex::populator::populate_codebase;
    use mago_codex::scanner::scan_program;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::cst::{Expression, Literal, Variable};
    use mago_syntax::parser::parse_file;

    struct Observe {
        reads: Vec<u32>,
        found: Arc<Mutex<BTreeMap<u32, (bool, bool)>>>,
    }
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "variable-transfers",
                "variable-transfers",
                "live scope certainty",
            );
            &META
        }
    }
    impl ExpressionHook for Observe {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            if matches!(expression, Expression::Literal(Literal::Integer(i)) if i.value == Some(777))
            {
                return Err(PluginError::Internal {
                    reason: "switch condition failed".into(),
                });
            }
            if let Expression::Variable(Variable::Direct(variable)) = expression
                && self.reads.contains(&variable.span().start.offset)
            {
                let certainty = context
                    .block_context()
                    .variable_certainty(mago_word::word(variable.name));
                self.found.lock().unwrap().insert(
                    variable.span().start.offset,
                    (certainty.is_defined(), certainty.is_absent()),
                );
            }
            Ok(ExpressionHookResult::Continue)
        }
    }

    // PHPStan 2.3.1 reports only "Undefined variable: $after" in the catch. Enabling Mago's
    // separate exception diagnostics must not change that scope or invent a pre-call assignment.
    const THROWS: &str = "<?php
/** @throws RuntimeException */
function documented(): void {}
function transfer(): void {
    try {
        $before = 1;
        documented();
        $after = 1;
    } catch (RuntimeException $caught) {
        echo $before;
        echo $after;
    }
}";
    const BRANCH: &str = "<?php
function budget(bool $flag): void {
    if ($flag) { $before = 1; }
    $after = 1;
    echo $before;
    echo $after;
}";
    let prelude = mago_prelude::Prelude::build();
    for (source, check_throws, threshold, expected) in [
        (THROWS, false, 200, Some([(true, false), (false, true)])),
        (THROWS, true, 200, Some([(true, false), (false, true)])),
        // Exhausting the condition algebra budget cannot discard the continuing implicit else edge.
        (BRANCH, false, 0, Some([(false, false), (true, false)])),
        (
            "<?php switch (true) { case 777: echo 'unreachable'; }",
            false,
            200,
            None,
        ),
    ] {
        let file = File::ephemeral(
            Cow::Borrowed(b"variable-engine.php"),
            Cow::Borrowed(source.as_bytes()),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let settings = Settings {
            check_throws,
            formula_size_threshold: threshold,
            ..Settings::default()
        };
        let mut codebase = prelude.metadata.clone();
        codebase.extend(scan_program(
            &arena,
            &file,
            program,
            &names,
            settings.version,
        ));
        let mut references = prelude.symbol_references.clone();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let reads: Vec<_> = source
            .match_indices("echo $")
            .map(|(offset, _)| offset as u32 + 5)
            .collect();
        let found = Arc::new(Mutex::new(BTreeMap::new()));
        let mut registry = PluginRegistry::with_library_providers();
        registry.register_expression_hook(Observe {
            reads: reads.clone(),
            found: Arc::clone(&found),
        });
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
        let mut result = AnalysisResult::new(references);
        let analysed = analyzer.analyze_with_artifacts(program, &mut result);
        if let Some(expected) = expected {
            analysed.unwrap();
            assert_eq!(
                *found.lock().unwrap(),
                BTreeMap::from([(reads[0], expected[0]), (reads[1], expected[1])]),
                "check_throws={check_throws}",
            );
        } else {
            assert!(
                matches!(analysed, Err(AnalysisError::Plugin(PluginError::Internal { reason }))
                if reason == "switch condition failed")
            );
        }
    }
}

#[test]
fn variable_flow_boundaries_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for fixture in ["variables-flow", "variables-transfers"] {
        let expected: Value =
            serde_json::from_slice(&std::fs::read(root.join(format!("{fixture}.json"))).unwrap())
                .unwrap();
        let mut params = pun_phpstan::Array::new();
        params.set("level".into(), pun_phpstan::Value::Int(2));
        params.set(
            "checkMaybeUndefinedVariables".into(),
            pun_phpstan::Value::Bool(true),
        );
        let mut project = Project::from_config(&root, &params);
        project.analysed = vec![root.join(format!("{fixture}.php"))];
        let mut got: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .filter(|e| e.identifier.starts_with("variable."))
            .map(|e| (e.line, e.identifier, e.message))
            .collect();
        let mut want: Vec<_> = expected
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                (
                    e[0].as_i64().unwrap(),
                    e[1].as_str().unwrap().to_string(),
                    e[2].as_str().unwrap().to_string(),
                )
            })
            .collect();
        got.sort();
        want.sort();
        assert_eq!(got, want, "{fixture}");
    }
}

#[test]
fn undefined_variables_match_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let mut failures = Vec::new();
    for rule in ["CompactVariablesRuleTest", "DefinedVariableRuleTest"] {
        let test: Value = serde_json::from_slice(
            &std::fs::read(root.join(format!("Variables/{rule}.json"))).unwrap(),
        )
        .unwrap();
        for case in test["cases"].as_array().unwrap() {
            let files = case["files"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| root.join("data").join(p.as_str().unwrap()))
                .collect();
            let mut got: Vec<_> = run(&Project::new(files, Vec::new()))
                .unwrap()
                .into_iter()
                .filter(|e| e.identifier.starts_with("variable."))
                .map(|e| (e.line, e.identifier, e.message))
                .collect();
            let mut want: Vec<_> = case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|e| e["identifier"].as_str().unwrap().starts_with("variable."))
                .map(|e| {
                    (
                        e["line"].as_i64().unwrap(),
                        e["identifier"].as_str().unwrap().to_string(),
                        e["message"].as_str().unwrap().to_string(),
                    )
                })
                .collect();
            got.sort();
            want.sort();
            if got != want {
                failures.push(format!(
                    "{rule}::{}\n got {got:?}\nwant {want:?}",
                    case["method"]
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
