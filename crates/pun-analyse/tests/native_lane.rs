//! The partial native API is exercised through the public Analyzer, never Pun's converter.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::artifacts::AnalysisArtifacts;
use mago_analyzer::plugin::{
    ExpressionHook, HookContext, HookResult, PluginRegistry, Provider, ProviderMeta,
};
use mago_analyzer::settings::Settings;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::TType;
use mago_database::file::{File, FileType};
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::{Expression, Variable};
use mago_syntax::parser::parse_file;

#[derive(Default)]
struct Observed {
    visits: BTreeMap<(u32, u32), usize>,
    native: BTreeMap<(u32, u32), Option<String>>,
}

struct Observer(Arc<Mutex<Observed>>);
impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "native-observer",
            "native-observer",
            "single-walk observations",
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
        expr: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let span = expr.span();
        let range = (span.start.offset, span.end.offset);
        let native = context
            .get_native_expression_type(expr)
            .map(|ty| oracle_mago_id(&ty.get_id().to_string()));
        if let Expression::Variable(Variable::Direct(variable)) = expr {
            assert_eq!(
                native,
                context
                    .get_native_variable_type(variable.name)
                    .map(|ty| oracle_mago_id(&ty.get_id().to_string()))
            );
        }
        let mut observed = self.0.lock().unwrap();
        *observed.visits.entry(range).or_default() += 1;
        observed.native.insert(range, native);
        Ok(())
    }
}

fn analyze(
    code: &[u8],
    native: bool,
    mut registry: PluginRegistry,
) -> (AnalysisArtifacts, Observed, String) {
    let file = File::new(
        Cow::Borrowed(b"native.php"),
        FileType::Host,
        None,
        Cow::Owned(code.to_vec()),
    );
    let arena = LocalArena::new();
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
    let observed = Arc::<Mutex<Observed>>::default();
    registry.register_expression_hook(Observer(Arc::clone(&observed)));
    let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
    let mut result = AnalysisResult::new(references);
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    let found = std::mem::take(&mut *observed.lock().unwrap());
    (artifacts, found, format!("{:?}", result.issues))
}

fn marked_ranges(code: &[u8], marker: &str) -> Vec<(usize, (u32, u32))> {
    let text = std::str::from_utf8(code).unwrap();
    let mut offset = 0;
    let mut ranges = Vec::new();
    for (line, source) in text.split_inclusive('\n').enumerate() {
        if let Some((expression, _)) = source.split_once(marker) {
            let expression = expression.trim_end().strip_suffix(';').unwrap();
            let start = offset + expression.len() - expression.trim_start().len();
            ranges.push((line + 1, (start as u32, (offset + expression.len()) as u32)));
        }
        offset += source.len();
    }
    ranges
}

fn span(range: (u32, u32)) -> mago_span::Span {
    mago_span::Span::new(
        mago_database::file::FileId::zero(),
        mago_span::Position::new(range.0),
        mago_span::Position::new(range.1),
    )
}

// Only adapt the two tools' scalar display syntax, not their inferred types.
fn oracle_mago_id(phpstan: &str) -> String {
    if phpstan.contains('|') {
        let mut types: Vec<_> = phpstan.split('|').map(oracle_mago_id).collect();
        types.sort();
        types.join("|")
    } else if phpstan.starts_with('\'') {
        format!("string({phpstan})")
    } else if phpstan.parse::<i64>().is_ok() {
        format!("int({phpstan})")
    } else if phpstan.parse::<f64>().is_ok() {
        format!("float({phpstan})")
    } else {
        phpstan.to_owned()
    }
}

#[test]
fn scalar_native_values_match_phpstan_and_do_not_change_the_full_walk() {
    let code = include_bytes!("fixtures/native-lane/scalars.php");
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/native-lane/expected-phpstan.json")).unwrap();
    let (enabled, observed, issues) = analyze(code, true, PluginRegistry::new());
    let (disabled, off_observed, off_issues) = analyze(code, false, PluginRegistry::new());
    assert_eq!(issues, off_issues);
    assert_eq!(enabled.expression_types, disabled.expression_types);
    assert_eq!(observed.visits, off_observed.visits);
    assert!(observed.visits.values().all(|count| *count == 1));
    let marked = marked_ranges(code, "// native");
    assert_eq!(marked.len(), expected.as_array().unwrap().len());
    for ((line, range), expected) in marked.into_iter().zip(expected.as_array().unwrap()) {
        assert_eq!(line as u64, expected["line"].as_u64().unwrap());
        let want = expected["message"]
            .as_str()
            .unwrap()
            .strip_prefix("Dumped type: ")
            .unwrap();
        let want = oracle_mago_id(want);
        assert_eq!(
            enabled
                .get_native_expression_type(&span(range))
                .map(|ty| oracle_mago_id(&ty.get_id().to_string()))
                .as_deref(),
            Some(want.as_str()),
            "line {line}"
        );
        assert_eq!(
            observed.native[&range].as_deref(),
            Some(want.as_str()),
            "observer line {line}"
        );
        assert!(disabled.get_native_expression_type(&span(range)).is_none());
    }
}

