//! Public inference observations, independently of Pun's rule/type converter.
use mago_allocator::LocalArena;
use mago_analyzer::{
    Analyzer, analysis_result::AnalysisResult, artifacts::AnalysisArtifacts,
    observations::CallScopeRequest, plugin::*, settings::Settings,
};
use mago_codex::ttype::{
    TType,
    atomic::{
        TAtomic,
        array::{TArray, key::ArrayKey},
        callable::TCallable,
        object::TObject,
        scalar::{TScalar, int::TInteger, string::TStringLiteral},
    },
    union::TUnion,
};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Position, Span};
use mago_syntax::{
    cst::{
        ArgumentList, Call, ClassLikeMemberSelector, Expression, FunctionCall, MethodCall,
        NullSafeMethodCall, StaticMethodCall,
    },
    parser::parse_file,
};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/matrix.php");
const ORACLE: &str = include_str!("fixtures/call-scope-observations/expected.json");

// Type unions are sets. PHPStan's presentation order differs from Mago's; keep
// every member, shape key and literal while canonicalizing only union order.
fn canonical(s: &str) -> String {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    for (i, byte) in s.bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            b'|' if depth == 0 => {
                parts.push(canonical(&s[start..i]));
                start = i + 1;
            }
            _ => {}
        }
    }
    if start != 0 {
        parts.push(canonical(&s[start..]));
        parts.sort();
        return parts.join("|");
    }
    if let Some(inner) = s.strip_prefix("array{").and_then(|s| s.strip_suffix('}')) {
        let mut items = Vec::new();
        let mut start = 0;
        let mut depth = 0;
        for (i, b) in inner.bytes().enumerate() {
            match b {
                b'{' => depth += 1,
                b'}' => depth -= 1,
                b',' if depth == 0 => {
                    items.push(canonical(inner[start..i].trim()));
                    start = i + 1;
                }
                _ => {}
            }
        }
        items.push(canonical(inner[start..].trim()));
        return format!("array{{{}}}", items.join(", "));
    }
    s.into()
}

