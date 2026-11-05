//! The rules on a function-like's throw points ([`super::throw_points`]): MissingCheckedExceptionIn*ThrowsRule
//! (`missingType.checkedException`, with `exceptions.check.missingCheckedExceptionInThrows`), TooWide*ThrowTypeRule
//! (`throws.unusedType`, level 4, `exceptions.check.tooWideThrowType`) and CatchWithUnthrownExceptionRule's
//! `catch.neverThrown` (level 4), with DefaultExceptionTypeResolver's checked exceptions.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Type, is_super_type_of, union};

use super::super::{Checker, declared, invalid_throws, var_tags};
use super::missing_return::method_name;
use super::throw_points::{Point, Points, flatten, throwable};

pub const IDENTIFIERS: &[&str] = &["throws.unusedType", "catch.neverThrown"];

/// DefaultExceptionTypeResolver::isCheckedException().
pub(in super::super) fn is_checked_exception(c: &Checker, name: &str) -> bool {
    let e = c.exceptions;
    let matches = |regexes: &[String]| {
        regexes.iter().any(|r| pun_phpstan::php::preg(r).is_ok_and(|re| pun_phpstan::php::is_match(&re, name)))
    };
    if matches(&e.unchecked_exception_regexes) || e.unchecked_exception_classes.iter().any(|u| u == name) {
        return false;
    }
    let class = c.codebase.get_class_like(name.as_bytes());
    let is = |k: &ClassLikeMetadata, other: &str| {
        k.name.as_bytes().eq_ignore_ascii_case(other.as_bytes())
            || c.codebase.is_instance_of(k.name.as_bytes(), other.as_bytes())
    };
    if let Some(k) = class
        && e.unchecked_exception_classes.iter().any(|u| is(k, u))
    {
        return false;
    }
    // isCheckedExceptionInternal()
    if matches(&e.checked_exception_regexes) || e.checked_exception_classes.iter().any(|k| k == name) {
        return true;
    }
    if let Some(k) = class
        && e.checked_exception_classes.iter().any(|checked| is(k, checked))
    {
        return true;
    }
    e.checked_exception_regexes.is_empty() && e.checked_exception_classes.is_empty()
}

/// MissingCheckedExceptionInThrowsCheck::check(): the explicit points' checked exceptions `declared` lacks.
fn missing_checked(c: &mut Checker, declared: Option<&Type>, points: &[Point], of: &str) {
    let mut errors = Vec::new();
    declared::with_classes(c, || {
        for p in points.iter().filter(|p| p.explicit) {
            for t in flatten(&p.ty) {
                if is_super_type_of(&t, &throwable()).yes() || declared.is_some_and(|d| is_super_type_of(d, &t).yes()) {
                    continue;
                }
                if !t.object_class_names().iter().any(|n| is_checked_exception(c, n)) {
                    continue;
                }
                errors.push((
                    p.at,
                    format!(
                        "{of} throws checked exception {} but it's missing from the PHPDoc @throws tag.",
                        t.describe(Level::TypeOnly)
                    ),
                ));
            }
        }
    });
    for (at, message) in errors {
        c.report(at, "missingType.checkedException", message, None);
    }
}

/// TooWideThrowTypeCheck::check(): the `@throws` types no throw point may throw.
fn unused(c: &Checker, declared: &Type, points: &[Point]) -> Vec<String> {
    if declared.is_void().yes() {
        return Vec::new();
    }
    declared::with_classes(c, || {
        let thrown = union(
            points
                .iter()
                .map(|p| {
                    if !c.exceptions.implicit_throws && !p.explicit && !p.unknown {
                        Type::Never(pun_phpstan::types::NeverKind::Implicit)
                    } else {
                        p.ty.clone()
                    }
                })
                .collect(),
        );
        flatten(declared)
            .into_iter()
            .filter(|t| thrown.is_never() || is_super_type_of(t, &thrown).no())
            .map(|t| t.describe(Level::TypeOnly))
            .collect()
    })
}

