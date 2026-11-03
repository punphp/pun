//! The level-2 rules on a class-like's `@phpstan-require-extends`, `@phpstan-require-implements` and
//! `@phpstan-sealed` tags: RequireExtendsDefinitionClassRule, RequireExtendsDefinitionTraitRule (both through
//! RequireExtendsCheck), RequireImplementsDefinitionClassRule, RequireImplementsDefinitionTraitRule and
//! SealedDefinitionClassRule.

use std::collections::HashSet;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::Span;
use pun_phpstan::phpdoc::TagValue;
use pun_phpstan::resolve::resolve_in;
use pun_phpstan::types::{Level, Type};

use super::class_names::{self, Pair};
use super::reflection::known;
use super::{Checker, DISCOVERING_SYMBOLS_TIP};

pub const IDENTIFIERS: &[&str] = &[
    "requireExtends.duplicate",
    "requireExtends.nonObject",
    "requireExtends.interface",
    "requireExtends.trait",
    "requireExtends.enum",
    "requireExtends.finalClass",
    "requireExtends.onClass",
    "requireExtends.onEnum",
    "requireImplements.nonObject",
    "requireImplements.class",
    "requireImplements.trait",
    "requireImplements.enum",
    "requireImplements.onClass",
    "requireImplements.onEnum",
    "requireImplements.onInterface",
    "sealed.onEnum",
    "sealed.nonObject",
];

/// The resolved types of a class-like's tags named `names` (PhpDocNodeResolver: the tags of the first name first).
pub(super) fn tag_types(c: &Checker, k: &ClassLikeMetadata, names: [&str; 2]) -> Vec<Type> {
    let Some(doc) = c.class_doc(k) else {
        return Vec::new();
    };
    let tags = super::doc::phpdoc_tags(&doc);
    let scope = super::declared::class_scope(c, k.span, k);
    // (the project's classes known, for `self&\stdClass` in a trait to be `never`)
    super::declared::with_classes(c, || {
        names
            .iter()
            .flat_map(|name| tags.iter().filter(move |t| t.name == *name))
            .filter_map(|t| match &t.value {
                TagValue::Type(node) => Some(resolve_in(node, &scope)),
                _ => None,
            })
            .collect()
    })
}

pub(super) const EXTENDS: [&str; 2] = ["@psalm-require-extends", "@phpstan-require-extends"];
const IMPLEMENTS: [&str; 2] = ["@psalm-require-implements", "@phpstan-require-implements"];
const SEALED: [&str; 2] = ["@psalm-inheritors", "@phpstan-sealed"];

/// RequireExtendsPropertiesClassReflectionExtension: own tags, then nominal parent interfaces in source order.
/// Keep whole Types: a successful annotation/union/intersection reflection must stop lookup too.
pub(super) fn property_requirements(c: &Checker, k: &ClassLikeMetadata) -> Vec<Type> {
    if k.kind != SymbolKind::Interface { return Vec::new() }
    let mut pending = vec![k];
    let mut seen = HashSet::new();
    let mut types = Vec::new();
    while let Some(interface) = pending.pop() {
        if seen.insert(interface.name) {
            types.extend(tag_types(c, interface, EXTENDS));
            pending.extend(super::overriding_property::declaration_interfaces(c, interface).into_iter().rev());
        }
    }
    types
}

/// The rules on InClassNode: a class, an interface or an enum declared at `at`.
pub fn class_like(c: &mut Checker, at: Span, k: &ClassLikeMetadata) {
    if c.level < 2 {
        return;
    }
    let kind = match k.kind {
        SymbolKind::Interface => "Interface",
        SymbolKind::Enum => "Enum",
        _ => "Class",
    };
    let extends = tag_types(c, k, EXTENDS);
    if !extends.is_empty() {
        if kind == "Interface" {
            extends_check(c, at, &extends);
        } else {
            c.report(
                at,
                &format!("requireExtends.on{kind}"),
                "PHPDoc tag @phpstan-require-extends is only valid on trait or interface."
                    .to_string(),
                None,
            );
        }
    }
    if !tag_types(c, k, IMPLEMENTS).is_empty() {
        c.report(
            at,
            &format!("requireImplements.on{kind}"),
            "PHPDoc tag @phpstan-require-implements is only valid on trait.".to_string(),
            None,
        );
    }
    let sealed = tag_types(c, k, SEALED);
    if sealed.is_empty() {
        return;
    }
    if kind == "Enum" {
        c.report(
            at,
            "sealed.onEnum",
            "PHPDoc tag @phpstan-sealed is only valid on class or interface.".to_string(),
            None,
        );
        return;
    }
    for ty in sealed {
        let classes = ty.object_class_names();
        if classes.is_empty() {
            c.report(
                at,
                "sealed.nonObject",
                format!(
                    "PHPDoc tag @phpstan-sealed contains non-object type {}.",
                    ty.describe(Level::TypeOnly)
                ),
                None,
            );
            continue;
        }
        for class in classes {
            if known(c, &class).is_none() {
                unknown(c, at, "@phpstan-sealed", &class);
                continue;
            }
            class_names::check(c, &[Pair::node(class, at)], c.case_sensitivity);
        }
    }
}

