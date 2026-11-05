//! DeferrableServiceProviderMissingProvidesRule (InClassNode) and NoPublicModelScopeAndAccessorRule
//! (InClassMethodNode).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::{Checker, members};
use super::{MODEL, class_is, is_a};

const SERVICE_PROVIDER: &[u8] = b"illuminate\\support\\serviceprovider";
const DEFERRABLE_PROVIDER: &[u8] = b"illuminate\\contracts\\support\\deferrableprovider";
const ATTRIBUTE: &[u8] = b"illuminate\\database\\eloquent\\casts\\attribute";
const SCOPE_ATTRIBUTE: &str = "Illuminate\\Database\\Eloquent\\Attributes\\Scope";

/// DeferrableServiceProviderMissingProvidesRule on a class declaration.
pub(crate) fn class<'arena>(c: &mut Checker<'_, 'arena>, class: &Class<'arena>) {
    let Some(k) = c.larastan.and(c.class_scope()) else {
        return;
    };
    if !class_is(c, k, SERVICE_PROVIDER)
        || !c
            .codebase
            .class_implements(k.name.as_bytes(), DEFERRABLE_PROVIDER)
    {
        return;
    }
    // the provides method is overwritten somewhere in the class hierarchy
    if members::method(c, k, b"provides").is_none_or(|m| {
        !m.declaring_name()
            .as_bytes()
            .eq_ignore_ascii_case(SERVICE_PROVIDER)
    }) {
        return;
    }
    c.report(
        class,
        "larastan.deferrableServiceProvider.missingProvides",
        "ServiceProviders that implement the \"DeferrableProvider\" interface should implement the \"provides\" \
         method that returns an array of strings or class-strings"
            .to_string(),
        None,
    );
}

/// NoPublicModelScopeAndAccessorRule on a method of a model.
pub(crate) fn method<'arena>(c: &mut Checker<'_, 'arena>, m: &Method<'arena>) {
    let Some(k) = c
        .larastan
        .filter(|l| {
            l.switches
                .contains(crate::LarastanSwitches::CHECK_MODEL_METHOD_VISIBILITY)
        })
        .and(c.class_scope())
    else {
        return;
    };
    if !class_is(c, k, MODEL) {
        return;
    }
    let name = String::from_utf8_lossy(m.name.value).into_owned();
    if k.direct_parent_class
        .and_then(|p| c.codebase.get_class_like(p.as_bytes()))
        .is_some_and(|p| members::method(c, p, m.name.value).is_some())
    {
        return;
    }
    if m.modifiers.contains_protected() {
        return;
    }
    let (identifier, message) = if is_scope(c, m) {
        (
            "larastan.noPublicModelScopeMethod",
            format!("Local query scope method '{name}' should be declared as protected."),
        )
    } else if is_accessor(c, k, m) {
        (
            "larastan.noPublicModelAccessorMethod",
            format!("Model accessor method '{name}' should be declared as protected."),
        )
    } else {
        return;
    };
    c.report(m.span(), identifier, message, None);
}

/// `scope*` with a `Builder` first parameter, or `#[Scope]`.
fn is_scope(c: &Checker, m: &Method) -> bool {
    if m.name.value.starts_with(b"scope")
        && let Some(Hint::Identifier(id)) = m
            .parameter_list
            .parameters
            .first()
            .and_then(|p| p.hint.as_ref())
    {
        let name = c.names.get(&id.span());
        if name == b"Builder" || name.ends_with(b"\\Builder") {
            return true;
        }
    }
    m.attribute_lists
        .iter()
        .flat_map(|l| l.attributes.iter())
        .any(|a| c.resolved(&a.name) == SCOPE_ATTRIBUTE)
}

/// The method's return type (PHPDoc, else native) is an `Attribute`.
fn is_accessor(
    c: &Checker,
    k: &mago_codex::metadata::class_like::ClassLikeMetadata,
    m: &Method,
) -> bool {
    c.codebase
        .get_method(k.name.as_bytes(), m.name.value)
        .and_then(|f| f.return_type_metadata.as_ref())
        .is_some_and(|t| {
            !t.type_union.types.is_empty()
                && t.type_union.types.iter().all(|t| match t {
                    TAtomic::Object(TObject::Named(n)) => is_a(c, n.name.as_bytes(), ATTRIBUTE),
                    _ => false,
                })
        })
}
