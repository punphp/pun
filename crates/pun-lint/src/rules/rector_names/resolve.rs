//! How Rector resolves a class name written in a doc comment: `IdentifierPhpDocTypeMapper::mapIdentifierTypeNode()`
//! with `ObjectTypeSpecifier`, `ClassAnnotationMatcher` with `UseImportNameMatcher`, and PHPStan's
//! `NameScope::resolveStringName()`, over the imports `FileNode::getUsesAndGroupUses()` gives.

use crate::Context;
use crate::rules::types::facts::facts;

use super::view::UseView;

/// Where a doc comment is: its namespace, the class-like it is in, and the file's imports as Rector reads them.
#[derive(Clone, Copy)]
pub(crate) struct Scope<'a> {
    pub namespace: Option<&'a str>,
    pub class: Option<&'a str>,
    pub uses: &'a [UseView],
}

/// The type `IdentifierPhpDocTypeMapper` maps a doc identifier to, as far as the name handling tells them apart.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DocType {
    /// No object type: a keyword type, `self`, `static`, `parent`, or a class template.
    Other,
    /// `FullyQualifiedObjectType`.
    FullyQualified(String),
    /// `AliasedObjectType` (the alias as written, the class it imports).
    Aliased(String, String),
    /// `ShortenedObjectType` (the name as written, the class it stands for).
    Shortened(String, String),
    /// `NonExistingObjectType`.
    NonExisting(String),
}

impl DocType {
    /// The class name `ClassRenamePhpDocNodeVisitor::ensureFQCNObject()` compares, `None` for no object type.
    pub(crate) fn class_name(&self) -> Option<&str> {
        match self {
            DocType::Other => None,
            DocType::FullyQualified(name) | DocType::NonExisting(name) => Some(name),
            DocType::Aliased(_, fq) | DocType::Shortened(_, fq) => Some(fq),
        }
    }

    /// `NameImportingPhpDocNodeVisitor::resolveFullyQualified()`: the class of a fully qualified, shortened or
    /// aliased type.
    pub(crate) fn fully_qualified(&self) -> Option<&str> {
        match self {
            DocType::FullyQualified(name) => Some(name),
            DocType::Aliased(_, fq) | DocType::Shortened(_, fq) => Some(fq),
            _ => None,
        }
    }
}

/// `ScalarStringToTypeMapper::mapScalarStringToType()` gives a type other than a plain `MixedType`.
fn is_keyword(lowered: &str) -> bool {
    matches!(
        lowered,
        "false"
            | "true"
            | "positive-int"
            | "negative-int"
            | "non-negative-int"
            | "non-positive-int"
            | "array-key"
            | "callable-string"
            | "non-empty-lowercase-string"
            | "non-empty-uppercase-string"
            | "non-empty-literal-string"
            | "decimal-int-string"
            | "non-decimal-int-string"
            | "string"
            | "non-empty-string"
            | "literal-string"
            | "numeric-string"
            | "non-falsy-string"
            | "truthy-string"
            | "lowercase-string"
            | "uppercase-string"
            | "non-empty-array"
            | "class-string"
            | "float"
            | "real"
            | "double"
            | "int"
            | "integer"
            | "bool"
            | "boolean"
            | "null"
            | "void"
            | "resource"
            | "callback"
            | "callable"
            | "object"
            | "never"
            | "never-return"
            | "never-returns"
            | "no-return"
            | "list"
            | "array"
            | "iterable"
            | "mixed"
            | "self"
            | "parent"
            | "static"
            | "scalar"
    )
}

/// `ReflectionProvider::hasClass()`: a class, interface, trait or enum the codebase knows.
pub(crate) fn has_class(ctx: &Context<'_>, name: &str) -> bool {
    facts(ctx).codebase.class_like_exists(name.as_bytes())
}

/// `IdentifierPhpDocTypeMapper::mapIdentifierTypeNode()` of the doc identifier `name`.
pub(crate) fn map_identifier(ctx: &Context<'_>, scope: Scope<'_>, name: &str) -> DocType {
    if is_keyword(&name.to_ascii_lowercase()) {
        return DocType::Other;
    }
    // a `FullyQualifiedObjectType` without the `\`, narrowed `$withPreslash`
    match name.strip_prefix('\\') {
        Some(class_name) => narrow_from(ctx, scope, class_name, true),
        None => narrow(ctx, scope, name.trim_start_matches('@')),
    }
}

