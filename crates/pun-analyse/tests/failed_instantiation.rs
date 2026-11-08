//! Failed instantiations retain values independently of legality diagnostics.
//! Frozen independently by PHPStan 2.3.1; capture.php is the reproducible oracle.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer,
    analysis_result::AnalysisResult,
    plugin::{
        ExpressionHook, HookContext, HookResult, InvocationInfo, MethodReturnTypeProvider,
        MethodTarget, Provider, ProviderContext, ProviderMeta, create_registry_with_plugins,
    },
    settings::Settings,
};
use mago_codex::{
    populator::populate_codebase,
    reference::SymbolReferences,
    scanner::scan_program,
    ttype::{
        TType,
        atomic::{
            TAtomic,
            array::{TArray, key::ArrayKey},
            object::TObject,
            scalar::{TScalar, class_like_string::TClassLikeString},
        },
        get_explicit_never, get_literal_int, get_never, get_non_accepting_never,
        union::{NeverKind, TUnion},
    },
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{cst::Expression, parser::parse_file};
use mago_word::word;
use pun_analyse::{Project, run};
use pun_phpstan::{
    resolve::resolve_str,
    types::{Level, Type},
};
use serde_json::{Value, json};

fn oracle() -> Value {
    serde_json::from_str(include_str!("fixtures/failed-instantiation/expected.json")).unwrap()
}

#[test]
fn failed_instantiations_keep_complete_primary_diagnostics() {
    check_diagnostics(&["illegal.php", "controls.php"], oracle());
}

#[test]
fn nonexistent_parent_keeps_registered_diagnostics_and_object_rejection() {
    check_diagnostics(&["parent-sentinel.php"], parent_oracle());
}

fn check_diagnostics(files: &[&str], expected: Value) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/failed-instantiation");
    let mut failures = Vec::new();
    for level in [0, 5, 9] {
        let mut project = Project::new(files.iter().map(|name| root.join(name)).collect(), vec![]);
        project.level = level;
        project.php_version = 80500;
        project.check_this_only = Some(false);
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
                    error.can_be_ignored,
                ])
            })
            .collect();
        actual.sort_by_key(Value::to_string);
        // Keep the complete oracle capture, including NonexistentOffsetInArrayDimFetchRule,
        // whose mapping is not ported. Compare every registered diagnostic, without hiding extras.
        let registered: Vec<_> = expected["errors"][level.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| pun_analyse::reports(row[2].as_str().unwrap()))
            .cloned()
            .collect();
        if actual != registered {
            failures.push(json!({"level": level, "actual": actual, "expected": registered}));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}

struct Continuation {
    source: &'static [u8],
    observed: Arc<Mutex<BTreeMap<String, u32>>>,
}

impl ExpressionHook for Continuation {
    fn after_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expression.span();
        let text = &self.source[span.start.offset as usize..span.end.offset as usize];
        if text.starts_with(b"'after-") {
            self.observed.lock().unwrap().insert(
                std::str::from_utf8(&text[1..text.len() - 1])
                    .unwrap()
                    .to_owned(),
                span.start.offset,
            );
        }
        Ok(())
    }
}

#[test]
fn illegal_new_values_writes_and_continuation_match_phpstan_in_both_native_modes() {
    check_types(
        "illegal.php",
        include_bytes!("fixtures/failed-instantiation/illegal.php"),
    );
}

#[test]
fn successful_reference_private_true_never_and_dynamic_controls_remain_distinct() {
    check_types(
        "controls.php",
        include_bytes!("fixtures/failed-instantiation/controls.php"),
    );
}

fn parent_oracle() -> Value {
    serde_json::from_str(include_str!(
        "fixtures/failed-instantiation/expected-parent-sentinel.json"
    ))
    .unwrap()
}

#[test]
fn nonexistent_parent_preserves_identity_values_and_continuation_in_both_native_modes() {
    check_types(
        "parent-sentinel.php",
        include_bytes!("fixtures/failed-instantiation/parent-sentinel.php"),
    );
}

struct ConstructorProvider;

impl Provider for ConstructorProvider {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "test.constructor",
            "Constructor",
            "Matches the independent PHPStan extension",
        );
        &META
    }
}

