//! Strict PHPStan 2.3.1 value-read/write contracts for closure imports. No gap allowances.

use std::path::Path;

use serde_json::Value;

fn check(case: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/closure-capture-writes");
    let mut expected: Value = serde_json::from_str(include_str!(
        "fixtures/closure-capture-writes/expected.json"
    ))
    .unwrap();
    let unreachable: Value = serde_json::from_str(include_str!(
        "fixtures/closure-capture-writes/unreachable.json"
    ))
    .unwrap();
    expected
        .as_object_mut()
        .unwrap()
        .extend(unreachable.as_object().unwrap().clone());
    let file = match case {
        "original-closure" => root.join("../rules/data/Functions/data/unused-closure-uses.php"),
        "original-constructor" => {
            root.join("../rules/data/Classes/data/unused-constructor-parameters.php")
        }
        _ => root.join(format!("{case}.php")),
    };
    let levels: &[u8] = if case == "captures" { &[0, 9] } else { &[0, 1] };
    let mut failures = Vec::new();
    for &level in levels {
        for precise in [false, true] {
            let mut project = pun_analyse::Project::new(vec![file.clone()], vec![]);
            project.level = level;
            project.php_version = 80500;
            // config.level1.neon enables this independently of the numeric level field.
            project.options.set(
                pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
                level >= 1,
            );
            project.options.set(
                pun_analyse::ProjectOptions::PRECISE_UNUSED_PARAMETER_LINE,
                precise,
            );
            let mut actual: Vec<_> = pun_analyse::run(&project)
                .unwrap()
                .into_iter()
                .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
                .collect();
            actual.sort();
            let actual = serde_json::to_value(actual).unwrap();
            let key = format!("{case}-{level}-{}", u8::from(precise));
            if actual != expected[&key] {
                let want = expected[&key].as_array().unwrap();
                let got = actual.as_array().unwrap();
                failures.push(serde_json::json!({
                    "case": key,
                    "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                    "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                }));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn conditional_entry_captures_keep_all_complete_records() {
    check("captures");
}

#[test]
fn replacing_imported_values_and_asymmetric_joins() {
    check("overwrites");
}

#[test]
fn reads_before_writes_aliases_and_dynamic_operations() {
    check("reads");
}

#[test]
fn semantic_producer_boundaries() {
    check("boundaries");
}

#[test]
fn original_unused_closure_control() {
    check("original-closure");
}

#[test]
fn original_unused_constructor_control() {
    check("original-constructor");
}

#[test]
fn skipped_closure_bodies_do_not_get_lexical_capture_reads() {
    check("unreachable");
}

#[test]
fn capture_producer_matches_independently_observed_write_identities() {
    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, plugin::PluginRegistry, settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{cst::*, parser::parse_file, walker::MutWalker};
    use std::borrow::Cow;

    struct Captures<'a> {
        source: &'a [u8],
        writes: &'a foldhash::HashMap<
            mago_span::Span,
            mago_analyzer::variable_flow::VariableWritesNode,
        >,
        records: Vec<Value>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Captures<'_> {
        fn walk_in_closure(&mut self, closure: &'ast Closure<'arena>, _: &mut ()) {
            let Some(uses) = &closure.use_clause else {
                return;
            };
            // The oracle emits no VariableWritesNode for closures in skipped catches. A missing
            // completed body elsewhere still fails the exact expected-record comparison below.
            let Some(node) = self.writes.get(&closure.span()) else {
                return;
            };
            for capture in &uses.variables {
                let line = |span: mago_span::Span| {
                    self.source[..span.start.offset as usize]
                        .iter()
                        .filter(|byte| **byte == b'\n')
                        .count()
                        + 1
                };
                let variable = capture.variable.span();
                let name = mago_word::word(capture.variable.name);
                let tracked = node.writes.contains_key(&variable);
                self.records.push(serde_json::json!({
                    "closureLine": line(closure.span()), "captureLine": line(variable),
                    "name": String::from_utf8_lossy(&capture.variable.name[1..]),
                    "byRef": capture.ampersand.is_some(), "tracked": tracked,
                    "read": tracked.then(|| node.read.contains(&variable)),
                    "used": tracked.then(|| node.used.contains(&variable)),
                    "referenced": node.is_variable_referenced(name),
                    "allNames": node.all_names, "opaque": node.opaque,
                }));
            }
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/closure-capture-writes");
    let mut expected: Value = serde_json::from_str(include_str!(
        "fixtures/closure-capture-writes/producer.json"
    ))
    .unwrap();
    let controls: Value = serde_json::from_str(include_str!(
        "fixtures/closure-capture-writes/producer-controls.json"
    ))
    .unwrap();
    expected
        .as_object_mut()
        .unwrap()
        .extend(controls.as_object().unwrap().clone());
    let prelude = mago_prelude::Prelude::build();
    let mut failures = Vec::new();
    for case in [
        "captures",
        "overwrites",
        "reads",
        "boundaries",
        "flow-controls",
    ] {
        let arena = LocalArena::new();
        let source = std::fs::read(root.join(format!("{case}.php"))).unwrap();
        let file = File::ephemeral(
            Cow::Owned(format!("{case}.php").into_bytes()),
            Cow::Owned(source.clone()),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let mut codebase = prelude.metadata.clone();
        codebase.extend(scan_program(
            &arena,
            &file,
            program,
            &names,
            Default::default(),
        ));
        populate_codebase(
            &mut codebase,
            &mut SymbolReferences::new(),
            Default::default(),
            Default::default(),
        );
        let registry = PluginRegistry::new();
        for native in [false, true] {
            let analyze = |enabled| {
                let settings = Settings {
                    track_variable_flow: enabled,
                    track_native_types: native,
                    ..Default::default()
                };
                let mut result = AnalysisResult::new(SymbolReferences::new());
                let artifacts =
                    Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                        .analyze_with_artifacts(program, &mut result)
                        .unwrap();
                (artifacts, format!("{:?}", result.issues))
            };
            let (enabled, on_issues) = analyze(true);
            let (disabled, off_issues) = analyze(false);
            assert_eq!(
                enabled.expression_types, disabled.expression_types,
                "{case}, native={native}"
            );
            assert_eq!(on_issues, off_issues, "{case}, native={native}");
            assert!(disabled.variable_writes.is_empty());
            let mut captures = Captures {
                source: &source,
                writes: &enabled.variable_writes,
                records: Vec::new(),
            };
            captures.walk_program(program, &mut ());
            captures.records.sort_by_key(Value::to_string);
            let mut want = expected[case].as_array().unwrap().clone();
            want.sort_by_key(Value::to_string);
            if captures.records != want {
                failures.push(serde_json::json!({
                    "case": case, "native": native,
                    "missing": want.iter().filter(|record| !captures.records.contains(record)).collect::<Vec<_>>(),
                    "extra": captures.records.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
                }));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn completed_semantic_overrides_do_not_invent_unwalked_child_reads() {
    use mago_allocator::LocalArena;
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_syntax::{cst::Expression, parser::parse_file};
    use std::borrow::Cow;

    // This is a real semantic extension: a parenthesized expression denotes the literal 7,
    // without evaluating its operand. VariableFlowBuilder::child may consume only stored results.
    struct LiteralParentheses;
    impl Provider for LiteralParentheses {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("literal-parentheses", "test", "semantic replacement");
            &META
        }
    }
    impl ExpressionHook for LiteralParentheses {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            Ok(if matches!(expression, Expression::Parenthesized(_)) {
                ExpressionHookResult::SkipWithType(mago_codex::ttype::get_literal_int(7))
            } else {
                ExpressionHookResult::Continue
            })
        }
    }

    let source = include_bytes!("fixtures/closure-capture-writes/semantic-hook.php");
    let capture_start = source
        .windows(b"use ($value)".len())
        .position(|bytes| bytes == b"use ($value)")
        .unwrap()
        + 5;
    let parent_start = source
        .windows(b"($value)".len())
        .rposition(|bytes| bytes == b"($value)")
        .unwrap();
    let prelude = mago_prelude::Prelude::build();
    for native in [false, true] {
        for replace in [false, true] {
            let arena = LocalArena::new();
            let file = File::ephemeral(
                Cow::Borrowed(b"semantic-hook.php"),
                Cow::Borrowed(source.as_slice()),
            );
            let program = parse_file(&arena, &file);
            assert!(!program.has_errors());
            let names = NameResolver::new(&arena).resolve(program);
            let mut codebase = prelude.metadata.clone();
            codebase.extend(scan_program(
                &arena,
                &file,
                program,
                &names,
                Default::default(),
            ));
            populate_codebase(
                &mut codebase,
                &mut SymbolReferences::new(),
                Default::default(),
                Default::default(),
            );
            let mut registry = PluginRegistry::new();
            if replace {
                registry.register_expression_hook(LiteralParentheses);
            }
            let settings = Settings {
                track_variable_flow: true,
                track_native_types: native,
                ..Default::default()
            };
            let mut result = AnalysisResult::new(SymbolReferences::new());
            let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
            assert_eq!(artifacts.variable_writes.len(), 1);
            let node = artifacts.variable_writes.values().next().unwrap();
            let capture = node
                .writes
                .values()
                .find(|write| write.name == mago_word::word("$value"))
                .unwrap();
            assert_eq!(capture.node.start.offset as usize, capture_start);
            assert_eq!(capture.node.end.offset as usize, capture_start + 6);
            assert_eq!(
                node.read.contains(&capture.node),
                !replace,
                "native={native}, replace={replace}"
            );
            assert!(!node.used.contains(&capture.node));
            assert_eq!(
                artifacts
                    .expression_types
                    .keys()
                    .any(|span| span.0 as usize == parent_start + 1),
                !replace
            );
            let ty = artifacts
                .expression_types
                .iter()
                .find(|(span, _)| span.0 as usize == parent_start)
                .unwrap()
                .1;
            assert_eq!(
                ty.get_single_literal_int_value(),
                Some(if replace { 7 } else { 1 })
            );
        }
    }
}