fn unknown(c: &mut Checker, at: Span, tag: &str, class: &str) {
    c.report(
        at,
        "class.notFound",
        format!("PHPDoc tag {tag} contains unknown class {class}."),
        Some(DISCOVERING_SYMBOLS_TIP),
    );
}

/// The rules on a trait declared at `at` (Stmt\Trait_).
pub fn trait_(c: &mut Checker, at: Span, name: &str) {
    if c.level < 2 {
        return;
    }
    let k = c
        .codebase
        .get_class_like(name.as_bytes())
        .expect("a declared trait is in the codebase");
    let extends = tag_types(c, k, EXTENDS);
    extends_check(c, at, &extends);
    for ty in tag_types(c, k, IMPLEMENTS) {
        let classes = ty.object_class_names();
        if classes.is_empty() {
            c.report(
                at,
                "requireImplements.nonObject",
                format!(
                    "PHPDoc tag @phpstan-require-implements contains non-object type {}.",
                    ty.describe(Level::TypeOnly)
                ),
                None,
            );
            continue;
        }
        for class in classes {
            let Some(reflection) = known(c, &class) else {
                unknown(c, at, "@phpstan-require-implements", &class);
                continue;
            };
            if reflection.is_interface() {
                class_names::check(c, &[Pair::node(class, at)], c.case_sensitivity);
            } else {
                c.report(
                    at,
                    &format!(
                        "requireImplements.{}",
                        reflection.type_description().to_ascii_lowercase()
                    ),
                    format!(
                        "PHPDoc tag @phpstan-require-implements cannot contain non-interface type {class}."
                    ),
                    None,
                );
            }
        }
    }
}

/// RequireExtendsCheck::checkExtendsTags()
fn extends_check(c: &mut Checker, at: Span, tags: &[Type]) {
    if tags.len() > 1 {
        c.report(
            at,
            "requireExtends.duplicate",
            "PHPDoc tag @phpstan-require-extends can only be used once.".to_string(),
            None,
        );
    }
    for ty in tags {
        let classes = ty.object_class_names();
        if classes.is_empty() {
            c.report(
                at,
                "requireExtends.nonObject",
                format!(
                    "PHPDoc tag @phpstan-require-extends contains non-object type {}.",
                    ty.describe(Level::TypeOnly)
                ),
                None,
            );
            continue;
        }
        for class in classes {
            let Some(reflection) = known(c, &class) else {
                unknown(c, at, "@phpstan-require-extends", &class);
                continue;
            };
            if reflection.is_interface() {
                c.report(
                    at,
                    "requireExtends.interface",
                    format!(
                        "PHPDoc tag @phpstan-require-extends cannot contain an interface {class}, expected a class."
                    ),
                    Some("If you meant an interface, use @phpstan-require-implements instead."),
                );
            } else if !reflection.is_class() {
                c.report(
                    at,
                    &format!(
                        "requireExtends.{}",
                        reflection.type_description().to_ascii_lowercase()
                    ),
                    format!(
                        "PHPDoc tag @phpstan-require-extends cannot contain non-class type {class}."
                    ),
                    None,
                );
            } else if reflection.is_final() {
                c.report(
                    at,
                    "requireExtends.finalClass",
                    format!(
                        "PHPDoc tag @phpstan-require-extends cannot contain final class {class}."
                    ),
                    None,
                );
            } else {
                class_names::check(c, &[Pair::node(class, at)], c.case_sensitivity);
            }
        }
    }
}
