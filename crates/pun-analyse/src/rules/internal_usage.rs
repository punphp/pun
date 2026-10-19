//! RestrictedFunctionUsageRule, RestrictedFunctionCallableUsageRule and RestrictedClassConstantUsageRule with the
//! RestrictedInternal*UsageExtensions of config.level0.neon (under `featureToggles.internalTag`): using an
//! `@internal` function or class constant, or a constant of an `@internal` class-like, from outside the root
//! namespace that declares it.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Checker;
use super::constants::declaring;
use super::internal::{root_namespace, should_be_reported};

/// `f(...)`: FuncCall.
pub fn function_call(c: &mut Checker, call: &FunctionCall) {
    function(c, call, call.function);
}

/// `f(...)`: FunctionCallableNode.
pub fn function_callable(c: &mut Checker, f: &FunctionPartialApplication) {
    let whole = f.function.span().join(f.argument_list.span());
    let callable = matches!(
        f.argument_list.arguments.iter().collect::<Vec<_>>().as_slice(),
        [PartialArgument::VariadicPlaceholder(_)]
    );
    if callable || c.piped.contains(&whole) {
        function(c, whole, f.function);
    }
}

/// RestrictedInternalFunctionUsageExtension on the function `name` names.
fn function(c: &mut Checker, at: impl HasSpan, name: &Expression) {
    let Expression::Identifier(id) = name else {
        return;
    };
    if !c.internal_tag {
        return;
    }
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    let Some(f) = std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .find_map(|n| c.codebase.get_function(n))
    else {
        return;
    };
    let name = String::from_utf8_lossy(f.original_name.as_bytes()).into_owned();
    if !f.flags.is_internal() || !should_be_reported(c, &name) {
        return;
    }
    let message = match root_namespace(&name) {
        None => format!("Call to internal function {name}()."),
        Some(ns) => format!("Call to internal function {name}() from outside its root namespace {ns}."),
    };
    c.report(at, "function.internal", message, None);
}

/// `X::NAME`.
pub fn class_constant(c: &mut Checker, a: &ClassConstantAccess) {
    let ClassLikeConstantSelector::Identifier(name) = &a.constant else {
        return;
    };
    if !c.internal_tag {
        return;
    }
    let scope = c.class_scope();
    let referenced = match a.class {
        Expression::Identifier(id) => c.codebase.get_class_like(c.names.get(&id.span())),
        Expression::Self_(_) | Expression::Static(_) => scope,
        Expression::Parent(_) => scope
            .and_then(|k| k.direct_parent_class)
            .and_then(|p| c.codebase.get_class_like(p.as_bytes())),
        // RuleLevelHelper::findTypeToCheck() with checkThisOnly (levels 0 and 1)
        // ponytail: from level 2 the class expression's type is needed
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" && c.level < 2 && c.this_defined() => scope,
        _ => None,
    };
    let Some(k) = referenced else {
        return;
    };
    // `hasConstant()`: a constant (declared by the class-like holding it) or an enum case
    let found = match k.constants.get(&mago_word::word(name.value)) {
        Some(constant) => Some((declaring(c, k, constant), constant.flags.is_internal())),
        None => k.enum_cases.get(&mago_word::word(name.value)).map(|case| (k, case.flags.is_internal())),
    };
    if let Expression::Identifier(_) = a.class {
        class_name(c, a, k, name.value, found);
    }
    let Some((owner, constant_internal)) = found else {
        return;
    };
    let Some(message) = restricted(c, constant_internal, owner, name.value) else {
        return;
    };
    // RewrittenDeclaringClassClassConstantReflection: also restricted as the referenced class's own
    if owner.name != k.name && restricted(c, constant_internal, k, name.value).is_none() {
        return;
    }
    c.report(a, &message.1, message.0, None);
}

/// ClassConstantRule's RestrictedInternalClassNameUsageExtension at CLASS_CONSTANT_ACCESS: a constant of a named
/// `@internal` class-like that is not itself restricted (or `::class`).
fn class_name(
    c: &mut Checker,
    a: &ClassConstantAccess,
    k: &ClassLikeMetadata,
    name: &[u8],
    found: Option<(&ClassLikeMetadata, bool)>,
) {
    let class = String::from_utf8_lossy(k.original_name.as_bytes()).into_owned();
    if !k.flags.is_internal()
        || !should_be_reported(c, &class)
        || found.is_some_and(|(owner, internal)| internal || owner.flags.is_internal())
    {
        return;
    }
    let (description, kind) = describe(k);
    let part = format!("internal {description} {}", c.display(k));
    let message = match found {
        Some(_) => format!("Access to constant {} on {part}.", String::from_utf8_lossy(name)),
        None => format!("Access to constant on {part}."),
    };
    c.report(a, &format!("classConstant.internal{kind}"), message, None);
}

/// `getClassTypeDescription()`, lowercase and as is.
fn describe(k: &ClassLikeMetadata) -> (&'static str, &'static str) {
    match k.kind {
        SymbolKind::Interface => ("interface", "Interface"),
        SymbolKind::Trait => ("trait", "Trait"),
        SymbolKind::Enum => ("enum", "Enum"),
        _ => ("class", "Class"),
    }
}

/// RestrictedInternalClassConstantUsageExtension: the message and identifier.
fn restricted(
    c: &Checker,
    constant_internal: bool,
    owner: &ClassLikeMetadata,
    name: &[u8],
) -> Option<(String, String)> {
    if !constant_internal && !owner.flags.is_internal() {
        return None;
    }
    let owner_name = String::from_utf8_lossy(owner.original_name.as_bytes()).into_owned();
    if !should_be_reported(c, &owner_name) {
        return None;
    }
    let display = c.display(owner);
    let name = String::from_utf8_lossy(name);
    let (description, kind) = describe(owner);
    let outside = root_namespace(&owner_name).map_or_else(String::new, |ns| format!(" from outside its root namespace {ns}"));
    Some(if constant_internal {
        (format!("Access to internal constant {display}::{name}{outside}."), "classConstant.internal".to_string())
    } else {
        (format!("Access to constant {name} of internal {description} {display}{outside}."), format!("classConstant.internal{kind}"))
    })
}
