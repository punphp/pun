//! AttributesCheck and the level-0 rules that call it (ClassAttributesRule, ClassConstantAttributesRule,
//! EnumCaseAttributesRule, FunctionAttributesRule, ClosureAttributesRule, ArrowFunctionAttributesRule,
//! ParamAttributesRule, MethodAttributesRule, PropertyAttributesRule, PropertyHookAttributesRule,
//! ConstantAttributesRule, TraitAttributesRule), with NonClassAttributeClassRule and TraitAttributeClassRule.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::expander::{TypeExpansionOptions, expand_union};
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::class_names::{self, Pair};
use super::names::{Known, known};
use super::{Checker, members, params};

pub const TARGET_CLASS: i64 = 1;
pub const TARGET_FUNCTION: i64 = 2;
pub const TARGET_METHOD: i64 = 4;
pub const TARGET_PROPERTY: i64 = 8;
pub const TARGET_CLASS_CONSTANT: i64 = 16;
pub const TARGET_PARAMETER: i64 = 32;
pub const TARGET_CONSTANT: i64 = 64;
const TARGET_ALL: i64 = 127;
const IS_REPEATABLE: i64 = 128;

/// `$attribute->name->toString()` after php-parser's NameResolver: resolved, except `self`, `parent` and `static`.
fn name<'arena>(c: &Checker<'_, 'arena>, a: &Attribute<'arena>) -> String {
    let written = c.written(a.name.span());
    if ["self", "parent", "static"]
        .iter()
        .any(|s| written.eq_ignore_ascii_case(s))
    {
        return written;
    }
    c.resolved(&a.name)
}

/// The attribute names written on a node.
fn names<'arena>(c: &Checker<'_, 'arena>, lists: &[AttributeList<'arena>]) -> Vec<String> {
    lists
        .iter()
        .flat_map(|l| l.attributes.iter())
        .map(|a| name(c, a))
        .collect()
}

/// `getNativeReflection()->getAttributes($name)` is not empty: the name is matched as written (case-sensitively).
fn has_native(c: &Checker, lists: &[AttributeList], name: &str) -> bool {
    names(c, lists).iter().any(|n| n == name)
}

/// AttributesCheck::check().
pub fn check<'arena>(
    c: &mut Checker<'_, 'arena>,
    lists: &[AttributeList<'arena>],
    required: i64,
    target: &str,
) {
    let mut present: Vec<String> = Vec::new();
    for attribute in lists.iter().flat_map(|l| l.attributes.iter()) {
        let name = name(c, attribute);
        let Some(k) = known(c, &name) else {
            c.report(
                attribute,
                "attribute.notFound",
                format!("Attribute class {name} does not exist."),
                None,
            );
            continue;
        };
        let class = match k {
            Known::User(k) => Some(k),
            Known::Builtin(b) => c.codebase.get_class_like(b.name.as_bytes()),
        };
        // a built-in class Mago's prelude lacks is no attribute class (PHP's attribute classes are all core)
        let (Some(class), Some(flags)) = (class, class.and_then(|k| attribute_flags(c, k))) else {
            c.report(
                attribute,
                "attribute.notAttribute",
                format!(
                    "{} {} is not an Attribute class.",
                    k.type_description(),
                    k.display()
                ),
                None,
            );
            continue;
        };
        if class.flags.is_abstract() {
            c.report(
                attribute,
                "attribute.abstract",
                format!("Attribute class {name} is abstract."),
                None,
            );
        }
        class_names::check(c, &[Pair::node(name.clone(), attribute.span())], true);
        if flags & required == 0 {
            c.report(
                attribute,
                "attribute.target",
                format!("Attribute class {name} does not have the {target} target."),
                None,
            );
        }
        if flags & IS_REPEATABLE == 0 {
            let lower = name.to_ascii_lowercase();
            if present.contains(&lower) {
                c.report(
                    attribute,
                    "attribute.nonRepeatable",
                    format!("Attribute class {name} is not repeatable but is already present above the {target}."),
                    None,
                );
            }
            present.push(lower);
        }
        // attribute.deprecated: only with phpstan-deprecation-rules installed
        let arguments: Vec<&PartialArgument> = attribute
            .argument_list
            .iter()
            .flat_map(|a| a.arguments.iter())
            .collect();
        let Some(constructor) = members::method(c.codebase, class, b"__construct") else {
            if !arguments.is_empty() {
                c.report(
                    attribute,
                    "attribute.noConstructor",
                    format!("Attribute class {name} does not have a constructor and must be instantiated without any parameters."),
                    None,
                );
            }
            continue;
        };
        if !constructor.visibility().is_public() {
            c.report(
                attribute,
                "attribute.constructorNotPublic",
                format!("Constructor of attribute class {name} is not public."),
                None,
            );
        }
        // PHP rejects unpacking in an attribute's arguments, so they count as written
        let positional = arguments
            .iter()
            .filter(|a| matches!(a, PartialArgument::Positional(_)))
            .count();
        let counts = (positional, positional < arguments.len(), false);
        params::check(
            c,
            &params::select(constructor.variants(), counts.0, false),
            counts,
            attribute,
            params::Call {
                kind: "attribute",
                subject: &format!("Attribute class {} constructor", k.display()),
                void_message: None,
            },
            false,
        );
    }
}

