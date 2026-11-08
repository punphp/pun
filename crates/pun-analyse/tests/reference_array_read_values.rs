//! Array references retain each operand's read, including ErrorType, before escape.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
use mago_codex::{
    populator::populate_codebase,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            array::{TArray, key::ArrayKey},
        },
        union::TUnion,
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::{
    cst::{Expression, UnaryPrefixOperator, Variable},
    parser::parse_file,
};
use pun_analyse::{Project, run};
use serde_json::{Value, json};

const SOURCE: &str = include_str!("fixtures/reference-array-read-values/values.php");
const ORACLE: &str = include_str!("fixtures/reference-array-read-values/expected.json");

#[test]
fn reference_array_reads_match_complete_phpstan_records() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/reference-array-read-values");
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let mut failures = Vec::new();
    for level in [1, 5] {
        let mut project = Project::new(vec![root.join("values.php")], Vec::new());
        project.php_version = 80500;
        project.level = level;
        project.options.set(
            pun_analyse::ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES,
            true,
        );
        let mut actual: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|e| {
                json!([
                    Path::new(&e.file).file_name().unwrap().to_str().unwrap(),
                    e.line,
                    e.identifier,
                    e.message,
                    e.tip,
                    e.can_be_ignored,
                ])
            })
            .collect();
        let mut expected = oracle["diagnostics"][level.to_string()]
            .as_array()
            .unwrap()
            .clone();
        actual.sort_by_cached_key(Value::to_string);
        expected.sort_by_cached_key(Value::to_string);
        println!(
            "reference-diagnostics={}",
            json!({"level":level,"actual":actual,"oracle":expected})
        );
        if actual != expected {
            failures.push(format!(
                "level {level}\nextra: {:?}\nmissing: {:?}",
                actual
                    .iter()
                    .filter(|e| !expected.contains(e))
                    .collect::<Vec<_>>(),
                expected
                    .iter()
                    .filter(|e| !actual.contains(e))
                    .collect::<Vec<_>>()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

// Render only the scalar and finite-array forms in this oracle. No analyzer conversion
// is used, so ErrorType cannot be repaired by the PHPStan-style consumer under test.
fn describe(ty: &TUnion) -> String {
    if ty.is_error() {
        return "*ERROR*".to_owned();
    }
    if let Some(value) = ty.get_single_literal_int_value() {
        return value.to_string();
    }
    if let [TAtomic::Array(array)] = ty.types.as_ref() {
        let fields = match array {
            TArray::List(list) if list.known_elements.is_some() => {
                assert!(list.element_type.is_never());
                list.known_elements
                    .as_ref()
                    .unwrap()
                    .values()
                    .map(|(optional, value)| {
                        assert!(!optional);
                        describe(value)
                    })
                    .collect::<Vec<_>>()
            }
            TArray::Keyed(array) if array.known_items.is_some() => {
                assert!(array.parameters.is_none());
                array
                    .known_items
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(key, (optional, value))| {
                        assert!(!optional);
                        let key = match key {
                            ArrayKey::String(key) => key.to_string(),
                            ArrayKey::Integer(key) => key.to_string(),
                            _ => panic!("unexpected nonliteral array key: {key}"),
                        };
                        format!("{key}: {}", describe(value))
                    })
                    .collect::<Vec<_>>()
            }
            _ => panic!("unexpected non-finite array in read fixture: {ty:?}"),
        };
        return format!("array{{{}}}", fields.join(", "));
    }
    ty.get_id().to_string()
}

#[derive(Default)]
struct Events {
    reads: BTreeMap<u32, (Option<String>, String, Option<String>)>,
    references: BTreeMap<u32, String>,
    sequence: Vec<(u32, u32)>,
}
struct Observe {
    marks: BTreeMap<u32, u32>,
    operands: BTreeMap<u32, u32>,
    found: Arc<Mutex<Events>>,
}
impl Provider for Observe {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "reference-array-reads",
            "reference-array-reads",
            "passive array reads",
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
        let mut found = self.found.lock().unwrap();
        found.sequence.push((span.start.offset, span.end.offset));
        if let Expression::UnaryPrefix(reference) = expression
            && matches!(reference.operator, UnaryPrefixOperator::Reference(_))
            && self
                .operands
                .contains_key(&reference.operand.span().start.offset)
        {
            let ty = context.get_expression_type(expression).unwrap();
            assert!(ty.by_reference(), "reference marker was lost");
            assert!(
                found
                    .references
                    .insert(reference.operand.span().start.offset, describe(ty))
                    .is_none(),
                "reference evaluated twice"
            );
        }
        if self.marks.get(&span.start.offset) != Some(&span.end.offset) {
            return Ok(());
        }
        let certainty = if let Expression::Variable(Variable::Direct(variable)) = expression {
            let certainty = context
                .block_context()
                .variable_certainty(mago_word::word(variable.name));
            Some(
                if certainty.is_defined() {
                    "defined"
                } else if certainty.is_absent() {
                    "absent"
                } else {
                    "possible"
                }
                .to_owned(),
            )
        } else {
            None
        };
        let selected = describe(context.get_expression_type(expression).unwrap());
        let native = context.get_native_expression_type(expression).map(describe);
        assert!(
            found
                .reads
                .insert(span.start.offset, (certainty, selected, native))
                .is_none(),
            "marked expression evaluated twice"
        );
        Ok(())
    }
}

#[test]
fn reference_array_source_observations_are_once_only_with_native_off_and_on() {
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let observations = oracle["observations"].as_array().unwrap();
    let operands: BTreeMap<u32, u32> = oracle["reference_operands"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(start, end)| (start.parse().unwrap(), end.as_u64().unwrap() as u32))
        .collect();
    let file = File::ephemeral(
        b"values.php".as_slice().into(),
        SOURCE.as_bytes().to_vec().into(),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata.clone();
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        Settings::default().version,
    ));
    let mut references = prelude.symbol_references;
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut previous = None;
    let mut failures = Vec::new();
    for native in [false, true] {
        let found = Arc::new(Mutex::new(Events::default()));
        let mut registry = PluginRegistry::with_library_providers();
        registry.register_expression_hook(Observe {
            marks: observations
                .iter()
                .map(|row| {
                    (
                        row["start"].as_u64().unwrap() as u32,
                        row["end"].as_u64().unwrap() as u32,
                    )
                })
                .collect(),
            operands: operands.clone(),
            found: Arc::clone(&found),
        });
        let settings = Settings {
            track_native_types: native,
            ..Settings::default()
        };
        let mut result = AnalysisResult::new(references.clone());
        let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        let found = std::mem::take(&mut *found.lock().unwrap());
        println!(
            "reference-observations={}",
            json!({"native":native,"actual":found.reads,"references":found.references,"oracle":observations})
        );
        assert_eq!(found.reads.len(), observations.len());
        assert_eq!(found.references.len(), operands.len());
        for row in observations {
            let at = row["start"].as_u64().unwrap() as u32;
            let (certainty, selected, native_type) = &found.reads[&at];
            if serde_json::to_value(certainty).unwrap() != row["certainty"]
                || selected != row["selected"].as_str().unwrap()
            {
                failures.push(format!(
                    "native={native}, line {}: {:?} versus {row}",
                    row["line"], found.reads[&at]
                ));
            }
            if operands.contains_key(&at)
                && found.references[&at] != row["selected"].as_str().unwrap()
            {
                failures.push(format!(
                    "native={native}, reference at {at}: {} versus {}",
                    found.references[&at], row["selected"]
                ));
            }
            if !native {
                assert!(native_type.is_none());
            } else if row["native_required"] == true {
                assert_eq!(native_type.as_deref(), row["native"].as_str());
            } else if let Some(native_type) = native_type {
                assert_eq!(
                    native_type,
                    row["native"].as_str().unwrap(),
                    "native at {at}"
                );
            }
        }
        let unchanged = (
            artifacts.expression_types,
            artifacts.symbol_references,
            format!("{:?}", result.issues),
            found.sequence,
        );
        if let Some(previous) = &previous {
            assert_eq!(&unchanged, previous);
        } else {
            previous = Some(unchanged);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