#[test]
fn unsupported_transfers_poison_the_scope_before_children_and_never_recover_by_assignment() {
    let bodies = [
        "if ($flag) { $value = 2; } else { $value = 3; }",
        "while ($flag) { $value = 2; break; }",
        "for (; $flag;) { $value = 2; break; }",
        "do { $value = 2; } while ($flag);",
        "foreach ([2] as $value) {}",
        "switch ($flag) { case true: $value = 2; break; }",
        "try { $value = 2; } finally { $value = 3; }",
        "$alias =& $value; $alias = 2;",
        "$value += 2;",
        "$array = [2];",
        "$object = new stdClass();",
        "$value = $object->property;",
        "$object->method();",
        "Unknown::method();",
        "$callback = fn () => $value;",
        "$callback = function () use ($value) { return $value; };",
        "$value = $value + 1;",
        "$value = $flag ? 2 : 3;",
        "$value = missingFunction();",
        "$name = 'missingFunction'; $value = $name();",
        "$value = $missing;",
        "static $cached = 2;",
        "unset($value);",
        "/** @var positive-int */ $value;",
        "$GLOBALS = 2;",
        "$_SESSION = 2;",
        "$this = 2;",
    ];
    for body in bodies {
        let code = format!(
            "<?php function example(bool $flag): void {{\n$value = 1;\n$value; // before\n{body}\n$value = 99;\n$value; // unavailable\n}}\n"
        );
        let (artifacts, observed, issues) = analyze(code.as_bytes(), true, PluginRegistry::new());
        let (disabled, off_observed, off_issues) =
            analyze(code.as_bytes(), false, PluginRegistry::new());
        assert_eq!(
            artifacts.expression_types, disabled.expression_types,
            "{body}"
        );
        assert_eq!(observed.visits, off_observed.visits, "{body}");
        assert_eq!(issues, off_issues, "{body}");
        let before = marked_ranges(code.as_bytes(), "// before")[0].1;
        assert_eq!(
            artifacts
                .get_native_expression_type(&span(before))
                .unwrap()
                .get_id()
                .as_str(),
            Some("int(1)"),
            "{body}"
        );
        let unavailable = marked_ranges(code.as_bytes(), "// unavailable")[0].1;
        assert!(
            artifacts
                .get_native_expression_type(&span(unavailable))
                .is_none(),
            "{body}"
        );
    }
}

#[test]
fn unsupported_function_scopes_and_named_call_contracts_are_not_widened() {
    for parameters in [
        "int &$value",
        "int ...$value",
        "array $value",
        "object $value",
        "callable $value",
        "int $value = 1 + 2",
    ] {
        let code =
            format!("<?php function example({parameters}): void {{\n$value; // unavailable\n}}\n");
        let (artifacts, observed, _) = analyze(code.as_bytes(), true, PluginRegistry::new());
        let range = marked_ranges(code.as_bytes(), "// unavailable")[0].1;
        assert!(
            artifacts.get_native_expression_type(&span(range)).is_none(),
            "{parameters}"
        );
        assert_eq!(observed.native[&range], None);
    }
    for declaration in [
        "function target(int &$value): int { return $value; }",
        "function target(int ...$value): int { return 1; }",
        "function target(int $value): array { return []; }",
        "function &target(int $value): int { return $value; }",
        "function target(int $value) { yield $value; }",
        "function target(int $value): int { global $global; return $value; }",
        "/** @param-out int $value */ function target(int &$value): int { return $value; }",
        "/** @phpstan-assert positive-int $value */ function target(int $value): int { return $value; }",
    ] {
        let code = format!(
            "<?php {declaration}\nfunction example(int $value): void {{\n$result = target($value);\n$result; // unavailable\n}}\n"
        );
        let (artifacts, _, _) = analyze(code.as_bytes(), true, PluginRegistry::new());
        let range = marked_ranges(code.as_bytes(), "// unavailable")[0].1;
        assert!(
            artifacts.get_native_expression_type(&span(range)).is_none(),
            "{declaration}"
        );
    }
}

