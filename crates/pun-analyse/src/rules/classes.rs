//! Class instantiation and class-name checks.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_syntax::cst::*;

use mago_span::HasSpan;

use super::class_names::{self, Pair};
use super::{Checker, DISCOVERING_SYMBOLS_TIP, members, params};

/// NewStaticRule, then InstantiationRule (level 0: no argument types).
pub fn instantiation<'arena>(c: &mut Checker<'_, 'arena>, new: &Instantiation<'arena>) {
    if matches!(new.class, Expression::Static(_)) {
        new_static(c, new);
    }
    let args = new.argument_list.as_ref();
    // `new Foo(...)`: InstantiationCallableRule's business
    if args.is_some_and(|a| {
        a.arguments
            .iter()
            .any(|x| matches!(x, Argument::Positional(p) if p.ellipsis.is_some() && matches!(p.value, Expression::Error(_))))
    }) {
        return;
    }
    let in_class = !c.classes.is_empty();
    let keyword = |c: &Checker| {
        String::from_utf8_lossy(&c.file.contents[new.class.span().to_range_usize()]).into_owned()
    };
    let (class, is_static) = match new.class {
        Expression::Identifier(id) => {
            let name = c.resolved(id);
            named(c, new, &name, true, Pair::name(name.clone(), id.span()));
            return;
        }
        Expression::Static(_) | Expression::Self_(_) | Expression::Parent(_) if !in_class => {
            let written = keyword(c);
            let identifier = match new.class {
                Expression::Static(_) => "outOfClass.static",
                Expression::Self_(_) => "outOfClass.self",
                _ => "outOfClass.parent",
            };
            c.report(
                new,
                identifier,
                format!("Using {written} outside of class scope."),
                None,
            );
            return;
        }
        Expression::Static(_) => {
            let Some(class) = c.class_scope() else {
                return; // ponytail: trait and anonymous class scopes
            };
            if !class.flags.is_final()
                && (!has_constructor(c, class) || !consistent_constructor(c, class))
            {
                return;
            }
            (class, true)
        }
        Expression::Self_(_) => match c.class_scope() {
            Some(class) => (class, false),
            None => return,
        },
        Expression::Parent(_) => {
            let Some(scope) = c.class_scope() else {
                return;
            };
            let parent = scope
                .direct_parent_class
                .and_then(|p| c.class_like(&String::from_utf8_lossy(p.as_bytes())));
            match parent {
                Some(p) => (p, false),
                None => {
                    let display =
                        String::from_utf8_lossy(scope.original_name.as_bytes()).into_owned();
                    let function = c
                        .functions
                        .last()
                        .map(|f| f.name.clone())
                        .unwrap_or_default();
                    c.report(
                        new,
                        "class.noParent",
                        format!("{display}::{function}() calls new parent but {display} does not extend any class."),
                        None,
                    );
                    return;
                }
            }
        }
        Expression::AnonymousClass(_) => return, // ponytail: anonymous class constructors unchecked
        expr => {
            for (name, is_name) in class_names(c, expr) {
                let pair = Pair::node(name.clone(), expr.span());
                named(c, new, &name, is_name, pair);
            }
            return;
        }
    };
    check_class(c, new, class, is_static, true);
}

/// `getClassNames()` for `new $expr`: the class-strings' concrete classes, else the constant strings (as names)
/// and the object types' classes (not as names).
fn class_names(c: &Checker, expr: &Expression) -> Vec<(String, bool)> {
    let Some(t) = c.type_of(expr) else {
        return Vec::new();
    };
    let mut strings = Vec::new();
    let mut objects = Vec::new();
    let mut class_strings = true;
    for atomic in t.types.iter() {
        match atomic {
            TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => {
                strings.push(String::from_utf8_lossy(value.as_bytes()).into_owned());
            }
            TAtomic::Scalar(s) if s.get_known_literal_string_value().is_some() => {
                class_strings = false;
                strings.push(
                    String::from_utf8_lossy(s.get_known_literal_string_value().unwrap_or_default())
                        .into_owned(),
                );
            }
            TAtomic::Object(TObject::Named(o)) => {
                class_strings = false;
                objects.push(String::from_utf8_lossy(o.name.as_bytes()).into_owned());
            }
            _ => class_strings = false,
        }
    }
    if class_strings && !strings.is_empty() {
        let concretes: Vec<(String, bool)> = strings
            .iter()
            .filter_map(|n| c.class_like(n))
            .filter(|k| !k.flags.is_abstract() && !k.kind.is_interface())
            .map(|k| {
                (
                    String::from_utf8_lossy(k.original_name.as_bytes()).into_owned(),
                    true,
                )
            })
            .collect();
        if !concretes.is_empty() {
            return concretes;
        }
    }
    strings
        .into_iter()
        .map(|n| (n, true))
        .chain(objects.into_iter().map(|n| (n, false)))
        .collect()
}

