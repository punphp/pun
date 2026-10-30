//! RequireExtendsRule and RequireImplementsRule (InClassNode, level 2): an interface's or used trait's
//! `@phpstan-require-extends` (or `@psalm-`) class the class does not extend, or a used trait's
//! `@phpstan-require-implements` interface it does not implement.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::Span;
use pun_phpstan::types::{Level, Type};

use super::Checker;
use super::declarations::doc_before;

/// Both rules on class-like `k` (a class, an enum or an anonymous class) declared at `at`.
pub fn check(c: &mut Checker, at: Span, k: &ClassLikeMetadata) {
    if c.level < 2 {
        return;
    }
    let class = c.display(k);
    let mut errors: Vec<(&str, String)> = Vec::new();
    // getInterfaces()
    for i in k.all_parent_interfaces.iter().filter_map(|i| c.codebase.get_class_like(i.as_bytes())) {
        for t in tags(c, i, "require-extends") {
            if object_class_names(&t).iter().any(|n| !is(k, n)) {
                let message = format!(
                    "Interface {} requires implementing class to extend {}, but {class} does not.",
                    c.display(i),
                    t.describe(Level::TypeOnly)
                );
                errors.push(("class.missingExtends", message));
            }
        }
    }
    let traits = traits(c, k);
    for t in &traits {
        for ty in tags(c, t, "require-extends") {
            if object_class_names(&ty).iter().any(|n| !is(k, n)) {
                let message = format!(
                    "Trait {} requires using class to extend {}, but {class} does not.",
                    c.display(t),
                    ty.describe(Level::TypeOnly)
                );
                errors.push(("class.missingExtends", message));
            }
        }
    }
    for t in &traits {
        for ty in tags(c, t, "require-implements") {
            // `instanceof ObjectType`, generic or not
            if let Type::Object(name, _, _) = &ty
                && !k.all_parent_interfaces.iter().any(|i| i.as_bytes().eq_ignore_ascii_case(name.as_bytes()))
            {
                let message = format!(
                    "Trait {} requires using class to implement {}, but {class} does not.",
                    c.display(t),
                    ty.describe(Level::TypeOnly)
                );
                errors.push(("class.missingImplements", message));
            }
        }
    }
    for (identifier, message) in errors {
        c.report(at, identifier, message, None);
    }
}

/// `getRequireExtendsTags()` / `getRequireImplementsTags()`: the `@psalm-` then the `@phpstan-` tags of `k`'s own
/// PHPDoc whose type parses, resolved in its name scope.
fn tags(c: &Checker, k: &ClassLikeMetadata, kind: &str) -> Vec<Type> {
    let Some(contents) = c.file_contents(k.span.file_id) else {
        return Vec::new();
    };
    let Some(doc) = doc_before(&contents, k.span.start.offset as usize) else {
        return Vec::new();
    };
    let tags = super::doc::tags(doc);
    let scope = super::declared::class_scope(c, k.span, k);
    ["@psalm-", "@phpstan-"]
        .iter()
        .flat_map(|prefix| {
            let name = format!("{prefix}{kind}");
            tags.iter()
                .filter(move |(t, _)| *t == name)
                .filter_map(|(_, v)| pun_phpstan::phpdoc::parse_prefix(v).ok())
                .collect::<Vec<_>>()
        })
        .map(|(node, _)| pun_phpstan::resolve::resolve_in(&node, &scope))
        .collect()
}

/// `getObjectClassNames()`.
fn object_class_names(t: &Type) -> Vec<String> {
    match t {
        Type::Object(name, _, _) => vec![name.clone()],
        Type::Union(members) => members.iter().flat_map(object_class_names).collect(),
        _ => Vec::new(),
    }
}

/// `ClassReflection::is()`: the class itself or a subclass (parents and interfaces).
fn is(k: &ClassLikeMetadata, name: &str) -> bool {
    let name = name.as_bytes();
    k.name.as_bytes().eq_ignore_ascii_case(name)
        || k.all_parent_classes.iter().any(|p| p.as_bytes().eq_ignore_ascii_case(name))
        || k.all_parent_interfaces.iter().any(|p| p.as_bytes().eq_ignore_ascii_case(name))
}

/// `getTraits(true)`: the traits of the class, of its traits and of its parents, each once.
fn traits<'a>(c: &Checker<'a, '_>, k: &'a ClassLikeMetadata) -> Vec<&'a ClassLikeMetadata> {
    let mut out: Vec<&ClassLikeMetadata> = Vec::new();
    let mut pending = vec![k];
    // a circular hierarchy (which PHP refuses) ends: each class-like is pending at most once
    let mut seen = vec![k.name];
    while let Some(k) = pending.pop() {
        let next = k.used_traits.iter().chain(k.direct_parent_class.iter());
        for n in next.filter_map(|n| c.codebase.get_class_like(n.as_bytes())) {
            if !seen.contains(&n.name) {
                seen.push(n.name);
                pending.push(n);
                if n.kind == SymbolKind::Trait {
                    out.push(n);
                }
            }
        }
    }
    out
}