fn describe(t: &TUnion) -> String {
    let mut parts: Vec<_> = t
        .types
        .iter()
        .map(|a| match a {
            TAtomic::Scalar(TScalar::Integer(TInteger::Literal(n))) => n.to_string(),
            TAtomic::Scalar(TScalar::String(s))
                if matches!(s.literal, Some(TStringLiteral::Value(_))) =>
            {
                let Some(TStringLiteral::Value(s)) = s.literal else {
                    unreachable!()
                };
                format!("'{}'", s)
            }
            TAtomic::Object(TObject::Named(o)) => o.get_name().to_string(),
            TAtomic::Callable(TCallable::Signature(c))
                if c.is_closure && c.parameters.is_empty() =>
            {
                format!("Closure(): {}", describe(c.return_type.as_ref().unwrap()))
            }
            TAtomic::Array(TArray::List(a)) => format!(
                "array{{{}}}",
                a.known_elements
                    .as_ref()
                    .unwrap()
                    .values()
                    .map(|(_, t)| describe(t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TAtomic::Array(TArray::Keyed(a)) => format!(
                "array{{{}}}",
                a.known_items
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, (_, t))| match k {
                        ArrayKey::String(s) => format!("{s}: {}", describe(t)),
                        ArrayKey::Integer(n) => format!("{n}: {}", describe(t)),
                        _ => panic!("unexpected symbolic key in oracle fixture"),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TAtomic::Never => "*NEVER*".into(),
            _ => a.get_id().to_string(),
        })
        .collect();
    parts.sort();
    parts.join("|")
}

fn analyze(
    code: &'static [u8],
    oracle: &str,
    observe: bool,
    registry: &PluginRegistry,
) -> (File, AnalysisArtifacts, AnalysisResult) {
    analyze_with_settings(code, oracle, observe, registry, Settings::default())
}

fn analyze_with_settings(
    code: &'static [u8],
    oracle: &str,
    observe: bool,
    registry: &PluginRegistry,
    mut settings: Settings,
) -> (File, AnalysisArtifacts, AnalysisResult) {
    let expected: Value = serde_json::from_str(oracle).unwrap();
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"observations.php"), Cow::Borrowed(code));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    for (call, data) in expected["calls"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|_| observe)
    {
        let start: u32 = call.parse().unwrap();
        let call = Span::new(
            file.id,
            Position::new(start),
            Position::new(start + data["expression"].as_str().unwrap().len() as u32),
        );
        let expressions = data["names"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|n| n["types"].as_array().unwrap())
            .map(|e| {
                Span::new(
                    file.id,
                    Position::new(e["start"].as_u64().unwrap() as u32),
                    Position::new(e["end"].as_u64().unwrap() as u32),
                )
            })
            // A requested span not evaluated by the real walk must stay absent.
            .chain(std::iter::once(Span::new(
                file.id,
                Position::new(code.len() as u32),
                Position::new(code.len() as u32),
            )))
            .collect();
        settings
            .call_scope_observations
            .push(CallScopeRequest { call, expressions });
    }
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let mut result = AnalysisResult::new(references);
    let artifacts = Analyzer::new(&arena, &file, &names, &codebase, registry, settings)
        .analyze_with_artifacts(program, &mut result)
        .unwrap();
    (file, artifacts, result)
}

fn check(file: &File, artifacts: &AnalysisArtifacts, oracle: &str) {
    let expected: Value = serde_json::from_str(oracle).unwrap();
    let mut differences = BTreeMap::new();
    for (call, expected) in expected["calls"].as_object().unwrap() {
        let start: u32 = call.parse().unwrap();
        let call_span = Span::new(
            file.id,
            Position::new(start),
            Position::new(start + expected["expression"].as_str().unwrap().len() as u32),
        );
        let observed = artifacts
            .call_scope_observations
            .get(&call_span)
            .unwrap_or_else(|| {
                panic!(
                    "missing {call_span:?}; have {:?}",
                    artifacts.call_scope_observations.keys()
                )
            });
        assert_eq!(
            observed.len(),
            expected["names"].as_array().unwrap().len(),
            "{call}"
        );
        for name in expected["names"].as_array().unwrap() {
            let observed = observed
                .iter()
                .find(|o| o.name.as_str() == name["name"].as_str())
                .unwrap();
            assert_eq!(observed.filtered, name["filtered"].as_bool().unwrap());
            assert_eq!(
                observed.types.len(),
                name["types"].as_array().unwrap().len()
            );
            for wanted in name["types"].as_array().unwrap() {
                let span = Span::new(
                    file.id,
                    Position::new(wanted["start"].as_u64().unwrap() as u32),
                    Position::new(wanted["end"].as_u64().unwrap() as u32),
                );
                let actual = observed.types.get(&span).map(describe);
                if actual.as_deref().map(canonical) != wanted["type"].as_str().map(canonical) {
                    differences.insert(
                        format!("{call}/{}:{}", name["name"], wanted["expression"]),
                        (wanted["type"].clone(), actual),
                    );
                }
            }
        }
    }
    assert!(differences.is_empty(), "{differences:#?}");
}

#[test]
fn requested_call_observations_match_the_frozen_phpstan_contract() {
    let (file, artifacts, _) = analyze(
        CODE,
        ORACLE,
        true,
        &PluginRegistry::with_library_providers(),
    );
    check(&file, &artifacts, ORACLE);
}

#[derive(Clone, Default)]
struct Events(Arc<Mutex<BTreeMap<(&'static str, u32, u32), usize>>>);
impl Events {
    fn add(&self, kind: &'static str, span: Span) {
        *self
            .0
            .lock()
            .unwrap()
            .entry((kind, span.start.offset, span.end.offset))
            .or_default() += 1;
    }
}
impl Provider for Events {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("pricing-events", "pricing-events", "real walk events");
        &META
    }
    fn is_observer(&self) -> bool {
        true
    }
}
impl ExpressionHook for Events {
    fn is_observer(&self) -> bool {
        true
    }
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        self.add("expression-before", expression.span());
        Ok(ExpressionHookResult::Continue)
    }
    fn before_arguments(
        &self,
        invocation: &InvocationInfo<'_, '_, '_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("arguments", invocation.span());
        Ok(())
    }
    fn after_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("expression-after", expression.span());
        Ok(())
    }
}
impl FunctionCallHook for Events {
    fn before_function_call(
        &self,
        call: &FunctionCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        self.add("function-before", call.span());
        Ok(ExpressionHookResult::Continue)
    }
    fn after_function_call(
        &self,
        call: &FunctionCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("function-after", call.span());
        Ok(())
    }
}
impl MethodCallHook for Events {
    fn before_method_call(
        &self,
        call: &MethodCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        self.add("method-before", call.span());
        Ok(ExpressionHookResult::Continue)
    }
    fn after_method_call(
        &self,
        call: &MethodCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("method-after", call.span());
        Ok(())
    }
}
impl StaticMethodCallHook for Events {
    fn before_static_method_call(
        &self,
        call: &StaticMethodCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        self.add("static-before", call.span());
        Ok(ExpressionHookResult::Continue)
    }
    fn after_static_method_call(
        &self,
        call: &StaticMethodCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("static-after", call.span());
        Ok(())
    }
}
impl NullSafeMethodCallHook for Events {
    fn before_nullsafe_method_call(
        &self,
        call: &NullSafeMethodCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        self.add("nullsafe-before", call.span());
        Ok(ExpressionHookResult::Continue)
    }
    fn after_nullsafe_method_call(
        &self,
        call: &NullSafeMethodCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("nullsafe-after", call.span());
        Ok(())
    }
}

impl StatementHook for Events {
    fn before_statement(
        &self,
        statement: &mago_syntax::cst::Statement<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<HookAction> {
        self.add("statement-before", statement.span());
        Ok(HookAction::Continue)
    }
    fn after_statement(
        &self,
        statement: &mago_syntax::cst::Statement<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.add("statement-after", statement.span());
        Ok(())
    }
}

macro_rules! declaration_events {
    ($hook:ident, $node:ty, $metadata:ty, $enter:ident, $leave:ident, $label:literal) => {
        impl $hook for Events {
            fn $enter(
                &self,
                node: &$node,
                _: &$metadata,
                _: &mut HookContext<'_, '_>,
            ) -> HookResult<()> {
                self.add(concat!($label, "-enter"), node.span());
                Ok(())
            }
            fn $leave(
                &self,
                node: &$node,
                _: &$metadata,
                _: &mut HookContext<'_, '_>,
            ) -> HookResult<()> {
                self.add(concat!($label, "-leave"), node.span());
                Ok(())
            }
        }
    };
}
declaration_events!(
    ClassDeclarationHook,
    mago_syntax::cst::Class<'_>,
    mago_codex::metadata::class_like::ClassLikeMetadata,
    on_enter_class,
    on_leave_class,
    "class"
);
declaration_events!(
    InterfaceDeclarationHook,
    mago_syntax::cst::Interface<'_>,
    mago_codex::metadata::class_like::ClassLikeMetadata,
    on_enter_interface,
    on_leave_interface,
    "interface"
);
declaration_events!(
    TraitDeclarationHook,
    mago_syntax::cst::Trait<'_>,
    mago_codex::metadata::class_like::ClassLikeMetadata,
    on_enter_trait,
    on_leave_trait,
    "trait"
);
declaration_events!(
    EnumDeclarationHook,
    mago_syntax::cst::Enum<'_>,
    mago_codex::metadata::class_like::ClassLikeMetadata,
    on_enter_enum,
    on_leave_enum,
    "enum"
);
declaration_events!(
    FunctionDeclarationHook,
    mago_syntax::cst::Function<'_>,
    mago_codex::metadata::function_like::FunctionLikeMetadata,
    on_enter_function,
    on_leave_function,
    "function"
);

struct Identity;
impl Provider for Identity {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("pricing-identity", "pricing-identity", "semantic identity");
        &META
    }
}
impl FunctionReturnTypeProvider for Identity {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"pricingFunction")
    }
    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        context
            .get_expression_type(invocation.get_argument(0, &[])?)
            .cloned()
    }
}
impl MethodReturnTypeProvider for Identity {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: &[MethodTarget] = &[
            MethodTarget {
                class: b"PricingWorker",
                method: b"fromProvider",
            },
            MethodTarget {
                class: b"PricingWorker",
                method: b"staticProvider",
            },
        ];
        TARGETS
    }
    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        _: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        context
            .get_expression_type(invocation.get_argument(0, &[])?)
            .cloned()
    }
}
fn identity(call: &impl HasSpan, args: &ArgumentList<'_>, context: &mut HookContext<'_, '_>) {
    let value = context
        .get_expression_type(args.arguments.first().unwrap().value())
        .unwrap()
        .clone();
    context.set_expression_type(call, value);
}
impl FunctionCallHook for Identity {
    fn after_function_call(
        &self,
        call: &FunctionCall<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(call.function, Expression::Identifier(id) if id.value() == b"pricingHook") {
            identity(call, &call.argument_list, context);
        }
        Ok(())
    }
}
impl ExpressionHook for Identity {
    fn after_expression(
        &self,
        expression: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if let Expression::Call(Call::Function(call)) = expression
            && matches!(call.function, Expression::Identifier(id) if id.value() == b"pricingExpression")
        {
            identity(call, &call.argument_list, context);
        }
        Ok(())
    }
}
impl MethodCallHook for Identity {
    fn after_method_call(
        &self,
        call: &MethodCall<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value == b"fromHook")
        {
            identity(call, &call.argument_list, context);
        }
        Ok(())
    }
}
impl StaticMethodCallHook for Identity {
    fn after_static_method_call(
        &self,
        call: &StaticMethodCall<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value == b"staticHook")
        {
            identity(call, &call.argument_list, context);
        }
        Ok(())
    }
}
impl NullSafeMethodCallHook for Identity {
    fn after_nullsafe_method_call(
        &self,
        call: &NullSafeMethodCall<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value == b"fromHook")
        {
            identity(call, &call.argument_list, context);
        }
        Ok(())
    }
}

