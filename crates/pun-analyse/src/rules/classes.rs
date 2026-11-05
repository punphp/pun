//! Class instantiation and class-name checks.

use mago_allocator::LocalArena;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Type, intersect, union};

use mago_span::{HasSpan, Span};

use super::class_names::{self, Pair};
use super::reflection::Lookup;
use super::{Checker, DISCOVERING_SYMBOLS_TIP, declared, infer, members, params};

/// NewStaticRule, then InstantiationRule (level 0: no argument types).
pub fn instantiation<'arena>(c: &mut Checker<'_, 'arena>, new: &Instantiation<'arena>) {
    if matches!(new.class, Expression::Static(_)) {
        new_static(c, new);
    }
    match new.class {
        Expression::Identifier(id) => {
            let name = c.resolved(id);
            named(c, new, &name, true, Pair::name(name.clone(), id.span()));
        }
        Expression::Static(_) | Expression::Self_(_) | Expression::Parent(_) => {
            let name = c.written(new.class.span()).to_string();
            named(
                c,
                new,
                &name,
                true,
                Pair::node(name.clone(), new.class.span()),
            );
        }
        expr => {
            // php-parser drops the parentheses of `new ($expr)`
            let expr = super::types::flow::unparen(expr);
            if c.toggled.on.contains(crate::rules::driver::Toggles::NEW_ON_NON_OBJECT) && non_object(c, new, expr) {
                return;
            }
            for (name, is_name) in class_names(c, expr) {
                let pair = Pair::node(name.clone(), expr.span());
                named(c, new, &name, is_name, pair);
            }
        }
    }
}

/// `checkClassNameExprType()` (`featureToggles.newOnNonObject`): `new $expr` whose type is not `string|object`.
/// Whether it reported, which ends the rule for this node.
fn non_object(c: &mut Checker, new: &Instantiation, expr: &Expression) -> bool {
    let accepted = Type::Union(vec![Type::String, Type::ObjectWithoutClass]);
    let accepts = |t: &Type| pun_phpstan::types::is_super_type_of(&accepted, t).yes();
    let described = declared::with_classes(c, || {
        c.find_type_to_check(expr, &accepts)
            .ty
            .filter(|t| !accepts(t))
            .map(|t| t.describe(pun_phpstan::types::Level::TypeOnly))
    });
    let Some(described) = described else {
        return false;
    };
    c.report(
        new,
        "new.nonObject",
        format!("Cannot instantiate class using {described}."),
        None,
    );
    true
}

/// `getClassNames()` for `new $expr`: the class-strings' concrete classes, else the constant strings (as names)
/// and the object types' classes (not as names).
fn class_names(c: &Checker, expr: &Expression) -> Vec<(String, bool)> {
    let Some(mut ty) = c.phpstan_type(expr) else {
        return Vec::new();
    };
    // Mago erases unknown objects to mixed/never. Restore only this expression's own source edge, and only
    // an object naming an unknown class; a concrete inferred narrowing must never regain a broader declaration.
    if matches!(ty, Type::Mixed { .. } | Type::Never(_))
        && let Some(source) = object_source(c, expr.span())
        && names(&source, false)
            .iter()
            .any(|n| matches!(class_like(c, n), Lookup::NotFound))
    {
        ty = source;
    }
    if let Some(object) = class_string_object(c, &ty)
        // UnionType::getObjectClassReflections(): no reflections if any alternative cannot be reflected.
        && let Some(classes) = names(&object, false).iter().map(|n| class_like(c, n).found()).collect::<Option<Vec<_>>>()
    {
        let concretes: Vec<(String, bool)> = classes
            .into_iter()
            .filter(|k| !k.flags.is_abstract() && !k.kind.is_interface())
            // Mago's anonymous reflection key, just as getName() supplies PHPStan's internal key.
            .map(|k| (k.original_name.to_string(), true))
            .collect();
        if !concretes.is_empty() {
            return concretes;
        }
    }
    names(&ty, true)
        .into_iter()
        .map(|n| (n, true))
        .chain(names(&ty, false).into_iter().map(|n| (n, false)))
        .collect()
}

/// `getConstantStrings()` or `getObjectClassNames()`: unions require names in every alternative, intersections
/// collect their members, and a template delegates to its bound. In particular `Object|'Name'` has neither.
fn names(ty: &Type, strings: bool) -> Vec<String> {
    match ty {
        Type::ConstString(s) if strings => vec![String::from_utf8_lossy(s).into_owned()],
        Type::Template(t) => names(&t.bound, strings),
        Type::Union(parts) | Type::Benevolent(parts) | Type::Intersection(parts) => {
            let mut out = Vec::new();
            for part in parts {
                let inner = names(part, strings);
                if inner.is_empty() && !matches!(ty, Type::Intersection(_)) {
                    return Vec::new();
                }
                for n in inner {
                    if !out.contains(&n) {
                        out.push(n);
                    }
                }
            }
            out
        }
        _ if !strings => ty.object_class_names(),
        _ => Vec::new(),
    }
}

