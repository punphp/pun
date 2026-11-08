//! Class-hierarchy declaration rules that read only class metadata: the final and readonly parts of
//! ExistingClassInClassExtendsRule, ExistingClassInTraitUseRule, AbstractMethodInNonAbstractClassRule,
//! MissingMethodImplementationRule, ConsistentConstructorDeclarationRule, SealedDefinitionTraitRule,
//! InvalidCallablePropertyTypeRule and MissingMagicSerializationMethodsRule.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::class_names::{self, Pair};
use super::declarations::has_tag;
use super::members;
use super::reflection::{Known, known};
use super::{Checker, DISCOVERING_SYMBOLS_TIP};

/// `ClassReflection::implementsInterface(Serializable::class)`.
pub(super) fn implements_serializable(k: &ClassLikeMetadata) -> bool {
    k.all_parent_interfaces
        .iter()
        .any(|i| i.as_bytes().eq_ignore_ascii_case(b"serializable"))
}

impl<'a> Checker<'a, '_> {
    /// The PHPDoc right before a class-like's declaration (`getResolvedPhpDoc()` of its own comment).
    pub(super) fn class_doc(&self, k: &ClassLikeMetadata) -> Option<(&'a [u8], std::borrow::Cow<'a, super::facts::DocEntry>)> {
        self.php_doc(k.span.file_id, k.span.start.offset)
    }

    pub(super) fn class_tags(&self, k: &ClassLikeMetadata) -> Vec<String> {
        self.class_doc(k).map(|(_, entry)| entry.names.clone()).unwrap_or_default()
    }

    /// `isFinalByKeyword()` of a declared class: Mago's FINAL flag also holds an `@final` tag (even one between the
    /// attributes and the keyword, which is not the class's PHPDoc), so the declaration's modifiers decide. Without
    /// the source (PHPStan's API stub, which has no `@final` tags) the flag is the keyword.
    fn final_by_keyword(&self, k: &ClassLikeMetadata) -> bool {
        k.flags.is_final()
            && k.name_span
                .and_then(|name| self.has_modifier(k.span, name, b"final"))
                .unwrap_or(true)
    }

    /// Whether the declaration at `span` has the modifier `modifier` before its name (after its attributes), or
    /// `None` when its file can't be read (a built-in symbol's stub).
    pub(super) fn has_modifier(&self, span: Span, name: Span, modifier: &[u8]) -> Option<bool> {
        let contents = self.file_contents(span.file_id)?;
        let head = &contents[span.start.offset as usize..name.start.offset as usize];
        let modifiers = head
            .iter()
            .rposition(|b| *b == b']')
            .map_or(head, |i| &head[i + 1..]);
        // the words outside the comments between them
        let mut words = Vec::with_capacity(modifiers.len());
        let mut rest = modifiers;
        while let Some(&b) = rest.first() {
            if let Some(comment) = rest.strip_prefix(b"/*") {
                let end = comment.windows(2).position(|w| w == b"*/");
                rest = end.map_or(&[], |e| &comment[e + 2..]);
            } else {
                words.push(b);
                rest = &rest[1..];
            }
        }
        Some(
            words
                .split(|b| !b.is_ascii_alphanumeric() && *b != b'_')
                .any(|w| w.eq_ignore_ascii_case(modifier)),
        )
    }

    /// BetterReflection's `ReflectionMethod::isAbstract()`: an interface's method, or one declared `abstract` (Mago
    /// also flags a method without a body).
    pub(super) fn method_abstract(
        &self,
        declaring: &ClassLikeMetadata,
        f: &mago_codex::metadata::function_like::FunctionLikeMetadata,
    ) -> bool {
        if declaring.kind == SymbolKind::Interface {
            return true;
        }
        f.method_metadata.as_ref().is_some_and(|m| m.is_abstract)
            && f.name_span
                .and_then(|name| self.has_modifier(f.span, name, b"abstract"))
                .unwrap_or(true)
    }
}