fn providers(events: &Events) -> PluginRegistry {
    let mut registry = PluginRegistry::with_library_providers();
    registry.register_expression_hook(events.clone());
    registry.register_function_call_hook(events.clone());
    registry.register_method_call_hook(events.clone());
    registry.register_static_method_call_hook(events.clone());
    registry.register_nullsafe_method_call_hook(events.clone());
    registry.register_statement_hook(events.clone());
    registry.register_class_hook(events.clone());
    registry.register_interface_hook(events.clone());
    registry.register_trait_hook(events.clone());
    registry.register_enum_hook(events.clone());
    registry.register_function_decl_hook(events.clone());
    registry.register_function_provider(Identity);
    registry.register_method_provider(Identity);
    registry.register_expression_hook(Identity);
    registry.register_function_call_hook(Identity);
    registry.register_method_call_hook(Identity);
    registry.register_static_method_call_hook(Identity);
    registry.register_nullsafe_method_call_hook(Identity);
    registry
}

#[test]
fn pricing_keeps_semantic_providers_without_publishing_another_walk() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/providers.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/providers-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    let (file, observed, observed_result) =
        analyze(CODE, ORACLE, true, &providers(&observed_events));
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap(),
        "pricing must not publish expression, invocation, constructor or call events"
    );
    assert_eq!(
        baseline.expression_types, observed.expression_types,
        "no type writes escape pricing"
    );
    assert_eq!(
        baseline_result.issues, observed_result.issues,
        "no diagnostics escape pricing"
    );
    assert!(baseline.call_scope_observations.is_empty());
    check(&file, &observed, ORACLE);
}

