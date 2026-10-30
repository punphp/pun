//! Sequential optional-key joins and their tagged and empty-array boundaries.

use std::path::Path;
use std::time::Instant;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn sequential_scope_joins_match_phpstan() {
    assert_fixture("joins.php", "expected.json", 3);
}

#[test]
fn optional_array_pair_order_matches_phpstan() {
    assert_fixture("ordering.php", "expected-ordering.json", 4);
}

#[test]
fn array_constraint_order_matches_phpstan() {
    assert_fixture("constraints.php", "expected-constraints.json", 4);
}

#[test]
fn deferred_offset_constraints_keep_the_oracle_member_order() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::plugin::{
        ExpressionHook, HookContext, HookResult, PluginRegistry, Provider, ProviderMeta,
    };
    use mago_analyzer::settings::Settings;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::derived::{TDerived, intersection::TDerivedIntersection};
    use mago_codex::ttype::atomic::object::TObject;
    use mago_codex::ttype::union::TUnion;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::cst::Expression;
    use mago_syntax::parser::parse_file;

    struct Deferred {
        offsets: Vec<u32>,
    }
    impl Provider for Deferred {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "deferred-offset-order",
                "deferred-offset-order",
                "retain the offset constraint order",
            );
            &META
        }
    }
    impl ExpressionHook for Deferred {
        fn after_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if expression.span().length() == 7
                && self.offsets.contains(&expression.span().start.offset)
            {
                let base = context.get_expression_type(expression).unwrap().clone();
                context.set_expression_type(
                    expression,
                    TUnion::from_atomic(TAtomic::Derived(TDerived::Intersection(
                        TDerivedIntersection::new(base),
                    ))),
                );
            }
            Ok(())
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sequential-scope-joins");
    let source = std::fs::read_to_string(root.join("constraints.php")).unwrap();
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-offset-order.json")).unwrap())
            .unwrap();
    let offsets: Vec<_> = source
        .match_indices("$result['object']")
        .map(|(offset, _)| offset as u32)
        .collect();
    assert_eq!(offsets.len(), oracle.as_array().unwrap().len());
    let file = File::ephemeral(
        Cow::Borrowed(b"constraints.php"),
        Cow::Owned(source.as_bytes().to_vec()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings::default();
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    for deferred in [false, true] {
        let mut registry = PluginRegistry::new();
        if deferred {
            registry.register_expression_hook(Deferred {
                offsets: offsets.clone(),
            });
        }
        let mut result = AnalysisResult::new(references.clone());
        let artifacts = mago_analyzer::Analyzer::new(
            &arena,
            &file,
            &names,
            &codebase,
            &registry,
            settings.clone(),
        )
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
        let actual: Vec<_> = offsets
            .iter()
            .map(|offset| {
                let ty = &artifacts.expression_types[&(*offset, *offset + 17)];
                let names: Vec<_> = ty
                    .types
                    .iter()
                    .map(|atomic| match atomic {
                        TAtomic::Object(TObject::Named(object)) => object.name.to_string(),
                        other => panic!("unexpected offset value {other:?}"),
                    })
                    .collect();
                let line = source.as_bytes()[..*offset as usize]
                    .iter()
                    .filter(|byte| **byte == b'\n')
                    .count()
                    + 1;
                serde_json::json!([line, names])
            })
            .collect();
        assert_eq!(
            serde_json::to_value(actual).unwrap(),
            oracle,
            "deferred={deferred}"
        );
    }
}

fn assert_fixture(fixture: &str, capture: &str, level: u8) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sequential-scope-joins");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join(capture)).unwrap()).unwrap();
    let mut failures = Vec::new();
    for mode in ["default", "quiet", "foreach-off", "loop-off"] {
        let config = resolve(&Options {
            config: Some(
                root.join(format!("{mode}.neon"))
                    .to_string_lossy()
                    .into_owned(),
            ),
            ..Options::for_project(&root.to_string_lossy())
        })
        .unwrap();
        let mut project = Project::from_config(&root, &config.parameters);
        project.analysed = vec![root.join(fixture)];
        project.level = level;
        let mut actual: Vec<_> = run(&project)
            .unwrap()
            .into_iter()
            .map(|error| {
                (
                    Path::new(&error.file)
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    error.line,
                    error.identifier,
                    error.message,
                    error.tip,
                    error.can_be_ignored,
                )
            })
            .collect();
        actual.sort();
        let actual = serde_json::to_value(actual).unwrap();
        if actual != expected[mode] {
            let want = expected[mode].as_array().unwrap();
            let got = actual.as_array().unwrap();
            failures.push(serde_json::json!({
                "mode": mode,
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
fn original_nullable_property_serialization_matches_phpstan() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("sequential-scope-joins/expected-original.json")).unwrap(),
    )
    .unwrap();
    let mut project = Project::new(
        vec![root.join("rules/data/Methods/data/bug-12110.php")],
        Vec::new(),
    );
    project.level = 3;
    project.php_version = 80500;
    let start = Instant::now();
    eprintln!("starting unchanged bug-12110.php through public run()");
    let mut actual: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| {
            (
                Path::new(&error.file)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                error.line,
                error.identifier,
                error.message,
                error.tip,
                error.can_be_ignored,
            )
        })
        .collect();
    eprintln!(
        "bug-12110.php: {:?}, {} records",
        start.elapsed(),
        actual.len()
    );
    actual.sort();
    assert_eq!(serde_json::to_value(actual).unwrap(), expected["default"]);
}
