//! `ClassReflection::getMethod()`: a method by name along the class's own methods, its traits (with `as` aliases),
//! its parents and its interfaces. User code comes from Mago's codex; built-in classes from PHPStan's reflection
//! (data/symbols.json), whose class entries list only the methods each class declares.

use std::collections::HashSet;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::visibility::Visibility;
use mago_word::word;
use pun_phpstan::symbols;

use super::Checker;
use super::params::Variants;

pub enum Method<'a> {
    User {
        declaring: &'a ClassLikeMetadata,
        f: &'a FunctionLikeMetadata,
    },
    Builtin {
        declaring: &'static symbols::Class,
        f: &'static symbols::Function,
    },
}

impl Method<'_> {
    pub fn declaring_display(&self, c: &Checker) -> String {
        match self {
            Method::User { declaring, .. } => c.display(declaring),
            Method::Builtin { declaring, .. } => declaring.name.clone(),
        }
    }

    /// The declaring class's name (an anonymous class's codex name), for visibility checks.
    pub fn declaring_name(&self) -> String {
        match self {
            Method::User { declaring, .. } => {
                String::from_utf8_lossy(declaring.original_name.as_bytes()).into_owned()
            }
            Method::Builtin { declaring, .. } => declaring.name.clone(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Method::User { f, .. } => {
                String::from_utf8_lossy(f.original_name.as_bytes()).into_owned()
            }
            Method::Builtin { f, .. } => f.name.clone(),
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            Method::User { f, .. } => f.method_metadata.as_ref().is_some_and(|m| m.is_static),
            Method::Builtin { f, .. } => f.is_static,
        }
    }

    pub fn is_abstract(&self) -> bool {
        match self {
            Method::User { declaring, f } => {
                f.method_metadata.as_ref().is_some_and(|m| m.is_abstract)
                    || declaring.kind.is_interface()
            }
            Method::Builtin { .. } => false,
        }
    }

    pub fn is_final(&self) -> bool {
        match self {
            Method::User { f, .. } => f.method_metadata.as_ref().is_some_and(|m| m.is_final),
            Method::Builtin { .. } => false,
        }
    }

    pub fn visibility(&self) -> Visibility {
        match self {
            Method::User { f, .. } => f
                .method_metadata
                .as_ref()
                .map_or(Visibility::Public, |m| m.visibility),
            Method::Builtin { .. } => Visibility::Public,
        }
    }

    pub fn declared_in_interface(&self) -> bool {
        match self {
            Method::User { declaring, .. } => declaring.kind.is_interface(),
            Method::Builtin { declaring, .. } => declaring.is_interface,
        }
    }

    pub fn variants(&self) -> Variants {
        match self {
            Method::User { f, declaring } => {
                let mut v = Variants::of(f);
                for a in v.all.iter_mut() {
                    let class = String::from_utf8_lossy(declaring.name.as_bytes()).into_owned();
                    a.callee = a.callee.take().map(|(_, name)| (Some(class), name));
                }
                v
            }
            Method::Builtin { f, declaring } => {
                let mut v = Variants::of_builtin(f);
                for a in v.all.iter_mut().chain(v.named.iter_mut().flatten()) {
                    a.class = Some(declaring.name.clone());
                }
                v
            }
        }
    }
}

/// `$class->getMethod($name)`, `None` when `hasMethod()` is false (magic methods aside).
pub fn method<'a>(
    c: &Checker<'a, '_>,
    class: &'a ClassLikeMetadata,
    name: &[u8],
) -> Option<Method<'a>> {
    let codebase = c.codebase;
    let lower = word(name.to_ascii_lowercase());
    let mut current = Some(class);
    let mut seen = HashSet::new();
    while let Some(k) = current
        && seen.insert(k.name)
    {
        if let Some(m) = declared(c, k, &lower) {
            return Some(m);
        }
        current = k
            .direct_parent_class
            .and_then(|p| codebase.get_class_like(p.as_bytes()));
    }
    std::iter::once(&class.name)
        .chain(class.all_parent_interfaces.iter())
        .filter_map(|i| codebase.get_class_like(i.as_bytes()))
        .filter(|i| i.kind.is_interface())
        .find_map(|i| declared(c, i, &lower))
}

