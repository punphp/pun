//! php-parser's `NameResolver` for class names: a name as written, resolved through the `use` imports and the
//! namespace of the place it is written.

use mago_syntax::cst::{Identifier, Namespace, NamespaceBody, Node, Statement, UseItem, UseItems};

pub(crate) use crate::rules::types::names::resolve_class_name;
use crate::rules::types::names::scope;

/// PHPStan's `NameScope::resolveStringName` for a class name `written` in a doc comment at the node `ancestors` lead
/// to, in the name scope Rector makes (`NameScopeFactory`): the namespace, and every import of the file by its alias
/// in lowercase (the last import of an alias wins). A name whose first part is an alias goes through that import,
/// else it is in the namespace.
pub(crate) fn resolve_doc_string_name(ancestors: &[Node<'_, '_>], written: &str) -> String {
    if written.starts_with('\\') {
        return written.trim_start_matches('\\').to_string();
    }
    let (first, rest) = match written.split_once('\\') {
        Some((first, rest)) => (first, Some(rest)),
        None => (written, None),
    };
    let uses = file_uses(ancestors);
    let alias_of = |(name, alias): &&(String, Option<String>)| {
        let last = name.rsplit('\\').next().unwrap_or(name);
        alias.as_deref().unwrap_or(last).eq_ignore_ascii_case(first)
    };
    if let Some((name, _)) = uses.iter().rev().find(alias_of) {
        return rest.map_or(name.clone(), |rest| format!("{name}\\{rest}"));
    }
    match scope(ancestors).0 {
        Some(ns) => format!("{}\\{written}", String::from_utf8_lossy(ns)),
        None => written.to_string(),
    }
}

/// Rector's `ObjectTypeSpecifier::narrowToFullyQualifiedOrAliasedObjectType` for a class name `written` in a doc comment
/// at the node `ancestors` lead to: the name the type compares by. An alias, then the last part of a `use` without
/// one, must match exactly; then the name itself if it is a class (`exists`, `ReflectionProvider::hasClass`), else
/// the name in the namespace.
pub(crate) fn resolve_doc_class_name(
    ancestors: &[Node<'_, '_>],
    written: &str,
    exists: impl Fn(&str) -> bool,
) -> String {
    if let Some(fq) = written.strip_prefix('\\') {
        return fq.to_string();
    }
    let uses = file_uses(ancestors);
    // `matchAliasedObjectType`
    for (name, alias) in &uses {
        let Some(alias) = alias else { continue };
        if alias == written || name == written {
            return name.clone();
        }
        if let Some(rest) = written
            .strip_prefix(alias.as_str())
            .filter(|r| r.starts_with('\\'))
        {
            return format!("{name}{rest}");
        }
    }
    // `matchShortenedObjectType`
    for (name, _) in uses.iter().filter(|(_, alias)| alias.is_none()) {
        let last = name.rsplit('\\').next().unwrap_or(name);
        if written == last {
            return name.clone();
        }
        if let Some((_, after)) = written
            .strip_prefix(last)
            .filter(|r| r.starts_with('\\'))
            .and(written.split_once('\\'))
        {
            let connected = format!("{name}\\{after}");
            if connected != written {
                return connected;
            }
        }
    }
    if exists(written) {
        return written.to_string();
    }
    match scope(ancestors).0 {
        Some(ns) => {
            let full = format!("{}\\{written}", String::from_utf8_lossy(ns));
            if exists(&full) || !written.contains('\\') {
                full
            } else {
                written.to_string()
            }
        }
        None => written.to_string(),
    }
}

/// `FileNode::getUsesAndGroupUses()`: the imports (functions' and constants' too) of the file's only namespace, or of
/// the file outside namespaces, in order: each name with its group's prefix, and its alias.
fn file_uses(ancestors: &[Node<'_, '_>]) -> Vec<(String, Option<String>)> {
    let text = |id: &Identifier<'_>| {
        String::from_utf8_lossy(id.value())
            .trim_start_matches('\\')
            .to_string()
    };
    let mut uses = Vec::new();
    for node in ancestors.iter().take(1) {
        let Node::Program(p) = node else { continue };
        let namespaces: Vec<&Namespace> = (p.statements.iter())
            .filter_map(|s| match s {
                Statement::Namespace(ns) => Some(ns),
                _ => None,
            })
            .collect();
        let statements: Vec<&Statement<'_>> = match namespaces[..] {
            [ns] => match &ns.body {
                NamespaceBody::Implicit(body) => body.statements.iter().collect(),
                NamespaceBody::BraceDelimited(block) => block.statements.iter().collect(),
            },
            _ => p.statements.iter().collect(),
        };
        for statement in statements {
            let Statement::Use(r#use) = statement else {
                continue;
            };
            let (prefix, items): (String, Vec<&UseItem<'_>>) = match &r#use.items {
                UseItems::Sequence(s) => (String::new(), s.items.iter().collect()),
                UseItems::TypedSequence(s) => (String::new(), s.items.iter().collect()),
                UseItems::TypedList(l) => (
                    format!("{}\\", text(&l.namespace)),
                    l.items.iter().collect(),
                ),
                UseItems::MixedList(l) => (
                    format!("{}\\", text(&l.namespace)),
                    l.items.iter().map(|i| &i.item).collect(),
                ),
            };
            for item in items {
                let alias = item
                    .alias
                    .as_ref()
                    .map(|a| String::from_utf8_lossy(a.identifier.value).into_owned());
                uses.push((format!("{prefix}{}", text(&item.name)), alias));
            }
        }
    }
    uses
}
