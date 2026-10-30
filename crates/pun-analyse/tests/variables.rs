//! DefinedVariableRule and CompactVariablesRule through the public analyser. The expectations are PHPStan
//! 2.3.1's exported level-0 diagnostics, not Mago's native undefined-variable issues.
//! The custom oracle runs use PHP's `-d phpstan.restarted=1 -d register_argc_argv=0|1` and PHPStan's `--debug`:
//! the turbo restart otherwise drops the INI setting that supplies cliArgumentsVariablesRegistered.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn block_pollution_matches_resolved_phpstan_configuration() {
    use pun_phpstan::config::{Options, resolve};

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/block-pollution");
    let expected: Value = serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let selected = |id: &str| matches!(id, "variable.undefined" | "phpstan.dumpType" | "include.fileNotFound" | "match.alwaysTrue" | "match.alwaysFalse" | "match.unhandled" | "deadCode.unreachable");
    let mut failures = Vec::new();
    for mode in ["default", "enabled", "disabled", "quiet", "independent", "loops-only", "secondary", "secondary-enabled"] {
        let config = resolve(&Options {
            config: Some(root.join(format!("{mode}.neon")).to_string_lossy().into_owned()),
            ..Options::for_project(&root.to_string_lossy())
        }).unwrap();
        let project = Project::from_config(&root, &config.parameters);
        let mut actual: Vec<_> = run(&project).unwrap().into_iter()
            .filter(|e| selected(&e.identifier))
            .map(|e| (Path::new(&e.file).file_name().unwrap().to_string_lossy().into_owned(), e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
            .collect();
        actual.sort();
        let wanted: Vec<_> = expected[mode].as_array().unwrap().iter()
            .filter(|e| selected(e[2].as_str().unwrap())).cloned().collect();
        let actual = serde_json::to_value(actual).unwrap();
        if actual != serde_json::to_value(&wanted).unwrap() {
            let actual = actual.as_array().unwrap();
            let missing: Vec<_> = wanted.iter().filter(|e| !actual.contains(e)).collect();
            let extra: Vec<_> = actual.iter().filter(|e| !wanted.contains(e)).collect();
            failures.push(format!("{mode}: {}/{} exact\nmissing {missing:?}\nextra {extra:?}", wanted.len() - missing.len(), wanted.len()));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn block_pollution_preserves_paired_native_types_and_single_walk() {
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::Analyzer;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::plugin::{ExpressionHook, HookContext, HookResult, PluginRegistry, Provider, ProviderMeta};
    use mago_analyzer::settings::Settings;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::TType;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::cst::Expression;
    use mago_syntax::parser::parse_file;

    struct Observe {
        marked: BTreeMap<u32, i64>,
        found: Arc<Mutex<BTreeMap<i64, Option<String>>>>,
    }
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new("block-native", "block-native", "paired block exit types");
            &META
        }
    }
    impl ExpressionHook for Observe {
        fn after_expression(&self, expression: &Expression<'_>, context: &mut HookContext<'_, '_>) -> HookResult<()> {
            if let Some(line) = self.marked.get(&expression.span().start.offset) {
                let ty = context.get_native_expression_type(expression).map(|ty| {
                    let id = ty.get_id().to_string();
                    let mut types: Vec<_> = id.split('|').collect();
                    types.sort();
                    types.join("|")
                });
                assert!(self.found.lock().unwrap().insert(*line, ty).is_none(), "body read visited twice");
            }
            Ok(())
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/block-pollution");
    let source = std::fs::read_to_string(root.join("native.php")).unwrap();
    let oracle: Value = serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut marked = BTreeMap::new();
    let mut offset = 0;
    for (line, text) in source.split_inclusive('\n').enumerate() {
        if text.contains("// native") {
            marked.insert((offset + text.len() - text.trim_start().len()) as u32, line as i64 + 1);
        }
        offset += text.len();
    }
    let file = File::ephemeral(b"native.php".as_slice().into(), source.as_bytes().to_vec().into());
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut codebase = scan_program(&arena, &file, program, &names, Settings::default().version);
    let mut references = SymbolReferences::new();
    populate_codebase(&mut codebase, &mut references, Default::default(), Default::default());
    for (mode, pollution) in [("enabled", true), ("disabled", false)] {
        let expected: BTreeMap<_, _> = oracle[mode].as_array().unwrap().iter()
            .filter(|e| e[0] == "native.php" && e[2] == "phpstan.dumpNativeType")
            .map(|e| {
                let mut types: Vec<_> = e[3].as_str().unwrap().strip_prefix("Dumped type: ").unwrap()
                    .split('|').map(|t| format!("int({})", t.parse::<i64>().unwrap())).collect();
                types.sort();
                (e[1].as_i64().unwrap(), Some(types.join("|")))
            }).collect();
        assert_eq!(expected.len(), marked.len());
        let mut previous = None;
        for native in [false, true] {
            let found = Arc::new(Mutex::new(BTreeMap::new()));
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(Observe { marked: marked.clone(), found: Arc::clone(&found) });
            let settings = Settings { track_native_types: native, pollute_scope_with_block: pollution, ..Settings::default() };
            let mut result = AnalysisResult::new(references.clone());
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result).unwrap();
            let wanted = if native { expected.clone() } else { expected.keys().map(|line| (*line, None)).collect() };
            assert_eq!(*found.lock().unwrap(), wanted, "{mode}, native={native}");
            let full = (artifacts.expression_types, format!("{:?}", result.issues));
            if let Some(previous) = &previous { assert_eq!(&full, previous); } else { previous = Some(full); }
        }
    }
}

#[test]
fn foreach_pollution_matches_resolved_phpstan_configuration() {
    use pun_phpstan::config::{Options, resolve};

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foreach-pollution");
    let expected: Value = serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let selected = |id: &str| matches!(id, "variable.undefined" | "phpstan.dumpType");
    let mut failures = Vec::new();
    for mode in ["default", "enabled", "disabled", "quiet", "independent"] {
        let config = resolve(&Options {
            config: Some(root.join(format!("{mode}.neon")).to_string_lossy().into_owned()),
            ..Options::for_project(&root.to_string_lossy())
        }).unwrap();
        let project = Project::from_config(&root, &config.parameters);
        let mut actual: Vec<_> = run(&project).unwrap().into_iter()
            .filter(|e| selected(&e.identifier))
            .map(|e| (Path::new(&e.file).file_name().unwrap().to_string_lossy().into_owned(), e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
            .collect();
        actual.sort();
        let wanted: Vec<_> = expected[mode].as_array().unwrap().iter()
            .filter(|e| selected(e[2].as_str().unwrap())).cloned().collect();
        let actual = serde_json::to_value(actual).unwrap();
        if actual != serde_json::to_value(&wanted).unwrap() {
            let actual = actual.as_array().unwrap();
            let missing: Vec<_> = wanted.iter().filter(|e| !actual.contains(e)).collect();
            let extra: Vec<_> = actual.iter().filter(|e| !wanted.contains(e)).collect();
            failures.push(format!("{mode}: {}/{} exact\nmissing {missing:?}\nextra {extra:?}", wanted.len() - missing.len(), wanted.len()));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn loop_initial_pollution_matches_resolved_phpstan_configuration() {
    use pun_phpstan::config::{Options, resolve};

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/loop-pollution");
    let expected: Value = serde_json::from_slice(&std::fs::read(root.join("expected.json")).unwrap()).unwrap();
    let selected = |id: &str| matches!(id, "variable.undefined" | "phpstan.dumpType");
    let mut failures = Vec::new();
    for mode in ["default", "enabled", "disabled", "quiet"] {
        let config = resolve(&Options {
            config: Some(root.join(format!("{mode}.neon")).to_string_lossy().into_owned()),
            ..Options::for_project(&root.to_string_lossy())
        }).unwrap();
        let project = Project::from_config(&root, &config.parameters);
        let mut actual: Vec<_> = run(&project).unwrap().into_iter()
            .filter(|e| selected(&e.identifier))
            .map(|e| (Path::new(&e.file).file_name().unwrap().to_string_lossy().into_owned(), e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
            .collect();
        actual.sort();
        let wanted: Vec<_> = expected[mode].as_array().unwrap().iter()
            .filter(|e| selected(e[2].as_str().unwrap())).cloned().collect();
        let actual = serde_json::to_value(actual).unwrap();
        if actual != serde_json::to_value(&wanted).unwrap() {
            let actual = actual.as_array().unwrap();
            let missing: Vec<_> = wanted.iter().filter(|e| !actual.contains(e)).collect();
            let extra: Vec<_> = actual.iter().filter(|e| !wanted.contains(e)).collect();
            failures.push(format!("{mode}: {}/{} exact\nmissing {missing:?}\nextra {extra:?}", wanted.len() - missing.len(), wanted.len()));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

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
    use mago_analyzer::code::IssueCode;
    use mago_analyzer::error::AnalysisError;
    use mago_analyzer::plugin::context::InvocationInfo;
    use mago_analyzer::plugin::{
        ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginError, PluginRegistry,
        Provider, ProviderMeta,
    };
    use mago_analyzer::settings::Settings;
    use mago_codex::populator::populate_codebase;
    use mago_codex::scanner::scan_program;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_reporting::{Annotation, Issue};
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
        fn before_arguments(
            &self,
            invocation: &InvocationInfo<'_, '_, '_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if invocation.function_name() == "abortArguments" {
                return Err(PluginError::Internal {
                    reason: "argument scope failed".into(),
                });
            }
            if invocation.function_name() == "documented" {
                context.report(
                    IssueCode::UndefinedVariable,
                    Issue::error("resolved argument diagnostic")
                        .with_annotation(Annotation::primary(invocation.span())),
                );
            }
            Ok(())
        }

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
    const BOOLEAN_AND: &str = "<?php
function transfer(bool $flag, string $input): void {
    if ($flag && preg_match('/x/', $input, $matches)) {
        echo $matches;
    } else {
        echo $matches;
    }
}";
    const BOOLEAN_OR: &str = "<?php
function transfer(bool $flag, string $input): void {
    $matches = [];
    unset($matches);
    if ($flag || preg_match('/x/', $input, $matches)) {
        echo $matches;
    } else {
        echo $matches;
    }
}";
    const BOOLEAN_POST: &str = "<?php
function transfer(bool $flag, string $input): void {
    $flag && preg_match('/x/', $input, $matches);
    echo $matches;
    preg_match('/x/', $input, $matches);
    echo $matches;
}";
    const BOOLEAN_NESTED: &str = "<?php
function transfer(bool $flag, bool $other, string $input): void {
    if ($flag && ($other || preg_match('/x/', $input, $matches))) {
        echo $matches;
    } else {
        echo $matches;
    }
}";
    let prelude = mago_prelude::Prelude::build();
    for (source, check_throws, threshold, expected) in [
        (THROWS, false, 200, Some([(true, false), (false, true)])),
        (THROWS, true, 200, Some([(true, false), (false, true)])),
        // Exhausting the condition algebra budget cannot discard the continuing implicit else edge.
        (BRANCH, false, 0, Some([(false, false), (true, false)])),
        // Fresh PHPStan 2.3.1 reads: &&'s true edge and ||'s false edge ran the output
        // argument; the opposite edge and the post-expression join may have skipped it.
        (
            BOOLEAN_AND,
            false,
            200,
            Some([(true, false), (false, false)]),
        ),
        (
            BOOLEAN_OR,
            false,
            200,
            Some([(false, false), (true, false)]),
        ),
        (
            BOOLEAN_POST,
            false,
            200,
            Some([(false, false), (true, false)]),
        ),
        (
            BOOLEAN_NESTED,
            false,
            200,
            Some([(false, false), (false, false)]),
        ),
        (
            "<?php switch (true) { case 777: echo 'unreachable'; }",
            false,
            200,
            None,
        ),
        (
            "<?php { switch (true) { case 777: echo 'unreachable'; } }",
            false,
            200,
            None,
        ),
        (
            "<?php function abortArguments($value): void {} abortArguments($argument);",
            false,
            200,
            None,
        ),
        (
            "<?php function abortArguments($value): void {} { abortArguments($argument); }",
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
            pollute_scope_with_block: false,
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
            if source == THROWS {
                assert!(result.issues.iter().any(|issue| {
                    issue.message == "resolved argument diagnostic"
                        && issue.code.as_deref() == Some(IssueCode::UndefinedVariable.as_str())
                }));
            }
            assert_eq!(
                *found.lock().unwrap(),
                BTreeMap::from([(reads[0], expected[0]), (reads[1], expected[1])]),
                "check_throws={check_throws}",
            );
        } else {
            let expected = if source.contains("abortArguments") {
                "argument scope failed"
            } else {
                "switch condition failed"
            };
            assert!(
                matches!(analysed, Err(AnalysisError::Plugin(PluginError::Internal { reason }))
                if reason == expected)
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