/// `ObjectTypeSpecifier::narrowToFullyQualifiedOrAliasedObjectType()` of `ObjectType($class_name)`, without a
/// leading `\`.
pub(crate) fn narrow(ctx: &Context<'_>, scope: Scope<'_>, class_name: &str) -> DocType {
    if let Some(fq) = class_name.strip_prefix('\\') {
        return DocType::FullyQualified(fq.to_string());
    }
    narrow_from(ctx, scope, class_name, false)
}

/// The rest of `narrowToFullyQualifiedOrAliasedObjectType()`, for a class name without a leading `\`; `preslash`
/// is `$withPreslash`, a doc identifier written with one.
fn narrow_from(ctx: &Context<'_>, scope: Scope<'_>, class_name: &str, preslash: bool) -> DocType {
    if !preslash {
        if let Some(aliased) = match_aliased(scope.uses, class_name) {
            return aliased;
        }
        if let Some(shortened) = match_shortened(scope.uses, class_name) {
            return shortened;
        }
    }
    // a class in the current namespace wins over a global class with the same short name
    let unqualified = !preslash && !class_name.contains('\\');
    if let Some(ns) = scope.namespace
        && unqualified
    {
        let namespaced = format!("{ns}\\{class_name}");
        if has_class(ctx, &namespaced) {
            return DocType::FullyQualified(namespaced);
        }
    }
    if has_class(ctx, class_name) {
        return DocType::FullyQualified(class_name.to_string());
    }
    if let Some(ns) = scope.namespace
        && !unqualified
    {
        let namespaced = format!("{ns}\\{class_name}");
        if has_class(ctx, &namespaced) {
            return DocType::FullyQualified(namespaced);
        }
    }
    if let Some(class) = scope.class
        && is_class_template(ctx, class, class_name)
    {
        return DocType::Other;
    }
    // `resolveNamespacedNonExistingObjectType()`
    match scope.namespace {
        Some(ns) if !preslash && !class_name.contains('\\') => {
            DocType::NonExisting(format!("{ns}\\{class_name}"))
        }
        _ => DocType::NonExisting(class_name.to_string()),
    }
}

/// Whether `ClassReflection::getTemplateTags()` of `class` has `name`.
fn is_class_template(ctx: &Context<'_>, class: &str, name: &str) -> bool {
    // the templates are read from the docblocks
    facts(ctx)
        .documented()
        .get_class_like(class.as_bytes())
        .is_some_and(|c| {
            c.template_types
                .iter()
                .any(|(k, _)| k.as_bytes() == name.as_bytes())
        })
}

/// `ObjectTypeSpecifier::matchAliasedObjectType()`.
fn match_aliased(uses: &[UseView], class_name: &str) -> Option<DocType> {
    for r#use in uses {
        let prefix = r#use.prefix();
        for item in &r#use.items {
            let Some(alias) = &item.alias else {
                continue;
            };
            let use_name = format!("{prefix}{}", item.name);
            // A. the alias, B. the imported class itself
            if alias == class_name || use_name == class_name {
                return Some(DocType::Aliased(class_name.to_string(), use_name));
            }
            // `$fullyQualifiedName . ltrim($className, $alias)`: the alias's characters stop at its `\`
            if let Some(rest) = class_name.strip_prefix(&format!("{alias}\\")) {
                return Some(DocType::Aliased(
                    class_name.to_string(),
                    format!("{use_name}\\{rest}"),
                ));
            }
        }
    }
    None
}

/// `ObjectTypeSpecifier::matchShortenedObjectType()` (its `matchClassWithLastUseImportPart()` never decides: an
/// import whose last part is the name was matched before).
fn match_shortened(uses: &[UseView], class_name: &str) -> Option<DocType> {
    for r#use in uses {
        let prefix = r#use.prefix();
        for item in r#use.items.iter().filter(|i| i.alias.is_none()) {
            // `matchPartialNamespaceObjectType()`
            let last = item.last();
            if class_name == last {
                return Some(DocType::Shortened(
                    class_name.to_string(),
                    format!("{prefix}{}", item.name),
                ));
            }
            if let Some(after) = class_name.strip_prefix(&format!("{last}\\")) {
                let connected = format!("{prefix}{}\\{after}", item.name);
                if connected != class_name {
                    return Some(DocType::Shortened(class_name.to_string(), connected));
                }
            }
        }
    }
    None
}