/// ExistingClassInClassExtendsRule on a parent that is a class: `class.extendsFinal` (`isFinalByKeyword()`) and
/// `class.extendsFinalByPhpDoc` (`isFinal()` from an `@final` tag).
pub(super) fn extends_final(c: &mut Checker, at: Span, current: &str, parent: Known) {
    let (by_keyword, by_doc) = match parent {
        Known::User(k) => (
            c.final_by_keyword(k),
            has_tag(&c.class_tags(k), &["@final"]),
        ),
        Known::Builtin(k) => (k.is_final, false),
    };
    let display = parent.display();
    if by_keyword {
        c.report_non_ignorable(
            at,
            "class.extendsFinal",
            format!("{current} extends final class {display}."),
            None,
        );
    } else if by_doc {
        c.report(
            at,
            "class.extendsFinalByPhpDoc",
            format!("{current} extends @final class {display}."),
            None,
        );
    }
}

/// ExistingClassInClassExtendsRule: a readonly class extends a non-readonly one, or the other way round. `current`
/// is `Class Foo` or `Anonymous class`.
pub(super) fn extends_readonly(
    c: &mut Checker,
    at: Span,
    current: &str,
    readonly: bool,
    parent: Known,
) {
    let parent_readonly = matches!(parent, Known::User(k) if k.flags.is_readonly());
    let name = current.strip_prefix("Class ");
    let display = parent.display();
    if readonly && !parent_readonly {
        let current = name.map_or_else(
            || "Anonymous readonly class".to_string(),
            |n| format!("Readonly class {n}"),
        );
        c.report_non_ignorable(
            at,
            "class.readOnly",
            format!("{current} extends non-readonly class {display}."),
            None,
        );
    } else if !readonly && parent_readonly {
        let current = name.map_or_else(
            || "Anonymous non-readonly class".to_string(),
            |n| format!("Non-readonly class {n}"),
        );
        c.report_non_ignorable(
            at,
            "class.nonReadOnly",
            format!("{current} extends readonly class {display}."),
            None,
        );
    }
}

/// ExistingClassInTraitUseRule (its ClassNameCheck part, the `*.nameCase` errors, is not ported).
pub(super) fn trait_use<'arena>(c: &mut Checker<'_, 'arena>, u: &TraitUse<'arena>) {
    // a trait's own body is walked only in a using class's context
    let decl = c.decls.last().expect("in a class-like");
    let (kind, display) = (decl.kind, decl.display.clone());
    let in_trait = c.in_trait().map(|t| {
        c.codebase
            .get_class_like(t.as_bytes())
            .map_or_else(|| t.to_string(), |k| c.display(k))
    });
    let pairs: Vec<Pair> = u
        .trait_names
        .iter()
        .map(|id| Pair::name(c.resolved(id), id.span()))
        .collect();
    let class = c
        .class_scope()
        .filter(|k| !k.name.as_bytes().starts_with(b"{anonymous-class:"))
        .map(|k| String::from_utf8_lossy(k.original_name.as_bytes()).into_owned());
    class_names::check_at(c, &pairs, Some(&class_names::Location::TraitUse { class }), true);
    // (an interface is never a trait's context)
    if kind == SymbolKind::Interface {
        let display = display.expect("a named interface");
        for id in &u.trait_names {
            let name = c.resolved(id);
            c.report_non_ignorable(
                u,
                "interface.traitUse",
                format!("Interface {display} uses trait {name}."),
                None,
            );
        }
        return;
    }
    let current = match (in_trait, display) {
        (Some(t), _) => format!("Trait {t}"),
        (None, Some(d)) if !c.in_anonymous_class() => format!("Class {d}"),
        (None, _) => "Anonymous class".to_string(),
    };
    for id in &u.trait_names {
        let name = c.resolved(id);
        let Some(k) = known(c, &name) else {
            c.report_non_ignorable(
                u,
                "trait.notFound",
                format!("{current} uses unknown trait {name}."),
                Some(DISCOVERING_SYMBOLS_TIP),
            );
            continue;
        };
        let what = if k.is_interface() {
            "interface"
        } else if k.is_enum() {
            "enum"
        } else if k.is_class() {
            "class"
        } else {
            continue;
        };
        c.report_non_ignorable(
            u,
            &format!("traitUse.{what}"),
            format!("{current} uses {what} {}.", k.display()),
            None,
        );
    }
}

