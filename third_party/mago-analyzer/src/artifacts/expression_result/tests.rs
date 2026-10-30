use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::{TType, get_int, get_mixed, get_null, get_string, get_void};
use mago_database::file::{File, FileId};
use mago_names::resolver::NameResolver;
use mago_span::{Position, Span};
use mago_syntax::parser::parse_file;

use crate::Analyzer;
use crate::analysis_result::AnalysisResult;
use crate::artifacts::CheckpointAction;
use crate::plugin::*;
use crate::settings::Settings;

use super::*;

fn id(ty: &TUnion) -> String {
    ty.get_id().to_string()
}
fn span(start: u32, end: u32) -> Span {
    Span::new(FileId::zero(), Position::new(start), Position::new(end))
}

#[derive(Default)]
struct Reads {
    // Source ranges, not expression identity: two written calls must both evaluate.
    values: BTreeMap<(u32, u32), (String, String, String)>,
    calls: Vec<String>,
}

struct Observer(Arc<Mutex<Reads>>);
impl Provider for Observer {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new("result-observer", "result-observer", "result boundary");
        &META
    }
}
impl ExpressionHook for Observer {
    fn is_observer(&self) -> bool {
        true
    }
    fn after_expression(&self, expr: &Expression<'_>, context: &mut HookContext<'_, '_>) -> HookResult<()> {
        if let Some(result) = context.get_expression_result(expr) {
            assert_eq!(context.get_raw_expression_type(expr), Some(result.get_raw_type()));
            assert_eq!(context.get_keep_void_expression_type(expr), Some(result.get_keep_void_type()));
            let range = get_expression_range(expr);
            self.0
                .lock()
                .unwrap()
                .values
                .insert(range, (id(result.get_type()), id(result.get_raw_type()), id(result.get_keep_void_type())));
        }
        Ok(())
    }
    fn before_arguments(&self, invocation: &InvocationInfo<'_, '_, '_>, _: &mut HookContext<'_, '_>) -> HookResult<()> {
        self.0.lock().unwrap().calls.push(invocation.function_name());
        Ok(())
    }
}

struct Overrides;
impl Provider for Overrides {
    fn meta() -> &'static ProviderMeta {
        Observer::meta()
    }
}
impl ExpressionHook for Overrides {
    fn before_arguments(
        &self,
        invocation: &InvocationInfo<'_, '_, '_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        if invocation.function_name() == "ResultHolder::foo" {
            // Model the source-owned nullsafe ensure at the argument boundary, without its WIP.
            let receiver = context.get_variable_type(b"$sink").unwrap().as_ref().clone();
            context.set_variable_type(b"$sink->read()", receiver);
        }
        Ok(())
    }
    fn after_expression(&self, expr: &Expression<'_>, context: &mut HookContext<'_, '_>) -> HookResult<()> {
        if let Expression::Call(Call::Function(call)) = expr
            && let Expression::Identifier(name) = call.function
        {
            match name.value() {
                b"resultEager" => context.set_expression_type(expr, get_void()),
                b"resultOverride" => context.set_expression_type(expr, get_int()),
                _ => {}
            }
        }
        Ok(())
    }
}
impl FunctionReturnTypeProvider for Overrides {
    fn targets() -> FunctionTarget {
        FunctionTarget::exact(b"resultprovider")
    }
    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let argument = invocation.get_argument(0, &[]).unwrap();
        assert_eq!(context.get_expression_type(argument), Some(&get_null()));
        assert_eq!(context.get_raw_expression_type(argument), Some(&get_void()));
        assert_eq!(context.get_keep_void_expression_type(argument), Some(&get_void()));
        assert_eq!(context.get_expression_result(argument).unwrap().get_raw_type(), &get_void());
        Some(get_void())
    }
}
impl MethodReturnTypeProvider for Overrides {
    fn targets() -> &'static [MethodTarget] {
        const TARGETS: &[MethodTarget] = &[MethodTarget::exact(b"ResultSink", b"fromProvider")];
        TARGETS
    }
    fn get_return_type(
        &self,
        _: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        _: &[u8],
        _: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        Some(get_void())
    }
}

