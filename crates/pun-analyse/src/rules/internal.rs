//! RestrictedMethodUsageRule and RestrictedStaticMethodUsageRule with RestrictedInternalMethodUsageExtension
//! (config.level2.neon, under `featureToggles.internalTag`): a call to an `@internal` method, or a method of an
//! `@internal` class-like, from outside the root namespace that declares it.

use std::collections::HashSet;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::Checker;
use super::members::Method;

/// The lowercase names of the methods the rules can report: `@internal` ones and those of `@internal` class-likes.
/// A method call whose name is not among them needs no type.
pub struct Names(HashSet<Vec<u8>>);

impl Names {
    pub fn of(codebase: &CodebaseMetadata) -> Names {
        let internal_class = |class: &[u8]| {
            codebase
                .get_class_like(class)
                .is_some_and(|k| k.flags.is_internal())
        };
        Names(
            codebase
                .function_likes
                .iter()
                .filter(|((class, _), f)| {
                    !class.is_empty() && (f.flags.is_internal() || internal_class(class.as_bytes()))
                })
                .map(|((_, method), _)| method.as_bytes().to_ascii_lowercase())
                .collect(),
        )
    }

    fn contains(&self, name: &[u8]) -> bool {
        self.0.contains(&name.to_ascii_lowercase())
    }
}

/// The calls whose object or class expression the rules type.
struct Typed<'n> {
    names: &'n Names,
    spans: Vec<Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Typed<'_> {
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        if let ClassLikeMemberSelector::Identifier(id) = &call.method
            && self.names.contains(id.value)
        {
            self.spans.push(call.object.span());
        }
    }
    fn walk_in_null_safe_method_call(
        &mut self,
        call: &'ast NullSafeMethodCall<'arena>,
        _: &mut (),
    ) {
        if let ClassLikeMemberSelector::Identifier(id) = &call.method
            && self.names.contains(id.value)
        {
            self.spans.push(call.object.span());
        }
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.static_class(call.class, &call.method);
    }
    fn walk_in_method_partial_application(&mut self, m: &'ast MethodPartialApplication<'arena>, _: &mut ()) {
        if let ClassLikeMemberSelector::Identifier(id) = &m.method
            && self.names.contains(id.value)
        {
            self.spans.push(m.object.span());
        }
    }
    fn walk_in_static_method_partial_application(
        &mut self,
        m: &'ast StaticMethodPartialApplication<'arena>,
        _: &mut (),
    ) {
        self.static_class(m.class, &m.method);
    }
}

impl Typed<'_> {
    fn static_class(&mut self, class: &Expression, method: &ClassLikeMemberSelector) {
        if let ClassLikeMemberSelector::Identifier(id) = method
            && self.names.contains(id.value)
            && !matches!(
                class,
                Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
            )
        {
            self.spans.push(class.span());
        }
    }
}

/// The expressions whose types the rules read.
pub fn typed_expressions(names: Option<&Names>, program: &Program) -> Vec<Span> {
    let Some(names) = names else {
        return Vec::new();
    };
    let mut typed = Typed {
        names,
        spans: Vec::new(),
    };
    typed.walk_program(program, &mut ());
    typed.spans
}

/// The classes of an expression's object types (`getObjectClassNames()`), as Mago infers them.
fn object_classes<'a>(c: &Checker<'a, '_>, e: &Expression) -> Vec<&'a ClassLikeMetadata> {
    c.type_of(e)
        .into_iter()
        .flat_map(|t| t.types.iter())
        .filter_map(|a| match a {
            TAtomic::Object(TObject::Named(o)) => c.codebase.get_class_like(o.name.as_bytes()),
            TAtomic::Object(TObject::Enum(o)) => c.codebase.get_class_like(o.name.as_bytes()),
            _ => None,
        })
        .collect()
}

/// `$obj->name()` and `$obj?->name()` (`call` spans the whole call: the error is on its first line).
pub fn method_call(
    c: &mut Checker,
    call: Span,
    object: &Expression,
    method: &ClassLikeMemberSelector,
) {
    let ClassLikeMemberSelector::Identifier(id) = method else {
        return;
    };
    if !c.internal.is_some_and(|n| n.contains(id.value)) {
        return;
    }
    for k in object_classes(c, object) {
        if let Some(m) = super::members::method(c, k, id.value)
            && let Some((message, identifier)) = restricted(c, &m, None)
        {
            c.report(call, &identifier, message, None);
        }
    }
}

/// `Name::name()`, `self::`/`static::`/`parent::name()` and `$expr::name()`.
pub fn static_method_call(c: &mut Checker, call: &StaticMethodCall) {
    static_method(c, call.span(), call.class, &call.method);
}

