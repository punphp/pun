//! Native observation authority and source-order conversion through public analyzers.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::{Value, json};

#[test]
fn project_keeps_literal_nested_integer_and_reassigned_array_records() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native-array-order");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/native-array-order/expected.json")).unwrap();
    let mut failures = Vec::new();
    for level in [0, 4, 5] {
        let mut direct = Project::new(vec![root.join("project.php")], Vec::new());
        let mut configured = pun_phpstan::Array::new();
        configured.set(
            pun_phpstan::Key::from("level"),
            pun_phpstan::Value::Int(level),
        );
        let mut configured = Project::from_config(&root, &configured);
        configured.analysed = direct.analysed.clone();
        direct.level = level as u8;
        for (constructor, mut project) in [("new", direct), ("from_config", configured)] {
            project.php_version = 80500;
            let mut actual: Vec<_> = run(&project)
                .unwrap()
                .into_iter()
                .map(|error| {
                    json!([
                        Path::new(&error.file)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                        error.line,
                        error.identifier,
                        error.message,
                        error.tip,
                        error.can_be_ignored
                    ])
                })
                .collect();
            actual.sort_by_key(Value::to_string);
            let expected = oracle["errors"][format!("project/{level}/certain")]
                .as_array()
                .unwrap();
            if &actual != expected {
                failures.push(json!({"level": level, "constructor": constructor,
                    "expected": expected, "actual": actual}));
            }
        }
    }
    assert!(failures.is_empty(), "{}", json!(failures));
}

