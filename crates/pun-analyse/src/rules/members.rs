//! `ClassReflection::getMethod()`: a method by name along the class's own methods, its traits (with `as` aliases),
//! its parents and its interfaces. User code comes from Mago's codex; built-in classes from PHPStan's reflection
//! (data/symbols.json), whose class entries list only the methods each class declares.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::visibility::Visibility;
use mago_word::word;
use pun_phpstan::symbols;

use super::Checker;
use super::params::Acceptor;

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
    pub fn declaring_display(&self) -> String {
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

    pub fn variants(&self) -> Vec<Acceptor> {
        match self {
            Method::User { f, .. } => vec![Acceptor::of(f)],
            Method::Builtin { f, .. } => f.variants.iter().map(Acceptor::of_builtin).collect(),
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
    while let Some(k) = current {
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

/// AnnotationsMethodsClassReflectionExtension: a `@method` tag of the class, its traits (declared by the class), its
/// parents or its interfaces, which `getMethod()` falls back to when the class has no such native method.
pub fn annotation_method<'a>(
    c: &Checker<'a, '_>,
    class: &'a ClassLikeMetadata,
    name: &[u8],
) -> Option<Method<'a>> {
    let lower = word(name.to_ascii_lowercase());
    tagged(c, class, class, &lower, 0)
}

/// findClassReflectionWithMethod().
fn tagged<'a>(
    c: &Checker<'a, '_>,
    k: &'a ClassLikeMetadata,
    declaring: &'a ClassLikeMetadata,
    lower: &mago_word::Word,
    depth: usize,
) -> Option<Method<'a>> {
    // a cyclic hierarchy is PHPStan's error elsewhere
    (depth <= 64).then_some(())?;
    let codebase = c.codebase;
    if (k.pseudo_methods.contains(lower) || k.static_pseudo_methods.contains(lower))
        && let Some(f) = codebase
            .function_likes
            .get(&(k.name, *lower))
            .map(Box::as_ref)
    {
        return Some(Method::User { declaring, f });
    }
    let get = |n: &mago_word::Word| codebase.get_class_like(n.as_bytes());
    k.used_traits
        .iter()
        .filter_map(get)
        .find_map(|t| tagged(c, t, declaring, lower, depth + 1))
        .or_else(|| {
            let p = k.direct_parent_class.as_ref().and_then(get)?;
            tagged(c, p, p, lower, depth + 1)
        })
        .or_else(|| {
            k.all_parent_interfaces
                .iter()
                .filter_map(get)
                .find_map(|i| tagged(c, i, i, lower, depth + 1))
        })
}

/// A method `k` itself declares or takes from a trait (private ones of parents count, as in PHP's reflection).
fn declared<'a>(
    c: &Checker<'a, '_>,
    k: &'a ClassLikeMetadata,
    lower: &mago_word::Word,
) -> Option<Method<'a>> {
    let codebase = c.codebase;
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
    let traits = || {
        k.used_traits
            .iter()
            .filter_map(|t| codebase.get_class_like(t.as_bytes()))
    };
    // `use T { foo as bar; }`
    if let Some(original) = k.trait_alias_map.get(lower)
        && let Some(f) = traits().find_map(|t| {
            codebase
                .get_method(t.name.as_bytes(), original.as_bytes())
                .filter(|_| {
                    t.methods
                        .contains(&word(original.as_bytes().to_ascii_lowercase()))
                })
        })
    {
        return Some(Method::User { declaring: k, f });
    }
    // a trait's methods are declared by the class using it, for PHPStan; of two traits with the method, the one
    // `insteadof` keeps (the method PHP's reflection declares)
    let declaring = codebase.get_declaring_method(k.name.as_bytes(), lower.as_bytes());
    traits().find_map(|t| match declared(c, t, lower)? {
        Method::User { f, .. } => declaring
            .is_none_or(|d| d.span == f.span)
            .then_some(Method::User { declaring: k, f }),
        builtin => Some(builtin),
    })
}