/// `findAttributeFlags()`: the flags of the class's one `#[Attribute]` (the constructor's first argument after
/// ArgumentsNormalizer, else its default `Attribute::TARGET_ALL`), `None` when it is not an attribute class.
fn attribute_flags(c: &Checker, class: &ClassLikeMetadata) -> Option<i64> {
    if class.kind != SymbolKind::Class {
        return None;
    }
    let attributes: Vec<_> = class
        .attributes
        .iter()
        .filter(|a| a.name.as_bytes() == b"Attribute")
        .collect();
    let [attribute] = attributes.as_slice() else {
        return None;
    };
    if attribute.arguments.is_empty() {
        return Some(TARGET_ALL);
    }
    let argument = attribute
        .arguments
        .iter()
        .find(|a| a.name.is_none())
        .or_else(|| {
            attribute
                .arguments
                .iter()
                .find(|a| a.name.is_some_and(|n| n.as_bytes() == b"flags"))
        })
        // ArgumentsNormalizer appends unknown named arguments
        .or_else(|| attribute.arguments.first())?;
    // the flags must be a ConstantIntegerType: Mago's scan type, with class and global constants resolved
    let mut t = argument.value_type.clone()?;
    expand_union(c.codebase, &mut t, &TypeExpansionOptions::default());
    t.get_single_literal_int_value()
}

/// The InClassNode rules for a class, interface or enum: NonClassAttributeClassRule, then ClassAttributesRule.
/// `at` is the declaration; `display` is `None` for an anonymous class.
pub fn class_like<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    lists: &[AttributeList<'arena>],
    kind: SymbolKind,
    readonly: bool,
    display: Option<&str>,
) {
    if let Some(display) = display {
        non_class_attribute_class(c, at, lists, kind, display);
    }
    let type_name = match kind {
        SymbolKind::Interface => "interface",
        SymbolKind::Enum => "enum",
        _ => "class",
    };
    if has_native(c, lists, "Deprecated") {
        // ponytail: an anonymous class's `class@anonymous` display name is not ported
        if let Some(display) = display {
            c.report_non_ignorable(
                at,
                &format!("{type_name}.deprecatedAttribute"),
                format!("Attribute class Deprecated cannot be used with {type_name} {display}."),
                None,
            );
        }
        return;
    }
    check(c, lists, TARGET_CLASS, "class");
    if (readonly || kind != SymbolKind::Class) && has_native(c, lists, "AllowDynamicProperties") {
        let (what, identifier) = match kind {
            SymbolKind::Interface => ("interface", "interface.allowDynamicProperties"),
            SymbolKind::Enum => ("enum", "enum.allowDynamicProperties"),
            _ => ("readonly class", "class.allowDynamicPropertiesReadonly"),
        };
        c.report_non_ignorable(
            at,
            identifier,
            format!("Attribute class AllowDynamicProperties cannot be used with {what}."),
            None,
        );
    }
}