#[test]
fn pricing_closures_and_anonymous_constructors_keeps_declaration_events_isolated() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/isolation.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/isolation-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    let (file, observed, observed_result) =
        analyze(CODE, ORACLE, true, &providers(&observed_events));
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap(),
        "pricing must not publish closure-body statements or declarations"
    );
    for kind in [
        "statement-before",
        "statement-after",
        "class-enter",
        "class-leave",
        "interface-enter",
        "interface-leave",
        "trait-enter",
        "trait-leave",
        "enum-enter",
        "enum-leave",
        "function-enter",
        "function-leave",
    ] {
        assert!(
            baseline_events
                .0
                .lock()
                .unwrap()
                .keys()
                .any(|(actual, _, _)| *actual == kind),
            "fixture did not exercise {kind}"
        );
    }
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn nested_requests_keep_their_own_predicates_for_shared_dependency_spans() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/nested.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/nested-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    let (file, observed, observed_result) =
        analyze(CODE, ORACLE, true, &providers(&observed_events));
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn closure_results_follow_resolved_argument_context_not_captured_variable_changes() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/argument-context.php");
    const ORACLE: &str =
        include_str!("fixtures/call-scope-observations/argument-context-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    let (file, observed, observed_result) =
        analyze(CODE, ORACLE, true, &providers(&observed_events));
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn absence_and_possible_variables_follow_the_expression_position() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/certainty.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/certainty-expected.json");
    let (file, artifacts, _) = analyze(
        CODE,
        ORACLE,
        true,
        &PluginRegistry::with_library_providers(),
    );
    check(&file, &artifacts, ORACLE);
}

