//! Independent full/native values and selected-scope class-constant consumers.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts,
    observations::CallScopeRequest, plugin::*, settings::Settings,
};
use mago_codex::{
    metadata::CodebaseMetadata,
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        atomic::{
            TAtomic,
            array::{TArray, key::ArrayKey},
            object::TObject,
            reference::TReference,
            scalar::{
                TScalar, class_like_string::TClassLikeString, int::TInteger, string::TStringLiteral,
            },
        },
        union::TUnion,
    },
};
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{cst::Expression, parser::parse_file};
use pun_analyse::{Project, analyse_project};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::{Value, json};

fn oracle() -> Value {
    serde_json::from_str(include_str!(
        "fixtures/conversion-dependencies/expected.json"
    ))
    .unwrap()
}

fn project_records(group: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/conversion-dependencies");
    let oracle = oracle();
    let mut failures = Vec::new();
    let mut configurations = 0;
    for (case, expected) in oracle["diagnostics"].as_object().unwrap() {
        let parts = case.split('/').collect::<Vec<_>>();
        if parts[0] != group {
            continue;
        }
        configurations += 1;
        let level: u8 = parts[1].parse().unwrap();
        let certain = parts[2] == "1";
        let mut parameters = Array::new();
        parameters.set(Key::from("level"), Parameter::Int(i64::from(level)));
        parameters.set(Key::from("phpVersion"), Parameter::Int(80500));
        parameters.set(
            Key::from("treatPhpDocTypesAsCertain"),
            Parameter::Bool(certain),
        );
        let mut configured = Project::from_config(&root, &parameters);
        configured.analysed = vec![root.join(format!("{group}-controls.php"))];
        let mut direct = Project::new(configured.analysed.clone(), Vec::new());
        direct.level = level;
        direct.php_version = 80500;
        direct.treat_php_doc_types_as_certain = Some(certain);
        let mut expected = expected.as_array().unwrap().clone();
        expected.sort_by_cached_key(Value::to_string);
        for (constructor, project) in [("new", direct), ("from_config", configured)] {
            let result = analyse_project(&project).unwrap();
            assert!(
                result.general.is_empty(),
                "{case}/{constructor}: {:?}",
                result.general
            );
            let mut actual = result
                .errors
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
                .collect::<Vec<_>>();
            actual.sort_by_cached_key(Value::to_string);
            println!(
                "records={}",
                json!({"case": case, "constructor": constructor, "actual": actual, "oracle": expected})
            );
            if actual != expected {
                failures.push(format!("{case}/{constructor}"));
            }
        }
    }
    assert_eq!(configurations, if group == "native" { 6 } else { 2 });
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn native_dependency_consumers_match_complete_records() {
    project_records("native");
}

#[test]
fn selected_class_constants_match_complete_records() {
    project_records("selected");
}

// Raw Mago arrays do not store insertion order. Match their key/value trees here;
// Project's complete isset and argument diagnostics independently check conversion order.
fn value(ty: &TUnion, codebase: &CodebaseMetadata) -> Value {
    let shape = |mut items: Vec<(Value, Value)>| {
        items.sort_by_cached_key(|(key, _)| key.to_string());
        json!({"shape": items})
    };
    let mut parts = ty.types.iter().map(|atomic| match atomic {
        TAtomic::Null => Value::Null,
        TAtomic::Mixed(mixed) => json!(if mixed.is_error() { "*ERROR*" } else { "mixed" }),
        TAtomic::Scalar(TScalar::Integer(TInteger::Literal(integer))) => json!(integer),
        TAtomic::Scalar(TScalar::Integer(_)) => json!("int"),
        TAtomic::Scalar(TScalar::Bool(boolean)) => boolean.value.map_or_else(|| json!("bool"), |value| json!(value)),
        TAtomic::Scalar(TScalar::String(string)) => match &string.literal {
            Some(TStringLiteral::Value(string)) => json!(String::from_utf8_lossy(string.as_bytes())),
            _ => json!("string"),
        },
        TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => json!(String::from_utf8_lossy(value.as_bytes())),
        TAtomic::Array(TArray::List(array)) if array.known_elements.is_some() => shape(array.known_elements.as_ref().unwrap().iter().map(|(key, (optional, item))| {
            assert!(!optional);
            (json!(key), value(item, codebase))
        }).collect()),
        TAtomic::Array(TArray::Keyed(array)) if array.parameters.is_none() => shape(array.known_items.iter().flat_map(|items| items.iter()).map(|(key, (optional, item))| {
            assert!(!optional);
            let key = match key {
                ArrayKey::Integer(key) => json!(key),
                ArrayKey::String(key) => json!(String::from_utf8_lossy(key.as_bytes())),
                _ => panic!("symbolic key is not part of these observations"),
            };
            (key, value(item, codebase))
        }).collect()),
        TAtomic::Array(_) => json!("array"),
        TAtomic::Object(TObject::Named(object)) => json!({"object": codebase.get_class_like(object.name.as_bytes()).map_or(object.name, |class| class.original_name).to_string()}),
        TAtomic::Reference(TReference::Symbol { name, .. }) => json!({"object": codebase.get_class_like(name.as_bytes()).map_or(*name, |class| class.original_name).to_string()}),
        _ => panic!("unexpected public value: {atomic:?}"),
    }).collect::<Vec<_>>();
    parts.sort_by_cached_key(Value::to_string);
    if parts.len() == 1 {
        parts.pop().unwrap()
    } else {
        json!({"union": parts})
    }
}

#[derive(Default)]
struct Events {
    sequence: Vec<(u32, u32)>,
    pairs: BTreeMap<(u32, u32), (Value, Option<Value>)>,
}

struct Observe {
    marks: Vec<(u32, u32)>,
    events: Arc<Mutex<Events>>,
}

impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "conversion-dependencies",
            "conversion-dependencies",
            "passive paired observations",
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
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expression.span();
        let range = (span.start.offset, span.end.offset);
        let mut events = self.events.lock().unwrap();
        events.sequence.push(range);
        if self.marks.contains(&range) {
            let full = value(
                context.get_expression_type(expression).unwrap(),
                context.codebase(),
            );
            let native = context
                .get_native_expression_type(expression)
                .map(|ty| value(ty, context.codebase()));
            assert!(
                events.pairs.insert(range, (full, native)).is_none(),
                "once-only marked evaluations"
            );
        }
        Ok(())
    }
}

