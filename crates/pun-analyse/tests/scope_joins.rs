//! Real-edge scope joins and foreach projection, against PHPStan 2.3.1.

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};
use serde_json::Value;

#[test]
fn foreach_entry_scopes_match_phpstan() {
    assert_fixture("foreach-entry.php", "expected-foreach-entry.json");
}

#[test]
fn shared_scope_joins_match_phpstan() {
    assert_fixture("matrix.php", "expected.json");
}

#[test]
fn null_context_specifications_match_phpstan() {
    assert_fixture(
        "null-specifications.php",
        "expected-null-specifications.json",
    );
}

#[test]
fn plugin_null_context_specifications_match_phpstan() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::analysis_result::AnalysisResult;
    use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
    use mago_analyzer::plugin::provider::assertion::{
        FunctionAssertionProvider, InvocationAssertions,
    };
    use mago_analyzer::plugin::provider::function::FunctionTarget;
    use mago_analyzer::plugin::{PluginRegistry, Provider, ProviderMeta};
    use mago_analyzer::settings::Settings;
    use mago_codex::assertion::Assertion;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_codex::ttype::TType;
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_syntax::parser::parse_file;

    struct RequiresString;
    impl Provider for RequiresString {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("scope-joins", "scope-joins", "string assertion");
            &META
        }
    }
    impl FunctionAssertionProvider for RequiresString {
        fn targets() -> FunctionTarget {
            FunctionTarget::Exact(b"sharednullspecifications\\providerrequiresstring")
        }

        fn get_assertions(
            &self,
            _: &ProviderContext<'_, '_, '_>,
            _: &InvocationInfo<'_, '_, '_>,
        ) -> Option<InvocationAssertions> {
            let mut assertions = InvocationAssertions::new();
            assertions.add_immediate(
                mago_word::word("$value"),
                vec![Assertion::IsType(
                    mago_codex::ttype::get_string().get_single().clone(),
                )],
            );
            Some(assertions)
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scope-joins");
    let source = std::fs::read_to_string(root.join("null-specifications.php")).unwrap();
    // Exercise the plugin contract without also receiving the declaration's same assertion.
    // Spaces preserve the independently captured oracle's expression spans and line numbers.
    let annotation = "/** @phpstan-assert string $value */";
    let code = source.replace(annotation, &" ".repeat(annotation.len()));
    let file = File::new(
        Cow::Borrowed(b"null-specifications.php"),
        FileType::Host,
        None,
        Cow::Owned(code.into_bytes()),
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
    let mut registry = PluginRegistry::new();
    registry.register_function_assertion_provider(RequiresString);
    let analyzer =
        mago_analyzer::Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
    let mut result = AnalysisResult::new(references);
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join("expected-null-specifications.json")).unwrap(),
    )
    .unwrap();
    let mut offset = 0;
    let mut checked = 0;
    for (line, text) in source.split_inclusive('\n').enumerate() {
        if text.contains("// provider observation") {
            let start = (offset + text.find("$value").unwrap()) as u32;
            let actual = artifacts.expression_types.get(&(start, start + 6)).unwrap();
            let record = expected["default"]
                .as_array()
                .unwrap()
                .iter()
                .find(|record| record[1] == line + 1 && record[2] == "phpstan.dumpType")
                .unwrap();
            let want = record[3]
                .as_str()
                .unwrap()
                .strip_prefix("Dumped type: ")
                .unwrap();
            assert_eq!(actual.get_id().to_string(), want, "line {}", line + 1);
            checked += 1;
        }
        offset += text.len();
    }
    assert_eq!(checked, 3);
}

fn assert_fixture(fixture: &str, capture: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/scope-joins");
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
            let missing: Vec<_> = want.iter().filter(|e| !got.contains(e)).collect();
            let extra: Vec<_> = got.iter().filter(|e| !want.contains(e)).collect();
            failures.push(serde_json::json!({
                "mode": mode,
                "actual": got.len(),
                "expected": want.len(),
                "missing": missing,
                "extra": extra,
            }));
        }
    }
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
}
