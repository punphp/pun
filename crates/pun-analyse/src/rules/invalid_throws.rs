//! PHPStan 2.3.1's InvalidThrowsPhpDocValueRule (level 2). Raw class/function/method declarations are skipped:
//! their virtual nodes supply the declaration's own name/template scope. Hooks are virtual nodes too; closure
//! expressions are not statements and do not acquire a declaration check.

use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::{AnonymousClass, Function, Method, Statement};
use pun_phpstan::phpdoc::{self, TagValue};
use pun_phpstan::resolve::resolve_in;
use pun_phpstan::types::{Level, Type, intersect, is_super_type_of, union};

use super::doc::{Doc, NameScope, doc_before};
use super::{Checker, declared, require_tags, var_tags};

pub const IDENTIFIERS: &[&str] = &["throws.notThrowable"];

/// `processNode()` for php-parser's real statements. The virtual declaration entry points run in their new scope.
pub fn statement(c: &mut Checker, statement: &Statement) {
    if matches!(
        statement,
        Statement::Class(_)
            | Statement::Interface(_)
            | Statement::Enum(_)
            | Statement::Trait(_)
            | Statement::Function(_)
            | Statement::OpeningTag(_)
            | Statement::ClosingTag(_)
            | Statement::Inline(_)
            | Statement::Block(_)
    ) {
        return;
    }
    node(c, statement.span());
}

/// `InFunctionNode`: not an inherited reflection throw type, and not the surrounding function's templates.
pub fn function(c: &mut Checker, function: &Function) {
    check(c, function.span(), |c, raw| {
        declared::function_scope(c, function, &Doc::parse(raw))
    });
}

/// `InClassMethodNode`: the method's own tags, resolved in the declaring trait's/using class's context.
pub fn method(c: &mut Checker, method: &Method) {
    check(c, method.span(), |c, raw| {
        let context = c.class_scope().expect("a method has a class scope");
        let declaring =
            super::missing_types::declaring(c, method.span()).expect("a method has a declaration");
        declared::method_scope(c, method, &Doc::parse(raw), declaring, context)
    });
}

/// The anonymous `InClassNode` starts at its attributes/modifiers/`class`, not its enclosing `new` expression.
pub fn anonymous(c: &mut Checker, class: &AnonymousClass) {
    let start = class
        .attribute_lists
        .first()
        .map(HasSpan::span)
        .or_else(|| class.modifiers.first().map(HasSpan::span))
        .unwrap_or_else(|| class.class.span());
    node(c, start.join(class.span()));
}

/// Other statements and virtual class/hook nodes: FileTypeMapper's class and enclosing function name scope.
pub fn node(c: &mut Checker, at: Span) {
    check(c, at, |c, _| statement_scope(c, at));
}

/// FileTypeMapper also pushes a hook's own template map, both for its virtual node and its body statements.
/// Use the source hook's span/name, including when walking a trait's property in a using class's context.
fn statement_scope(c: &Checker, at: Span) -> NameScope {
    let scope = var_tags::scope(c, at);
    let Some(context) = c.class_scope() else {
        return scope;
    };
    let owner = c
        .in_trait()
        .and_then(|name| c.codebase.get_class_like(name.as_bytes()))
        .unwrap_or(context);
    for property in owner.properties.values() {
        for hook in property.hooks.values() {
            if hook.span.file_id == at.file_id
                && hook.span.start.offset <= at.start.offset
                && at.end.offset <= hook.span.end.offset
            {
                let raw = doc_before(&c.file.contents, hook.span.start.offset as usize)
                    .unwrap_or_default();
                let name = format!("{}::{}", property.name.0, hook.name);
                return declared::with_method_templates(
                    &scope,
                    context,
                    &name,
                    &Doc::parse(raw).templates,
                );
            }
        }
    }
    scope
}

fn check(c: &mut Checker, at: Span, scope: impl FnOnce(&Checker, &[u8]) -> NameScope) {
    if c.level < 2 {
        return;
    }
    let Some(raw) = doc_before(&c.file.contents, at.start.offset as usize) else {
        return;
    };
    let message = declared::with_classes(c, || {
        let scope = scope(c, raw);
        let ty = throws_type(raw, &scope)?;
        (!is_throws_valid(c, &ty)).then(|| {
            format!(
                "PHPDoc tag @throws with type {} is not subtype of Throwable",
                ty.describe(Level::TypeOnly)
            )
        })
    });
    if let Some(message) = message {
        c.report(at, "throws.notThrowable", message, None);
    }
}

/// `PhpDocNodeResolver::resolveThrowsTags()`: all valid tags of the first supported name, unioned. A malformed
/// preferred tag does not hide valid ordinary tags, and unrelated tools' tag names are not aliases.
fn throws_type(raw: &[u8], scope: &NameScope) -> Option<Type> {
    let tags = phpdoc::tags(&String::from_utf8_lossy(raw)).unwrap_or_default();
    for name in ["@phpstan-throws", "@throws"] {
        let types: Vec<_> = tags
            .iter()
            .filter(|t| t.name == name)
            .filter_map(|t| match &t.value {
                TagValue::Type(node) => Some(resolve_in(node, scope)),
                _ => None,
            })
            .collect();
        if !types.is_empty() {
            return Some(union(types));
        }
    }
    None
}

/// `InvalidThrowsPhpDocValueRule::isThrowsValid()`, in its upstream branch order.
fn is_throws_valid(c: &Checker, ty: &Type) -> bool {
    if ty.is_void().yes() {
        return true;
    }
    if let Type::Conditional(conditional) = ty {
        return is_throws_valid(c, &conditional.then) && is_throws_valid(c, &conditional.otherwise);
    }
    if let Type::Union(types) = ty {
        return types
            .iter()
            .all(|inner| !inner.is_void().yes() && is_throws_valid(c, inner));
    }
    let mut constrained = vec![ty.clone()];
    // TemplateObjectType inherits ObjectType's class reflections. The bound supplies those reflections, not an
    // unrelated declaration or native inference flag; the actual throws type remains the template itself.
    let mut reflected = ty;
    while let Type::Template(template) = reflected {
        reflected = &template.bound;
    }
    for name in reflected.object_class_names() {
        if let Some(class) = c.codebase.get_class_like(name.as_bytes())
            && class.kind == SymbolKind::Interface
        {
            constrained.extend(require_tags::tag_types(
                c,
                class,
                ["@psalm-require-extends", "@phpstan-require-extends"],
            ));
        }
    }
    is_super_type_of(
        &Type::Object("Throwable".into(), vec![]),
        &intersect(constrained),
    )
    .yes()
}