/// `$obj->name(...)` (RestrictedMethodCallableUsageRule).
pub fn method_callable(c: &mut Checker, m: &MethodPartialApplication) {
    if is_callable(&m.argument_list) {
        method_call(c, m.span(), m.object, &m.method);
    }
}

/// `Name::name(...)` (RestrictedStaticMethodCallableUsageRule).
pub fn static_method_callable(c: &mut Checker, m: &StaticMethodPartialApplication) {
    if is_callable(&m.argument_list) {
        static_method(c, m.span(), m.class, &m.method);
    }
}

/// A first-class callable's `(...)`.
fn is_callable(args: &PartialArgumentList) -> bool {
    matches!(args.arguments.iter().collect::<Vec<_>>().as_slice(), [PartialArgument::VariadicPlaceholder(_)])
}

fn static_method(c: &mut Checker, span: Span, class: &Expression, method: &ClassLikeMemberSelector) {
    let ClassLikeMemberSelector::Identifier(id) = method else {
        return;
    };
    if !c.internal.is_some_and(|n| n.contains(id.value)) {
        return;
    }
    let classes = match class {
        Expression::Identifier(name) => c
            .codebase
            .get_class_like(c.resolved(name).as_bytes())
            .into_iter()
            .collect(),
        Expression::Self_(_) | Expression::Static(_) => c.class_scope().into_iter().collect(),
        Expression::Parent(_) => c
            .class_scope()
            .and_then(|k| k.direct_parent_class)
            .and_then(|p| c.codebase.get_class_like(p.as_bytes()))
            .into_iter()
            .collect(),
        e => object_classes(c, e),
    };
    for k in classes {
        let Some(m) = super::members::method(c, k, id.value) else {
            continue;
        };
        let Some((message, identifier)) = restricted(c, &m, None) else {
            continue;
        };
        // RewrittenDeclaringClassMethodReflection: the call must stay restricted with the class it names declaring it
        if !k
            .name
            .as_bytes()
            .eq_ignore_ascii_case(m.declaring_name().as_bytes())
            && restricted(c, &m, Some(k)).is_none()
        {
            continue;
        }
        c.report(span, &identifier, message, None);
    }
}

/// RestrictedInternalMethodUsageExtension::isRestrictedMethodUsage(): the message and identifier, with `declaring`
/// standing in for the method's declaring class when given.
fn restricted(
    c: &Checker,
    m: &Method,
    declaring: Option<&ClassLikeMetadata>,
) -> Option<(String, String)> {
    let Method::User { declaring: own, f } = m else {
        return None;
    };
    let declaring = declaring.unwrap_or(own);
    let method_internal = f.flags.is_internal();
    if !method_internal && !declaring.flags.is_internal() {
        return None;
    }
    let declaring_name = String::from_utf8_lossy(declaring.original_name.as_bytes()).into_owned();
    if !should_be_reported(c, &declaring_name) {
        return None;
    }
    let (kind, prefix) = if m.is_static() {
        ("staticMethod", "static ")
    } else {
        ("method", "")
    };
    let description = match declaring.kind {
        SymbolKind::Interface => "Interface",
        SymbolKind::Trait => "Trait",
        SymbolKind::Enum => "Enum",
        _ => "Class",
    };
    let display = c.display(declaring);
    let name = m.name();
    let outside = match root_namespace(&declaring_name) {
        Some(ns) => format!(" from outside its root namespace {ns}"),
        None => String::new(),
    };
    Some(if method_internal {
        (
            format!("Call to internal {prefix}method {display}::{name}(){outside}."),
            format!("{kind}.internal"),
        )
    } else {
        (
            format!(
                "Call to {prefix}method {name}() of internal {} {display}{outside}.",
                description.to_ascii_lowercase()
            ),
            format!("{kind}.internal{description}"),
        )
    })
}

/// The first segment of a name's namespace (`array_slice(explode('\\', $name), 0, -1)[0]`).
pub(super) fn root_namespace(name: &str) -> Option<&str> {
    let (namespace, _) = name.rsplit_once('\\')?;
    namespace.split('\\').next()
}

/// RestrictedInternalUsageHelper::shouldBeReported().
pub(super) fn should_be_reported(c: &Checker, name: &str) -> bool {
    match c.namespaces.last().cloned().flatten() {
        None => match c.class_scope() {
            None => true,
            Some(k) => !k.original_name.as_bytes().eq(name.as_bytes()),
        },
        Some(current) => {
            let current = current.split('\\').next().unwrap_or_default();
            let namespace = root_namespace(name).unwrap_or_default();
            !format!("{namespace}\\").starts_with(&format!("{current}\\"))
        }
    }
}
