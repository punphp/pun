//! php-parser's `NameResolver` for class names: a name as written, resolved through the `use` imports and the
//! namespace of the place it is written.

use mago_syntax::cst::{Identifier, NamespaceBody, Node, Statement, UseItem, UseItems};

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
    resolve_written(ancestors, &String::from_utf8_lossy(name.value())).0
}

/// [`resolve_class_name`] for a name `written` as text (a docblock's), and whether a `use` import resolved it.
pub(crate) fn resolve_written(ancestors: &[Node<'_, '_>], written: &str) -> (String, bool) {
    if let Some(fq) = written.strip_prefix('\\') {
        return (fq.to_string(), false);
    }
    let (namespace, imports) = class_imports(ancestors);
    let (first, rest) = match written.split_once('\\') {
        Some((first, rest)) => (first, Some(rest)),
        None => (written, None),
    };
    for (full, alias) in &imports {
        let alias = alias
            .as_deref()
            .unwrap_or_else(|| full.rsplit('\\').next().unwrap_or(full));
        if alias.eq_ignore_ascii_case(first) {
            let resolved = match rest {
                Some(rest) => format!("{full}\\{rest}"),
                None => full.clone(),
            };
            return (resolved, true);
        }
    }
    let resolved = match namespace {
        Some(ns) => format!("{ns}\\{written}"),
        None => written.to_string(),
    };
    (resolved, false)
}

/// The namespace of the place `ancestors` lead to and the class imports of its scope, in order: each imported name,
/// fully qualified without the leading `\`, with its alias.
pub(crate) fn class_imports(
    ancestors: &[Node<'_, '_>],
) -> (Option<String>, Vec<(String, Option<String>)>) {
    let (namespace, statements) = scope(ancestors);
    let mut imports = Vec::new();
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
            let alias = (item.alias.as_ref())
                .map(|a| String::from_utf8_lossy(a.identifier.value).into_owned());
            imports.push((full.trim_start_matches('\\').to_string(), alias));
        }
    }
    (
        namespace.map(|n| String::from_utf8_lossy(n).into_owned()),
        imports,
    )
}

/// [`resolve_class_name`] of a name as written (in a doc comment too).
pub(crate) fn resolve_written_class_name(ancestors: &[Node<'_, '_>], written: &str) -> String {
    resolve_written(ancestors, written).0
}