#[test]
fn semantic_hook_mutations_invalidate_native_answers() {
    use mago_codex::ttype::get_literal_int;

    struct Mutate(u8);
    impl Provider for Mutate {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "native-mutate",
                "native-mutate",
                "unsupported semantic mutation",
            );
            &META
        }
    }
    impl ExpressionHook for Mutate {
        fn after_expression(
            &self,
            expr: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if matches!(expr, Expression::Literal(mago_syntax::cst::Literal::Integer(value)) if value.value == Some(777))
            {
                match self.0 {
                    0 => context.set_expression_type(expr, get_literal_int(1)),
                    1 => context.set_variable_type(b"$value", get_literal_int(1)),
                    2 => {
                        context.artifacts_mut().expression_types.clear();
                    }
                    3 => {
                        *context.artifacts_mut() = AnalysisArtifacts::new();
                    }
                    _ => {
                        context.block_context_mut().locals.clear();
                    }
                }
            }
            Ok(())
        }
    }
    for mode in 0..5 {
        for code in [
            b"<?php function example(): void { $value = 777;\n$value; // unavailable\n}\n"
                .as_slice(),
            b"<?php function example(int $value = 777): void {\n$value; // unavailable\n}\n"
                .as_slice(),
        ] {
            let mut registry = PluginRegistry::new();
            registry.register_expression_hook(Mutate(mode));
            let (artifacts, _, _) = analyze(code, true, registry);
            let range = marked_ranges(code, "// unavailable")[0].1;
            assert!(
                artifacts.get_native_expression_type(&span(range)).is_none(),
                "mode {mode}"
            );
        }
    }
}

#[test]
fn opaque_hook_replacement_during_an_assignment_checkpoint_restores_both_histories() {
    struct Replace;
    impl Provider for Replace {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta = ProviderMeta::new(
                "native-checkpoint",
                "native-checkpoint",
                "opaque artifact replacement",
            );
            &META
        }
    }
    impl ExpressionHook for Replace {
        fn after_expression(
            &self,
            expr: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<()> {
            if matches!(expr, Expression::Literal(mago_syntax::cst::Literal::Integer(value)) if value.value == Some(777))
            {
                *context.artifacts_mut() = AnalysisArtifacts::new();
                // Repeated opaque access must not detach a second time or lose the saved native checkpoints.
                context.artifacts_mut().expression_types.clear();
            }
            Ok(())
        }
    }
    let code = b"<?php function example(): void {\n$value = 1;\n$value; // before\n$value += 777;\n$value; // unavailable\n}\n";
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Replace);
    let (artifacts, _, _) = analyze(code, true, registry);
    let before = span(marked_ranges(code, "// before")[0].1);
    assert_eq!(
        artifacts
            .get_expression_type(&before)
            .unwrap()
            .get_id()
            .as_str(),
        Some("int(1)")
    );
    assert_eq!(
        artifacts
            .get_native_expression_type(&before)
            .unwrap()
            .get_id()
            .as_str(),
        Some("int(1)")
    );
    let after = span(marked_ranges(code, "// unavailable")[0].1);
    assert!(artifacts.get_native_expression_type(&after).is_none());
}

#[test]
fn raw_callback_gathers_do_not_publish_native_state_or_extra_observer_visits() {
    let original = include_str!("fixtures/callback-gather/error.php");
    let code = original.replacen("<?php", "<?php\n$safe = 8;\n$safe; // before\n", 1);
    let (enabled, observed, issues) = analyze(code.as_bytes(), true, PluginRegistry::new());
    let (disabled, off_observed, off_issues) =
        analyze(code.as_bytes(), false, PluginRegistry::new());
    assert_eq!(enabled.expression_types, disabled.expression_types);
    assert_eq!(issues, off_issues);
    assert_eq!(observed.visits, off_observed.visits);
    let calls: Vec<_> = observed
        .visits
        .iter()
        .filter(|((start, end), _)| {
            &code.as_bytes()[*start as usize..*end as usize] == b"failGather()"
        })
        .collect();
    assert_eq!(calls.len(), 1);
    assert_eq!(
        *calls[0].1, 1,
        "the raw gather must not publish an observer event"
    );
    assert_eq!(
        observed.native[calls[0].0], None,
        "callback scopes are deliberately unsupported"
    );
    let before = span(marked_ranges(code.as_bytes(), "// before")[0].1);
    assert_eq!(
        enabled
            .get_native_expression_type(&before)
            .unwrap()
            .get_id()
            .as_str(),
        Some("int(8)")
    );
}

