//! MacroMethodsClassReflectionExtension's methods, as a Mago return type provider: a method a class lacks is a macro
//! registered on it ([`crate::larastan::macros`]). Models and relations reach their builder's and the query builder's
//! macros, as ModelForwardsCallsExtension, RelationForwardsCallsExtension and
//! EloquentBuilderForwardsCallsExtension forward the calls.

use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::ttype::union::TUnion;

use crate::larastan::app::App;
use crate::larastan::macros::Macros as Registered;

static META: ProviderMeta = ProviderMeta::new("pun-larastan-macros", "pun-larastan-macros", "Larastan's macros");

pub(crate) struct Macros(pub(crate) Arc<App>, pub(crate) Arc<Registered>);

impl Provider for Macros {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for Macros {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 1] = [MethodTarget::any_class(b"*")];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        class: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        if self.1.is_empty() {
            return None;
        }
        let codebase = context.codebase();
        let receiver = invocation.inner().target.get_method_context().map_or(class, |t| t.class_like_metadata.name.as_bytes());
        if codebase.method_exists(receiver, method) {
            return None;
        }
        let homes = self.1.homes(codebase, &self.0, receiver);
        self.1.get(&homes, &String::from_utf8_lossy(method)).map(|m| m.ty.clone())
    }
}

/// MacroMethodsClassReflectionExtension: the macro of a method the class declares no native method
/// for (a `@method` tag does not hide one).
pub(crate) fn macro_method(
    c: &super::super::Checker,
    class: &mago_codex::metadata::class_like::ClassLikeMetadata,
    method: &[u8],
) -> Option<crate::larastan::macros::Macro> {
    let larastan = c.larastan?;
    if larastan.macros.is_empty() {
        return None;
    }
    let lower = mago_word::word(&method.to_ascii_lowercase());
    if let Some(declaring) = c.codebase.get_declaring_method_class(class.name.as_bytes(), method)
        && let Some(k) = c.codebase.get_class_like(declaring.as_bytes())
        && !k.pseudo_methods.contains(&lower)
        && !k.static_pseudo_methods.contains(&lower)
    {
        return None;
    }
    let homes = larastan.macros.homes(c.codebase, &larastan.app, class.name.as_bytes());
    larastan.macros.get(&homes, &String::from_utf8_lossy(method)).cloned()
}