/// AnnotationsMethodsClassReflectionExtension: a `@method` tag of the class, its traits, its parents or its
/// interfaces, which `getMethod()` falls back to when the class has no such native method.
pub fn annotation_method<'a>(
    c: &Checker<'a, '_>,
    class: &'a ClassLikeMetadata,
    name: &[u8],
) -> Option<Method<'a>> {
    let lower = word(name.to_ascii_lowercase());
    let codebase = c.codebase;
    let mut pending = vec![(class, class)];
    let mut seen = HashSet::new();
    while let Some((k, declaring)) = pending.pop() {
        if !seen.insert(k.name) {
            continue;
        }
        if (k.pseudo_methods.contains(&lower) || k.static_pseudo_methods.contains(&lower))
            && let Some(f) = codebase
                .function_likes
                .get(&(k.name, lower))
                .map(Box::as_ref)
        {
            return Some(Method::User { declaring, f });
        }
        // findClassReflectionWithMethod(): traits first, then the parent, then interfaces, depth first.
        let start = pending.len();
        let get = |n: &mago_word::Word| codebase.get_class_like(n.as_bytes());
        pending.extend(
            k.direct_trait_names()
                .filter_map(|t| get(&t))
                .map(|t| (t, k)),
        );
        if let Some(p) = k.direct_parent_class.as_ref().and_then(get) {
            pending.push((p, p));
        }
        pending.extend(
            k.all_parent_interfaces
                .iter()
                .filter_map(get)
                .map(|i| (i, i)),
        );
        pending[start..].reverse();
    }
    None
}

/// A method `k` itself declares or takes from a trait (private ones of parents count, as in PHP's reflection).
fn declared<'a>(
    c: &Checker<'a, '_>,
    k: &'a ClassLikeMetadata,
    lower: &mago_word::Word,
) -> Option<Method<'a>> {
    let codebase = c.codebase;
    let own = |k: &'a ClassLikeMetadata| -> Option<Method<'a>> {
        if k.flags.is_built_in() {
            let class = c
                .builtins
                .classes
                .get(&String::from_utf8_lossy(k.name.as_bytes()).to_ascii_lowercase())?;
            let f = class
                .methods
                .get(String::from_utf8_lossy(lower.as_bytes()).as_ref())?;
            return Some(Method::Builtin {
                declaring: class,
                f,
            });
        }
        // after population `methods` (and get_method()) also cover inherited and trait methods: `k`'s own are the
        // ones written inside its declaration
        if k.methods.contains(lower)
            && let Some(f) = codebase.get_method(k.name.as_bytes(), lower.as_bytes())
            && f.span.file_id == k.span.file_id
            && k.span.start.offset <= f.span.start.offset
            && f.span.end.offset <= k.span.end.offset
        {
            return Some(Method::User { declaring: k, f });
        }
        // `use T { foo as bar; }`
        if let Some(original) = k.trait_alias_map.get(lower)
            && let Some(f) = k
                .direct_trait_names()
                .filter_map(|t| codebase.get_class_like(t.as_bytes()))
                .find_map(|t| codebase.get_declaring_method(t.name.as_bytes(), original.as_bytes()))
        {
            return Some(Method::User { declaring: k, f });
        }
        None
    };
    if let Some(method) = own(k) {
        return Some(method);
    }
    // Populated `used_traits` also contains parents' traits; only a direct use changes the method's owner.
    // Keep sibling searches independent, as BetterReflection's per-trait AlreadyVisitedClasses copies do.
    let mut pending = vec![(
        k,
        k.direct_trait_names(),
        codebase.get_declaring_method(k.name.as_bytes(), lower.as_bytes()),
    )];
    let mut active = HashSet::from([k.name]);
    let mut found = None;
    while let Some((parent, traits, declaring)) = pending.last_mut() {
        // A trait's methods are declared by the class using it; `insteadof` keeps the selected declaration.
        if let Some(Method::User { f, .. }) = found.take()
            && declaring.is_none_or(|d| d.span == f.span)
        {
            found = Some(Method::User {
                declaring: *parent,
                f,
            });
            active.remove(&parent.name);
            pending.pop();
            continue;
        }
        let Some(k) = traits.find_map(|t| codebase.get_class_like(t.as_bytes())) else {
            active.remove(&parent.name);
            pending.pop();
            continue;
        };
        if !active.insert(k.name) {
            continue;
        }
        found = own(k);
        if found.is_some() {
            active.remove(&k.name);
        } else {
            pending.push((
                k,
                k.direct_trait_names(),
                codebase.get_declaring_method(k.name.as_bytes(), lower.as_bytes()),
            ));
        }
    }
    found
}