/// `isClassString()->yes()` followed by `getClassStringObjectType()`, including templates and compound bounds.
fn class_string_object(c: &Checker, ty: &Type) -> Option<Type> {
    match ty {
        Type::ClassString(object) => Some(
            object
                .as_deref()
                .cloned()
                .unwrap_or(Type::ObjectWithoutClass),
        ),
        Type::ConstString(n) if class_like(c, &String::from_utf8_lossy(n)).found().is_some() => {
            Some(Type::Object(String::from_utf8_lossy(n).into_owned(), vec![], false))
        }
        // TemplateTypeFactory has no TemplateGenericClassStringType: a direct class-string bound falls back
        // to TemplateMixedType. Its union/intersection and ordinary string template subclasses retain bounds.
        Type::Template(t) if !matches!(t.bound, Type::ClassString(_)) => {
            class_string_object(c, &t.bound)
        }
        Type::Union(parts) | Type::Benevolent(parts) => {
            let objects = parts
                .iter()
                .map(|t| class_string_object(c, t))
                .collect::<Option<Vec<_>>>()?;
            Some(declared::with_classes(c, || union(objects)))
        }
        Type::Intersection(parts) => {
            let objects: Vec<_> = parts
                .iter()
                .filter_map(|t| class_string_object(c, t))
                .collect();
            (!objects.is_empty()).then(|| declared::with_classes(c, || intersect(objects)))
        }
        _ => None,
    }
}

/// Converted anonymous objects use getDisplayName(); reflection still needs Mago's internal declaration key.
fn class_like<'a>(c: &Checker<'a, '_>, name: &str) -> Lookup<'a> {
    if name.starts_with("class@anonymous/") {
        return c
            .codebase
            .class_likes
            .values()
            .find(|k| k.name.as_bytes().starts_with(b"{anonymous-class:") && c.display(k) == name)
            .map_or(Lookup::NotFound, |k| Lookup::Found(k));
    }
    c.class_like(name)
}

/// The bounded erased-object fallback: existing provenance already forgets writes, reference escapes and joins.
fn object_source(c: &Checker, at: Span) -> Option<Type> {
    match c.origins.get(&at)? {
        infer::Origin::Copy(from) => object_source(c, *from).or_else(|| {
            // A copied `new Unknown` has no declaration edge. Its own AST and inferred never establish the
            // name; reuse conversion's existing unknown-instantiation contract, not a textual name guess.
            if !c.types.get(from)?.is_never() {
                return None;
            }
            struct Find<'a, 'c, 'arena> {
                c: &'a Checker<'c, 'arena>,
                at: Span,
                ty: Option<Type>,
            }
            impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Find<'_, '_, 'arena> {
                fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
                    if e.span() == self.at {
                        self.ty = self.c.phpstan_type(e);
                    }
                }
            }
            let arena = LocalArena::new();
            let (program, _) = crate::codebase::parse(&arena, c.file);
            let mut find = Find {
                c,
                at: *from,
                ty: None,
            };
            find.walk_program(program, &mut ());
            find.ty
        }),
        _ => None,
    }
}

/// `checkClassName()` for a class given by name, `pair` the name and the node `new` names it with.
fn named<'arena>(
    c: &mut Checker<'_, 'arena>,
    new: &Instantiation<'arena>,
    name: &str,
    is_name: bool,
    pair: Pair,
) {
    let lower = name.to_ascii_lowercase();
    let (class, is_static) = match (lower.as_str(), c.class_scope()) {
        ("static" | "self" | "parent", None) => {
            c.report(
                new,
                &format!("outOfClass.{lower}"),
                format!("Using {name} outside of class scope."),
                None,
            );
            return;
        }
        ("static", Some(class)) => {
            if !class.flags.is_final()
                && (!has_constructor(c, class) || !consistent_constructor(c, class))
            {
                return;
            }
            (class, true)
        }
        ("self", Some(class)) => (class, false),
        ("parent", Some(scope)) => {
            match scope
                .direct_parent_class
                .map(|p| c.class_like(&String::from_utf8_lossy(p.as_bytes())))
            {
                Some(Lookup::Found(parent)) => (parent, false),
                Some(Lookup::Opaque) => return,
                None | Some(Lookup::NotFound) => {
                    let action = format!(
                        "{}::{}() calls new parent",
                        c.display(scope),
                        c.function_name()
                    );
                    c.report_no_parent(new, scope, &action);
                    return;
                }
            }
        }
        _ => match class_like(c, name) {
            Lookup::Found(class) => {
                class_names::check_at(
                    c,
                    &[pair],
                    Some(&class_names::Location::Instantiation),
                    true,
                );
                (class, false)
            }
            Lookup::Opaque => {
                class_names::check(c, &[pair], true);
                return;
            }
            Lookup::NotFound if c.in_class_exists(name) => return,
            Lookup::NotFound => {
                c.report(
                    new,
                    "class.notFound",
                    format!("Instantiated class {name} not found."),
                    Some(DISCOVERING_SYMBOLS_TIP),
                );
                return;
            }
        },
    };
    check_class(
        c,
        new.span(),
        new.argument_list.as_ref().map(params::count),
        class,
        is_static,
        is_name,
    );
}