/// AbstractMethodInNonAbstractClassRule and ConsistentConstructorDeclarationRule on a method (`name` is its name, a
/// trait method's alias).
pub(super) fn method<'arena>(c: &mut Checker<'_, 'arena>, m: &Method<'arena>, name: &str) {
    let decl = c.decls.last().expect("in a class-like");
    let display = decl.display.clone().unwrap_or_default();
    let (kind, class_abstract) = (decl.kind, decl.is_abstract);
    let class = c.class_scope();
    let modifiers = m.modifiers.as_slice();
    if !class_abstract && modifiers.iter().any(Modifier::is_abstract) {
        let lower = name.to_ascii_lowercase();
        // an enum's native cases(), and a backed enum's from() and tryFrom()
        let native = kind == SymbolKind::Enum
            && (lower == "cases"
                || (class.is_some_and(|k| backed(c, k))
                    && (lower == "from" || lower == "tryfrom")));
        if !native {
            let description = match kind {
                SymbolKind::Interface => "Interface",
                SymbolKind::Enum => "Enum",
                _ => "Non-abstract class",
            };
            c.report_non_ignorable(
                m,
                "method.abstract",
                format!("{description} {display} contains abstract method {name}()."),
                None,
            );
        }
    } else if !class_abstract
        && kind != SymbolKind::Interface
        && matches!(m.body, MethodBody::Abstract(_))
    {
        c.report_non_ignorable(
            m,
            "method.nonAbstract",
            format!("Non-abstract method {display}::{name}() must contain a body."),
            None,
        );
    }
    // ConsistentConstructorDeclarationRule (InClassMethodNode)
    if name.eq_ignore_ascii_case("__construct")
        && modifiers.iter().any(Modifier::is_private)
        && let Some(k) = class
        && !k.flags.is_final()
        && has_tag(
            &c.class_tags(k),
            &[
                "@consistent-constructor",
                "@phpstan-consistent-constructor",
                "@psalm-consistent-constructor",
            ],
        )
    {
        c.report(
            m,
            "consistentConstructor.private",
            "Private constructor cannot be enforced as consistent for child classes.".to_string(),
            None,
        );
    }
}