impl MethodReturnTypeProvider for ConstructorProvider {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: &[MethodTarget] = &[MethodTarget::exact(
            b"FailedInstantiationProviders\\*",
            b"__construct",
        )];
        TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        class: &[u8],
        _: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        match class
            .rsplit(|byte| *byte == b'\\')
            .next()
            .unwrap()
            .to_ascii_lowercase()
            .as_slice()
        {
            b"provided" | b"illegalprovided" | b"emptyprovided" | b"inheritedprovided" => context
                .get_expression_type(invocation.get_argument(0, &[])?)
                .cloned(),
            b"overriddennever" | b"noargumentprovided" => Some(get_literal_int(17)),
            b"implicitbottom" => Some(get_never()),
            b"explicitbottom" => Some(get_explicit_never()),
            b"nonacceptingbottom" => Some(get_non_accepting_never()),
            _ => None,
        }
    }
}

#[test]
fn providers_keep_argument_positions_bottom_kinds_and_dynamic_asymmetry() {
    check_types(
        "providers.php",
        include_bytes!("fixtures/failed-instantiation/providers.php"),
    );
}

fn describe(ty: &TUnion) -> Value {
    let text = if ty.is_error() {
        "*ERROR*".to_owned()
    } else if matches!(
        ty.never_kind(),
        Some(NeverKind::Implicit | NeverKind::Explicit)
    ) {
        "*NEVER*".to_owned()
    } else if let Some(integer) = ty.get_single_literal_int_value() {
        integer.to_string()
    } else if let Some(string) = ty.get_single_literal_string_value() {
        format!(
            "'{}'",
            std::str::from_utf8(string)
                .unwrap()
                .replace('\\', "\\\\")
                .replace('\'', "\\'")
        )
    } else if let Some(string) = ty.get_single_class_string_value() {
        format!("'{}'", string.to_string().replace('\\', "\\\\"))
    } else {
        ty.types
            .iter()
            .map(|atomic| match atomic {
                TAtomic::Array(TArray::List(array)) if array.known_elements.is_some() => {
                    assert!(array.element_type.is_never());
                    let fields = array
                        .known_elements
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(key, (optional, value))| {
                            format!(
                                "{key}{}: {}",
                                if *optional { "?" } else { "" },
                                describe(value)["type"].as_str().unwrap()
                            )
                        })
                        .collect::<Vec<_>>();
                    format!("array{{{}}}", fields.join(", "))
                }
                TAtomic::Array(TArray::Keyed(array)) if array.known_items.is_some() => {
                    assert!(array.parameters.is_none());
                    let fields = array
                        .known_items
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|(key, (optional, value))| {
                            let key = match key {
                                ArrayKey::Integer(value) => value.to_string(),
                                ArrayKey::String(value) => format!("'{value}'"),
                                _ => panic!("unexpected symbolic key in oracle fixture"),
                            };
                            format!(
                                "{key}{}: {}",
                                if *optional { "?" } else { "" },
                                describe(value)["type"].as_str().unwrap()
                            )
                        })
                        .collect::<Vec<_>>();
                    format!("array{{{}}}", fields.join(", "))
                }
                _ if ty.types.len() > 1 => describe(&TUnion::from_atomic(atomic.clone()))["type"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                _ => atomic.get_id().to_string(),
            })
            .collect::<Vec<_>>()
            .join("|")
    };
    json!({
        "type": text,
        "error": ty.is_error(),
        "kind": match ty.never_kind() {
            Some(NeverKind::Implicit) => Some("implicit"),
            Some(NeverKind::Explicit) => Some("explicit"),
            Some(NeverKind::NonAccepting) => Some("nonaccepting"),
            None => None,
        },
    })
}

// Compare values and shape entries, not Mago's presentation order. Sentinel identity is checked separately.
fn canonical(mut description: Value) -> Value {
    fn ordered(ty: &Type) -> Type {
        let mut ty = ty.traverse(&ordered);
        if let Type::ConstArray(array) = &mut ty {
            let mut indices: Vec<_> = (0..array.keys.len()).collect();
            indices.sort_by_key(|&i| array.keys[i].describe(Level::Precise));
            array.keys = indices.iter().map(|&i| array.keys[i].clone()).collect();
            array.values = indices.iter().map(|&i| array.values[i].clone()).collect();
            array.optional = indices
                .iter()
                .enumerate()
                .filter_map(|(new, old)| array.optional.contains(old).then_some(new))
                .collect();
        }
        ty
    }
    if !description.is_null() {
        description["type"] = json!(
            ordered(&resolve_str(description["type"].as_str().unwrap(), None))
                .describe(Level::Precise)
        );
    }
    description
}