/// `checkClassName()` for a class given by name, `pair` the name and the node `new` names it with.
fn named<'arena>(
    c: &mut Checker<'_, 'arena>,
    new: &Instantiation<'arena>,
    name: &str,
    is_name: bool,
    pair: Pair,
) {
    match c.class_like(name) {
        Some(class) => {
            class_names::check(c, &[pair], true);
            check_class(c, new, class, false, is_name);
        }
        None if c.in_class_exists(name) => {}
        None => c.report(
            new,
            "class.notFound",
            format!("Instantiated class {name} not found."),
            Some(DISCOVERING_SYMBOLS_TIP),
        ),
    }
}

fn check_class<'arena>(
    c: &mut Checker<'_, 'arena>,
    new: &Instantiation<'arena>,
    class: &ClassLikeMetadata,
    is_static: bool,
    is_name: bool,
) {
    let args = new.argument_list.as_ref();
    let display = String::from_utf8_lossy(class.original_name.as_bytes()).into_owned();
    let refusal = match class.kind {
        SymbolKind::Trait => Some(("new.trait", format!("Cannot instantiate trait {display}."))),
        SymbolKind::Enum => Some(("new.enum", format!("Cannot instantiate enum {display}."))),
        SymbolKind::Interface if !is_static => Some((
            "new.interface",
            format!("Cannot instantiate interface {display}."),
        )),
        _ if !is_static && class.flags.is_abstract() => Some((
            "new.abstract",
            format!("Instantiated class {display} is abstract."),
        )),
        _ => None,
    };
    if let Some((identifier, message)) = refusal.filter(|_| is_name) {
        c.report(new, identifier, message, None);
        return;
    }
    if !is_name {
        return;
    }
    let Some(constructor) = members::method(c.codebase, class, b"__construct") else {
        if args.is_some_and(|a| !a.arguments.is_empty()) {
            c.report(
                new,
                "new.noConstructor",
                format!("Class {display} does not have a constructor and must be instantiated without any parameters."),
                None,
            );
        }
        return;
    };
    let declaring = constructor.declaring_display();
    let visibility = constructor.visibility();
    if !super::calls::can_call(c, c.class_scope(), &declaring, visibility) {
        let v = if visibility.is_private() {
            "private"
        } else {
            "protected"
        };
        c.report(
            new,
            if visibility.is_private() {
                "new.privateConstructor"
            } else {
                "new.protectedConstructor"
            },
            format!(
                "Cannot instantiate class {display} via {v} constructor {declaring}::{}().",
                constructor.name()
            ),
            None,
        );
    }
    let counts = args.map_or((0, false, false), params::count);
    params::check(
        c,
        &params::select(constructor.variants(), counts.0, counts.2),
        counts,
        new,
        params::Call {
            kind: "new",
            subject: &format!("Class {display} constructor"),
            void_message: None,
        },
        false,
    );
}

/// The constructor checks NewStaticRule and InstantiationRule share: `@phpstan-consistent-constructor` on the
/// class or a parent, or a constructor that is final, abstract or declared by an interface (or overrides an
/// abstract or interface one).
fn consistent_constructor(c: &Checker, class: &ClassLikeMetadata) -> bool {
    let consistent = std::iter::once(&class.name)
        .chain(class.all_parent_classes.iter())
        .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
        .any(|k| k.flags.has_consistent_constructor());
    if consistent {
        return true;
    }
    let Some(constructor) = members::method(c.codebase, class, b"__construct") else {
        return false;
    };
    if constructor.is_final() || constructor.is_abstract() {
        return true;
    }
    // the prototype: an abstract or interface constructor it overrides
    class
        .all_parent_interfaces
        .iter()
        .chain(class.all_parent_classes.iter())
        .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
        .filter_map(|a| members::method(c.codebase, a, b"__construct"))
        .any(|m| m.declared_in_interface() || m.is_abstract())
}

fn has_constructor(c: &Checker, class: &ClassLikeMetadata) -> bool {
    members::method(c.codebase, class, b"__construct").is_some()
}

/// NewStaticRule.
fn new_static(c: &mut Checker, new: &Instantiation) {
    let Some(class) = c.class_scope() else {
        return;
    };
    if class.flags.is_final() || consistent_constructor(c, class) {
        return;
    }
    // in a trait (PHP 8.0+ has abstract trait methods) that declares an abstract constructor
    if has_constructor(c, class)
        && c.php_version >= 80000
        && c.in_target
        && let Some(t) = c.context_trait.as_deref()
        && let Some(t) = c.codebase.get_class_like(t.as_bytes())
        && members::method(c.codebase, t, b"__construct").is_some_and(|m| m.is_abstract())
    {
        return;
    }
    c.report(
        new,
        "new.static",
        "Unsafe usage of new static().".to_string(),
        Some("See: https://phpstan.org/blog/solving-phpstan-error-unsafe-usage-of-new-static"),
    );
}
