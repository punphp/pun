//! RestrictedClassConstantUsageRule with RestrictedInternalClassConstantUsageExtension (config.level0.neon, under
//! `featureToggles.internalTag`) and phpstan-deprecation-rules' RestrictedDeprecatedClassConstantUsageExtension:
//! using an `@internal` class constant, or a constant of an `@internal` class-like, from outside the root namespace
//! that declares it, or a deprecated one; and ClassConstantRule's class-name check at `X::NAME`.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Checker;
use super::class_names::{self, Location, Member};
use super::constants::declaring;
use super::internal::{root_namespace, should_be_reported};

/// `X::NAME`.
pub(super) fn class_constant(c: &mut Checker, a: &ClassConstantAccess) {
    let ClassLikeConstantSelector::Identifier(name) = &a.constant else {
        return;
    };
    if !c.on(crate::rules::Switches::INTERNAL_TAG) && !c.on(crate::rules::Switches::DEPRECATIONS) {
        return;
    }
    let scope = c.class_scope();
    let referenced = match a.class {
        Expression::Identifier(id) => c.codebase.get_class_like(c.names.get(&id.span())),
        Expression::Self_(_) | Expression::Static(_) => scope,
        Expression::Parent(_) => scope
            .and_then(|k| k.direct_parent_class)
            .and_then(|p| c.codebase.get_class_like(p.as_bytes())),
        // RuleLevelHelper::findTypeToCheck() with checkThisOnly (levels 0 and 1), else the expression's type
        // ponytail: the first class of a union only
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" && c.level < 2 && c.this_defined() => scope,
        _ if c.level >= 2 => super::internal::object_classes(c, a.class).into_iter().next(),
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
    let deprecation = match found {
        Some((owner, _)) if c.on(crate::rules::Switches::DEPRECATIONS) => c.constant_deprecation(owner, name.value),
        _ => None,
    };
    if let Expression::Identifier(id) = a.class {
        // ClassConstantRule's ClassNameCheck at CLASS_CONSTANT_ACCESS
        let member = found.map(|(owner, internal)| Member {
            name: String::from_utf8_lossy(name.value).into_owned(),
            internal: internal || owner.flags.is_internal(),
            deprecated: deprecation.is_some() || (c.on(crate::rules::Switches::DEPRECATIONS) && c.class_deprecation(owner).is_some()),
        });
        for (message, identifier) in class_names::restricted(c, k, &Location::ClassConstantAccess(member)) {
            c.report(id, &identifier, message, None);
        }
    }
    let Some((owner, constant_internal)) = found else {
        return;
    };
    let constant = String::from_utf8_lossy(name.value).into_owned();
    let internal = |owner: &ClassLikeMetadata| restricted(c, constant_internal, owner, name.value);
    let deprecated = |owner: &ClassLikeMetadata| {
        super::deprecations::class_constant(c, owner, &constant, deprecation.clone())
    };
    let mut usages = Vec::new();
    // RewrittenDeclaringClassClassConstantReflection: also restricted as the referenced class's own
    if let Some(u) = internal(owner).filter(|_| owner.name == k.name || internal(k).is_some()) {
        usages.push(u);
    }
    if let Some(u) = deprecated(owner).filter(|_| owner.name == k.name || deprecated(k).is_some()) {
        usages.push(u);
    }
    for (message, identifier) in usages {
        c.report(a, &identifier, message, None);
    }
}

/// RestrictedInternalClassConstantUsageExtension: the message and identifier.
fn restricted(
    c: &Checker,
    constant_internal: bool,
    owner: &ClassLikeMetadata,
    name: &[u8],
) -> Option<(String, String)> {
    if !c.on(crate::rules::Switches::INTERNAL_TAG) || (!constant_internal && !owner.flags.is_internal()) {
        return None;
    }
    let owner_name = String::from_utf8_lossy(owner.original_name.as_bytes()).into_owned();
    if !should_be_reported(c, &owner_name) {
        return None;
    }
    let display = c.display(owner);
    let name = String::from_utf8_lossy(name);
    let kind = super::deprecations::type_description(owner);
    let description = kind.to_ascii_lowercase();
    let outside = root_namespace(&owner_name).map_or_else(String::new, |ns| format!(" from outside its root namespace {ns}"));
    Some(if constant_internal {
        (format!("Access to internal constant {display}::{name}{outside}."), "classConstant.internal".to_string())
    } else {
        (format!("Access to constant {name} of internal {description} {display}{outside}."), format!("classConstant.internal{kind}"))
    })
}
