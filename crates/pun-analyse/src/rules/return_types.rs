//! PHPStan's dynamic return types that Mago's own types lack, as Mago return type providers for the rules' inference.

use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::Provider;
use mago_analyzer::plugin::provider::ProviderMeta;
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};

/// The registry the rules' inference runs with; `contents` is the analysed file's (`bindTo()` finds its receiver in it).
pub(super) fn registry(contents: &[u8]) -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    registry.register_function_provider(Benevolent);
    registry.register_method_provider(Bind(contents.to_vec()));
    registry
}

/// Marks a Mago type as one of PHPStan's `BenevolentUnionType`s (`to_phpstan()` reads both flags together).
pub(super) fn benevolent(mut t: TUnion) -> TUnion {
    t.set_ignore_nullable_issues(true);
    t.set_ignore_falsable_issues(true);
    t
}

pub(super) fn is_benevolent(t: &TUnion) -> bool {
    t.ignore_nullable_issues() && t.ignore_falsable_issues()
}

static META: ProviderMeta = ProviderMeta::new("pun", "pun", "PHPStan's dynamic return types");

/// The functions whose functionMap return type is a `__benevolent<…>` union.
struct Benevolent;

impl Provider for Benevolent {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for Benevolent {
    fn targets() -> FunctionTarget {
        FunctionTarget::ExactMultiple(&[
            b"apcu_cache_info",
            b"apcu_sma_info",
            b"curl_init",
            b"get_include_path",
            b"getopt",
            b"imagecreate",
            b"imagecreatetruecolor",
            b"php_sapi_name",
            b"scandir",
            b"stream_get_contents",
            b"tempnam",
            b"tmpfile",
        ])
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let function = context
            .codebase()
            .get_function(invocation.function_name().as_bytes())?;
        Some(benevolent(
            function.return_type_metadata.as_ref()?.type_union.clone(),
        ))
    }
}

/// `ClosureBindDynamicReturnTypeExtension`, `ClosureBindToDynamicReturnTypeExtension`
struct Bind(Vec<u8>);

impl Provider for Bind {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for Bind {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 2] = [
            MethodTarget::exact(b"closure", b"bind"),
            MethodTarget::exact(b"closure", b"bindto"),
        ];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let (closure, new_this) = if method.eq_ignore_ascii_case(b"bind") {
            let closure =
                context.get_expression_type(invocation.get_argument(0, &[b"closure"])?)?;
            (closure, invocation.get_argument(1, &[b"newThis"]))
        } else {
            let closure = context.get_expression_type(&self.receiver(invocation)?)?;
            (closure, invocation.get_argument(0, &[b"newThis"]))
        };
        // Mago expands a closure alias when a variable holding it is read (see `to_phpstan()`)
        let [atom @ TAtomic::Callable(TCallable::Signature(signature))] = closure.types.as_ref()
        else {
            return None;
        };
        if !signature.is_closure() {
            return None;
        }
        let source = signature.get_source();
        let maybe_null = || benevolent(TUnion::from_vec(vec![atom.clone(), TAtomic::Null]));
        if !self.is_static(context.codebase(), source) {
            return Some(maybe_null());
        }
        let Some(new_this) = new_this.and_then(|e| context.get_expression_type(e)) else {
            return Some(closure.clone());
        };
        Some(if new_this.is_null() {
            closure.clone()
        } else if !new_this.has_null() && !new_this.has_mixed() {
            TUnion::from_vec(vec![TAtomic::Null])
        } else {
            maybe_null()
        })
    }
}

impl Bind {
    /// `$closure` of `$closure->bindTo(…)`: from the call's start to the `->` before the method name.
    fn receiver(&self, invocation: &InvocationInfo<'_, '_, '_>) -> Option<Span> {
        let call = invocation.span();
        let arguments = invocation.inner().arguments_source.span();
        let before = self
            .0
            .get(call.start.offset as usize..arguments.start.offset as usize)?
            .trim_ascii_end();
        let before = before[..before.len() - b"bindTo".len()]
            .trim_ascii_end()
            .strip_suffix(b"->")?;
        let before = before.strip_suffix(b"?").unwrap_or(before).trim_ascii_end();
        Some(Span::new(
            call.file_id,
            call.start,
            call.start.forward(before.len() as u32),
        ))
    }

    /// `ClosureType::isStaticClosure()`: Mago's metadata has no static closures, its source does (`static fn`).
    fn is_static(
        &self,
        codebase: &CodebaseMetadata,
        source: Option<FunctionLikeIdentifier>,
    ) -> bool {
        source
            .and_then(|id| codebase.get_function_like(&id))
            .and_then(|f| self.0.get(f.span.start.offset as usize..))
            .is_some_and(|text| text.starts_with(b"static"))
    }
}
