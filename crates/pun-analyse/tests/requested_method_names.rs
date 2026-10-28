//! Requested method names are distinct from their unchanged trait storage IDs.
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use mago_allocator::LocalArena;
use mago_analyzer::{Analyzer, analysis_result::AnalysisResult, plugin::*, settings::Settings};
use mago_codex::{
    populator::populate_codebase, reference::SymbolReferences, scanner::scan_program,
    ttype::union::TUnion,
};
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;

const CODE: &[u8] = include_bytes!("fixtures/call-scope-observations/method-identities.php");

type Record = (usize, String, String, String, String);
#[derive(Clone, Default)]
struct Names(Arc<Mutex<BTreeSet<Record>>>);
impl Names {
    fn collect(&self, call: &InvocationInfo<'_, '_, '_>) {
        let Some(context) = call.inner().target.get_method_context() else {
            return;
        };
        let storage = context
            .declaring_method_id
            .expect("fixture methods have storage");
        let line = CODE[..call.span().start.offset as usize]
            .iter()
            .filter(|b| **b == b'\n')
            .count()
            + 1;
        self.0.lock().unwrap().insert((
            line,
            context.requested_name.to_string(),
            storage.get_class_name().to_string(),
            storage.get_method_name().to_string(),
            context.class_like_metadata.original_name.to_string(),
        ));
    }
}
impl Provider for Names {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta =
            ProviderMeta::new("method-names", "method-names", "resolved identity contract");
        &META
    }
}
impl ExpressionHook for Names {
    fn is_observer(&self) -> bool {
        true
    }
    fn before_arguments(
        &self,
        call: &InvocationInfo<'_, '_, '_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        self.collect(call);
        Ok(())
    }
}
impl MethodReturnTypeProvider for Names {
    fn targets() -> &'static [MethodTarget] {
        &[MethodTarget {
            class: b"*",
            method: b"*",
        }]
    }
    fn get_return_type(
        &self,
        _: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        _: &[u8],
        call: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        // Implicit __toString invocations price returns without analysing an argument list.
        self.collect(call);
        None
    }
}

#[test]
fn lookup_names_survive_aliases_inheritance_dynamic_static_constructor_and_implicit_calls() {
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"method-identities.php"), Cow::Borrowed(CODE));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors());
    let resolved = NameResolver::new(&arena).resolve(program);
    let settings = Settings::default();
    let mut codebase = scan_program(&arena, &file, program, &resolved, settings.version);
    let mut references = SymbolReferences::new();
    populate_codebase(
        &mut codebase,
        &mut references,
        Default::default(),
        Default::default(),
    );
    let names = Names::default();
    let mut registry = PluginRegistry::new();
    registry.register_expression_hook(names.clone());
    registry.register_method_provider(names.clone());
    let mut result = AnalysisResult::new(references);
    Analyzer::new(&arena, &file, &resolved, &codebase, &registry, settings)
        .analyze(program, &mut result)
        .unwrap();
    // Derived by hand from each selector and the trait method declarations, not from storage lookup.
    let expected = [
        (20, "traitMutation", "traitmutation"),
        (21, "aliasMutation", "traitmutation"),
        (22, "ALIASMutation", "traitmutation"),
        (23, "aliasMutation", "traitmutation"),
        (25, "traitMutation", "traitmutation"),
        (25, "aliasMutation", "traitmutation"),
        (26, "traitStatic", "traitstatic"),
        (27, "aliasStatic", "traitstatic"),
        (29, "traitStatic", "traitstatic"),
        (29, "aliasStatic", "traitstatic"),
        (30, "__construct", "boot"),
        (31, "__toString", "render"),
    ]
    .into_iter()
    .map(|(line, name, storage)| {
        (
            line,
            name.to_ascii_lowercase(),
            "identitymethods".into(),
            storage.into(),
            "IdentityChild".into(),
        )
    })
    .collect();
    assert_eq!(*names.0.lock().unwrap(), expected);
}
