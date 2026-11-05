//! RestrictedFunctionUsageRule and RestrictedFunctionCallableUsageRule with RestrictedInternalFunctionUsageExtension
//! (config.level0.neon, under `featureToggles.internalTag`): a call to, or first-class callable of, an `@internal`
//! function from outside the root namespace that declares it; and with phpstan-deprecation-rules'
//! RestrictedDeprecatedFunctionUsageExtension, of a deprecated function.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::{Checker, function_checks, internal};

pub(super) fn function_call<'arena>(c: &mut Checker<'_, 'arena>, call: &FunctionCall<'arena>) {
    if let Expression::Identifier(id) = call.function {
        check(c, id, call);
    }
}

/// `x |> f(...)` is the call `f(x)` (PipeHandler).
pub(super) fn pipe<'arena>(c: &mut Checker<'_, 'arena>, pipe: &Pipe<'arena>) {
    if let Some(id) = function_checks::piped_function(pipe) {
        check(c, id, pipe.callable);
    }
}

/// `f(...)` outside a pipe (FunctionCallableNode).
pub(super) fn callable<'arena>(
    c: &mut Checker<'_, 'arena>,
    callable: &FunctionPartialApplication<'arena>,
) {
    if !c.piped.contains(&callable.span())
        && let [PartialArgument::VariadicPlaceholder(_)] =
            callable.argument_list.arguments.as_slice()
        && let Expression::Identifier(id) = callable.function
    {
        check(c, id, callable);
    }
}

/// The RestrictedFunctionUsageExtension services: RestrictedInternalFunctionUsageExtension, then
/// RestrictedDeprecatedFunctionUsageExtension.
fn check<'arena>(c: &mut Checker<'_, 'arena>, id: &Identifier<'arena>, at: impl HasSpan) {
    if !c.on(crate::rules::Switches::INTERNAL_TAG) && !c.on(crate::rules::Switches::DEPRECATIONS) {
        return;
    }
    let at = at.span();
    let resolved = function_checks::function_name(c.names, c.codebase, id);
    // a built-in's deprecation is PHPStan's (its signature map), not the stub's PHPDoc
    let user = c.codebase.get_function(resolved.as_bytes()).filter(|f| f.flags.is_user_defined());
    let (name, internal, deprecation) = match user {
        Some(function) => (
            String::from_utf8_lossy(function.original_name.as_bytes()).into_owned(),
            function.flags.is_internal(),
            if c.on(crate::rules::Switches::DEPRECATIONS) { c.function_deprecation(function) } else { None },
        ),
        None => match c.builtins.functions.get(&resolved) {
            Some(f) => (f.name.clone(), false, f.deprecated.then_some(None)),
            None => return,
        },
    };
    if c.on(crate::rules::Switches::INTERNAL_TAG)
        && internal
        && internal::should_be_reported(c, &name)
    {
        let message = match internal::root_namespace(&name) {
            Some(root) => {
                format!("Call to internal function {name}() from outside its root namespace {root}.")
            }
            None => format!("Call to internal function {name}()."),
        };
        c.report(at, "function.internal", message, None);
    }
    if let Some((message, identifier)) = super::deprecations::function(c, &name, deprecation) {
        c.report(at, &identifier, message, None);
    }
}
