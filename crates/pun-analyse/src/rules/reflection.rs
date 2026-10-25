//! PHPStan's `ReflectionProvider`: user code from Mago's codex, built-ins from PHPStan's reflection at the project's
//! PHP version (data/symbols.json).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::visibility::Visibility;
use mago_word::word;
use pun_phpstan::symbols::{self, PhpVersion, Symbols};
use pun_phpstan::types::{Level, Type};

use mago_allocator::LocalArena;

use super::{Checker, anonymous, params};
use crate::codebase::parse;

/// PHPStan's reflection of built-ins for a `phpVersion` id: 8.5's from 80500 on, 8.4's below.
pub(crate) fn builtins(php_version: u32) -> &'static Symbols {
    symbols::symbols(if php_version >= 80500 {
        PhpVersion::Php85
    } else {
        PhpVersion::Php84
    })
}

impl Checker<'_, '_> {
    /// `ClassReflection::getDisplayName()`: the name as declared, or an anonymous class's
    /// `class@anonymous/<path>:<line>` (see [`anonymous::names()`]).
    pub(super) fn display(&self, k: &ClassLikeMetadata) -> String {
        let name = k.original_name.as_bytes();
        if let Some(display) = self.anonymous.get(name) {
            return display.clone();
        }
        // one declared in another file (an inferred `new class`)
        if name.starts_with(b"{anonymous-class:")
            && let Some(display) = self
                .files
                .get(&k.span.file_id)
                .and_then(|p| self.sources.get(p))
                .and_then(|file| {
                    let arena = LocalArena::new();
                    let (program, names) = parse(&arena, file);
                    anonymous::names(file, program, &names, self.relative_path).remove(name)
                })
        {
            return display;
        }
        String::from_utf8_lossy(name).into_owned()
    }

    /// `ClassReflection::getName()`: the display name, or an anonymous class's internal `AnonymousClass<md5>`.
    pub(super) fn class_name(&self, k: &ClassLikeMetadata) -> String {
        let display = self.display(k);
        if !k.original_name.as_bytes().starts_with(b"{anonymous-class:") {
            return display;
        }
        let file = self
            .files
            .get(&k.span.file_id)
            .map(|p| crate::relative_path::absolute(p))
            .unwrap_or_default();
        anonymous::internal_name(&display, &file, self.relative_path)
    }

    /// `describe(VerbosityLevel::typeOnly())` of the static object type of `$this` in class `k`: its name, with its
    /// template types as arguments.
    pub(super) fn describe_object(&self, k: &ClassLikeMetadata) -> String {
        let display = self.display(k);
        if k.template_types.is_empty() {
            return display;
        }
        let args = super::declared::class_templates(self, k)
            .into_iter()
            .map(|t| Type::Template(Box::new(t)))
            .collect();
        Type::Object(display, args).describe(Level::TypeOnly)
    }
}

