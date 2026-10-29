//! Function-entry conditional parameter holders, through public diagnostics and hooks.

use std::path::Path;

use serde_json::Value;

fn check(case: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parameter-conditions");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/parameter-conditions/expected.json")).unwrap();
    let file = if case == "bug-12875" {
        root.join("../rules/data/Methods/data/bug-12875.php")
    } else {
        root.join(format!("{case}.php"))
    };
    let mut failures = Vec::new();
    for level in [0, 9] {
        let mut project = pun_analyse::Project::new(vec![file.clone()], vec![]);
        project.level = level;
        project.php_version = 80500;
        let mut actual: Vec<_> = pun_analyse::run(&project)
            .unwrap()
            .into_iter()
            .map(|e| (e.line, e.identifier, e.message, e.tip, e.can_be_ignored))
            .collect();
        actual.sort();
        let actual = serde_json::to_value(actual).unwrap();
        let key = format!("{case}-{level}");
        if actual != oracle[&key] {
            let want = oracle[&key].as_array().unwrap();
            let got = actual.as_array().unwrap();
            failures.push(serde_json::json!({
                "case": key,
                "missing": want.iter().filter(|record| !got.contains(record)).collect::<Vec<_>>(),
                "extra": got.iter().filter(|record| !want.contains(record)).collect::<Vec<_>>(),
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

#[test]
fn original_dynamic_method_regression_matches_complete_records() {
    check("bug-12875");
}

#[test]
fn entry_forward_references_negation_and_writes_match_complete_records() {
    check("entry");
}

#[test]
fn captures_and_shadowing_match_complete_records() {
    check("captures");
}

#[test]
fn conditional_guards_use_shared_type_subtraction() {
    check("subtraction");
}

#[test]
fn parameter_consequences_do_not_narrow_native_captures_or_add_visits() {
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
        ttype::TType,
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{cst::Expression, parser::parse_file};

    type Reads = BTreeMap<u32, (String, Option<String>, usize)>;
    struct Observer(Arc<Mutex<Reads>>);
    impl Provider for Observer {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "parameter-captures",
                "parameter-captures",
                "full and native capture types",
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
            if !matches!(expression, Expression::Variable(_)) {
                return Ok(());
            }
            let source = include_bytes!("fixtures/parameter-conditions/native.php");
            let span = expression.span();
            let tail = &source[span.end.offset as usize..];
            if !tail.starts_with(b"; // observe") && !tail.starts_with(b" /* observe */") {
                return Ok(());
            }
            let full = context
                .get_expression_type(expression)
                .unwrap()
                .get_id()
                .to_string();
            let native = context
                .get_native_expression_type(expression)
                .map(|ty| ty.get_id().to_string());
            let mut reads = self.0.lock().unwrap();
            let previous = reads.get(&span.start.offset).map_or(0, |read| read.2);
            reads.insert(span.start.offset, (full, native, previous + 1));
            Ok(())
        }
    }

    let source = include_bytes!("fixtures/parameter-conditions/native.php");
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/parameter-conditions/expected.json")).unwrap();
    let mut previous = None;
    for native in [false, true] {
        let arena = LocalArena::new();
        let file = File::ephemeral(
            Cow::Borrowed(b"native.php"),
            Cow::Borrowed(source.as_slice()),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let mut settings = Settings::default();
        settings.track_native_types = native;
        let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let observed = Arc::new(Mutex::new(Reads::default()));
        let mut registry = PluginRegistry::new();
        registry.register_expression_hook(Observer(Arc::clone(&observed)));
        let mut result = AnalysisResult::new(references);
        Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        let reads = observed.lock().unwrap();
        assert_eq!(
            reads.len(),
            expected["native-types"].as_object().unwrap().len()
        );
        let sorted_union = |value: &str| {
            let mut parts: Vec<_> = value.split('|').collect();
            parts.sort();
            parts.join("|")
        };
        for (start, (full, actual_native, visits)) in reads.iter() {
            let want = &expected["native-types"][start.to_string()];
            assert_eq!(
                sorted_union(full),
                sorted_union(want["full"].as_str().unwrap()),
                "full at {start}"
            );
            assert_eq!(*visits, 1, "visits at {start}");
            let want_native = native.then(|| sorted_union(want["native"].as_str().unwrap()));
            assert_eq!(
                actual_native.as_deref().map(sorted_union),
                want_native,
                "native at {start}"
            );
        }
        let full_and_visits: Vec<_> = reads
            .iter()
            .map(|(start, (full, _, count))| (*start, full.clone(), *count))
            .collect();
        if let Some(previous) = &previous {
            assert_eq!(&full_and_visits, previous);
        }
        previous = Some(full_and_visits);
    }
}