fn analyze(code: &[u8], native: bool, mut registry: PluginRegistry) -> (AnalysisArtifacts, Reads) {
    let reads = Arc::<Mutex<Reads>>::default();
    registry.register_expression_hook(Observer(Arc::clone(&reads)));
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"result.php"), Cow::Owned(code.to_vec()));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let names = NameResolver::new(&arena).resolve(program);
    let mut settings = Settings::default();
    settings.track_native_types = native;
    let mut codebase = scan_program(&arena, &file, program, &names, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(&mut codebase, &mut references, Default::default(), Default::default());
    let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &registry, settings);
    let artifacts = analyzer.analyze_with_artifacts(program, &mut AnalysisResult::new(references)).unwrap();
    let reads = std::mem::take(&mut *reads.lock().unwrap());
    (artifacts, reads)
}

#[test]
fn public_results_and_hook_provider_reads_match_fresh_phpstan_capture() {
    let code = include_str!("../../../tests/expression-result/matrix.php");
    for native in [false, true] {
        let mut registry = PluginRegistry::new();
        registry.register_expression_hook(Overrides);
        registry.register_function_provider(Overrides);
        registry.register_method_provider(Overrides);
        let (artifacts, reads) = analyze(code.as_bytes(), native, registry);
        for row in include_str!("../../../tests/expression-result/expected.tsv").lines() {
            let fields: Vec<_> = row.split('\t').collect();
            let line: usize = fields[0].parse().unwrap();
            let range = (fields[8].parse().unwrap(), fields[9].parse().unwrap());
            let expr = span(range.0, range.1);
            let result = artifacts.get_expression_result(&expr).unwrap();
            let adapt = |text: &str| text.replace("array{null}", "list{null}");
            let expected = (adapt(fields[2]), adapt(fields[3]), adapt(fields[4]));
            assert_eq!(
                (id(result.get_type()), id(result.get_raw_type()), id(result.get_keep_void_type())),
                expected,
                "line {line}"
            );
            assert_eq!(reads.values.get(&range), Some(&expected), "public hook line {line}");
            if fields[1] == "resultEager()" || fields[1] == "resultOverride()" {
                assert!(result.has_value_override());
                assert!(!result.is_eager(), "resolver override is not the own eager return");
                let mut copied = AnalysisArtifacts::new();
                copied.set_expression_result(&expr, result.clone());
                let restored = copied.get_expression_result(&expr).unwrap();
                assert!(restored.has_value_override());
                assert!(!restored.is_eager());
                assert_eq!(restored.get_keep_void_type(), result.get_keep_void_type());
            }
            if !native {
                assert!(result.get_native_type().is_none());
                assert!(result.get_raw_native_type().is_none());
            }
            for (actual, expected) in [
                (result.get_native_type(), fields[5]),
                (result.get_raw_native_type(), fields[6]),
                (result.get_keep_void_native_type(), fields[7]),
            ] {
                if let Some(actual) = actual {
                    assert_eq!(id(actual), adapt(expected), "native line {line}");
                }
            }
            // The partial native lane never substitutes the full null value for declared void.
            if line == 35 && native {
                assert_eq!(result.get_native_type().map(id), Some(fields[5].into()));
            }
        }
        let start = code.find("(resultVoid())").unwrap() as u32;
        let parenthesized = artifacts.get_expression_result(&span(start, start + 14)).unwrap();
        // PHPStan elides parentheses in its AST; they retain the named call's result.
        assert_eq!(parenthesized.get_type(), &get_null());
        assert_eq!(parenthesized.get_raw_type(), &get_void());
        assert_eq!(parenthesized.get_keep_void_type(), &get_void());
        assert_eq!(
            reads.calls.iter().filter(|name| *name == "ResultHolder::read").count(),
            2,
            "a holder must not skip evaluation"
        );
    }
}

