//! PHPStan's `ReflectionProvider`: user code from Mago's codex, built-ins from PHPStan's reflection at the project's
//! PHP version (data/symbols.json).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
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

impl Known<'_> {
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
    pub(super) fn function(&self, name: &[u8]) -> Option<(String, Vec<params::Acceptor>)> {
        if let Some(f) = self
            .codebase
            .get_function(name)
            .filter(|f| !f.flags.is_built_in())
        {
            return Some((
                String::from_utf8_lossy(f.original_name.as_bytes()).into_owned(),
                vec![params::Acceptor::of(f)],
            ));
        }
        let lower = String::from_utf8_lossy(name).to_ascii_lowercase();
        let f = self.builtins.functions.get(&lower)?;
        Some((
            f.name.clone(),
            f.variants
                .iter()
                .map(params::Acceptor::of_builtin)
                .collect(),
        ))
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
