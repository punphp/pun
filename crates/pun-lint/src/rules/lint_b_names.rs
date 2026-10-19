//! Name resolution for lint-b's ports of Mago rules, standing in for Mago's `ResolvedNames` (mago-names 1.57.0)
//! and its linter's `function_call_matches_any` (rule/utils/call.rs) until pun-lint has a name resolver of its own.
//!
//! A name is resolved against the namespace that holds it and the `use` imports of that namespace that come before
//! it, as mago-names does while it walks the file.

use mago_span::HasSpan;
use mago_syntax::cst::{
    Expression, FunctionCall, Identifier, NamespaceBody, Node, Statement, UseItem, UseItems,
};

/// What `use` brings in: a class-like name, a function or a constant.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Class,
    Function,
    Const,
}

/// The namespace of the node whose ancestors are given (empty outside one), and the imports before `offset`.
struct Scope {
    namespace: Vec<u8>,
    /// (kind, the name it is known by, the fully qualified name without a leading `\`).
    imports: Vec<(Kind, Vec<u8>, Vec<u8>)>,
}

fn scope(ancestors: &[Node<'_, '_>], offset: u32) -> Scope {
    let mut namespace = Vec::new();
    let mut statements: Vec<&Statement<'_>> = Vec::new();
    match ancestors
        .iter()
        .rev()
        .find(|n| matches!(n, Node::Namespace(_)))
    {
        Some(Node::Namespace(ns)) => {
            if let Some(name) = &ns.name {
                namespace = name.value().to_vec();
            }
            match &ns.body {
                NamespaceBody::Implicit(body) => statements.extend(body.statements.iter()),
                NamespaceBody::BraceDelimited(block) => statements.extend(block.statements.iter()),
            }
        }
        _ => {
            if let Some(Node::Program(program)) = ancestors.first() {
                statements.extend(program.statements.iter());
            }
        }
    }
    let mut imports = Vec::new();
    for statement in statements {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        if r#use.span().start.offset > offset {
            break;
        }
        let mut add = |kind: Kind, prefix: &[u8], item: &UseItem<'_>| {
            let mut fqn = prefix.to_vec();
            if !fqn.is_empty() {
                fqn.push(b'\\');
            }
            fqn.extend_from_slice(
                item.name
                    .value()
                    .strip_prefix(b"\\")
                    .unwrap_or(item.name.value()),
            );
            let alias = match &item.alias {
                Some(alias) => alias.identifier.value.to_vec(),
                None => item.name.last_segment().to_vec(),
            };
            imports.push((kind, alias, fqn));
        };
        let kind_of = |t: &mago_syntax::cst::UseType<'_>| {
            if t.is_function() {
                Kind::Function
            } else {
                Kind::Const
            }
        };
        match &r#use.items {
            UseItems::Sequence(s) => s.items.iter().for_each(|i| add(Kind::Class, b"", i)),
            UseItems::TypedSequence(s) => {
                let kind = kind_of(&s.r#type);
                s.items.iter().for_each(|i| add(kind, b"", i));
            }
            UseItems::TypedList(l) => {
                let kind = kind_of(&l.r#type);
                let prefix = l.namespace.value();
                let prefix = prefix.strip_prefix(b"\\").unwrap_or(prefix);
                l.items.iter().for_each(|i| add(kind, prefix, i));
            }
            UseItems::MixedList(l) => {
                let prefix = l.namespace.value();
                let prefix = prefix.strip_prefix(b"\\").unwrap_or(prefix);
                for i in l.items.iter() {
                    let kind = i.r#type.as_ref().map_or(Kind::Class, kind_of);
                    add(kind, prefix, &i.item);
                }
            }
        }
    }
    Scope { namespace, imports }
}

impl Scope {
    /// The fully qualified name an alias stands for: the last `use` of that alias wins, ignoring ASCII case.
    fn import(&self, kind: Kind, name: &[u8]) -> Option<&[u8]> {
        self.imports
            .iter()
            .rev()
            .find(|(k, alias, _)| *k == kind && alias.eq_ignore_ascii_case(name))
            .map(|(_, _, fqn)| fqn.as_slice())
    }

    /// mago-names' `NamespaceScope::resolve`: the resolved name, and whether an explicit rule (a leading `\`,
    /// `namespace\` or an import) resolved it rather than the namespace prefix. (Its rule for `clone`, `exit` and
    /// `die` as functions never applies here: Mago parses those as constructs, not function calls.)
    fn resolve(&self, kind: Kind, name: &[u8]) -> (Vec<u8>, bool) {
        if let Some(fqn) = name.strip_prefix(b"\\") {
            return (fqn.to_vec(), true);
        }
        let (first, suffix) = match name.iter().position(|&b| b == b'\\') {
            Some(i) => (&name[..i], Some(&name[i + 1..])),
            None => (name, None),
        };
        if let Some(suffix) = suffix
            && first.eq_ignore_ascii_case(b"namespace")
        {
            return (self.prefixed(suffix), true);
        }
        let alias_kind = if suffix.is_some() { Kind::Class } else { kind };
        if let Some(fqn) = self.import(alias_kind, first) {
            return match suffix {
                Some(suffix) => ([fqn, &b"\\"[..], suffix].concat(), true),
                None => (fqn.to_vec(), true),
            };
        }
        (self.prefixed(name), false)
    }

    fn prefixed(&self, name: &[u8]) -> Vec<u8> {
        if self.namespace.is_empty() {
            return name.to_vec();
        }
        let mut out = self.namespace.clone();
        out.push(b'\\');
        out.extend_from_slice(name);
        out
    }
}

/// The fully qualified class-like name `identifier` stands for, without a leading `\` (mago-names' resolution of
/// a class-like name).
pub fn resolve_class(ancestors: &[Node<'_, '_>], identifier: &Identifier<'_>) -> Vec<u8> {
    scope(ancestors, identifier.span().start.offset)
        .resolve(Kind::Class, identifier.value())
        .0
}

/// `function_call_matches_any`: the first of `names` (compared ignoring ASCII case) that the called function may
/// be. A name resolved by an explicit rule is compared by its resolved name; any other by itself as written (PHP's
/// global fallback) and, inside a namespace, by its namespaced name.
pub fn function_call_matches_any<'n>(
    ancestors: &[Node<'_, '_>],
    call: &FunctionCall<'_>,
    names: &[&'n str],
) -> Option<&'n str> {
    let Expression::Identifier(identifier) = call.function else {
        return None;
    };
    let find = |candidate: &[u8]| {
        names
            .iter()
            .find(|n| candidate.eq_ignore_ascii_case(n.as_bytes()))
            .copied()
    };
    let scope = scope(ancestors, identifier.span().start.offset);
    let (resolved, imported) = scope.resolve(Kind::Function, identifier.value());
    if imported {
        return find(&resolved);
    }
    find(identifier.value()).or_else(|| find(&resolved))
}

/// `function_call_matches` for one name.
pub fn function_call_matches(
    ancestors: &[Node<'_, '_>],
    call: &FunctionCall<'_>,
    name: &str,
) -> bool {
    function_call_matches_any(ancestors, call, &[name]).is_some()
}