/// `UseImportNameMatcher::matchNameWithUses()`: the first import whose short name starts `tag`
/// (`#^short(\\[\w]+)?$#i`), the class the tag names through it.
pub(crate) fn match_name_with_uses(uses: &[UseView], tag: &str) -> Option<String> {
    for r#use in uses {
        let prefix = r#use.prefix();
        for item in &r#use.items {
            let short = item.alias_or_last();
            let matches = tag.len() >= short.len()
                && tag.is_char_boundary(short.len())
                && tag[..short.len()].eq_ignore_ascii_case(short)
                && match &tag[short.len()..] {
                    "" => true,
                    rest => rest.strip_prefix('\\').is_some_and(|w| {
                        !w.is_empty() && w.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    }),
                };
            if !matches {
                continue;
            }
            // `resolveName()`
            return Some(match &item.alias {
                None => {
                    let last = item.last();
                    match tag.strip_prefix(&format!("{last}\\")) {
                        Some(after) => format!("{prefix}{}\\{after}", item.name),
                        None => format!("{prefix}{}", item.name),
                    }
                }
                Some(alias) => {
                    let unaliased = tag.get(alias.len()..).unwrap_or_default();
                    match unaliased.starts_with('\\') {
                        true => format!("{prefix}{}{unaliased}", item.name),
                        false => format!("{prefix}{}\\{unaliased}", item.name),
                    }
                }
            });
        }
    }
    None
}

/// `ClassAnnotationMatcher::resolveTagFullyQualifiedName()` of a tag name (its `@` included or not).
pub(crate) fn resolve_tag(ctx: &Context<'_>, scope: Scope<'_>, tag: &str) -> String {
    let tag = tag.trim_start_matches('@');
    resolve_tag_class(ctx, scope, tag).unwrap_or_else(|| tag.to_string())
}

/// `ClassAnnotationMatcher::resolveFullyQualifiedClass()`.
fn resolve_tag_class(ctx: &Context<'_>, scope: Scope<'_>, tag: &str) -> Option<String> {
    if let Some(ns) = scope.namespace {
        let namespaced = format!("{ns}\\{tag}");
        if has_class(ctx, &namespaced) {
            return Some(namespaced);
        }
        if !tag.contains('\\') {
            // `resolveAsAliased()`
            for r#use in scope.uses {
                let prefix = r#use.prefix();
                for item in &r#use.items {
                    if item.alias.as_deref() == Some(tag) {
                        return Some(format!("{prefix}{}", item.name));
                    }
                }
            }
            return match_name_with_uses(scope.uses, tag);
        }
        // `isPreslashedExistingClass()` gives the tag itself, as `resolve_tag()` does without a match (no import
        // matches a name with a leading `\\`)
    }
    match_name_with_uses(scope.uses, tag)
}

/// PHPStan's `NameScope::resolveStringName()` with the file's imports (`NameScopeFactory`): a fully qualified name
/// without its `\`; else through the class import of its first part; else in the namespace.
pub(crate) fn resolve_string_name(scope: Scope<'_>, name: &str) -> String {
    if let Some(fq) = name.strip_prefix('\\') {
        return fq.to_string();
    }
    let (first, rest) = match name.split_once('\\') {
        Some((first, rest)) => (first, Some(rest)),
        None => (name, None),
    };
    let lowered = first.to_ascii_lowercase();
    // the last import of the alias wins, as in the factory's map
    let imported = scope
        .uses
        .iter()
        .flat_map(|u| u.items.iter().map(move |i| (u, i)))
        .rfind(|(_, i)| i.alias_or_last().to_ascii_lowercase() == lowered);
    if let Some((r#use, item)) = imported {
        let full = format!("{}{}", r#use.prefix(), item.name);
        return match rest {
            Some(rest) => format!("{full}\\{rest}"),
            None => full,
        };
    }
    match scope.namespace {
        Some(ns) => format!("{ns}\\{name}"),
        None => name.to_string(),
    }
}