fn check_types(name: &'static str, source: &'static [u8]) {
    let expected = match name {
        "providers.php" => serde_json::from_str(include_str!(
            "fixtures/failed-instantiation/expected-providers.json"
        ))
        .unwrap(),
        "parent-sentinel.php" => parent_oracle(),
        _ => oracle(),
    };
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(name.as_bytes()), Cow::Borrowed(source));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let prelude = mago_prelude::Prelude::build();
    let mut codebase = prelude.metadata;
    let mut references = prelude.symbol_references;
    codebase.extend(scan_program(
        &arena,
        &file,
        program,
        &names,
        Settings::default().version,
    ));
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut baseline = None;
    for native in [false, true] {
        let observed = Arc::new(Mutex::new(BTreeMap::new()));
        let mut registry = create_registry_with_plugins(&[], false);
        registry.register_expression_hook(Continuation {
            source,
            observed: Arc::clone(&observed),
        });
        registry.register_method_provider(ConstructorProvider);
        let mut result = AnalysisResult::new(SymbolReferences::new());
        let artifacts = Analyzer::new(
            &arena,
            &file,
            &names,
            &codebase,
            &registry,
            Settings {
                track_native_types: native,
                ..Default::default()
            },
        )
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
        let mut failures = Vec::new();
        for row in expected["types"][name].as_array().unwrap() {
            let start = row["start"].as_u64().unwrap() as u32;
            let end = row["end"].as_u64().unwrap() as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(end));
            assert_eq!(
                &source[start as usize..end as usize],
                row["expression"].as_str().unwrap().as_bytes()
            );
            let ty = artifacts.get_expression_type(&span);
            let actual = ty.map_or(Value::Null, describe);
            let want = json!({"type": row["type"], "error": row["error"], "kind": row["kind"]});
            if canonical(actual.clone()) != canonical(want.clone()) {
                failures.push(json!({"expression": row["expression"], "start": start, "actual": actual, "expected": want}));
            }
            if row["class"] == "PHPStan\\Type\\NonexistentParentClassType" {
                assert_eq!(
                    ty.unwrap().get_single(),
                    &TAtomic::Object(TObject::NonexistentParent)
                );
                assert!(ty.unwrap().get_single_named_object().is_none());
            }
            if row["type"] == "class-string&literal-string" {
                assert_eq!(
                    ty.unwrap().get_single(),
                    &TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::LiteralAny))
                );
                assert!(ty.unwrap().get_single_class_string_value().is_none());
            }
            if let Some(expected_final) = row.get("final") {
                let finality = ty
                    .and_then(|ty| ty.get_single_named_object())
                    .and_then(|object| {
                        codebase
                            .get_class_like_by_name(object.name)
                            .map(|class| object.is_exact || class.flags.is_final())
                    });
                if json!(finality) != *expected_final {
                    failures.push(json!({"expression": row["expression"], "start": start, "final": finality, "expected_final": expected_final}));
                }
            }
            if row["expression"].as_str().unwrap().starts_with("new ") {
                let raw = artifacts
                    .get_raw_expression_type(&span)
                    .map_or(Value::Null, describe);
                if canonical(raw.clone()) != canonical(want.clone()) {
                    failures.push(json!({"expression": row["expression"], "start": start, "raw": raw, "expected": want}));
                }
            }
        }
        assert!(
            failures.is_empty(),
            "{name} native={native}: {}",
            serde_json::to_string_pretty(&failures).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&*observed.lock().unwrap()).unwrap(),
            expected["continued"][name],
            "{name} native={native}"
        );
        if name == "illegal.php" {
            for issue in &result.issues {
                assert!(
                    !matches!(
                        issue.code.as_deref(),
                        Some(
                            "invalid-class-string-expression"
                                | "possibly-null-property-access"
                                | "impossible-assignment"
                        )
                    ),
                    "default instantiation must not emit secondary non-object/bottom issues: {issue:?}"
                );
            }
        }
        if name == "controls.php" {
            for class in [
                "ordinary",
                "abstractwithconstructor",
                "interfacewithconstructor",
                "traitwithconstructor",
                "privateconstructor",
                "stops",
                "abstractstops",
            ] {
                assert!(
                    result.symbol_references.has_reference_to_symbol((
                        word(format!("failedinstantiationcontrols\\{class}")),
                        word("__construct")
                    )),
                    "missing constructor reference for {class}"
                );
            }
        }
        let state = (artifacts.expression_types, result.issues);
        if let Some(previous) = &baseline {
            assert_eq!(previous, &state, "native tracking changed full analysis");
        }
        baseline = Some(state);
    }
}
