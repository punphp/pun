//! InvalidPhpDocVarTagTypeRule (level 2): the `@var` tags of the doc comment of each php-parser statement but a
//! property, constant, class-like, function or method: unresolvable types, unknown classes, traits and generic
//! types (the `checkMissingVarTagTypehint` part is level 6).

use mago_span::{Position, Span};
use pun_phpstan::types::{Template, Type};


use super::{Checker, DISCOVERING_SYMBOLS_TIP, FunctionScopeKind, declared, generics};

pub(super) const IDENTIFIERS: &[&str] = &["varTag.unresolvableType", "varTag.trait"];

/// The `NameScope` FileTypeMapper resolves a statement's doc in: the class's, with the templates of the function
/// or method the statement is in.
pub(super) fn scope(c: &Checker, at: Span) -> super::doc::NameScope {
    let k = c.class_scope();
    let mut scope = declared::scope_at(c, at, k);
    let (Some(function), Some(span)) = (c.functions.last(), c.var_tag_functions.last()) else {
        return scope;
    };
    let Some((_, entry)) = c.doc_before(c.file.id, span.start.offset) else {
        return scope;
    };
    let described = match (function.kind, k) {
        (FunctionScopeKind::Method { .. }, Some(k)) => {
            format!("method {}::{}()", declared::original_name(k), function.name)
        }
        _ => format!("function {}()", function.name),
    };
    for t in &entry.doc.templates {
        let template = Template {
            name: t.name.clone(),
            scope: described.clone(),
            bound: t.bound.as_ref().map_or(Type::Mixed { explicit: false }, |b| {
                pun_phpstan::resolve::resolve_in(b, &scope)
            }),
            default: t
                .default
                .as_ref()
                .map(|d| pun_phpstan::resolve::resolve_in(d, &scope)),
            argument: false,
        };
        scope
            .templates
            .insert(t.name.clone(), Type::Template(Box::new(template)));
    }
    scope
}

/// The statement at `at`.
pub(super) fn node(c: &mut Checker, at: Span) {
    check(c, at, at);
}

/// The statement at `at`, its doc's names those at `names` (a `Nop` after a namespace's last statement is still in
/// that namespace).
pub(super) fn check(c: &mut Checker, at: Span, names: Span) {
    if c.level < 2 {
        return;
    }
    let file = c.file;
    let Some((raw, entry)) = c.doc_before(file.id, at.start.offset) else {
        return;
    };
    let start = raw.as_ptr() as usize - file.contents.as_ptr() as usize;
    let doc_line = Span {
        file_id: at.file_id,
        start: Position {
            offset: start as u32,
        },
        end: Position {
            offset: start as u32,
        },
    };
    let scope = scope(c, names);
    let mut errors: Vec<(Span, &'static str, String, Option<&'static str>)> = Vec::new();
    let mut classes = Vec::new();
    declared::with_classes(c, || {
        for (name, ty) in entry.doc.var_types(&scope) {
            let what = if name.is_empty() {
                "PHPDoc tag @var".to_string()
            } else {
                format!("PHPDoc tag @var for variable ${name}")
            };
            let mut unresolvable = false;
            super::type_aliases::each(&ty, &mut |t| {
                unresolvable |= matches!(t, Type::Error | Type::Never(pun_phpstan::types::NeverKind::Implicit));
            });
            if unresolvable {
                errors.push((
                    doc_line,
                    "varTag.unresolvableType",
                    format!("{what} contains unresolvable type."),
                    None,
                ));
                continue;
            }
            let s = generics::escape(&what);
            let messages = generics::GenericMessages::at(&s);
            let mut found = generics::Found::default();
            generics::generic_object_check(&ty, &messages, &mut found);
            errors.extend(found.errors.into_iter().map(|(id, m, tip)| (at, id, m, tip)));
            for class in generics::referenced_classes(&ty) {
                match pun_phpstan::symbols::lookup(&class) {
                    Some(k) if k.is_trait => errors.push((
                        at,
                        "varTag.trait",
                        format!("{what} has invalid type {class}."),
                        None,
                    )),
                    Some(_) => {}
                    None if c.in_class_exists(&class) => {}
                    None => errors.push((
                        at,
                        "class.notFound",
                        format!("{what} contains unknown class {class}."),
                        Some(DISCOVERING_SYMBOLS_TIP),
                    )),
                }
                classes.push(class);
            }
        }
    });
    let pairs: Vec<_> = classes
        .into_iter()
        .map(|name| super::class_names::Pair::node(name, at))
        .collect();
    let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
    super::class_names::check(c, &pairs, case);
    for (at, identifier, message, tip) in errors {
        c.report(at, identifier, message, tip);
    }
}