fn too_wide_on(c: &Checker) -> bool {
    c.level >= 4 && c.exceptions.too_wide_throw_type
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    let missing = c.exceptions.missing_checked_exception_in_throws;
    if !missing && !too_wide_on(c) {
        return;
    }
    let name = c.resolved_span(f.name.span());
    let Some(meta) = c.codebase.get_function(name.as_bytes()) else { return };
    let declared = declared::with_classes(c, || declared::reflected_throws(c, meta, None, &name));
    let mut p = Points::new(c);
    p.walk_block(&f.body, &mut ());
    let points = p.points;
    let of = format!("Function {name}()");
    if missing {
        missing_checked(c, declared.as_ref(), &points, &of);
    }
    if too_wide_on(c)
        && let Some(declared) = &declared
    {
        for class in unused(c, declared, &points) {
            let message = format!("{of} has {class} in PHPDoc @throws tag but it's not thrown.");
            c.report(f.span(), "throws.unusedType", message, None);
        }
    }
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    let (MethodBody::Concrete(b), Some(context)) = (&m.body, c.class_scope()) else { return };
    let missing = c.exceptions.missing_checked_exception_in_throws;
    if !missing && !too_wide_on(c) {
        return;
    }
    let name = method_name(c, m);
    let Some(meta) = c.codebase.get_method(context.name.as_bytes(), name.to_ascii_lowercase().as_bytes()) else {
        return;
    };
    let declared = declared::with_classes(c, || declared::reflected_throws(c, meta, Some(context), &name));
    let mut p = Points::new(c);
    p.walk_block(b, &mut ());
    let points = p.points;
    let of = format!("Method {}::{name}()", c.display(context));
    if missing {
        missing_checked(c, declared.as_ref(), &points, &of);
    }
    let Some(declared) = declared.filter(|_| too_wide_on(c)) else { return };
    // TooWideMethodThrowTypeRule: private or final methods (or of a final class) unless
    // checkTooWideThrowTypesInProtectedAndPublicMethods, which still skips a method's first declaration
    let private = meta.method_metadata.as_ref().is_some_and(|mm| mm.visibility == Visibility::Private);
    let is_final = meta.method_metadata.as_ref().is_some_and(|mm| mm.is_final) || m.modifiers.contains_final();
    if !private && !context.flags.is_final() && !is_final {
        if !c.exceptions.check_too_wide_throw_types_in_protected_and_public_methods
            || first_declaration(c, context, &name)
        {
            return;
        }
    }
    let unused = unused(c, &declared, &points);
    if unused.is_empty() {
        return;
    }
    let explicit = c.doc_before(c.file.id, m.span().start.offset).is_some_and(|(_, entry)| {
        let scope = var_tags::scope(c, m.span());
        declared::with_classes(c, || invalid_throws::throws_type(&entry.phpdoc, &scope)).is_some()
    });
    if !explicit && !c.exceptions.too_wide_implicit_throw_type {
        return;
    }
    let all: Vec<String> = declared::with_classes(c, || flatten(&declared).iter().map(|t| t.describe(Level::TypeOnly)).collect());
    let used: Vec<String> = all.into_iter().filter(|t| !unused.contains(t)).collect();
    let tip = (!explicit).then(|| {
        format!(
            "You can narrow the thrown type with PHPDoc tag @throws {}.",
            if used.is_empty() { "void".to_string() } else { used.join("|") }
        )
    });
    for class in unused {
        let message = format!("{of} has {class} in PHPDoc @throws tag but it's not thrown.");
        c.report(m.span(), "throws.unusedType", message, tip.as_deref());
    }
}

/// `$method->getPrototype()->getDeclaringClass() === $method->getDeclaringClass()`: no ancestor declares it.
fn first_declaration(c: &Checker, class: &ClassLikeMetadata, name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let declared_in = |k: &[u8]| c.codebase.get_method(k, lower.as_bytes()).is_some();
    let parent = class.direct_parent_class.is_some_and(|p| declared_in(p.as_bytes()));
    let interface = class.all_parent_interfaces.iter().any(|i| declared_in(i.as_bytes()));
    !parent && !interface
}