fn span(file: &File, expression: &Value) -> Span {
    Span::new(
        file.id,
        Position::new(expression["start"].as_u64().unwrap() as u32),
        Position::new(expression["end"].as_u64().unwrap() as u32),
    )
}

fn observe(group: &str, native: bool) -> (AnalysisArtifacts, Events, String) {
    let source: &[u8] = match group {
        "native" => include_bytes!("fixtures/conversion-dependencies/native-controls.php"),
        "selected" => include_bytes!("fixtures/conversion-dependencies/selected-controls.php"),
        _ => unreachable!(),
    };
    let oracle = oracle();
    let arena = LocalArena::new();
    let file = File::new(
        Cow::Owned(format!("{group}-controls.php").into_bytes()),
        FileType::Host,
        None,
        Cow::Borrowed(source),
    );
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut settings = Settings {
        track_native_types: native,
        ..Settings::default()
    };
    for call in oracle["calls"][group].as_array().unwrap() {
        let mut expressions = call["names"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|name| name["types"].as_array().unwrap())
            .map(|expression| span(&file, expression))
            .collect::<Vec<_>>();
        expressions.sort_by_key(|span| (span.start.offset, span.end.offset));
        expressions.dedup();
        settings.call_scope_observations.push(CallScopeRequest {
            call: span(&file, call),
            expressions,
        });
    }
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let events = Arc::new(Mutex::new(Events::default()));
    let marks = oracle["observations"][group]
        .as_array()
        .unwrap()
        .iter()
        .map(|expression| {
            let span = span(&file, expression);
            assert_eq!(
                &source[span.start.offset as usize..span.end.offset as usize],
                expression["expression"].as_str().unwrap().as_bytes()
            );
            (span.start.offset, span.end.offset)
        })
        .collect();
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Observe {
        marks,
        events: Arc::clone(&events),
    });
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let events = std::mem::take(&mut *events.lock().unwrap());
    for expression in oracle["observations"][group].as_array().unwrap() {
        let span = span(&file, expression);
        let range = (span.start.offset, span.end.offset);
        let (full, actual_native) = &events.pairs[&range];
        let expected_native = (native && expression["available"].as_bool().unwrap())
            .then(|| expression["native_value"].clone());
        println!(
            "pair={}",
            json!({"group": group, "label": expression["label"], "native_enabled": native, "full": full, "native": actual_native, "oracle": expression})
        );
        assert_eq!(
            *full, expression["full_value"],
            "{group}/{} full",
            expression["label"]
        );
        assert_eq!(
            *actual_native, expected_native,
            "{group}/{} native",
            expression["label"]
        );
        assert_eq!(
            artifacts
                .get_expression_type(&span)
                .map(|ty| value(ty, &codebase)),
            Some(expression["full_value"].clone())
        );
        assert_eq!(
            artifacts
                .get_native_expression_type(&span)
                .map(|ty| value(ty, &codebase)),
            expected_native
        );
    }
    assert_eq!(
        events.pairs.len(),
        oracle["observations"][group].as_array().unwrap().len()
    );
    assert_eq!(
        artifacts.call_scope_observations.len(),
        oracle["calls"][group].as_array().unwrap().len()
    );
    for call in oracle["calls"][group].as_array().unwrap() {
        let selected = &artifacts.call_scope_observations[&span(&file, call)];
        assert_eq!(selected.len(), call["names"].as_array().unwrap().len());
        for expected in call["names"].as_array().unwrap() {
            let selected = selected
                .iter()
                .find(|selected| selected.name.as_str() == expected["name"].as_str())
                .unwrap();
            assert_eq!(selected.filtered, expected["filtered"].as_bool().unwrap());
            for expression in expected["types"].as_array().unwrap() {
                let actual = value(
                    selected.types.get(&span(&file, expression)).unwrap(),
                    &codebase,
                );
                println!(
                    "selected={}",
                    json!({"name": expected["name"], "expression": expression["expression"], "actual": actual, "oracle": expression["full_value"]})
                );
                assert_eq!(
                    actual, expression["full_value"],
                    "{} {}",
                    expected["name"], expression["expression"]
                );
            }
        }
    }
    (artifacts, events, format!("{:?}", result.issues))
}

fn paired_observations(group: &str) {
    let (on, on_events, on_issues) = observe(group, true);
    let (off, off_events, off_issues) = observe(group, false);
    assert_eq!(on.expression_types, off.expression_types);
    assert_eq!(on.symbol_references, off.symbol_references);
    assert_eq!(on.call_terminations, off.call_terminations);
    assert_eq!(on_issues, off_issues);
    assert_eq!(on_events.sequence, off_events.sequence);
}

#[test]
fn native_dependency_pairs_and_unavailability_are_independent() {
    paired_observations("native");
}

#[test]
fn selected_constant_dependencies_keep_their_filtered_scope() {
    paired_observations("selected");
}