#[test]
fn all_void_match_arms_expose_keep_void_for_the_separately_owned_match_consumer() {
    let code = include_bytes!("../../../tests/expression-result/match.php");
    let (artifacts, reads) = analyze(code, false, PluginRegistry::new());
    let mut arms = 0;
    for row in include_str!("../../../tests/expression-result/match.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        if fields[1] != "resultVoid()" {
            // The match handler must union arm keep-void types before these other rows
            // can match PHPStan. That consumer is deliberately outside this API slice.
            continue;
        }
        let range = (fields[8].parse().unwrap(), fields[9].parse().unwrap());
        let result = artifacts.get_expression_result(&span(range.0, range.1)).unwrap();
        let expected = (fields[2].to_owned(), fields[3].to_owned(), fields[4].to_owned());
        assert_eq!((id(result.get_type()), id(result.get_raw_type()), id(result.get_keep_void_type())), expected);
        assert_eq!(reads.values.get(&range), Some(&expected));
        arms += 1;
    }
    assert_eq!(arms, 2);
}

#[test]
fn independent_native_void_is_not_projected_or_fabricated() {
    let code = b"<?php function resultVoid(): void {} function sample(): void { resultVoid(); }";
    let (artifacts, _) = analyze(code, true, PluginRegistry::new());
    let start = std::str::from_utf8(code).unwrap().rfind("resultVoid()").unwrap() as u32;
    let expr = span(start, start + 12);
    let result = artifacts.get_expression_result(&expr).unwrap();
    assert_eq!(result.get_type(), &get_null());
    assert_eq!(result.get_raw_native_type(), Some(&get_void()));
    assert_eq!(result.get_keep_void_native_type(), Some(&get_void()));
    let mut copied = AnalysisArtifacts::new().with_native_types(true);
    copied.set_expression_result(&expr, result);
    assert_eq!(copied.get_native_expression_type(&expr), Some(&get_void()));
    let (unavailable, _) = analyze(code, false, PluginRegistry::new());
    copied.set_expression_result(&expr, unavailable.get_expression_result(&expr).unwrap());
    let result = copied.get_expression_result(&expr).unwrap();
    assert!(result.get_native_type().is_none());
    assert!(result.get_raw_native_type().is_none());
    assert!(result.get_keep_void_native_type().is_none());
}

#[test]
fn public_method_native_return_is_independent_of_the_full_value() {
    let code = b"<?php class Sink { function stop(): void {} } function sample(Sink $sink): void { $sink->stop(); }";
    let (artifacts, _) = analyze(code, true, PluginRegistry::new());
    let start = std::str::from_utf8(code).unwrap().rfind("$sink->stop()").unwrap() as u32;
    let result = artifacts.get_expression_result(&span(start, start + 13)).unwrap();
    assert_eq!(result.get_type(), &get_null());
    assert_eq!(result.get_raw_type(), &get_void());
    assert_eq!(result.get_native_type(), Some(&get_void()));
    assert_eq!(result.get_raw_native_type(), Some(&get_void()));
}

struct Eager;
impl Provider for Eager {
    fn meta() -> &'static ProviderMeta {
        Observer::meta()
    }
}
impl ExpressionHook for Eager {
    fn before_expression(
        &self,
        expr: &Expression<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        Ok(if matches!(expr, Expression::Call(_)) {
            ExpressionHookResult::SkipWithType(get_void())
        } else {
            ExpressionHookResult::Continue
        })
    }
}

#[test]
fn public_eager_hook_return_does_not_get_lazy_projection() {
    let code = b"<?php function resultVoid(): void {} resultVoid();";
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(Eager);
    let (artifacts, reads) = analyze(code, true, registry);
    let start = std::str::from_utf8(code).unwrap().rfind("resultVoid()").unwrap() as u32;
    let result = artifacts.get_expression_result(&span(start, start + 12)).unwrap();
    assert!(result.is_eager());
    assert_eq!(result.get_type(), &get_void());
    assert_eq!(result.get_keep_void_type(), &get_void());
    assert!(result.get_native_type().is_none());
    assert!(reads.calls.is_empty(), "only the eager hook elects to skip the call");
}