/// NonClassAttributeClassRule.
fn non_class_attribute_class<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    lists: &[AttributeList<'arena>],
    kind: SymbolKind,
    display: &str,
) {
    if !names(c, lists)
        .iter()
        .any(|n| n.eq_ignore_ascii_case("attribute"))
    {
        return;
    }
    if kind != SymbolKind::Class {
        let description = if kind == SymbolKind::Interface {
            "Interface"
        } else {
            "Enum"
        };
        c.report(
            at,
            &format!("attribute.{}", description.to_ascii_lowercase()),
            format!("{description} cannot be an Attribute class."),
            None,
        );
        return;
    }
    if c.decls.last().is_some_and(|d| d.is_abstract) {
        c.report(
            at,
            "attribute.abstract",
            format!("Abstract class {display} cannot be an Attribute class."),
            None,
        );
        return;
    }
    let codebase = c.codebase;
    if codebase
        .get_class_like(display.as_bytes())
        .and_then(|k| members::method(codebase, k, b"__construct"))
        .is_some_and(|m| !m.visibility().is_public())
    {
        c.report(
            at,
            "attribute.constructorNotPublic",
            format!("Attribute class {display} constructor must be public."),
            None,
        );
    }
}

/// TraitAttributeClassRule (on the trait's declaration, outside any class's context).
pub fn trait_declaration<'arena>(c: &mut Checker<'_, 'arena>, t: &Trait<'arena>) {
    if names(c, t.attribute_lists.as_slice())
        .iter()
        .any(|n| n.eq_ignore_ascii_case("attribute"))
    {
        c.report(
            t,
            "attribute.trait",
            "Trait cannot be an Attribute class.".to_string(),
            None,
        );
    }
}

/// TraitAttributesRule (InTraitNode: the trait analysed in a using class's context).
pub fn trait_in_context<'arena>(c: &mut Checker<'_, 'arena>, t: &Trait<'arena>) {
    let lists = t.attribute_lists.as_slice();
    if c.php_version < 80500 && has_native(c, lists, "Deprecated") {
        c.report_non_ignorable(
            t,
            "trait.deprecatedAttribute",
            "Attribute class Deprecated can be used with traits only on PHP 8.5 and later."
                .to_string(),
            None,
        );
        return;
    }
    check(c, lists, TARGET_CLASS, "class");
    if has_native(c, lists, "AllowDynamicProperties") {
        c.report_non_ignorable(
            t,
            "trait.allowDynamicProperties",
            "Attribute class AllowDynamicProperties cannot be used with trait.".to_string(),
            None,
        );
    }
}

/// PropertyAttributesRule for one ClassPropertyNode at `at` (a declared property's variable, or a promoted
/// parameter).
pub fn property<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    lists: &[AttributeList<'arena>],
    promoted: bool,
) {
    if c.php_version < 80500 && has_native(c, lists, "Override") {
        c.report_non_ignorable(
            at,
            "property.overrideAttribute",
            "Attribute class Override can be used with properties only on PHP 8.5 and later."
                .to_string(),
            None,
        );
        return;
    }
    if !promoted {
        check(c, lists, TARGET_PROPERTY, "property");
    }
}

/// ParamAttributesRule.
pub fn parameter<'arena>(c: &mut Checker<'_, 'arena>, p: &FunctionLikeParameter<'arena>) {
    let (required, target) = if p.modifiers.is_empty() {
        (TARGET_PARAMETER, "parameter")
    } else {
        (TARGET_PARAMETER | TARGET_PROPERTY, "parameter or property")
    };
    check(c, p.attribute_lists.as_slice(), required, target);
}

/// PropertyHookAttributesRule.
pub fn property_hook<'arena>(c: &mut Checker<'_, 'arena>, h: &PropertyHook<'arena>) {
    let lists = h.attribute_lists.as_slice();
    check(c, lists, TARGET_METHOD, "method");
    if let Some(attribute) = lists
        .iter()
        .flat_map(|l| l.attributes.iter())
        .find(|a| name(c, a).eq_ignore_ascii_case("nodiscard"))
    {
        let name = name(c, attribute);
        c.report_non_ignorable(
            attribute,
            "attribute.target",
            format!("Attribute class {name} cannot be used on property hooks."),
            None,
        );
    }
}

/// ConstantAttributesRule (Pun runs as PHPStan does on a PHP 8.5 runtime, so constant.attributesRuleCannotRun
/// never applies).
pub fn constant<'arena>(c: &mut Checker<'_, 'arena>, k: &Constant<'arena>) {
    let Some(first) = k.attribute_lists.first() else {
        return;
    };
    if c.php_version < 80500 {
        c.report_non_ignorable(
            first.span().join(k.span()),
            "constant.attributesNotSupported",
            "Attributes on global constants are supported only on PHP 8.5 and later.".to_string(),
            None,
        );
        return;
    }
    check(c, k.attribute_lists.as_slice(), TARGET_CONSTANT, "constant");
}