#[test]
fn dynamic_full_return_providers_are_called_once_and_do_not_replace_native_declarations() {
    use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
    use mago_analyzer::plugin::provider::{FunctionReturnTypeProvider, FunctionTarget};
    use mago_codex::ttype::{get_literal_string, union::TUnion};
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct Return(Arc<AtomicUsize>);
    impl Provider for Return {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("native-return", "native-return", "full-only dynamic return");
            &META
        }
    }
    impl FunctionReturnTypeProvider for Return {
        fn targets() -> FunctionTarget {
            FunctionTarget::Exact(b"documented")
        }
        fn get_return_type(
            &self,
            _: &ProviderContext<'_, '_, '_>,
            _: &InvocationInfo<'_, '_, '_>,
        ) -> Option<TUnion> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Some(get_literal_string(mago_word::word(b"provider")))
        }
    }
    let calls = Arc::new(AtomicUsize::new(0));
    let mut registry = PluginRegistry::new();
    registry.register_function_provider(Return(Arc::clone(&calls)));
    let code = b"<?php function documented(): string { return 'body'; }\n$value = documented();\n$value; // native\n";
    let (artifacts, _, _) = analyze(code, true, registry);
    let value = span(marked_ranges(code, "// native")[0].1);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        artifacts
            .get_expression_type(&value)
            .unwrap()
            .get_id()
            .as_str(),
        Some("string('provider')")
    );
    // The oracle's documented(): string call in scalars.php also has native string.
    assert_eq!(
        artifacts
            .get_native_expression_type(&value)
            .unwrap()
            .get_id()
            .as_str(),
        Some("string")
    );
}

#[test]
fn hook_skips_do_not_leave_native_values_from_before_the_skipped_transfer() {
    use mago_analyzer::plugin::{
        ExpressionHookResult, FunctionCallHook, HookAction, StatementHook,
    };
    use mago_codex::ttype::get_literal_int;
    use mago_syntax::cst::{FunctionCall, Statement};

    struct Skip(u8);
    impl Provider for Skip {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("native-skip", "native-skip", "semantic skip");
            &META
        }
    }
    impl StatementHook for Skip {
        fn before_statement(
            &self,
            statement: &Statement<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<HookAction> {
            Ok(
                if self.0 == 0
                    && matches!(statement, Statement::Expression(value) if skipped_assignment(value.expression))
                {
                    HookAction::Skip
                } else {
                    HookAction::Continue
                },
            )
        }
    }
    fn skipped_assignment(expression: &Expression<'_>) -> bool {
        matches!(expression, Expression::Assignment(value) if matches!(value.rhs,
            Expression::Literal(mago_syntax::cst::Literal::Integer(value)) if value.value == Some(777)))
    }
    impl ExpressionHook for Skip {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            Ok(match (self.0, skipped_assignment(expression)) {
                (1, true) => ExpressionHookResult::Skip,
                (2, true) => ExpressionHookResult::SkipWithType(get_literal_int(2)),
                _ => ExpressionHookResult::Continue,
            })
        }
    }
    impl FunctionCallHook for Skip {
        fn before_function_call(
            &self,
            _: &FunctionCall<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            Ok(match self.0 {
                3 => ExpressionHookResult::Skip,
                4 => ExpressionHookResult::SkipWithType(get_literal_int(2)),
                _ => ExpressionHookResult::Continue,
            })
        }
    }
    for mode in 0..5 {
        let mut registry = PluginRegistry::new();
        registry.register_statement_hook(Skip(mode));
        registry.register_expression_hook(Skip(mode));
        registry.register_function_call_hook(Skip(mode));
        let code = b"<?php function target(): int { return 1; }\n$value = 1; $value = 777; target();\n$value; // unavailable\n";
        let (artifacts, _, _) = analyze(code, true, registry);
        let after = span(marked_ranges(code, "// unavailable")[0].1);
        assert!(
            artifacts.get_native_expression_type(&after).is_none(),
            "mode {mode}"
        );
    }
}
