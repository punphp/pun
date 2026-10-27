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
use mago_codex::ttype::atomic::scalar::string::TStringCasing;
use mago_codex::ttype::union::TUnion;
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
        let native = context.get_native_expression_type(expr).map(native_id);
        if let Expression::Variable(Variable::Direct(variable)) = expr {
            assert_eq!(
                native,
                context
                    .get_native_variable_type(variable.name)
                    .map(native_id)
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
    let observed = Arc::<Mutex<Observed>>::default();
    registry.register_expression_hook(Observer(Arc::clone(&observed)));
    let (artifacts, result) = analyze_with_registry(code, native, registry);
    let found = std::mem::take(&mut *observed.lock().unwrap());
    (artifacts, found, format!("{:?}", result.issues))
}

// Unlike analyze(), this path does not install a custom observer that disables convergence.
fn analyze_with_registry(
    code: &[u8],
    native: bool,
    registry: PluginRegistry,
) -> (AnalysisArtifacts, AnalysisResult) {
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
    let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
    let mut result = AnalysisResult::new(references);
    let artifacts = analyzer
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    (artifacts, result)
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

fn native_id(ty: &TUnion) -> String {
    let mut id = oracle_mago_id(&ty.get_id().to_string());
    // TString stores casing on numeric strings, but its display omits that property.
    if ty.get_single_string().is_some_and(|string| {
        string.literal.is_none() && string.is_numeric && string.casing == TStringCasing::Uppercase
    }) {
        id.push_str("&uppercase-string");
    }
    id
}

// Only adapt the two tools' scalar display syntax, not their inferred types.
fn oracle_mago_id(phpstan: &str) -> String {
    let phpstan = if phpstan.starts_with('(') && phpstan.ends_with(')') && phpstan.contains('|') {
        &phpstan[1..phpstan.len() - 1]
    } else {
        phpstan
    };
    if phpstan.contains('|') {
        let mut types: Vec<_> = phpstan.split('|').map(oracle_mago_id).collect();
        types.sort();
        types.join("|")
    } else if let Some(value) = phpstan
        .strip_prefix("float(")
        .and_then(|value| value.strip_suffix(')'))
    {
        format!("float({})", value.parse::<f64>().unwrap())
    } else if phpstan.starts_with('\'') {
        format!("string({phpstan})")
    } else if phpstan.parse::<i64>().is_ok() {
        format!("int({phpstan})")
    } else if let Ok(value) = phpstan.parse::<f64>() {
        format!("float({value})")
    } else if let Some(range) = match phpstan {
        "positive-int" => Some("int<1,max>"),
        "negative-int" => Some("int<min,-1>"),
        "non-positive-int" => Some("int<min,0>"),
        "non-negative-int" => Some("int<0,max>"),
        _ => None,
    } {
        range.to_owned()
    } else {
        phpstan
            .replace("non-falsy-string", "truthy-string")
            .replace("*NEVER*", "never")
            .replace(", ", ",")
    }
}

#[test]
fn scalar_native_values_match_phpstan_and_do_not_change_the_full_walk() {
    check_native_fixture(
        include_bytes!("fixtures/native-lane/scalars.php"),
        include_str!("fixtures/native-lane/expected-phpstan.json"),
    );
}

#[test]
fn operator_native_values_match_phpstan_and_do_not_change_the_full_walk() {
    check_native_fixture(
        include_bytes!("fixtures/native-lane/operators.php"),
        include_str!("fixtures/native-lane/expected-operators-phpstan.json"),
    );
}

#[test]
fn cast_native_values_match_phpstan_and_do_not_change_the_full_walk() {
    assert!(!Settings::default().track_native_types);
    check_native_fixture(
        include_bytes!("fixtures/native-lane/casts.php"),
        include_str!("fixtures/native-lane/expected-casts-phpstan.json"),
    );
}

#[test]
fn conditional_native_values_and_continuations_match_phpstan_without_another_walk() {
    check_native_fixture(
        include_bytes!("fixtures/native-lane/conditions.php"),
        include_str!("fixtures/native-lane/expected-conditions-phpstan.json"),
    );
}

#[test]
fn scalar_native_guards_match_phpstan_without_changing_full_assertions() {
    check_native_fixture(
        include_bytes!("fixtures/native-lane/condition-guards.php"),
        include_str!("fixtures/native-lane/expected-condition-guards-phpstan.json"),
    );
}

#[test]
fn scalar_native_conditional_edges_match_phpstan() {
    check_native_fixture(
        include_bytes!("fixtures/native-lane/condition-edges.php"),
        include_str!("fixtures/native-lane/expected-condition-edges-phpstan.json"),
    );
}

#[test]
fn conditional_native_unavailability_is_edge_local_and_does_not_recover_by_assignment() {
    check_native_fixture(
        include_bytes!("fixtures/native-lane/condition-boundaries.php"),
        include_str!("fixtures/native-lane/expected-condition-boundaries-phpstan.json"),
    );
}

#[test]
fn pure_condition_wrappers_retain_the_full_walks_existing_visits() {
    let observed = check_native_fixture_walk(
        include_bytes!("fixtures/native-lane/condition-wrappers.php"),
        include_str!("fixtures/native-lane/expected-condition-wrappers-phpstan.json"),
    );
    assert!(observed.visits.values().any(|count| *count > 1));
}

fn assert_full_artifacts_equal(enabled: &AnalysisArtifacts, disabled: &AnalysisArtifacts) {
    assert_eq!(enabled.expression_types, disabled.expression_types);
    assert_eq!(enabled.call_terminations, disabled.call_terminations);
    assert_eq!(enabled.if_true_assertions, disabled.if_true_assertions);
    assert_eq!(enabled.if_false_assertions, disabled.if_false_assertions);
    assert_eq!(
        enabled.true_branch_only_assertions,
        disabled.true_branch_only_assertions
    );
    assert_eq!(
        enabled.inferred_return_types,
        disabled.inferred_return_types
    );
    assert_eq!(
        enabled.inferred_yield_key_types,
        disabled.inferred_yield_key_types
    );
    assert_eq!(
        enabled.inferred_yield_value_types,
        disabled.inferred_yield_value_types
    );
    assert_eq!(enabled.symbol_references, disabled.symbol_references);
    assert_eq!(
        enabled.fully_matched_switch_offsets,
        disabled.fully_matched_switch_offsets
    );
    assert_eq!(
        enabled.inferred_parameter_types,
        disabled.inferred_parameter_types
    );
    assert_eq!(
        enabled.method_initialized_properties,
        disabled.method_initialized_properties
    );
    assert_eq!(
        enabled.method_calls_this_methods,
        disabled.method_calls_this_methods
    );
    assert_eq!(
        enabled.method_calls_parent_constructor,
        disabled.method_calls_parent_constructor
    );
    assert_eq!(
        enabled.method_calls_parent_initializer,
        disabled.method_calls_parent_initializer
    );
    assert_eq!(
        enabled.resolved_method_calls,
        disabled.resolved_method_calls
    );
    assert_eq!(
        enabled
            .closure_bind_scope
            .as_ref()
            .map(|scope| (scope.class_name, scope.has_this)),
        disabled
            .closure_bind_scope
            .as_ref()
            .map(|scope| (scope.class_name, scope.has_this))
    );
    assert_eq!(
        enabled
            .case_scopes
            .iter()
            .map(|scope| &scope.break_vars)
            .collect::<Vec<_>>(),
        disabled
            .case_scopes
            .iter()
            .map(|scope| &scope.break_vars)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        format!("{:?}", enabled.loop_scope),
        format!("{:?}", disabled.loop_scope)
    );
}

fn check_native_fixture(code: &[u8], oracle: &str) {
    let observed = check_native_fixture_walk(code, oracle);
    assert!(observed.visits.values().all(|count| *count == 1));
}

fn check_native_fixture_walk(code: &[u8], oracle: &str) -> Observed {
    let expected: serde_json::Value = serde_json::from_str(oracle).unwrap();
    let (enabled, observed, issues) = analyze(code, true, PluginRegistry::new());
    let (disabled, off_observed, off_issues) = analyze(code, false, PluginRegistry::new());
    assert_eq!(issues, off_issues);
    assert_full_artifacts_equal(&enabled, &disabled);
    assert_eq!(observed.visits, off_observed.visits);
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
                .map(native_id)
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
    for (line, range) in marked_ranges(code, "// unavailable") {
        assert!(
            enabled.get_native_expression_type(&span(range)).is_none(),
            "line {line}"
        );
        assert!(
            observed.native.get(&range).is_none_or(Option::is_none),
            "observer line {line}"
        );
    }
    observed
}

#[test]
fn unsupported_transfers_poison_the_scope_before_children_and_never_recover_by_assignment() {
    let bodies = [
        "if ($flag && true) { $value = 2; } else { $value = 3; }",
        "if ($value === ($value + 1)) { $value = 2; }",
        "if ($value + 1) { $value = 2; }",
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
        "$value = $value << 1;",
        "$value = ($flag && true) ? 2 : 3;",
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
fn unsupported_operator_shapes_invalidate_without_changing_the_full_walk() {
    for expression in [
        "-$text",
        "+true",
        "$integer + '2'",
        "null + 1",
        "$unknown + 2",
        "$nullable + 2",
        "$decimal % 2",
        "$integer % $integer",
        "$integer / 0",
        "$integer % 0",
        "1e308 * 1e308",
        "+1e999",
        "$unknown === 2",
        "$nullable == 2",
        "($integer % 2) == 1",
        "('prefix' . $text) === '0'",
        "$text == '1'",
        "$text < 2",
        "true == false",
        "'x' . 1",
        "1 . 'x'",
        "$integer ** 2",
        "$integer & 2",
        "$integer ^ 2",
        "$integer | 2",
        "$integer >> 2",
        "~$integer",
        "++$integer",
        "$integer++",
        "$flag && $flag",
        "$flag || $flag",
        "$flag and $flag",
        "$flag or $flag",
        "$nullable ?? 2",
        "($integer % 2) + 1",
        // Decimal-integer-string has no exact Mago scalar representation. Other casts below
        // deliberately exclude union operands, non-finite literals, and non-scalar syntax.
        "(string) $integer",
        "(string) ($integer % 2)",
        "(int) $nullable",
        "(float) $nullable",
        "(bool) $nullable",
        "(string) $nullable",
        "(int) (int) $flag",
        "(float) (string) 0.0",
        "(bool) 1e999",
        "(int) 1e999",
        "(float) 1e999",
        "(string) 1e999",
        "(float) '1e999'",
        "(array) $value",
        "(object) $value",
        "(int) [1]",
        "(bool) new stdClass()",
        "(int) $missing",
        "(int) (-$text)",
    ] {
        let code = format!(
            "<?php function example(int $integer, float $decimal, string $text, bool $flag, ?int $nullable, mixed $unknown): void {{\n$value = 1;\n$value; // before\n{expression}; // unavailable\n$value = 99;\n$value; // unavailable\n}}\n"
        );
        let (enabled, observed, issues) = analyze(code.as_bytes(), true, PluginRegistry::new());
        let (disabled, off_observed, off_issues) =
            analyze(code.as_bytes(), false, PluginRegistry::new());
        assert_full_artifacts_equal(&enabled, &disabled);
        assert_eq!(observed.visits, off_observed.visits, "{expression}");
        assert_eq!(issues, off_issues, "{expression}");
        let before = marked_ranges(code.as_bytes(), "// before")[0].1;
        assert_eq!(
            enabled
                .get_native_expression_type(&span(before))
                .unwrap()
                .get_id()
                .as_str(),
            Some("int(1)")
        );
        for (_, range) in marked_ranges(code.as_bytes(), "// unavailable") {
            assert!(
                enabled.get_native_expression_type(&span(range)).is_none(),
                "{expression}"
            );
            assert_eq!(observed.native[&range], None, "{expression}");
        }
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
        "int $value = 1 << 2",
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
    let code = b"<?php function documented(): string { return 'body'; }\n$value = documented();\n$value; // native\nfunction branches(bool $flag): void {\nif ($flag) { $value = documented(); } else { $value = documented(); }\n$flag ? documented() : documented();\n}\n";
    let (artifacts, observed, issues) = analyze(code, true, registry);
    let off_calls = Arc::new(AtomicUsize::new(0));
    let mut registry = PluginRegistry::new();
    registry.register_function_provider(Return(Arc::clone(&off_calls)));
    let (disabled, off_observed, off_issues) = analyze(code, false, registry);
    assert_full_artifacts_equal(&artifacts, &disabled);
    assert_eq!(issues, off_issues);
    assert_eq!(observed.visits, off_observed.visits);
    let value = span(marked_ranges(code, "// native")[0].1);
    // One straight-line call and two calls in each conditional, on the existing full walk.
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    assert_eq!(off_calls.load(Ordering::SeqCst), 5);
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

#[test]
fn replayed_calls_do_not_retain_termination_after_a_hook_replaces_the_evaluation() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use mago_analyzer::plugin::ExpressionHookResult;
    use mago_codex::ttype::get_literal_int;

    struct Rewrite {
        visits: Arc<AtomicUsize>,
        mode: u8,
    }
    impl Provider for Rewrite {
        fn meta() -> &'static ProviderMeta {
            static META: ProviderMeta =
                ProviderMeta::new("call-replay", "call-replay", "replace a replayed call");
            &META
        }
    }
    impl ExpressionHook for Rewrite {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            context: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            if !matches!(expression, Expression::Call(_))
                || self.visits.fetch_add(1, Ordering::SeqCst) == 0
            {
                return Ok(ExpressionHookResult::Continue);
            }
            Ok(match self.mode {
                0 => ExpressionHookResult::Skip,
                1 => ExpressionHookResult::SkipWithType(get_literal_int(123)),
                _ => {
                    context.set_variable_type(b"$f", get_literal_int(0));
                    ExpressionHookResult::Continue
                }
            })
        }
    }

    for loop_body in [
        "for (; $more; $f()) { $a = 's'; }",
        "do { $a = 's'; } while ($f());",
    ] {
        let code = format!(
            "<?php function stop(): never {{ throw new Exception(); }}\n\
             $a = 0; $more = true; $f = 'stop'; {loop_body}"
        );
        let start = code.rfind("$f()").unwrap() as u32;
        let range = (start, start + 4);
        for mode in 0..3 {
            for native in [false, true] {
                let visits = Arc::new(AtomicUsize::new(0));
                let mut registry = PluginRegistry::new();
                registry.register_expression_hook(Rewrite {
                    visits: Arc::clone(&visits),
                    mode,
                });
                let (artifacts, _, _) = analyze(code.as_bytes(), native, registry);
                assert!(visits.load(Ordering::SeqCst) > 1, "{loop_body}");
                assert_eq!(
                    artifacts.call_terminations.get(&range),
                    Some(&false),
                    "{loop_body}, mode {mode}, native {native}"
                );
                if mode == 1 {
                    assert_eq!(
                        artifacts.get_expression_type(&span(range)),
                        Some(&get_literal_int(123))
                    );
                }
            }
        }
    }
}

#[test]
fn inspection_and_confirmation_report_types_without_terminating_the_call_flow() {
    use mago_reporting::Level;

    let code = br"<?php
namespace Mago {
    function inspect(mixed ...$values): void {}
    function confirm(mixed $value, string $expected): void {}
}
namespace {
    Mago\inspect($value = 7); // call
    echo
        $value; // value
    Mago\confirm($value, 'int(7)'); // call
    echo
        $value; // value
    Mago\confirm($value, 'string'); // call
    echo
        $value; // value
}
";
    let (enabled, result) =
        analyze_with_registry(code, true, PluginRegistry::with_library_providers());
    let (disabled, off_result) =
        analyze_with_registry(code, false, PluginRegistry::with_library_providers());
    assert_full_artifacts_equal(&enabled, &disabled);
    assert_eq!(result.issues, off_result.issues);
    let diagnostics = result
        .issues
        .iter()
        .map(|issue| (issue.level, issue.code.as_deref(), issue.message.as_ref()))
        .collect::<Vec<_>>();
    assert_eq!(
        diagnostics,
        [
            (
                Level::Help,
                Some("type-inspection"),
                "Type information for arguments of `Mago\\inspect()` call."
            ),
            (
                Level::Help,
                Some("type-confirmation"),
                "Type of expression is `int(7)` as expected."
            ),
            (
                Level::Error,
                Some("type-confirmation"),
                "Type of expression is `int(7)`, but expected `string`."
            ),
        ]
    );
    assert!(
        result
            .issues
            .iter()
            .next()
            .unwrap()
            .annotations
            .iter()
            .any(|annotation| annotation.message.as_deref() == Some("Argument #1 type: `int(7)`"))
    );
    for (_, range) in marked_ranges(code, "// call") {
        assert_eq!(enabled.call_terminations.get(&range), Some(&false));
        assert_eq!(
            enabled
                .get_expression_type(&span(range))
                .map(native_id)
                .as_deref(),
            Some("void")
        );
    }
    for (_, range) in marked_ranges(code, "// value") {
        assert_eq!(
            enabled
                .get_expression_type(&span(range))
                .map(native_id)
                .as_deref(),
            Some("int(7)")
        );
    }
}

#[test]
fn converged_loops_preserve_continuing_and_terminating_call_facts() {
    let code = br"<?php
function keep(): int { return 7; }
function stop(): never { while (true) {} }
function example(bool $more, bool $halt): void {
    $a = 0; // before
    $b = 0; $c = 0;
    while ($more) {
        $a = 'a';
        while ($more) {
            $b = 'b';
            while ($more) {
                $c = 'c';
                keep(); // continuing
                if ($halt) {
                    stop(); // terminating
                }
                echo
                    $c; // inside
            }
        }
    }
    echo
        $a; // after
    echo
        $b; // after
    echo
        $c; // after
}
";
    let registry = PluginRegistry::with_library_providers();
    assert!(!registry.has_custom_plugins());
    let (enabled, result) = analyze_with_registry(code, true, registry);
    let (disabled, off_result) =
        analyze_with_registry(code, false, PluginRegistry::with_library_providers());
    let (unoptimized, _, unoptimized_issues) =
        analyze(code, false, PluginRegistry::with_library_providers());
    assert_full_artifacts_equal(&enabled, &disabled);
    assert_full_artifacts_equal(&disabled, &unoptimized);
    assert_eq!(result.issues, off_result.issues);
    assert_eq!(format!("{:?}", result.issues), unoptimized_issues);
    assert!(result.issues.is_empty(), "{:?}", result.issues);
    for (marker, terminates, ty) in [
        ("// continuing", false, "int"),
        ("// terminating", true, "never"),
        ("// inside", false, "string('c')"),
    ] {
        let range = marked_ranges(code, marker)[0].1;
        assert_eq!(
            enabled
                .get_expression_type(&span(range))
                .map(native_id)
                .as_deref(),
            Some(ty),
            "{marker}"
        );
        if marker != "// inside" {
            assert_eq!(enabled.call_terminations.get(&range), Some(&terminates));
        }
    }
    for ((_, range), ty) in marked_ranges(code, "// after").into_iter().zip([
        "int(0)|string('a')",
        "int(0)|string('b')",
        "int(0)|string('c')",
    ]) {
        assert_eq!(
            enabled
                .get_expression_type(&span(range))
                .map(native_id)
                .as_deref(),
            Some(ty)
        );
        assert!(enabled.get_native_expression_type(&span(range)).is_none());
    }
    let before = span(marked_ranges(code, "// before")[0].1);
    assert_eq!(
        enabled
            .get_native_expression_type(&before)
            .map(native_id)
            .as_deref(),
        Some("int(0)")
    );
    assert!(disabled.get_native_expression_type(&before).is_none());
}