#[test]
fn resolved_closure_repricing_keeps_raw_gathers_and_declarations_isolated() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/closure-declarations.php");
    const ORACLE: &str =
        include_str!("fixtures/call-scope-observations/closure-declarations-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    let (file, observed, observed_result) =
        analyze(CODE, ORACLE, true, &providers(&observed_events));
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn variadic_unpack_and_unknown_or_unset_dependencies_keep_their_positions() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/boundaries.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/boundaries-expected.json");
    let (file, artifacts, _) = analyze(
        CODE,
        ORACLE,
        true,
        &PluginRegistry::with_library_providers(),
    );
    check(&file, &artifacts, ORACLE);
}

#[test]
fn exhausted_formula_budget_keeps_independent_observations_and_isolation() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/budget.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/budget-expected.json");
    let run = |observe, events: &Events| {
        analyze_with_settings(
            CODE,
            ORACLE,
            observe,
            &providers(events),
            Settings {
                formula_size_threshold: 0,
                ..Settings::default()
            },
        )
    };
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = run(false, &baseline_events);
    let observed_events = Events::default();
    let (file, observed, observed_result) = run(true, &observed_events);
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

struct SuppliedReceiver;
impl ExpressionHook for SuppliedReceiver {
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        if matches!(expression, Expression::Call(Call::Function(call))
            if matches!(call.function, Expression::Identifier(id) if id.value() == b"boundaryReceiver"))
        {
            return Ok(ExpressionHookResult::SkipWithType(
                mago_codex::ttype::get_named_object(mago_word::word("BoundarySink"), None),
            ));
        }
        Ok(ExpressionHookResult::Continue)
    }
}

#[test]
fn semantic_receiver_short_circuit_retains_selector_scope_without_new_events() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/boundaries.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/boundaries-expected.json");
    let run = |observe, events: &Events| {
        let mut registry = providers(events);
        registry.register_expression_hook(SuppliedReceiver);
        analyze(CODE, ORACLE, observe, &registry)
    };
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = run(false, &baseline_events);
    let observed_events = Events::default();
    let (file, observed, observed_result) = run(true, &observed_events);
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn recovered_missing_selector_has_no_named_answers_or_extra_events() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/recovered-call.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/call-scope-observations/recovered-call-expected.json"
    ))
    .unwrap();
    let mut runs = Vec::new();
    for observe in [false, true] {
        let arena = LocalArena::new();
        let file = File::ephemeral(Cow::Borrowed(b"recovery.php"), Cow::Borrowed(CODE));
        let program = parse_file(&arena, &file);
        assert_eq!(
            program.has_errors(),
            !oracle["errors"].as_array().unwrap().is_empty()
        );
        let names = NameResolver::new(&arena).resolve(program);
        let mut settings = Settings::default();
        if observe {
            let text = b"$sink->()";
            let start = CODE.windows(text.len()).position(|s| s == text).unwrap() as u32;
            settings.call_scope_observations.push(CallScopeRequest {
                call: Span::new(
                    file.id,
                    Position::new(start),
                    Position::new(start + text.len() as u32),
                ),
                expressions: vec![Span::new(
                    file.id,
                    Position::new(start),
                    Position::new(start + 5),
                )],
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
        let mut result = AnalysisResult::new(references);
        let events = Events::default();
        let registry = providers(&events);
        let artifacts = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result)
            .unwrap();
        assert_eq!(
            artifacts.call_scope_observations.len(),
            oracle["calls"].as_array().unwrap().len()
        );
        runs.push((
            events.0.lock().unwrap().clone(),
            artifacts.expression_types,
            result.issues,
        ));
    }
    assert_eq!(runs[0], runs[1]);
}

struct UntypedUnpack;
impl ExpressionHook for UntypedUnpack {
    fn before_expression(
        &self,
        expression: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        Ok(
            if matches!(expression, Expression::Call(Call::Function(call))
            if matches!(call.function, Expression::Identifier(id) if id.value() == b"boundarySkipped"))
            {
                ExpressionHookResult::Skip
            } else {
                ExpressionHookResult::Continue
            },
        )
    }
}

#[test]
fn nonvariadic_unpack_is_evaluated_once_even_when_a_semantic_hook_has_no_type() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/boundaries.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/boundaries-expected.json");
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let events = Events::default();
    let mut registry = providers(&events);
    registry.register_expression_hook(UntypedUnpack);
    let (_, _, _) = analyze(CODE, ORACLE, false, &registry);
    let text = b"boundarySkipped()";
    let start = CODE.windows(text.len()).rposition(|s| s == text).unwrap() as u32;
    assert_eq!(
        events.0.lock().unwrap()[&("expression-before", start, start + text.len() as u32)],
        oracle["function_events"][start.to_string()]
            .as_u64()
            .unwrap() as usize,
    );
}