mod observations {
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts, plugin::*,
        settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase,
        reference::SymbolReferences,
        scanner::scan_program,
        ttype::{atomic::TAtomic, atomic::array::TArray, union::TUnion},
    };
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::{cst::Expression, parser::parse_file};
    use pun_phpstan::types::Type;
    use serde_json::{Value, json};

    const SOURCE: &[u8] = include_bytes!("fixtures/native-array-order/native.php");
    type Events = Vec<(u32, u32, Option<Value>)>;
    struct Observe(Arc<Mutex<Events>>);
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "native-array-order",
                "native-array-order",
                "passive native value observations",
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
            self.0.lock().unwrap().push((
                e.span().start.offset,
                e.span().end.offset,
                c.get_native_expression_type(e).map(native_value),
            ));
            Ok(())
        }
    }

    // Raw Mago maps cannot express insertion order. These adapters compare only native values;
    // complete consumer diagnostics, not this canonical map comparison, prove PHP key order.
    fn native_value(value: &TUnion) -> Value {
        if let [TAtomic::Array(array)] = value.types.as_ref() {
            let mut items = BTreeMap::new();
            match array {
                TArray::Keyed(array) if array.parameters.is_none() => {
                    for (key, (_, value)) in array.known_items.iter().flat_map(|items| items.iter())
                    {
                        items.insert(key.to_string(), native_value(value));
                    }
                }
                TArray::List(array) => {
                    for (key, (_, value)) in
                        array.known_elements.iter().flat_map(|items| items.iter())
                    {
                        items.insert(key.to_string(), native_value(value));
                    }
                }
                _ => return json!("array"),
            }
            return json!(items);
        }
        if let Some(value) = value.get_single_literal_int_value() {
            return json!(value);
        }
        if let Some(value) = value.get_single_literal_string_value() {
            return json!(String::from_utf8_lossy(value));
        }
        if value.is_null() {
            return Value::Null;
        }
        if value.is_mixed() {
            return json!("mixed");
        }
        // Other observer events are not fixture observation points.
        json!("other")
    }

    fn oracle_value(value: Type) -> Value {
        match value {
            Type::ConstArray(array) => json!(
                array
                    .keys
                    .iter()
                    .zip(array.values.iter())
                    .map(|(key, value)| {
                        let key = match key {
                            Type::ConstInt(key) => key.to_string(),
                            Type::ConstString(key) => format!("'{}'", String::from_utf8_lossy(key)),
                            _ => panic!("nonconstant oracle key"),
                        };
                        (key, oracle_value(value.clone()))
                    })
                    .collect::<BTreeMap<_, _>>()
            ),
            Type::Array(..) => json!("array"),
            Type::ConstInt(value) => json!(value),
            Type::ConstString(value) => json!(String::from_utf8(value).unwrap()),
            Type::Null => Value::Null,
            Type::Mixed { .. } => json!("mixed"),
            _ => panic!("unsupported oracle observation: {value:?}"),
        }
    }

    fn analyze(native: bool) -> (AnalysisArtifacts, Events, String) {
        let arena = LocalArena::new();
        let file = File::new(
            Cow::Borrowed(b"native.php"),
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
        let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
        let mut references = SymbolReferences::new();
        populate_codebase(
            &mut codebase,
            &mut references,
            Default::default(),
            Default::default(),
        );
        let events = Arc::new(Mutex::new(Vec::new()));
        let mut registry = PluginRegistry::new();
        registry.register_expression_hook(Observe(Arc::clone(&events)));
        let mut result = AnalysisResult::new(references);
        let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        let observed = std::mem::take(&mut *events.lock().unwrap());
        (artifacts, observed, format!("{:?}", result.issues))
    }

    #[test]
    fn raw_native_values_and_unavailability_do_not_change_full_artifacts_or_visits() {
        let oracle: Value =
            serde_json::from_str(include_str!("fixtures/native-array-order/expected.json"))
                .unwrap();
        let (on, visits, issues) = analyze(true);
        let (off, off_visits, off_issues) = analyze(false);
        assert_eq!(issues, off_issues);
        assert_eq!(on.expression_types, off.expression_types);
        assert_eq!(on.symbol_references, off.symbol_references);
        assert_eq!(on.call_terminations, off.call_terminations);
        assert_eq!(on.if_true_assertions, off.if_true_assertions);
        assert_eq!(on.if_false_assertions, off.if_false_assertions);
        assert_eq!(
            on.true_branch_only_assertions,
            off.true_branch_only_assertions
        );
        assert_eq!(on.inferred_return_types, off.inferred_return_types);
        assert_eq!(on.inferred_yield_key_types, off.inferred_yield_key_types);
        assert_eq!(
            on.inferred_yield_value_types,
            off.inferred_yield_value_types
        );
        assert_eq!(on.inferred_parameter_types, off.inferred_parameter_types);
        assert_eq!(
            on.method_initialized_properties,
            off.method_initialized_properties
        );
        assert_eq!(on.method_calls_this_methods, off.method_calls_this_methods);
        assert_eq!(
            on.method_calls_parent_constructor,
            off.method_calls_parent_constructor
        );
        assert_eq!(
            on.method_calls_parent_initializer,
            off.method_calls_parent_initializer
        );
        let writes = |artifacts: &AnalysisArtifacts| {
            artifacts
                .variable_writes
                .iter()
                .map(|(span, writes)| ((span.start.offset, span.end.offset), format!("{writes:?}")))
                .collect::<BTreeMap<_, _>>()
        };
        assert_eq!(writes(&on), writes(&off));
        assert_eq!(
            on.nullsafe_method_receiver_types,
            off.nullsafe_method_receiver_types
        );
        assert_eq!(
            visits.iter().map(|(a, b, _)| (a, b)).collect::<Vec<_>>(),
            off_visits
                .iter()
                .map(|(a, b, _)| (a, b))
                .collect::<Vec<_>>()
        );
        assert!(off_visits.iter().all(|(_, _, value)| value.is_none()));
        let mut counts = BTreeMap::new();
        for (start, end, _) in &visits {
            *counts.entry((start, end)).or_insert(0) += 1;
        }
        assert!(counts.values().all(|count| *count == 1));
        let mut observed = 0;
        for (index, line) in std::str::from_utf8(SOURCE).unwrap().lines().enumerate() {
            let Some((_, argument)) = line.split_once("\\PHPStan\\dumpNativeType(") else {
                continue;
            };
            let argument = argument.split_once(')').unwrap().0;
            let line_start = SOURCE
                .split_inclusive(|byte| *byte == b'\n')
                .take(index)
                .map(<[u8]>::len)
                .sum::<usize>();
            let start = line_start + line.find(argument).unwrap();
            let end = start + argument.len();
            let actual = &visits
                .iter()
                .find(|(a, b, _)| (*a as usize, *b as usize) == (start, end))
                .unwrap()
                .2;
            let expected = oracle["errors"]["native/0/certain"]
                .as_array()
                .unwrap()
                .iter()
                .find(|record| record[1] == index + 1 && record[2] == "phpstan.dumpNativeType")
                .unwrap();
            if line.contains("// unavailable") {
                assert!(actual.is_none(), "line {}", index + 1);
            } else {
                let description = expected[3]
                    .as_str()
                    .unwrap()
                    .strip_prefix("Dumped type: ")
                    .unwrap();
                assert_eq!(
                    *actual,
                    Some(oracle_value(pun_phpstan::resolve::resolve_str(
                        description,
                        None
                    ))),
                    "line {}",
                    index + 1
                );
            }
            let span = mago_span::Span::new(
                mago_database::file::FileId::zero(),
                mago_span::Position::new(start as u32),
                mago_span::Position::new(end as u32),
            );
            assert_eq!(
                on.get_native_expression_type(&span).map(native_value),
                *actual
            );
            assert!(off.get_native_expression_type(&span).is_none());
            observed += 1;
        }
        assert_eq!(observed, 9);
        println!(
            "9 independent native observations; {} once-only visits; identical full artifacts and provenance",
            visits.len()
        );
    }
}