#[test]
fn result_transport_checkpoints_and_repeated_evaluation_preserve_raw_state() {
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"lifecycle.php"), Cow::Borrowed(b"<?php resultVoid();"));
    let program = parse_file(&arena, &file);
    let mago_syntax::cst::Statement::Expression(statement) = &program.statements.as_slice()[1] else {
        panic!("expression")
    };
    let expr = statement.expression;
    let range = get_expression_range(expr);
    let mut artifacts = AnalysisArtifacts::new().with_native_types(true);
    assert!(artifacts.get_keep_void_expression_type(expr).is_none());
    artifacts.begin_expression_result(expr, Some(Rc::new(get_int())), Some(Rc::new(get_string())));
    artifacts.set_resolved_expression_type(expr, get_void());
    artifacts.set_resolved_native_expression_type(expr, Rc::new(get_void()));
    let original = artifacts.get_expression_result(expr).unwrap();
    assert_eq!(original.get_type(), &get_int());
    assert_eq!(original.get_raw_type(), &get_void());
    assert_eq!(original.get_native_type(), Some(&get_string()));
    assert_eq!(original.get_raw_native_type(), Some(&get_void()));
    assert_eq!(original.get_keep_void_native_type(), Some(&get_void()));
    artifacts.begin_expression_type_checkpoint();
    artifacts.set_resolved_native_expression_type(expr, Rc::new(get_int()));
    assert!(artifacts.expression_type_checkpoint_matches(|a, b| a == b), "native-only changes are passive");
    artifacts.begin_expression_result(expr, None, None);
    artifacts.set_resolved_expression_type(expr, get_int());
    assert!(!artifacts.expression_type_checkpoint_matches(|a, b| a == b), "same value but different own raw");
    artifacts.finish_expression_type_checkpoint(CheckpointAction::Rollback);
    assert_eq!(artifacts.get_raw_expression_type(expr), Some(&get_void()));
    assert_eq!(artifacts.get_native_expression_type(expr), Some(&get_string()));
    artifacts.begin_expression_type_checkpoint();
    artifacts.remove_expression_type(&range);
    artifacts.finish_expression_type_checkpoint(CheckpointAction::Merge);
    assert_eq!(artifacts.get_keep_void_expression_type(expr), Some(&get_void()));
    artifacts.begin_expression_type_checkpoint();
    let detached = artifacts.detach_expression_type_checkpoints().unwrap();
    artifacts.set_expression_type(expr, get_null());
    artifacts.restore_expression_type_checkpoints(detached);
    artifacts.finish_expression_type_checkpoint(CheckpointAction::Rollback);
    assert_eq!(artifacts.get_expression_type(expr), Some(&get_int()));
    assert_eq!(artifacts.get_raw_expression_type(expr), Some(&get_void()));
    for detached in [false, true] {
        for replaced in [false, true] {
            let mut merged = AnalysisArtifacts::new().with_native_types(true);
            merged.set_expression_result(expr, original.clone());
            merged.set_call_termination(expr, true);
            merged.begin_expression_type_checkpoint();
            let checkpoints = detached.then(|| merged.detach_expression_type_checkpoints().unwrap());
            merged.remove_expression_type(&range);
            if replaced {
                merged.set_expression_type(expr, get_string());
            }
            if let Some(checkpoints) = checkpoints {
                merged.restore_expression_type_checkpoints(checkpoints);
            }
            merged.finish_expression_type_checkpoint(CheckpointAction::Merge);
            if replaced {
                assert_eq!(
                    merged.get_raw_expression_type(expr),
                    Some(&get_string()),
                    "do not resurrect a replaced result's raw lane"
                );
                assert!(merged.get_native_expression_type(expr).is_none());
                assert!(!merged.call_terminations.contains_key(&range));
            } else {
                assert_eq!(merged.get_keep_void_expression_type(expr), Some(&get_void()));
                assert_eq!(merged.get_native_expression_type(expr), Some(&get_string()));
                assert_eq!(merged.call_terminations.get(&range), Some(&true));
            }
        }
    }
    artifacts.begin_expression_type_checkpoint();
    artifacts.remove_native_expression_type(&range);
    artifacts.finish_expression_type_checkpoint(CheckpointAction::Merge);
    assert_eq!(
        artifacts.get_native_expression_type(expr),
        Some(&get_string()),
        "native-only removal can merge back when the full result was not replaced"
    );
    for native in [false, true] {
        let mut copied = AnalysisArtifacts::new().with_native_types(native);
        copied.set_expression_result(expr, original.clone());
        assert_eq!(copied.get_raw_expression_type(expr), Some(&get_void()));
        assert_eq!(copied.get_native_expression_type(expr), Some(&get_string()));
        assert_eq!(copied.get_expression_result(expr).unwrap().get_raw_native_type(), Some(&get_void()));
        copied.begin_expression_result(expr, None, None);
        copied.set_expression_type(expr, get_void()); // new eager evaluation; no previous holder/raw/native
        let eager = copied.get_expression_result(expr).unwrap();
        assert!(eager.is_eager());
        assert!(!eager.has_value_override());
        assert_eq!(eager.get_type(), &get_void());
        assert_eq!(eager.get_keep_void_type(), &get_void());
        assert_eq!(eager.get_type_on_scope(Some(&get_int()), None).as_ref(), &get_null());
        copied.set_expression_type(expr, get_int());
        let eager_override = copied.get_expression_result(expr).unwrap();
        assert!(eager_override.is_eager());
        assert!(eager_override.has_value_override());
        assert_eq!(eager_override.get_type(), &get_int());
        assert_eq!(eager_override.get_keep_void_type(), &get_void());
        copied.set_expression_result(expr, eager_override);
        assert_eq!(copied.get_raw_expression_type(expr), Some(&get_void()));
    }
    for detached in [false, true] {
        let mut copied = AnalysisArtifacts::new();
        copied.begin_expression_type_checkpoint();
        copied.begin_expression_type_checkpoint();
        let checkpoints = detached.then(|| copied.detach_expression_type_checkpoints().unwrap());
        copied.set_expression_result(expr, original.clone());
        if let Some(checkpoints) = checkpoints {
            copied.restore_expression_type_checkpoints(checkpoints);
        }
        copied.finish_expression_type_checkpoint(CheckpointAction::Keep);
        assert_eq!(copied.get_native_expression_type(expr), Some(&get_string()));
        copied.finish_expression_type_checkpoint(CheckpointAction::Rollback);
        assert!(copied.get_expression_result(expr).is_none());
        assert!(copied.get_native_expression_type(expr).is_none());
    }
    artifacts.remove_native_expression_type(&range);
    let partial = artifacts.get_expression_result(expr).unwrap();
    assert_eq!(partial.get_raw_native_type(), Some(&get_void()));
    assert!(partial.get_native_type().is_none());
    let mut copied = AnalysisArtifacts::new();
    copied.set_expression_result(expr, partial);
    let partial = copied.get_expression_result(expr).unwrap();
    assert_eq!(partial.get_raw_native_type(), Some(&get_void()));
    assert!(partial.get_native_type().is_none());
    for native in [false, true] {
        let mut extended = AnalysisArtifacts::new().with_native_types(native);
        extended.set_expression_result(expr, original.clone());
        extended.extend_expression_types([(range, Rc::new(get_null()))]);
        assert_eq!(extended.get_raw_expression_type(expr), Some(&get_null()));
    }
    let mut parent = AnalysisArtifacts::new();
    parent.extend_expression_types(std::mem::take(&mut artifacts.expression_types));
    parent.transfer_expression_results(&mut artifacts);
    assert_eq!(parent.get_expression_type(expr), Some(&get_int()));
    assert_eq!(parent.get_keep_void_expression_type(expr), Some(&get_void()));
}