fn check_unpack_order(code: &'static [u8], oracle: &str) {
    let events = Events::default();
    let (_, artifacts, _) = analyze(code, oracle, false, &providers(&events));
    let oracle: Value = serde_json::from_str(oracle).unwrap();
    let events = events.0.lock().unwrap();
    let mut differences = BTreeMap::new();
    for (start, expected) in oracle["arguments"].as_object().unwrap() {
        let start = start.parse::<u32>().unwrap();
        let end = expected["end"].as_u64().unwrap() as u32;
        let actual = artifacts.expression_types.get(&(start, end)).map(|ty| {
            if ty
                .get_single_array()
                .is_some_and(|array| array.is_vanilla())
            {
                "array".to_owned()
            } else {
                describe(ty)
            }
        });
        let before = events
            .get(&("expression-before", start, end))
            .copied()
            .unwrap_or(0);
        let after = events
            .get(&("expression-after", start, end))
            .copied()
            .unwrap_or(0);
        let visits = expected["visits"].as_u64().unwrap() as usize;
        if actual.as_deref() != expected["type"].as_str() || before != visits || after != visits {
            differences.insert(start, (expected.clone(), actual, before, after));
        }
    }
    assert!(differences.is_empty(), "{differences:#?}");
}

#[test]
fn unpacked_boundary_operands_keep_earlier_and_later_writes_and_impurity_in_order() {
    check_unpack_order(
        include_bytes!("fixtures/call-scope-observations/unpack-boundaries.php"),
        include_str!("fixtures/call-scope-observations/unpack-boundaries-expected.json"),
    );
}

#[test]
fn unpacked_variadic_generic_reference_and_empty_callees_evaluate_each_operand_once() {
    check_unpack_order(
        include_bytes!("fixtures/call-scope-observations/unpack-order.php"),
        include_str!("fixtures/call-scope-observations/unpack-order-expected.json"),
    );
}

#[test]
fn variadic_and_empty_callees_do_not_replay_untyped_unpack_expressions() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/unpack-order.php");
    const ORACLE: &str =
        include_str!("fixtures/call-scope-observations/unpack-order-expected.json");
    let events = Events::default();
    let mut registry = providers(&events);
    registry.register_expression_hook(UntypedUnpack);
    let (_, artifacts, _) = analyze(CODE, ORACLE, false, &registry);
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    let events = events.0.lock().unwrap();
    let mut checked = 0;
    for (start, argument) in oracle["arguments"].as_object().unwrap() {
        if argument["expression"] != "boundarySkipped()" {
            continue;
        }
        let start = start.parse::<u32>().unwrap();
        let end = argument["end"].as_u64().unwrap() as u32;
        assert_eq!(events.get(&("expression-before", start, end)), Some(&1));
        assert!(!artifacts.expression_types.contains_key(&(start, end)));
        checked += 1;
    }
    assert_eq!(checked, 4);
}

