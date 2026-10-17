//! `ClassReflection::getMethod()`: a method by name along the class's own methods, its traits (with `as` aliases),
//! its parents and its interfaces. User code comes from Mago's codex; built-in classes from PHPStan's reflection
//! (data/symbols.json), whose class entries list only the methods each class declares.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::visibility::Visibility;
use mago_word::word;
use pun_phpstan::symbols;

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
    codebase: &'a CodebaseMetadata,
    class: &'a ClassLikeMetadata,
    name: &[u8],
) -> Option<Method<'a>> {
    let lower = word(name.to_ascii_lowercase());
    let mut current = Some(class);
    while let Some(k) = current {
        if let Some(m) = declared(codebase, k, &lower) {
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
        .find_map(|i| declared(codebase, i, &lower))
}

/// A method `k` itself declares or takes from a trait (private ones of parents count, as in PHP's reflection).
fn declared<'a>(
    codebase: &'a CodebaseMetadata,
    k: &'a ClassLikeMetadata,
    lower: &mago_word::Word,
) -> Option<Method<'a>> {
    if k.flags.is_built_in() {
        let c = symbols::class(&String::from_utf8_lossy(k.name.as_bytes()))?;
        let f = c
            .methods
            .get(String::from_utf8_lossy(lower.as_bytes()).as_ref())?;
        return Some(Method::Builtin { declaring: c, f });
    }
    if k.methods.contains(lower) {
        let f = codebase.get_method(k.name.as_bytes(), lower.as_bytes())?;
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
    // a trait's methods are declared by the class using it, for PHPStan
    traits().find_map(|t| match declared(codebase, t, lower)? {
        Method::User { f, .. } => Some(Method::User { declaring: k, f }),
        builtin => Some(builtin),
    })
}