/// MissingMethodImplementationRule and MissingMagicSerializationMethodsRule (InClassNode) on the class or enum `k`
/// declared at `at`.
pub(super) fn in_class(c: &mut Checker, at: Span, k: &ClassLikeMetadata) {
    // getNativeReflection() throws IdentifierNotFound on a missing ancestor; an ancestor of the wrong kind (a class
    // implementing an enum) is a broken hierarchy PHP refuses to load, which PHPStan reports elsewhere
    let wrong_kind = |n: &mago_word::Word, kind: SymbolKind| {
        c.codebase
            .get_class_like(n.as_bytes())
            .is_some_and(|p| p.kind != kind)
    };
    if k.kind == SymbolKind::Interface
        || k.flags.is_abstract()
        || k.has_incomplete_hierarchy()
        || k.direct_parent_interfaces
            .iter()
            .any(|i| wrong_kind(i, SymbolKind::Interface))
        || k.direct_parent_class
            .is_some_and(|p| wrong_kind(&p, SymbolKind::Class))
    {
        return;
    }
    let display = c.display(k);
    let is_enum = k.kind == SymbolKind::Enum;
    let description = if is_enum {
        "Enum"
    } else {
        "Non-abstract class"
    };
    // getNativeReflection()->getMethods(): each name resolved as PHP does (the class, its traits and parents, then
    // its interfaces); Mago's declaring ids also hold `@method` tags and can let an interface's method shadow a
    // trait's
    let mut names: Vec<_> = k.declaring_method_ids.keys().collect();
    names.sort_by_key(|lower| lower.as_bytes());
    for lower in names {
        let lower_name = lower.as_bytes();
        // BetterReflection adds an enum's native cases(), and a backed enum's from() and tryFrom()
        if is_enum
            && (lower_name == b"cases"
                || (backed(c, k) && (lower_name == b"from" || lower_name == b"tryfrom")))
        {
            continue;
        }
        let Some(m) = members::method(c, k, lower_name) else {
            continue; // a `@method` tag
        };
        let is_abstract = match &m {
            members::Method::User { declaring, f, .. } => {
                c.method_abstract(declaring, f) && !inherited_satisfies(c, declaring, f, lower_name)
            }
            // isProvidedByBuiltinAncestor(): a non-abstract built-in parent implementing the interface provides its
            // methods at runtime (SplFixedArray's stub `implements Iterator`, whose methods PHP 8 dropped)
            members::Method::Builtin { declaring, .. } => {
                (declaring.is_interface
                    || c.codebase
                        .get_method(declaring.name.as_bytes(), lower_name)
                        .and_then(|f| f.method_metadata.as_ref())
                        .is_some_and(|m| m.is_abstract))
                    && !(declaring.is_interface
                        && provided_by_builtin_ancestor(c, k, &declaring.name))
            }
        };
        if !is_abstract || (m.declared_in_interface() && trait_implements(c, k, lower_name, 0)) {
            continue;
        }
        let what = if m.declared_in_interface() {
            "interface"
        } else {
            "class"
        };
        c.report_non_ignorable(
            at,
            "method.abstract",
            format!(
                "{description} {display} contains abstract method {}() from {what} {}.",
                m.name(),
                m.declaring_display(c),
            ),
            None,
        );
    }
    // MissingMagicSerializationMethodsRule: PhpVersion::serializableRequiresMagicMethods()
    if c.php_version < 80100 || is_enum || !implements_serializable(k) {
        return;
    }
    for magic in ["__serialize", "__unserialize"] {
        if members::method(c, k, magic.as_bytes()).is_none() {
            c.report(
                at,
                "class.serializable",
                format!(
                    "Non-abstract class {display} implements the Serializable interface, but does not implement {magic}()."
                ),
                Some("See https://wiki.php.net/rfc/phase_out_serializable"),
            );
        }
    }
}

/// PHP keeps an inherited method over a trait's abstract one: `f`, declared by `declaring` from one of its traits,
/// is satisfied by a concrete method of a parent.
fn inherited_satisfies(
    c: &Checker,
    declaring: &ClassLikeMetadata,
    f: &mago_codex::metadata::function_like::FunctionLikeMetadata,
    lower: &[u8],
) -> bool {
    let own = f.span.file_id == declaring.span.file_id
        && declaring.span.start.offset <= f.span.start.offset
        && f.span.end.offset <= declaring.span.end.offset;
    !own && declaring
        .direct_parent_class
        .and_then(|p| c.codebase.get_class_like(p.as_bytes()))
        .and_then(|p| members::method(c, p, lower))
        .is_some_and(|m| !m.is_abstract() && !m.declared_in_interface())
}

/// Whether a trait of `k` or of a parent, or one they use, has a concrete method `lower`, which implements an
/// interface's method in PHP (`members::method()` can miss one that Mago's declaring ids hide behind another trait's
/// `@method` tag, as Carbon's `tomorrow()`).
fn trait_implements(c: &Checker, k: &ClassLikeMetadata, lower: &[u8], depth: usize) -> bool {
    // a cyclic hierarchy is another rule's error
    depth < 64
        && std::iter::once(&k.name)
            .chain(k.all_parent_classes.iter())
            .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
            .flat_map(|k| k.used_traits.iter())
            .filter_map(|t| c.codebase.get_class_like(t.as_bytes()))
            .any(|t| {
                c.codebase
                    .get_method(t.name.as_bytes(), lower)
                    .is_some_and(|f| {
                        f.span.file_id == t.span.file_id
                            && t.span.start.offset <= f.span.start.offset
                            && f.span.end.offset <= t.span.end.offset
                            && !c.method_abstract(t, f)
                    })
                    || trait_implements(c, t, lower, depth + 1)
            })
}