pub(in super::super) fn hooks(c: &mut Checker, name: &[u8], list: &PropertyHookList) {
    let Some(context) = c.class_scope() else { return };
    let missing = c.exceptions.missing_checked_exception_in_throws;
    if !missing && !too_wide_on(c) {
        return;
    }
    let display = c.display(context);
    let variable = name.to_vec();
    let name = name.strip_prefix(b"$").unwrap_or(name);
    let property = c.codebase.get_declaring_property(context.name.as_bytes(), &variable);
    for h in list.hooks.iter() {
        let PropertyHookBody::Concrete(body) = &h.body else { continue };
        let hook = String::from_utf8_lossy(h.name.value).to_ascii_lowercase();
        let mut chars = hook.chars();
        let hook: String = chars.next().map(|f| f.to_ascii_uppercase()).into_iter().chain(chars).collect();
        let of = format!("{hook} hook for property {display}::${}", String::from_utf8_lossy(name));
        let declared = c.doc_before(c.file.id, h.span().start.offset).and_then(|(_, entry)| {
            let scope = var_tags::scope(c, h.span());
            declared::with_classes(c, || invalid_throws::throws_type(&entry.phpdoc, &scope))
        });
        let mut inner = Points::new(c).in_hook(name);
        match body {
            PropertyHookConcreteBody::Block(b) => inner.walk_block(b, &mut ()),
            PropertyHookConcreteBody::Expression(e) => inner.walk_expression(e.expression, &mut ()),
        }
        let points = inner.points;
        if missing {
            missing_checked(c, declared.as_ref(), &points, &of);
        }
        let Some(declared) = declared.filter(|_| too_wide_on(c)) else { continue };
        let private = property.is_some_and(|p| p.read_visibility == Visibility::Private);
        let is_final = property.is_some_and(|p| p.flags.is_final())
            || h.modifiers.iter().any(|m| matches!(m, Modifier::Final(_)));
        if !private && !context.flags.is_final() && !is_final
            && !c.exceptions.check_too_wide_throw_types_in_protected_and_public_methods
        {
            continue;
        }
        for class in unused(c, &declared, &points) {
            let message = format!("{of} has {class} in PHPDoc @throws tag but it's not thrown.");
            c.report(h.span(), "throws.unusedType", message, None);
        }
    }
}

/// The property whose hook body holds `at` (the class's or, walked in a class's context, the trait's).
fn hooked_property(c: &Checker, at: Span) -> Option<Vec<u8>> {
    let context = c.class_scope()?;
    let owner = c.in_trait().and_then(|name| c.codebase.get_class_like(name.as_bytes())).unwrap_or(context);
    owner.properties.values().find_map(|p| {
        p.hooks
            .values()
            .any(|h| h.span.file_id == at.file_id && h.span.start.offset <= at.start.offset && at.end.offset <= h.span.end.offset)
            .then(|| p.name.0.as_bytes()[1..].to_vec())
    })
}

