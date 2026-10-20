//! SetPropertyHookParameterRule (InPropertyHookNode of a `set` hook with a parameter list; without one the parameter
//! takes the property's type). checkPhpDocMethodSignatures (propertySetHook.parameterType and the missing-typehint
//! checks) is off below level 3.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, NeverKind, Type, decide_type, is_super_type_of, mixed};

use super::doc::{Doc, doc_before};
use super::{Checker, declared, missing_types, signatures};

/// The hooks `list` of property `name` (with its `$`) declared with the native type `hint`.
pub fn hooks<'arena>(
    c: &mut Checker<'_, 'arena>,
    name: &[u8],
    hint: Option<&Hint<'arena>>,
    list: &PropertyHookList<'arena>,
) {
    let k = c.class_scope().expect("a property is walked inside its class-like");
    let class = c.display(k);
    let property = String::from_utf8_lossy(&name[1..]).into_owned();
    for hook in list.hooks.iter().filter(|h| h.name.value == b"set") {
        let Some(parameter) = hook.parameter_list.as_ref().and_then(|l| l.parameters.first()) else {
            continue;
        };
        if let Some(message) = native_error(c, k, &class, &property, hint, parameter) {
            c.report_non_ignorable(hook, "propertySetHook.nativeParameterType", message, None);
        }
    }
}

/// The propertySetHook.nativeParameterType message of a set hook's `parameter`, if any.
fn native_error(
    c: &Checker,
    k: &ClassLikeMetadata,
    class: &str,
    property: &str,
    hint: Option<&Hint>,
    parameter: &FunctionLikeParameter,
) -> Option<String> {
    let parameter_name = String::from_utf8_lossy(&parameter.variable.name[1..]).into_owned();
    Some(match (hint, &parameter.hint) {
        (None, None) => return None,
        (None, Some(_)) => format!(
            "Parameter ${parameter_name} of set hook has a native type but the property {class}::${property} does not."
        ),
        (Some(_), None) => format!(
            "Parameter ${parameter_name} of set hook does not have a native type but the property {class}::${property} does."
        ),
        (Some(theirs), Some(ours)) => {
            // an intersection TypeCombinator::intersect() turns into an implicit never
            let resolve = |c: &Checker, hint: &Hint| {
                if signatures::unresolvable(c, hint) {
                    Type::Never(NeverKind::Implicit)
                } else {
                    declared::native(c, hint, Some(k))
                }
            };
            let theirs = resolve(c, theirs);
            let ours = resolve(c, ours);
            if is_super_type_of(&ours, &theirs).yes() {
                return None;
            }
            format!(
                "Native type {} of set hook parameter ${parameter_name} is not contravariant with native type {} of property {class}::${property}.",
                ours.describe(Level::TypeOnly),
                theirs.describe(Level::TypeOnly)
            )
        }
    })
}

/// The rule's `checkMissingTypehints` checks (level 6) of the set hook parameter's type, unless its native type is
/// the property's readable type. The parameter of a hook without a parameter list is the property's.
pub fn missing_types<'arena>(
    c: &mut Checker<'_, 'arena>,
    name: &[u8],
    hint: Option<&Hint<'arena>>,
    list: &PropertyHookList<'arena>,
    decl: Span,
) {
    if c.level < 6 {
        return;
    }
    let (k, display) = c.member_scope();
    let property = String::from_utf8_lossy(&name[1..]).into_owned();
    let class = c.display(k);
    for hook in list.hooks.iter().filter(|h| h.name.value == b"set") {
        let parameter = hook.parameter_list.as_ref().and_then(|l| l.parameters.first());
        if parameter.is_some_and(|p| native_error(c, k, &class, &property, hint, p).is_some()) {
            continue;
        }
        let (param, native_hint) = match parameter {
            Some(p) => (String::from_utf8_lossy(&p.variable.name[1..]).into_owned(), p.hint.as_ref()),
            None => ("value".to_string(), hint),
        };
        let found = declared::with_classes(c, || {
            let readable = declared::property(c, hint, decl, &name[1..], k);
            let native = native_hint.map_or_else(mixed, |h| declared::native(c, h, Some(k)));
            if native == readable {
                return Vec::new();
            }
            // the hook's own `@param`, else the property's `@var`
            let own = doc_before(&c.file.contents, hook.span().start.offset as usize).and_then(|doc| {
                let scope = declared::scope_at(c, hook.span(), Some(k));
                Doc::parse(doc).param_types(&scope).remove(&param).map(|(t, _)| t)
            });
            let phpdoc = own.or_else(|| {
                Some(declared::property(c, None, decl, &name[1..], k)).filter(|t| !t.is_implicit_mixed())
            });
            let ty = decide_type(native, phpdoc);
            let subject = format!("Set hook for property {display}::${property}");
            missing_types::parameter(c, &ty, &subject, &format!("parameter ${param}"))
        });
        for (identifier, message, tip) in found {
            c.report(hook, identifier, message, tip);
        }
    }
}