/// What PHPStan's reflection knows of a class: user code from Mago, built-ins from data/symbols.json.
#[derive(Clone, Copy)]
pub(super) enum Known<'a> {
    User(&'a ClassLikeMetadata),
    Builtin(&'static symbols::Class),
}

/// The reflection data ClassConstantRule reads; enum cases and built-in constants are public.
pub(super) struct ClassConstant {
    pub visibility: Option<Visibility>,
    pub declaring: String,
    pub display: String,
}

impl Known<'_> {
    pub(super) fn constant(&self, c: &Checker, name: &str) -> Option<ClassConstant> {
        match self {
            Known::User(k) => {
                let visibility = if let Some(m) = k.constants.get(&word(name.as_bytes())) {
                    // Populated constants retain the original declaration's span.
                    let declaring = std::iter::once(&k.name)
                        .chain(k.all_parent_classes.iter())
                        .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
                        .filter(|a| {
                            a.constants
                                .get(&word(name.as_bytes()))
                                .is_some_and(|v| v.span == m.span)
                        })
                        .last()
                        .unwrap_or(k);
                    return Some(ClassConstant {
                        visibility: Some(m.visibility),
                        declaring: declaring.original_name.to_string(),
                        display: c.display(declaring),
                    });
                } else if k.enum_cases.contains_key(&word(name.as_bytes())) {
                    None
                } else {
                    return None;
                };
                Some(ClassConstant {
                    visibility,
                    declaring: k.original_name.to_string(),
                    display: c.display(k),
                })
            }
            Known::Builtin(k) => {
                let has = |k: &symbols::Class| {
                    k.constants.iter().any(|(n, _)| n == name) || k.cases.iter().any(|n| n == name)
                };
                let declaring = std::iter::once(*k)
                    .chain(
                        k.ancestors
                            .iter()
                            .filter_map(|a| c.builtins.classes.get(&a.to_ascii_lowercase())),
                    )
                    .find(|k| has(k))?;
                Some(ClassConstant {
                    visibility: None,
                    declaring: declaring.name.clone(),
                    display: declaring.name.clone(),
                })
            }
        }
    }

    pub(super) fn is_trait(&self) -> bool {
        matches!(self, Known::User(k) if k.kind == SymbolKind::Trait)
    }

    pub(super) fn is_interface(&self) -> bool {
        match self {
            Known::User(k) => k.kind == SymbolKind::Interface,
            Known::Builtin(k) => k.is_interface,
        }
    }

    pub(super) fn is_enum(&self) -> bool {
        match self {
            Known::User(k) => k.kind == SymbolKind::Enum,
            Known::Builtin(k) => !k.cases.is_empty(),
        }
    }

    /// `isFinal()`: the keyword or an `@final` tag (Mago's flag holds both).
    pub(super) fn is_final(&self) -> bool {
        match self {
            Known::User(k) => k.flags.is_final(),
            Known::Builtin(k) => k.is_final,
        }
    }

    pub(super) fn is_class(&self) -> bool {
        !self.is_trait() && !self.is_interface() && !self.is_enum()
    }

    /// `getClassTypeDescription()`.
    pub(super) fn type_description(&self) -> &'static str {
        if self.is_interface() {
            "Interface"
        } else if self.is_trait() {
            "Trait"
        } else if self.is_enum() {
            "Enum"
        } else {
            "Class"
        }
    }

    /// The name as declared (a name in the source never finds an anonymous class).
    pub(super) fn display(&self) -> String {
        match self {
            Known::User(k) => String::from_utf8_lossy(k.original_name.as_bytes()).into_owned(),
            Known::Builtin(k) => k.name.clone(),
        }
    }
}

/// `ReflectionProvider::hasClass()` / `getClass()` (a leading `\` is ignored).
pub(super) fn known<'a>(c: &Checker<'a, '_>, name: &str) -> Option<Known<'a>> {
    let name = name.trim_start_matches('\\');
    if let Some(k) = c
        .codebase
        .get_class_like(name.as_bytes())
        .filter(|k| !k.flags.is_built_in())
    {
        return Some(Known::User(k));
    }
    c.builtins
        .classes
        .get(&name.to_ascii_lowercase())
        .map(Known::Builtin)
}

impl<'a> Checker<'a, '_> {
    /// `ReflectionProvider::getFunction()`: the display name and the variants.
    pub(super) fn function(&self, name: &[u8]) -> Option<(String, params::Variants)> {
        if let Some(f) = self
            .codebase
            .get_function(name)
            .filter(|f| !f.flags.is_built_in())
        {
            return Some((
                String::from_utf8_lossy(f.original_name.as_bytes()).into_owned(),
                params::Variants::of(f),
            ));
        }
        let lower = String::from_utf8_lossy(name).to_ascii_lowercase();
        let f = self.builtins.functions.get(&lower)?;
        Some((f.name.clone(), params::Variants::of_builtin(f)))
    }

    /// [`known()`] with Mago's metadata, which the rules reading constructors, methods and flags need.
    pub(super) fn class_like(&self, name: &str) -> Lookup<'a> {
        match known(self, name) {
            None => Lookup::NotFound,
            Some(Known::User(k)) => Lookup::Found(k),
            Some(Known::Builtin(k)) => self
                .codebase
                .get_class_like(k.name.as_bytes())
                .map_or(Lookup::Opaque, Lookup::Found),
        }
    }
}

/// [`Checker::class_like()`].
pub(super) enum Lookup<'a> {
    /// PHPStan has no such class.
    NotFound,
    /// A built-in PHPStan has that Mago's prelude lacks: nothing to check on it.
    Opaque,
    Found(&'a ClassLikeMetadata),
}

impl<'a> Lookup<'a> {
    pub(super) fn found(self) -> Option<&'a ClassLikeMetadata> {
        match self {
            Lookup::Found(k) => Some(k),
            Lookup::NotFound | Lookup::Opaque => None,
        }
    }
}
