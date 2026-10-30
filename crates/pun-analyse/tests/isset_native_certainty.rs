//! Independent IssetCheck certainty through the public Project API.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

fn records(project: &Project, root: &Path) -> Vec<Value> {
    let mut records: Vec<_> = run(project)
        .unwrap()
        .into_iter()
        .map(|error| {
            json!([
                error.file.replace(root.to_str().unwrap(), "$ROOT"),
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored
            ])
        })
        .collect();
    records.sort_by_cached_key(Value::to_string);
    records
}

fn expected(records: &Value) -> Vec<Value> {
    let mut records: Vec<_> = records
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            json!([
                error["file"],
                error["line"],
                error["identifier"],
                error["message"],
                error.get("tip").unwrap_or(&Value::Null),
                error["ignorable"]
            ])
        })
        .collect();
    records.sort_by_cached_key(Value::to_string);
    records
}

#[test]
fn retained_certainty_matrix_matches_complete_phpstan_records() {
    compare(
        "native-certainty-matrix.php",
        include_str!("fixtures/isset-native-certainty/native-certainty-phpstan.json"),
        false,
    );
}

#[test]
fn fresh_certainty_boundaries_match_complete_phpstan_records() {
    compare(
        "certainty-boundaries.php",
        include_str!("fixtures/isset-native-certainty/certainty-boundaries-phpstan.json"),
        false,
    );
}

#[test]
fn static_reflection_keeps_full_generic_receivers() {
    compare(
        "certainty-static.php",
        include_str!("fixtures/isset-native-certainty/certainty-static-phpstan.json"),
        false,
    );
}

#[test]
fn unnecessary_coalesce_uses_its_independent_full_resolution() {
    compare(
        "certainty-unnecessary.php",
        include_str!("fixtures/isset-native-certainty/certainty-unnecessary-phpstan.json"),
        true,
    );
}

