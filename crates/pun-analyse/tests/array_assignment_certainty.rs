//! Array writes must replace stale absence without changing unrelated conditional holders or index reads.
//! Expectations are PHPStan 2.3.1's JSON from fixtures/array-assignment-certainty/phpstan.neon.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

#[test]
fn written_arrays_accept_input_reference_mutation_and_preserve_other_certainties() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-assignment-certainty");
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let mut project = Project::new(vec![root.join("transfers.php")], Vec::new());
    project.level = 2;
    project.check_maybe_undefined_variables = true;
    let got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message))
        .collect();
    let mut want: Vec<_> = expected["files"]["transfers.php"]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_string(),
                error["message"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    want.sort_by(|left, right| (left.0, &left.2).cmp(&(right.0, &right.2)));
    assert_eq!(got, want);
}

#[test]
fn root_is_defined_only_after_its_indices_are_read() {
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use std::sync::{Arc, Mutex};

    use mago_allocator::LocalArena;
    use mago_analyzer::Analyzer;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::plugin::{
        ExpressionHook, ExpressionHookResult, HookContext, HookResult, PluginRegistry, Provider,
        ProviderMeta,
    };
    use mago_analyzer::settings::Settings;
    use mago_codex::scanner::scan_program;
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_span::HasSpan;
    use mago_syntax::cst::{Expression, Variable};
    use mago_syntax::parser::parse_file;

    struct Observe {
        reads: [u32; 2],
        found: Arc<Mutex<BTreeMap<u32, (bool, bool)>>>,
    }
    impl Provider for Observe {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("array-write", "array-write", "assignment root certainty");
            &META
        }
    }
    impl ExpressionHook for Observe {
        fn is_observer(&self) -> bool {
            true
        }

        fn before_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
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

    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/array-assignment-certainty");
    let source = std::fs::read_to_string(root.join("root-index.php")).unwrap();
    let expected: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let errors = expected["files"]["root-index.php"]["messages"]
        .as_array()
        .unwrap();
    assert!(
        errors
            .iter()
            .any(|error| { error["identifier"] == "variable.undefined" && error["line"] == 8 })
    );
    assert!(
        !errors
            .iter()
            .any(|error| { error["identifier"] == "variable.undefined" && error["line"] == 9 })
    );

    let reads = [
        source.find("$root]").unwrap() as u32,
        source.rfind("$root;").unwrap() as u32,
    ];
    let file = File::ephemeral(
        Cow::Borrowed(b"root-index.php"),
        Cow::Owned(source.into_bytes()),
    );
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let settings = Settings::default();
    let codebase = scan_program(&arena, &file, program, &names, settings.version);
    let found = Arc::new(Mutex::new(BTreeMap::new()));
    let mut registry = PluginRegistry::with_library_providers();
    registry.register_expression_hook(Observe {
        reads,
        found: Arc::clone(&found),
    });
    let mut result = AnalysisResult::new(Default::default());
    Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    assert_eq!(
        *found.lock().unwrap(),
        BTreeMap::from([(reads[0], (false, true)), (reads[1], (true, false))])
    );
}