/// InstantiationRule on `new class(...) { ... }`: the class is the anonymous class itself.
pub fn anonymous(c: &mut Checker, a: &AnonymousClass, name: &str) {
    if let Some(class) = c.codebase.get_class_like(name.as_bytes()) {
        let args = a.argument_list.as_ref().map(params::count_partial);
        check_class(c, a.span(), args, class, false, true);
    }
}

/// `checkClassName()` on a found class, `args` the [`params::count()`] of the arguments.
fn check_class(
    c: &mut Checker,
    new: Span,
    args: Option<params::Counts>,
    class: &ClassLikeMetadata,
    is_static: bool,
    is_name: bool,
) {
    let display = c.display(class);
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
    let Some(constructor) = members::method(c, class, b"__construct") else {
        if args.is_some_and(|a| a.any) {
            c.report(
                new,
                "new.noConstructor",
                format!("Class {display} does not have a constructor and must be instantiated without any parameters."),
                None,
            );
        }
        return;
    };
    let declaring = constructor.declaring_display(c);
    let visibility = constructor.visibility();
    if !super::calls::can_call(
        c,
        c.class_scope(),
        &constructor.declaring_name(),
        visibility,
    ) {
        let v = super::restricted(visibility);
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
    let counts = args.unwrap_or_default();
    let mut acceptor = params::select(c, constructor.variants(), &counts);
    in_class_context(c, &mut acceptor, class);
    params::check(
        c,
        &acceptor,
        &counts,
        new,
        params::Call {
            kind: params::CallKind::New,
            subject: &format!("Class {display} constructor"),
            scope: None,
            void_message: None,
        },
        false,
    );
}

/// `getConstructor()` of a class without templates of its own resolves the declaring ancestor's class templates with
/// the arguments the class gives it (`@extends`, bug 14138).
fn in_class_context(c: &Checker, acceptor: &mut params::Acceptor, class: &ClassLikeMetadata) {
    let Some(declaring) = acceptor
        .class
        .as_deref()
        .filter(|d| !d.as_bytes().eq_ignore_ascii_case(class.name.as_bytes()))
        .and_then(|d| c.codebase.get_class_like(d.as_bytes()))
    else {
        return;
    };
    let names: Vec<String> = declared::class_templates(c, declaring)
        .into_iter()
        .map(|t| t.name)
        .collect();
    if names.is_empty() || !declared::class_templates(c, class).is_empty() {
        return;
    }
    let ancestor = declared::original_name(declaring);
    let Some(args) = declared::with_classes(c, || {
        pun_phpstan::symbols::lookup(&declared::original_name(class))
            .and_then(|k| pun_phpstan::types::ancestor_args(&k, &ancestor, &[]))
    }) else {
        return;
    };
    let scope = format!("class {ancestor}");
    for p in 0..acceptor.param_types.len() {
        if acceptor.param_types[p] != params::ParamType::User {
            continue;
        }
        let Some(t) = super::types::declared_type(c, acceptor, p) else {
            continue;
        };
        acceptor.param_types[p] =
            params::ParamType::Typed(super::types::with_args(&t, &scope, &names, &args));
    }
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
    let Some(constructor) = members::method(c, class, b"__construct") else {
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
        .filter_map(|a| members::method(c, a, b"__construct"))
        .any(|m| m.declared_in_interface() || m.is_abstract())
}

fn has_constructor(c: &Checker, class: &ClassLikeMetadata) -> bool {
    members::method(c, class, b"__construct").is_some()
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
        && let Some(t) = c.in_trait()
        && let Some(t) = c.codebase.get_class_like(t.as_bytes())
        && members::method(c, t, b"__construct").is_some_and(|m| m.is_abstract())
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