#[test]
fn read_precedence_projection_metadata_and_foreign_ask_contract() {
    let mut artifacts = AnalysisArtifacts::new().with_native_types(true);
    let expr = span(1, 10);
    // No syntax means no lazy call projection; a generic producer cannot invent eligibility.
    artifacts.set_resolved_expression_type(&expr, get_void());
    assert_eq!(artifacts.get_expression_type(&expr), Some(&get_void()));
    let result = artifacts.get_expression_result(&expr).unwrap();
    assert_eq!(result.get_native_type_on_scope(None, None), None);
    assert_eq!(result.get_native_type_on_scope(Some(&get_void()), None).as_deref(), Some(&get_void()));
    assert_eq!(result.get_native_type_on_scope(Some(&get_void()), Some(&get_int())).as_deref(), Some(&get_int()));
    artifacts.set_expression_type(&expr, get_int());
    assert_eq!(artifacts.get_keep_void_expression_type(&expr), Some(&get_void()));
    artifacts.set_resolved_expression_type(&expr, get_string());
    artifacts.set_expression_type(&expr, get_int());
    let result = artifacts.get_expression_result(&expr).unwrap();
    assert_eq!(result.get_keep_void_type(), &get_int());
    assert_eq!(artifacts.get_keep_void_expression_type(&expr), Some(&get_int()));
    assert_eq!(result.get_type_on_scope(Some(&get_void()), Some(&get_int())).as_ref(), &get_int());
    assert_eq!(result.get_type_on_scope(Some(&get_void()), None).as_ref(), &get_void());
    assert_eq!(result.get_type_on_scope(None, None).as_ref(), &get_string());
    assert_eq!(result.at_ask_position(None, None, None).get_type(), &get_string());
    let eager = result.at_ask_position(Some((&get_void(), Some(&get_int()))), None, None);
    assert_eq!(eager.get_type(), &get_void());
    assert_eq!(eager.get_raw_native_type(), Some(&get_int()));
    assert_eq!(eager.get_keep_void_native_type(), Some(&get_int()));
    assert_eq!(eager.get_native_type_on_scope(Some(&get_void()), None).as_deref(), Some(&get_int()));
    assert_eq!(eager.at_ask_position(Some((&get_int(), None)), None, None).get_type(), &get_void());
    let eager_missing_native = result.at_ask_position(Some((&get_int(), None)), None, None);
    assert!(eager_missing_native.get_native_type().is_none());
    let native_override = eager_missing_native.at_ask_position(None, None, Some(&get_void()));
    let mut copied = AnalysisArtifacts::new();
    copied.set_expression_result(&expr, native_override);
    let native_override = copied.get_expression_result(&expr).unwrap();
    assert_eq!(native_override.get_native_type(), Some(&get_void()));
    assert!(native_override.get_raw_native_type().is_none(), "a native override cannot invent an own native return");
    assert!(native_override.get_keep_void_native_type().is_none());
    let overridden = eager.at_ask_position(None, Some(&get_int()), Some(&get_string()));
    assert!(overridden.has_value_override());
    assert_eq!(overridden.get_type(), &get_int());
    assert_eq!(overridden.get_raw_type(), &get_void());
    assert_eq!(overridden.get_native_type(), Some(&get_string()));
    assert_eq!(overridden.get_raw_native_type(), Some(&get_int()));

    for (raw, keep_raw) in [
        (get_mixed(), true),
        (
            mago_codex::ttype::wrap_atomic(TAtomic::Mixed(
                mago_codex::ttype::atomic::mixed::TMixed::with_subtracted_type(get_void()),
            )),
            false,
        ),
    ] {
        artifacts.set_resolved_expression_type(&expr, raw.clone());
        artifacts.set_expression_type(&expr, get_int());
        let result = artifacts.get_expression_result(&expr).unwrap();
        let expected = if keep_raw { raw } else { get_int() };
        assert_eq!(result.get_keep_void_type(), &expected);
    }
}