#[test]
fn unpack_operand_errors_propagate_before_later_ordinary_operands() {
    use mago_analyzer::error::AnalysisError;

    struct FailAt(u32, u32);
    impl ExpressionHook for FailAt {
        fn before_expression(
            &self,
            expression: &Expression<'_>,
            _: &mut HookContext<'_, '_>,
        ) -> HookResult<ExpressionHookResult> {
            let span = expression.span();
            if (span.start.offset, span.end.offset) == (self.0, self.1) {
                Err(PluginError::Internal {
                    reason: "unpacked operand failed".into(),
                })
            } else {
                Ok(ExpressionHookResult::Continue)
            }
        }
    }

    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/unpack-order.php");
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/call-scope-observations/unpack-order-expected.json"
    ))
    .unwrap();
    let (start, expected) = oracle["arguments"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(start, expected)| (start.parse::<u32>().unwrap(), expected))
        .min_by_key(|(start, _)| *start)
        .unwrap();
    let end = expected["end"].as_u64().unwrap() as u32;
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"unpack-error.php"), Cow::Borrowed(CODE));
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
    let events = Events::default();
    let mut registry = providers(&events);
    registry.register_expression_hook(FailAt(start, end));
    let mut result = AnalysisResult::new(references);
    assert!(matches!(
        Analyzer::new(&arena, &file, &names, &codebase, &registry, settings)
            .analyze_with_artifacts(program, &mut result),
        Err(AnalysisError::Plugin(PluginError::Internal { reason })) if reason == "unpacked operand failed"
    ));
    let later = b"$arguments = ['right' => 1]";
    let later_start = CODE.windows(later.len()).position(|s| s == later).unwrap() as u32;
    assert!(later_start > end);
    let events = events.0.lock().unwrap();
    assert_eq!(events.get(&("expression-before", start, end)), Some(&1));
    assert!(!events.contains_key(&(
        "expression-before",
        later_start,
        later_start + later.len() as u32,
    )));
}

#[test]
fn nested_shared_scope_values_merge_without_erasing_unrelated_fields() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/nested-shared-joins.php");
    const ORACLE: &str =
        include_str!("fixtures/call-scope-observations/nested-shared-joins-expected.json");
    let registry = PluginRegistry::with_library_providers();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &registry);
    let (file, observed, observed_result) = analyze(CODE, ORACLE, true, &registry);
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn constant_method_names_follow_union_vetoes_and_template_bounds() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/selector-names.php");
    const ORACLE: &str =
        include_str!("fixtures/call-scope-observations/selector-names-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    let (file, observed, observed_result) =
        analyze(CODE, ORACLE, true, &providers(&observed_events));
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}

#[test]
fn historical_dependencies_keep_own_positions_and_identifier_asking_scope() {
    const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/historical.php");
    const ORACLE: &str = include_str!("fixtures/call-scope-observations/historical-expected.json");
    let baseline_events = Events::default();
    let (_, baseline, baseline_result) = analyze(CODE, ORACLE, false, &providers(&baseline_events));
    let observed_events = Events::default();
    // Request a real but not-yet-evaluated argument too. The original strict
    // oracle must still match: an AST index is not permission to invent answers.
    let mut requests: Value = serde_json::from_str(ORACLE).unwrap();
    let calls = requests["calls"].as_object_mut().unwrap();
    let later = calls.values().max_by_key(|c| c["line"].as_u64()).unwrap();
    let future = later["names"][0]["types"][1].clone();
    let earlier = calls
        .values_mut()
        .min_by_key(|c| c["line"].as_u64())
        .unwrap();
    earlier["names"][0]["types"]
        .as_array_mut()
        .unwrap()
        .push(future);
    let (file, observed, observed_result) = analyze(
        CODE,
        &requests.to_string(),
        true,
        &providers(&observed_events),
    );
    assert_eq!(
        *baseline_events.0.lock().unwrap(),
        *observed_events.0.lock().unwrap()
    );
    assert_eq!(baseline.expression_types, observed.expression_types);
    assert_eq!(baseline_result.issues, observed_result.issues);
    check(&file, &observed, ORACLE);
}