/// `isBackedEnum()`: an enum with a backing type, valid or not (Mago types only `int` and `string` ones).
fn backed(c: &Checker, k: &ClassLikeMetadata) -> bool {
    k.enum_type.is_some() || c.codebase.is_instance_of(k.name.as_bytes(), b"backedenum")
}

/// SealedDefinitionTraitRule: a `@phpstan-sealed` or `@psalm-inheritors` tag with a type on a trait.
pub(super) fn sealed_trait<'arena>(c: &mut Checker<'_, 'arena>, t: &Trait<'arena>) {
    let name = c.resolved_span(t.name.span);
    let Some(doc) = c
        .codebase
        .get_class_like(name.as_bytes())
        .and_then(|k| c.class_doc(k))
    else {
        return;
    };
    if sealed_tag(doc.0) {
        c.report(
            t,
            "sealed.onTrait",
            "PHPDoc tag @phpstan-sealed is only valid on class or interface.".to_string(),
            None,
        );
    }
}

/// Whether a PHPDoc has a `@psalm-inheritors` or `@phpstan-sealed` tag followed by a value (`getSealedTagValues()`).
fn sealed_tag(doc: &[u8]) -> bool {
    let body = doc.strip_prefix(b"/**").unwrap_or(doc);
    let body = body.strip_suffix(b"*/").unwrap_or(body);
    body.split(|b| *b == b'\n').any(|line| {
        let line = line.trim_ascii_start();
        let line = line.strip_prefix(b"*").unwrap_or(line).trim_ascii_start();
        ["@psalm-inheritors", "@phpstan-sealed"].iter().any(|tag| {
            line.strip_prefix(tag.as_bytes()).is_some_and(|rest| {
                rest.first().is_some_and(u8::is_ascii_whitespace) && !rest.trim_ascii().is_empty()
            })
        })
    })
}

/// InvalidCallablePropertyTypeRule on one ClassPropertyNode (`name` with its `$`).
pub(super) fn callable_property(c: &mut Checker, at: Span, name: &[u8], hint: Option<&Hint>) {
    if !hint.is_some_and(names_callable) {
        return;
    }
    if let Some(display) = c.decls.last().and_then(|d| d.display.clone()) {
        let property =
            String::from_utf8_lossy(name.strip_prefix(b"$").unwrap_or(name)).into_owned();
        c.report_non_ignorable(
            at,
            "property.callableType",
            format!(
                "Property {display}::${property} cannot have callable in its type declaration."
            ),
            None,
        );
    }
}

/// A CallableType in a native type, through its unions and intersections.
fn names_callable(hint: &Hint) -> bool {
    match hint {
        Hint::Callable(_) => true,
        Hint::Parenthesized(p) => names_callable(p.hint),
        Hint::Nullable(n) => names_callable(n.hint),
        Hint::Union(u) => names_callable(u.left) || names_callable(u.right),
        Hint::Intersection(i) => names_callable(i.left) || names_callable(i.right),
        _ => false,
    }
}

/// MethodVisibilityInInterfaceRule: a private or protected method declared in an interface.
pub(super) fn interface_method_visibility<'arena>(c: &mut Checker<'_, 'arena>, m: &Method<'arena>) {
    let Some(decl) = c
        .decls
        .last()
        .filter(|d| d.kind == SymbolKind::Interface && c.trait_context.is_none())
    else {
        return;
    };
    let non_public = m
        .modifiers
        .iter()
        .any(|x| matches!(x, Modifier::Private(_) | Modifier::Protected(_)));
    let Some(display) = decl.display.clone().filter(|_| non_public) else {
        return;
    };
    let name = String::from_utf8_lossy(m.name.value);
    c.report_non_ignorable(
        m,
        "method.visibility",
        format!("Method {display}::{name}() cannot use non-public visibility in interface."),
        None,
    );
}