#[test]
fn public_result_reads_match_phpstan_factory_controls() {
    use mago_codex::ttype::atomic::{generic::TGenericParameter, mixed::TMixed};
    use mago_codex::{
        misc::GenericParent,
        ttype::{get_keyed_array, wrap_atomic},
    };
    use mago_word::{empty_word, word};
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"controls.php"), Cow::Borrowed(b"<?php resultVoid();"));
    let program = parse_file(&arena, &file);
    let mago_syntax::cst::Statement::Expression(statement) = &program.statements.as_slice()[1] else {
        panic!("expression")
    };
    let expr = statement.expression;
    let template = wrap_atomic(TAtomic::GenericParameter(TGenericParameter::new(
        word("T"),
        Arc::new(get_mixed()),
        GenericParent::FunctionLike((word("sample"), empty_word())),
    )));
    for row in include_str!("../../../tests/expression-result/controls.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let name = fields[0];
        let mut raw = match name {
            "lazy-void" | "eager-void" | "held-void" => get_void(),
            "held-mixed" => get_mixed(),
            "held-nonvoid" => get_string(),
            "held-void-value" => get_int(),
            "union" => TUnion::from_vec(vec![TAtomic::Void, TAtomic::Null, get_int().get_single().clone()]),
            "nested" => get_keyed_array(get_int(), get_void()),
            "mixed-minus-void" => wrap_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(get_void()))),
            "template-mixed" => template.clone(),
            "mixed-minus-template" => wrap_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(template.clone()))),
            "mixed-minus-mixed-minus-void" => wrap_atomic(TAtomic::Mixed(TMixed::with_subtracted_type(wrap_atomic(
                TAtomic::Mixed(TMixed::with_subtracted_type(get_void())),
            )))),
            _ => panic!("unknown oracle control {name}"),
        };
        raw.set_possibly_undefined(true, None);
        let held = name.starts_with("held-")
            || name == "eager-void"
            || name.starts_with("mixed-minus-")
            || name == "template-mixed";
        let mut artifacts = AnalysisArtifacts::new().with_native_types(true);
        artifacts.begin_expression_result(
            expr,
            held.then(|| Rc::new(if name == "held-void-value" { get_void() } else { get_int() })),
            held.then(|| Rc::new(get_string())),
        );
        if name == "eager-void" {
            artifacts.set_expression_type(expr, raw.clone());
            artifacts.set_rc_native_expression_type(expr, Rc::new(raw.clone()));
        } else {
            artifacts.set_resolved_expression_type(expr, raw.clone());
            artifacts.set_resolved_native_expression_type(expr, Rc::new(raw.clone()));
        }
        let result = artifacts.get_expression_result(expr).unwrap();
        // Compare types structurally for the two tools' differently named array/template displays.
        let expected = |field: &str| {
            if field == fields[2] {
                raw.clone()
            } else if field == "null" {
                get_null()
            } else if field == "int" {
                get_int()
            } else if field == "string" {
                get_string()
            } else if field == "void" {
                get_void()
            } else if field == "int|null" {
                TUnion::from_vec(vec![TAtomic::Null, get_int().get_single().clone()])
            } else {
                panic!("unexpected oracle type {field}")
            }
        };
        assert_eq!(result.get_raw_type(), &raw, "{name} raw");
        for (actual, field) in [
            (result.get_type().clone(), fields[1]),
            (result.get_keep_void_type().clone(), fields[3]),
            (result.get_native_type().unwrap().clone(), fields[4]),
            (result.get_type_on_scope(Some(&get_int()), None).as_ref().clone(), fields[5]),
            (result.get_native_type_on_scope(Some(&get_string()), None).unwrap().as_ref().clone(), fields[6]),
            (result.at_ask_position(Some((&get_int(), Some(&get_string()))), None, None).get_type().clone(), fields[7]),
            (
                result
                    .at_ask_position(Some((&get_int(), Some(&get_string()))), None, None)
                    .get_native_type()
                    .unwrap()
                    .clone(),
                fields[8],
            ),
            (result.get_raw_native_type().unwrap().clone(), fields[9]),
            (result.get_keep_void_native_type().unwrap().clone(), fields[10]),
            (result.get_type_on_scope(Some(&get_void()), None).as_ref().clone(), fields[11]),
        ] {
            let expected = expected(field);
            let mut actual_types = actual.types.to_vec();
            let mut expected_types = expected.types.to_vec();
            actual_types.sort_unstable();
            expected_types.sort_unstable();
            assert_eq!(actual_types, expected_types, "{name}: {field}");
        }
        if name == "union" {
            assert_eq!(result.get_type().flags, raw.flags);
        }
    }
}
