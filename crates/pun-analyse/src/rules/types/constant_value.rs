//! Level 2's ValueAssignedToClassConstantRule (`classConstant.value`, `classConstant.phpDocType`): a class constant's
//! value its native type or `@var` tag does not take. (The constants of `dynamicConstantNames` are bleeding edge.)
//! With `featureToggles.checkDynamicConstantNameValues`, ValueAssignedToGlobalConstantRule (`constant.value`) and
//! ValueAssignedToDefineRule (`constant.defineValue`): a global constant's value its `dynamicConstantNames` type does
//! not accept.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, StrictMixed, Type, accepts_reasons, decide_type, is_super_type_of, recommended_level};

use super::super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &["classConstant.value", "classConstant.phpDocType", "constant.value", "constant.defineValue"];

/// The values, whose types the rule reads.
pub(super) fn typed(k: &ClassLikeConstant, spans: &mut Vec<Span>) {
    spans.extend(k.items.iter().map(|i| i.value.span()));
}

/// The values of a `const` statement, whose types ValueAssignedToGlobalConstantRule reads.
pub(super) fn typed_global(k: &Constant, spans: &mut Vec<Span>) {
    spans.extend(k.items.iter().map(|i| i.value.span()));
}

/// ValueAssignedToGlobalConstantRule: each constant of a `const` statement, by its namespaced name.
pub(in super::super) fn global_constant(c: &mut Checker, k: &Constant) {
    if c.toggled.constant_types.is_none() {
        return;
    }
    let at = k.attribute_lists.first().map_or(k.span(), |a| a.span().join(k.span()));
    for item in k.items.iter() {
        let name = String::from_utf8_lossy(item.name.value);
        let name = match c.namespaces.last() {
            Some(Some(namespace)) => format!("{namespace}\\{name}"),
            _ => name.into_owned(),
        };
        let Some(value) = c.phpstan_type(item.value) else { continue };
        configured_value(c, at, "constant.value", &name, &value);
    }
}

/// ValueAssignedToDefineRule: `define()` with the constant strings of its first argument as the names.
pub(in super::super) fn define_value(c: &mut Checker, call: &FunctionCall) {
    if c.toggled.constant_types.is_none() {
        return;
    }
    let Expression::Identifier(id) = call.function else { return };
    if !super::super::dead_code::node_name(c.names, id).eq_ignore_ascii_case(b"define") {
        return;
    }
    let [name, value, ..] = call.argument_list.arguments.as_slice() else { return };
    let Some(names) = c.phpstan_type(name.value()).map(|t| t.constant_strings()) else { return };
    if names.is_empty() {
        return;
    }
    let Some(value) = c.phpstan_type(value.value()) else { return };
    for name in names.iter().filter(|n| !n.is_empty()) {
        configured_value(c, call.span(), "constant.defineValue", &String::from_utf8_lossy(name), &value);
    }
}

/// `ConstantResolver::getConfiguredGlobalConstantType()` against the value: `accepts($valueType, true)`.
fn configured_value(c: &mut Checker, at: Span, identifier: &str, name: &str, value: &Type) {
    let Some(types) = &c.toggled.constant_types else { return };
    let Some((_, configured)) = types.iter().find(|(n, _)| n == name) else { return };
    let found = declared::with_classes(c, || {
        let configured = pun_phpstan::resolve::resolve_str(configured, None);
        if pun_phpstan::types::accepts(&configured, value, true).yes() {
            return None;
        }
        let message = format!(
            "Configuration defined type for constant {name} ({}) does not accept value {}.",
            configured.describe(Level::Value),
            value.describe(recommended_level(&configured, Some(value)))
        );
        // acceptsReasonsTip(): one reason as is, more as a bullet list
        let tip = match accepts_reasons(&configured, value, true, StrictMixed::default()).as_slice() {
            [] => None,
            [one] => Some(one.clone()),
            all => Some(all.iter().map(|r| format!("• {r}")).collect::<Vec<_>>().join("\n")),
        };
        Some((message, tip))
    });
    if let Some((message, tip)) = found {
        c.report(at, identifier, message, tip.as_deref());
    }
}

pub(in super::super) fn class_constant(c: &mut Checker, k: &ClassLikeConstant) {
    if c.level < 2 {
        return;
    }
    let Some(context) = c.class_scope() else { return };
    let display = c.display(context);
    for item in k.items.iter() {
        let Some(value) = c.phpstan_type(item.value) else { continue };
        let name = String::from_utf8_lossy(item.name.value);
        let found = declared::with_classes(c, || {
            let native = k.hint.as_ref().map(|h| declared::native(c, h, Some(context)));
            let phpdoc = declared::constant(c, k.span(), item.name.value, context);
            let level = |t| recommended_level(t, Some(&value));
            match (phpdoc, native) {
                (None, None) => None,
                (None, Some(native)) => (!pun_phpstan::types::accepts(&native, &value, true).yes()).then(|| {
                    let message = format!(
                        "Constant {display}::{name} ({}) does not accept value {}.",
                        native.describe(Level::TypeOnly),
                        value.describe(Level::Value)
                    );
                    ("classConstant.value", message, true)
                }),
                (Some(phpdoc), None) => {
                    let fits = is_super_type_of(&phpdoc, &value);
                    let relation = if fits.no() {
                        "is incompatible with"
                    } else if fits.maybe() {
                        "is not subtype of"
                    } else { return None };
                    let message = format!(
                        "PHPDoc tag @var for constant {display}::{name} with type {} {relation} value {}.",
                        phpdoc.describe(level(&phpdoc)),
                        value.describe(Level::Value)
                    );
                    Some(("classConstant.phpDocType", message, false))
                }
                (Some(phpdoc), Some(native)) => {
                    let ty = decide_type(native, Some(phpdoc));
                    (!pun_phpstan::types::accepts(&ty, &value, true).yes()).then(|| {
                        let message = format!(
                            "Constant {display}::{name} ({}) does not accept value {}.",
                            ty.describe(Level::TypeOnly),
                            value.describe(level(&ty))
                        );
                        ("classConstant.value", message, false)
                    })
                }
            }
        });
        match found {
            Some((identifier, message, true)) => c.report_non_ignorable(k, identifier, message, None),
            Some((identifier, message, false)) => c.report(k, identifier, message, None),
            None => {}
        }
    }
}
