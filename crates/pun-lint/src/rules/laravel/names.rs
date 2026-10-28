//! php-parser's `NameResolver` for class names: a name as written, resolved through the `use` imports and the
//! namespace of the place it is written.

use mago_syntax::cst::{Identifier, Namespace, NamespaceBody, Node, Statement, UseItem, UseItems};

/// The namespace (`None` for the global one) and the statements of the scope `ancestors` lead to: the nearest
/// `namespace` or the file.
fn scope<'a>(ancestors: &[Node<'a, 'a>]) -> (Option<&'a [u8]>, Vec<&'a Statement<'a>>) {
    for node in ancestors.iter().rev() {
        if let Node::Namespace(ns) = node {
            let statements = match &ns.body {
                NamespaceBody::Implicit(body) => body.statements.iter().collect(),
                NamespaceBody::BraceDelimited(block) => block.statements.iter().collect(),
            };
            return (ns.name.as_ref().map(|n| n.value()), statements);
        }
    }
    match ancestors.first() {
        Some(Node::Program(program)) => (None, program.statements.iter().collect()),
        _ => (None, Vec::new()),
    }
}

/// The fully qualified class name (without the leading `\`) that `name`, written at the node `ancestors` lead to,
/// stands for.
pub(crate) fn resolve_class_name(ancestors: &[Node<'_, '_>], name: &Identifier<'_>) -> String {
    resolve_name(ancestors, &String::from_utf8_lossy(name.value()))
}

/// [`resolve_class_name`] for a name `written` as text (PHPStan's `NameScope::resolveStringName` does the same).
pub(crate) fn resolve_name(ancestors: &[Node<'_, '_>], written: &str) -> String {
    let written = written.to_string();
    if let Some(fq) = written.strip_prefix('\\') {
        return fq.to_string();
    }
    let (namespace, statements) = scope(ancestors);
    let (first, rest) = match written.split_once('\\') {
        Some((first, rest)) => (first, Some(rest)),
        None => (written.as_str(), None),
    };
    for statement in statements {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        let items: Vec<(Option<&[u8]>, &UseItem)> = match &r#use.items {
            UseItems::Sequence(s) => s.items.iter().map(|i| (None, i)).collect(),
            UseItems::MixedList(l) => l
                .items
                .iter()
                .filter(|i| i.r#type.is_none())
                .map(|i| (Some(l.namespace.value()), &i.item))
                .collect(),
            UseItems::TypedSequence(_) | UseItems::TypedList(_) => continue,
        };
        for (prefix, item) in items {
            let name = String::from_utf8_lossy(item.name.value());
            let full = match prefix {
                Some(prefix) => format!("{}\\{name}", String::from_utf8_lossy(prefix)),
                None => name.into_owned(),
            };
            let full = full.trim_start_matches('\\');
            let alias = match &item.alias {
                Some(alias) => String::from_utf8_lossy(alias.identifier.value).into_owned(),
                None => full.rsplit('\\').next().unwrap_or(full).to_string(),
            };
            if alias.eq_ignore_ascii_case(first) {
                return match rest {
                    Some(rest) => format!("{full}\\{rest}"),
                    None => full.to_string(),
                };
            }
        }
    }
    match namespace {
        Some(ns) => format!("{}\\{written}", String::from_utf8_lossy(ns)),
        None => written,
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
        if let Some(rest) = written.strip_prefix(alias.as_str()).filter(|r| r.starts_with('\\')) {
            return format!("{name}{rest}");
        }
    }
    // `matchShortenedObjectType`
    for (name, _) in uses.iter().filter(|(_, alias)| alias.is_none()) {
        let last = name.rsplit('\\').next().unwrap_or(name);
        if written == last {
            return name.clone();
        }
        if let Some((_, after)) = written.strip_prefix(last).filter(|r| r.starts_with('\\')).and(written.split_once('\\')) {
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
            if exists(&full) || !written.contains('\\') { full } else { written.to_string() }
        }
        None => written.to_string(),
    }
}

/// `FileNode::getUsesAndGroupUses()`: the imports (functions' and constants' too) of the file's only namespace, or of
/// the file outside namespaces, in order: each name with its group's prefix, and its alias.
fn file_uses(ancestors: &[Node<'_, '_>]) -> Vec<(String, Option<String>)> {
    let Some(Node::Program(program)) = ancestors.first() else {
        return Vec::new();
    };
    let namespaces: Vec<&Namespace> = (program.statements.iter())
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
        _ => program.statements.iter().collect(),
    };
    let text = |id: &Identifier<'_>| String::from_utf8_lossy(id.value()).trim_start_matches('\\').to_string();
    let mut uses = Vec::new();
    for statement in statements {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        let (prefix, items): (String, Vec<&UseItem<'_>>) = match &r#use.items {
            UseItems::Sequence(s) => (String::new(), s.items.iter().collect()),
            UseItems::TypedSequence(s) => (String::new(), s.items.iter().collect()),
            UseItems::TypedList(l) => (format!("{}\\", text(&l.namespace)), l.items.iter().collect()),
            UseItems::MixedList(l) => (format!("{}\\", text(&l.namespace)), l.items.iter().map(|i| &i.item).collect()),
        };
        for item in items {
            let alias = item.alias.as_ref().map(|a| String::from_utf8_lossy(a.identifier.value).into_owned());
            uses.push((format!("{prefix}{}", text(&item.name)), alias));
        }
    }
    uses
}
