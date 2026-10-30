//! PHPDoc union construction through the public analyser, using real PHPStan 2.3.1 expectations.

use std::path::Path;

use pun_analyse::{Project, run};
use serde_json::Value;

fn check(file: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/phpdoc-unions");
    let oracle: Value =
        serde_json::from_slice(&std::fs::read(root.join("expected-phpstan.json")).unwrap())
            .unwrap();
    let want: Vec<_> = oracle["errors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|error| error["file"] == file)
        .map(|error| {
            (
                error["line"].as_i64().unwrap(),
                error["identifier"].as_str().unwrap().to_owned(),
                error["message"].as_str().unwrap().to_owned(),
                error["tip"].as_str().map(str::to_owned),
            )
        })
        .collect();
    let got: Vec<_> = run(&Project::new(vec![root.join(file)], Vec::new()))
        .unwrap()
        .into_iter()
        .map(|error| (error.line, error.identifier, error.message, error.tip))
        .collect();
    assert!(!want.is_empty());
    assert_eq!(got, want, "{file}");
}

#[test]
fn flat_129_arm_union_uses_the_normal_test_stack() {
    check("flat.php");
}

#[test]
fn null_order_nesting_and_template_controls() {
    check("controls.php");
}

#[test]
fn native_return_checks_accept_benevolent_members_without_relaxing_strict_unions() {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_analyzer::{
        Analyzer,
        analysis_result::AnalysisResult,
        plugin::{ExpressionHook, HookContext, HookResult, PluginRegistry},
        settings::Settings,
    };
    use mago_codex::{
        populator::populate_codebase,
        reference::SymbolReferences,
        scanner::scan_program,
        ttype::{get_float, get_int, get_mixed_keyed_array, union::TUnion},
    };
    use mago_database::file::File;
    use mago_names::resolver::NameResolver;
    use mago_syntax::{cst::Expression, parser::parse_file};

    struct Candidate(TUnion);
    impl ExpressionHook for Candidate {
        fn after_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if matches!(expression, Expression::Call(_)) {
                context.set_expression_type(expression, self.0.clone());
            }
            Ok(())
        }
    }

    // PHPStan 2.3.1 BenevolentUnionType::isAcceptedBy ORs member acceptance.
    // Fresh public Type::accepts(..., true) controls for array<mixed>|float|int:
    // int and float accept only the benevolent union; string and bool accept neither.
    for (return_type, accepts_benevolent) in [
        ("int", true),
        ("float", true),
        ("string", false),
        ("bool", false),
    ] {
        let source = format!(
            "<?php declare(strict_types=1);\nfunction candidate(): mixed {{ return 1; }}\nfunction example(): {return_type} {{ return candidate(); }}"
        );
        let arena = LocalArena::new();
        let file = File::ephemeral(
            Cow::Borrowed(b"return.php"),
            Cow::Owned(source.into_bytes()),
        );
        let program = parse_file(&arena, &file);
        assert!(!program.has_errors());
        let names = NameResolver::new(&arena).resolve(program);
        let mut codebase = scan_program(&arena, &file, program, &names, Default::default());
        populate_codebase(
            &mut codebase,
            &mut SymbolReferences::new(),
            Default::default(),
            Default::default(),
        );
        for benevolent in [false, true] {
            let mut ty = TUnion::from_vec(vec![
                get_mixed_keyed_array().get_single().clone(),
                get_float().get_single().clone(),
                get_int().get_single().clone(),
            ]);
            ty.set_ignore_nullable_issues(benevolent);
            ty.set_ignore_falsable_issues(benevolent);
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(Candidate(ty));
            for native in [false, true] {
                let mut result = AnalysisResult::new(SymbolReferences::new());
                Analyzer::new(
                    &arena,
                    &file,
                    &names,
                    &codebase,
                    &registry,
                    Settings {
                        track_native_types: native,
                        ..Settings::default()
                    },
                )
                .analyze_with_artifacts(program, &mut result)
                .unwrap();
                let codes: Vec<_> = result
                    .issues
                    .iter()
                    .map(|issue| issue.code.as_deref())
                    .collect();
                let expected = if benevolent && accepts_benevolent {
                    vec![]
                } else {
                    vec![Some("invalid-return-statement")]
                };
                assert_eq!(
                    codes, expected,
                    "{return_type}, benevolent={benevolent}, native={native}"
                );
            }
        }
    }
}