/// AllowedSubTypesRule on the class-like `k` declared at `at`: each direct parent (interfaces, then the parent
/// class) that lists its allowed subtypes must list this one.
pub(super) fn allowed_subtypes(c: &mut Checker, at: Span, k: &ClassLikeMetadata) {
    // getImmediateInterfaces() holds interfaces only (an enum named in `implements` is reported elsewhere)
    let parents: Vec<&ClassLikeMetadata> = k
        .direct_parent_interfaces
        .iter()
        .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
        .filter(|p| p.kind == SymbolKind::Interface)
        .chain(
            k.direct_parent_class
                .iter()
                .filter_map(|n| c.codebase.get_class_like(n.as_bytes())),
        )
        .collect();
    // getName(): an anonymous class's internal name
    let class_name = c.class_name(k);
    let kind = match k.kind {
        SymbolKind::Interface => "interface",
        SymbolKind::Enum => "enum",
        _ => "class",
    };
    for parent in parents {
        let Some(allowed) = allowed_sub_types(c, parent) else {
            continue;
        };
        if allowed.iter().any(|t| match t {
            pun_phpstan::types::Type::Object(n, _, _) => *n == class_name,
            _ => false,
        }) {
            continue;
        }
        let message = format!(
            "Type {class_name} is not allowed to be a subtype of {}.",
            c.display(parent)
        );
        c.report(at, &format!("{kind}.disallowedSubtype"), message, None);
    }
}

/// `ClassReflection::getAllowedSubTypes()` through SealedAllowedSubTypesClassReflectionExtension: the last
/// `@psalm-inheritors`/`@phpstan-sealed` tag's types (a union's members), plus the class itself when it can be
/// instantiated. An enum allows only its cases, so no class-like.
fn allowed_sub_types(c: &Checker, k: &ClassLikeMetadata) -> Option<Vec<pun_phpstan::types::Type>> {
    use pun_phpstan::types::Type;
    if k.kind == SymbolKind::Enum {
        // EnumAllowedSubTypesClassReflectionExtension: only the enum's cases
        return Some(Vec::new());
    }
    let (_, entry) = c.php_doc(k.span.file_id, k.span.start.offset)?;
    let tags = &entry.tags;
    // getSealedTagValues() skips a tag whose type does not parse
    let last = ["@psalm-inheritors", "@phpstan-sealed"]
        .iter()
        .flat_map(|name| tags.iter().filter(move |(t, _)| t == name))
        .filter_map(|(_, v)| pun_phpstan::phpdoc::parse_prefix(v).ok())
        .next_back()?;
    let scope = super::declared::class_scope(c, k.span, k);
    let mut types = match pun_phpstan::resolve::resolve_in(&last.0, &scope) {
        Type::Union(members) => members,
        t => vec![t],
    };
    if k.kind == SymbolKind::Class && !k.flags.is_abstract() {
        types.push(Type::Object(super::declared::original_name(k), Vec::new(), false));
    }
    Some(types)
}

/// `MissingMethodImplementationRule::isProvidedByBuiltinAncestor()`
fn provided_by_builtin_ancestor(c: &Checker, k: &ClassLikeMetadata, interface: &str) -> bool {
    k.all_parent_classes
        .iter()
        .filter_map(|p| c.codebase.get_class_like(p.as_bytes()))
        .any(|p| {
            p.flags.is_built_in()
                && !p.flags.is_abstract()
                && p.all_parent_interfaces
                    .iter()
                    .any(|i| i.as_bytes().eq_ignore_ascii_case(interface.as_bytes()))
        })
}