/// CatchWithUnthrownExceptionRule's `catch.neverThrown`: a catch type no throw point of the try block matches.
pub(in super::super) fn dead_catches(c: &mut Checker, t: &Try) {
    if c.level < 4 {
        return;
    }
    let own: Vec<Span> = t.catch_clauses.iter().map(|k| k.r#catch.span()).collect();
    let mut p = match hooked_property(c, t.span()) {
        Some(name) => Points::new(c).in_hook(&name),
        None => Points::new(c),
    };
    p.try_statement(t);
    let dead: Vec<(Span, Type)> = p.dead.into_iter().filter(|(at, _)| own.contains(at)).collect();
    for (at, caught) in dead {
        let (checked, described) = declared::with_classes(c, || {
            (
                caught.object_class_names().iter().any(|n| is_checked_exception(c, n)),
                caught.describe(Level::TypeOnly),
            )
        });
        if !c.exceptions.report_unchecked_exception_dead_catch && !checked {
            continue;
        }
        let message = format!("Dead catch - {described} is never thrown in the try block.");
        c.report(at, "catch.neverThrown", message, None);
    }
}

/// MethodThrowTypeCovarianceRule (`exceptions.check.throwTypeCovariance`): a method's `@throws` against the ones of
/// the methods it overrides (ParentMethodHelper::collectParentMethods()).
pub(in super::super) fn covariance(c: &mut Checker, m: &Method) {
    let Some(context) = c.class_scope() else { return };
    if !c.exceptions.throw_type_covariance {
        return;
    }
    let name = method_name(c, m);
    let lower = name.to_ascii_lowercase();
    if lower == "__construct" || m.modifiers.contains_private() {
        return;
    }
    let Some(meta) = c.codebase.get_method(context.name.as_bytes(), lower.as_bytes()) else { return };
    let implicit = c.exceptions.implicit_throws;
    let Some(throws) = declared::with_classes(c, || declared::reflected_throws(c, meta, Some(context), &name))
        .or_else(|| implicit.then(throwable))
    else {
        return;
    };
    if throws.is_void().yes() {
        return;
    }
    // (declaring class-like, method) of each parent method
    let mut parents: Vec<&ClassLikeMetadata> = Vec::new();
    let declaring = |k: &ClassLikeMetadata| {
        c.codebase.get_declaring_method_class(k.name.as_bytes(), lower.as_bytes()).and_then(|d| c.codebase.get_class_like(d.as_bytes()))
    };
    if let Some(parent) = context.direct_parent_class.and_then(|p| c.codebase.get_class_like(p.as_bytes()))
        && let Some(d) = declaring(parent)
        && c.codebase.get_method(d.name.as_bytes(), lower.as_bytes()).is_some_and(|f| {
            !f.flags.is_magic_method()
                && f.method_metadata.as_ref().is_none_or(|mm| mm.visibility != Visibility::Private)
        })
    {
        parents.push(d);
    }
    for i in context.all_parent_interfaces.iter().filter_map(|i| c.codebase.get_class_like(i.as_bytes())) {
        if let Some(d) = declaring(i) {
            parents.push(d);
        }
    }
    for t in context.used_traits.iter().filter_map(|t| c.codebase.get_class_like(t.as_bytes())) {
        if declaring(t).is_some_and(|d| d.name == t.name)
            && c.codebase.get_method(t.name.as_bytes(), lower.as_bytes()).is_some_and(|f| f.method_metadata.as_ref().is_some_and(|mm| mm.is_abstract))
        {
            parents.push(t);
        }
    }
    let display = c.display(context);
    let mut errors = Vec::new();
    declared::with_classes(c, || {
        for p in parents {
            let Some(f) = c.codebase.get_method(p.name.as_bytes(), lower.as_bytes()) else { continue };
            let parent_name = String::from_utf8_lossy(f.original_name.as_bytes()).into_owned();
            let parent_throws = if p.flags.is_built_in() {
                super::throw_points::builtin_method_throws(&String::from_utf8_lossy(p.name.as_bytes()).to_ascii_lowercase(), &lower)
            } else {
                declared::reflected_throws(c, f, Some(p), &parent_name)
            };
            let (parent_throws, explicit_void) = match parent_throws {
                Some(t) => {
                    let void = t.is_void().yes();
                    (t, void)
                }
                None if implicit => (throwable(), true),
                None => (Type::Void, false),
            };
            let parent_display = if p.flags.is_built_in() {
                c.ancestor_display(context, &String::from_utf8_lossy(p.original_name.as_bytes()))
            } else {
                c.ancestor_display(context, &c.display(p))
            };
            let described = throws.describe(Level::TypeOnly);
            if parent_throws.is_void().yes() {
                let (id, why) = if explicit_void {
                    ("throws.shouldBeVoid", "has PHPDoc tag @throws void")
                } else {
                    ("throws.notCovariantWithImplicitVoid", "does not have PHPDoc tag @throws")
                };
                errors.push((
                    id,
                    format!(
                        "Method {display}::{name}() should not throw {described} because parent method {parent_display}::{parent_name}() {why}."
                    ),
                ));
                continue;
            }
            if is_super_type_of(&parent_throws, &throws).yes() {
                continue;
            }
            errors.push((
                "throws.notCovariant",
                format!(
                    "PHPDoc tag @throws type {described} of method {display}::{name}() should be covariant with PHPDoc @throws type {} of method {parent_display}::{parent_name}().",
                    parent_throws.describe(Level::TypeOnly)
                ),
            ));
        }
    });
    for (id, message) in errors {
        c.report(m.span(), id, message, None);
    }
}