fn compare(source: &str, captured: &str, unnecessary: bool) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/isset-native-certainty");
    let oracle: Value = serde_json::from_str(captured).unwrap();
    let mut failures = Vec::new();
    for (case, captured) in oracle.as_object().unwrap() {
        let parts: Vec<_> = case.split('-').collect();
        let level: u8 = parts[0].parse().unwrap();
        let advanced = parts[1] == "on";
        let certainty = match parts[2] {
            "default" => None,
            mode => Some(mode == "on"),
        };
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        parameters.set(Key::from("checkAdvancedIsset"), Parameter::Bool(advanced));
        if parts[2] != "default" {
            parameters.set(
                Key::from("treatPhpDocTypesAsCertain"),
                Parameter::Bool(parts[2] == "on"),
            );
        }
        let file = root.join(source);
        let mut configured = Project::from_config(&root, &parameters);
        assert_eq!(configured.treat_php_doc_types_as_certain, certainty);
        configured.analysed = vec![file.clone()];
        // Resolve defaults only at run time, including after the caller changes the level.
        configured.level = 0;
        let mut direct = Project::new(vec![file], Vec::new());
        assert_eq!(direct.treat_php_doc_types_as_certain, None);
        direct.check_advanced_isset = Some(advanced);
        direct.treat_php_doc_types_as_certain = certainty;
        let projects = [("from_config", configured), ("new", direct)];
        let want = expected(captured);
        for (constructor, mut project) in projects {
            project.level = level;
            project.php_version = 80500;
            project.check_maybe_undefined_variables = true;
            project.bleeding_edge = unnecessary;
            let got = records(&project, &root);
            if got != want {
                failures.push(json!({
                    "case": format!("{case}/{constructor}"),
                    "extra": got.iter().filter(|error| !want.contains(error)).collect::<Vec<_>>(),
                    "missing": want.iter().filter(|error| !got.contains(error)).collect::<Vec<_>>(),
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

mod native_observations {
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts, plugin::*,
        settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
        ttype::TType,
    };
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{cst::Expression, parser::parse_file};
    use serde_json::{Value, json};

    const SOURCE: &[u8] = include_bytes!("fixtures/isset-native-certainty/native-observations.php");
    type Read = (u32, u32, usize, bool, Option<String>);
    struct Observe(Arc<Mutex<Vec<Read>>>);
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "isset-certainty-native",
                "isset-certainty-native",
                "passive native observations",
            );
            &META
        }
    }
    impl ExpressionHook for Observe {
        fn is_observer(&self) -> bool {
            true
        }
        fn after_expression(
            &self,
            e: &Expression<'_>,
            c: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            let span = e.span();
            let line = SOURCE[..span.start.offset as usize]
                .iter()
                .filter(|&&b| b == b'\n')
                .count()
                + 1;
            self.0.lock().unwrap().push((
                span.start.offset,
                span.end.offset,
                line,
                matches!(e, Expression::Variable(_)),
                c.get_native_expression_type(e)
                    .map(|t| t.get_id().to_string()),
            ));
            Ok(())
        }
    }

    fn analyze(native: bool) -> (AnalysisArtifacts, Vec<Read>, String) {
        let arena = LocalArena::new();
        let file = File::new(
            Cow::Borrowed(b"native-observations.php"),
            FileType::Host,
            None,
            Cow::Borrowed(SOURCE),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let settings = Settings {
            track_native_types: native,
            ..Settings::default()
        };
        let mut codebase = mago_prelude::Prelude::build().metadata;
        codebase.extend(scan_program(
            &arena,
            &file,
            program,
            &names,
            settings.version,
        ));
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let seen = Arc::new(Mutex::new(Vec::new()));
        let mut registry = PluginRegistry::new();
        registry.register_expression_hook(Observe(Arc::clone(&seen)));
        let mut result = AnalysisResult::new(references);
        let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        let observations = std::mem::take(&mut *seen.lock().unwrap());
        (artifacts, observations, format!("{:?}", result.issues))
    }

    // Display adaptation only. Raw Mago arrays have canonical map order, not PHP insertion order.
    fn canonical(s: &str) -> String {
        if s == "array<array-key, mixed>" {
            return "array".to_owned();
        }
        if let Some(s) = s.strip_prefix("int(").and_then(|s| s.strip_suffix(')')) {
            return s.to_owned();
        }
        if let Some(body) = s.strip_prefix("array{").and_then(|s| s.strip_suffix('}')) {
            let mut slots: Vec<_> = body
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let (key, value) = s.split_once(':').unwrap();
                    format!("{}:{}", key.trim_matches('\''), canonical(value.trim()))
                })
                .collect();
            slots.sort();
            return format!("array{{{}}}", slots.join(","));
        }
        s.to_owned()
    }

    #[test]
    fn live_native_boundaries_are_explicit_without_replaying_or_changing_full_results() {
        let oracle: Value = serde_json::from_str(include_str!(
            "fixtures/isset-native-certainty/native-observations-phpstan.json"
        ))
        .unwrap();
        let (on, reads, issues) = analyze(true);
        let (off, disabled, off_issues) = analyze(false);
        assert_eq!(on.expression_types, off.expression_types);
        assert_eq!(on.symbol_references, off.symbol_references);
        assert_eq!(on.call_terminations, off.call_terminations);
        assert_eq!(issues, off_issues);
        assert_eq!(
            reads
                .iter()
                .map(|r| (r.0, r.1, r.2, r.3))
                .collect::<Vec<_>>(),
            disabled
                .iter()
                .map(|r| (r.0, r.1, r.2, r.3))
                .collect::<Vec<_>>()
        );
        assert!(disabled.iter().all(|r| r.4.is_none()));
        let mut counts = BTreeMap::new();
        for r in &reads {
            *counts.entry((r.0, r.1)).or_insert(0) += 1;
        }
        assert!(
            counts.values().all(|&n| n == 1),
            "replayed expression: {counts:?}"
        );
        let source = std::str::from_utf8(SOURCE).unwrap();
        let mut evidence = Vec::new();
        for expected in oracle["0-on-default"].as_array().unwrap() {
            assert_eq!(expected["identifier"], "phpstan.dumpNativeType");
            let line = expected["line"].as_u64().unwrap() as usize;
            let found: Vec<_> = reads.iter().filter(|r| r.2 == line && r.3).collect();
            assert_eq!(found.len(), 1, "operand visits on line {line}");
            let native = &found[0].4;
            let available = !source
                .lines()
                .nth(line - 1)
                .unwrap()
                .contains("// unavailable");
            assert_eq!(
                native.is_some(),
                available,
                "native availability on line {line}"
            );
            let answer = expected["message"]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .unwrap();
            if let Some(native) = native {
                assert_eq!(
                    canonical(native),
                    canonical(answer),
                    "native operand on line {line}"
                );
            }
            evidence
                .push(json!({"line":line,"phpstan":answer,"native":native,"available":available}));
        }
        println!(
            "{}",
            json!({"observations":evidence,"once_only_events":reads.len()})
        );
    }
}
