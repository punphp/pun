//! FunctionPurityCheck, for PureFunctionRule, PureMethodRule and PurePropertyHookRule (level 2): a `@phpstan-pure`
//! declaration's impure points and its by-ref parameters or void return, an `@phpstan-impure` one without side
//! effects, the `@pure-unless-*` tags, and an unspecified declaration's `void.pure`.
//! The unspecified `void.pure` branch keeps its conservative producer (declaration_effects); the others use the
//! impure points of purity_points.

use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::purity_points::{self, Points};
use super::{
    Checker, assert_tags, declaration_effects as effects, declarations, declared, missing_types,
};

/// The identifiers FunctionPurityCheck reports (with `void.pure`).
pub(super) const IDENTIFIERS: &[&str] = &[
    "impure.betweenPhpTags",
    "impure.die",
    "impure.echo",
    "impure.eval",
    "impure.exit",
    "impure.functionCall",
    "impure.global",
    "impure.include",
    "impure.methodCall",
    "impure.new",
    "impure.print",
    "impure.propertyAssign",
    "impure.propertyAssignByRef",
    "impure.propertyUnset",
    "impure.require",
    "impure.static",
    "impure.staticPropertyAccess",
    "impure.superglobal",
    "impure.yield",
    "impure.yieldFrom",
    "impureFunction.pure",
    "impureMethod.pure",
    "impurePropertyHook.pure",
    "possiblyImpure.betweenPhpTags",
    "possiblyImpure.die",
    "possiblyImpure.echo",
    "possiblyImpure.eval",
    "possiblyImpure.exit",
    "possiblyImpure.functionCall",
    "possiblyImpure.global",
    "possiblyImpure.include",
    "possiblyImpure.methodCall",
    "possiblyImpure.new",
    "possiblyImpure.print",
    "possiblyImpure.propertyAssign",
    "possiblyImpure.propertyAssignByRef",
    "possiblyImpure.propertyUnset",
    "possiblyImpure.require",
    "possiblyImpure.static",
    "possiblyImpure.staticPropertyAccess",
    "possiblyImpure.superglobal",
    "possiblyImpure.yield",
    "possiblyImpure.yieldFrom",
    "pureFunction.nonOptionalParameterPassed",
    "pureFunction.parameterByRef",
    "pureFunction.parameterPassedNotByRef",
    "pureFunction.redundantUnlessCallable",
    "pureFunction.void",
    "pureMethod.nonOptionalParameterPassed",
    "pureMethod.parameterByRef",
    "pureMethod.parameterPassedNotByRef",
    "pureMethod.redundantUnlessCallable",
    "pureMethod.void",
    "purePropertyHook.nonOptionalParameterPassed",
    "purePropertyHook.parameterByRef",
    "purePropertyHook.parameterPassedNotByRef",
    "purePropertyHook.redundantUnlessCallable",
    "purePropertyHook.void",
    "void.pure",
];

/// What FunctionPurityCheck::check() reads off a declaration's reflection.
struct Declaration<'x> {
    span: Span,
    description: String,
    /// `Function`, `Method` or `PropertyHook`
    kind: &'static str,
    /// the parameters: name (without `$`), optional, by reference
    params: Vec<(String, bool, bool)>,
    /// the `@param` types (without `$`), for the pure-unless-callable parameters
    param_types: Vec<(String, pun_phpstan::types::Type)>,
    pure: Option<bool>,
    unless_callable: Vec<String>,
    unless_passed: Vec<String>,
    void: bool,
    /// getThrowType() is null or void
    throws_void: bool,
    asserts: bool,
    constructor: bool,
    /// a method: final (by modifier or PHPDoc), of a final class
    method: Option<bool>,
    raw: &'x [u8],
}

fn params(list: Option<&FunctionLikeParameterList>) -> Vec<(String, bool, bool)> {
    list.iter()
        .flat_map(|l| l.parameters.iter())
        .map(|p| {
            (
                String::from_utf8_lossy(&p.variable.name[1..]).into_owned(),
                p.default_value.is_some() || p.ellipsis.is_some(),
                p.ampersand.is_some(),
            )
        })
        .collect()
}

/// `@throws` tags other than `void` (`getThrowType()` not null and not void).
fn throws_non_void(raw: &[u8]) -> bool {
    let tags = super::doc::tags(raw);
    let kind = if tags.iter().any(|(name, _)| name == "@phpstan-throws") {
        "@phpstan-throws"
    } else {
        "@throws"
    };
    tags.iter()
        .filter(|(name, _)| name == kind)
        .any(|(_, value)| {
            value.split(u8::is_ascii_whitespace).find(|s| !s.is_empty()) != Some(b"void".as_slice())
        })
}

fn has_final_tag(raw: &[u8]) -> bool {
    declarations::doc_tags(raw).iter().any(|t| t == "@final")
}

pub(super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let doc = c.php_doc(c.file.id, f.span().start.offset);
    let (raw, phpdoc) = doc.as_ref().map_or((&b""[..], &[][..]), |(raw, entry)| (*raw, &entry.phpdoc[..]));
    let pure = effects::purity(raw);
    let unless_callable = declared::tagged_params(
        raw,
        &[
            "@pure-unless-callable-is-impure",
            "@phpstan-pure-unless-callable-is-impure",
        ],
    );
    let unless_passed = declared::tagged_params(
        raw,
        &[
            "@pure-unless-parameter-passed",
            "@phpstan-pure-unless-parameter-passed",
        ],
    );
    if pure.is_none() && unless_callable.is_empty() && unless_passed.is_empty() {
        void_pure_function(c, f, phpdoc);
        return;
    }
    let signature = declared::with_classes(c, || declared::function(c, f));
    let name = c.resolved_span(f.name.span);
    let d = Declaration {
        span: f.span(),
        description: format!("Function {name}()"),
        kind: "Function",
        params: params(Some(&f.parameter_list)),
        param_types: signature
            .params
            .iter()
            .map(|p| (p.0.clone(), p.1.clone()))
            .collect(),
        pure,
        unless_callable,
        unless_passed,
        void: signature.ret.is_void().yes(),
        throws_void: !throws_non_void(raw),
        asserts: !assert_tags::parameters(phpdoc).is_empty(),
        constructor: false,
        method: None,
        raw,
    };
    let points = purity_points::body(c, f.body.statements.as_slice(), false, raw);
    check(c, &d, points);
}

pub(super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 2 {
        return;
    }
    let MethodBody::Concrete(body) = &m.body else {
        return;
    };
    let context = c
        .class_scope()
        .expect("method is walked in a class context");
    let declaring =
        missing_types::declaring(c, m.span()).expect("method belongs to its class or trait");
    let (purity, unless_callable, unless_passed) =
        declared::method_purity(c, context, m.name.value);
    if purity.is_none() && !unless_callable && !unless_passed {
        void_pure_method(c, m, body);
        return;
    }
    if declaring.kind == SymbolKind::Trait || c.trait_context.is_some() {
        return;
    }
    let raw = c.php_doc_comment(c.file.id, m.span().start.offset).unwrap_or_default();
    let (unless_callable, unless_passed, throws) =
        declared::method_conditional_purity(c, context, m.name.value);
    let signature = declared::with_classes(c, || declared::method(c, m, declaring, context));
    let name = String::from_utf8_lossy(m.name.value);
    let constructor = m.name.value.eq_ignore_ascii_case(b"__construct");
    let class_raw = declared::doc_at(c, context.span)
        .map(|(raw, _, _)| raw)
        .unwrap_or_default();
    let d = Declaration {
        span: m.span(),
        description: format!("Method {}::{name}()", c.display(context)),
        kind: "Method",
        params: params(Some(&m.parameter_list)),
        param_types: signature
            .params
            .iter()
            .map(|p| (p.0.clone(), p.1.clone()))
            .collect(),
        pure: purity,
        unless_callable,
        unless_passed,
        void: signature.ret.is_void().yes(),
        throws_void: throws.is_none_or(|t| t.is_void().yes()),
        asserts: !signature.asserts.is_empty(),
        constructor,
        method: Some(
            m.modifiers.contains_final()
                || has_final_tag(raw)
                || context.flags.is_final()
                || has_final_tag(&class_raw),
        ),
        raw,
    };
    let points = purity_points::body(c, body.statements.as_slice(), constructor, raw);
    check(c, &d, points);
}

/// PurePropertyHookRule: the hooks of a property with a body.
pub(super) fn hooks(c: &mut Checker, property: &[u8], list: &PropertyHookList) {
    if c.level < 2 || c.trait_context.is_some() {
        return;
    }
    let Some(context) = c.class_scope() else {
        return;
    };
    let class_raw = declared::doc_at(c, context.span)
        .map(|(raw, _, _)| raw)
        .unwrap_or_default();
    for h in list.hooks.iter() {
        let PropertyHookBody::Concrete(body) = &h.body else {
            continue;
        };
        let raw = c.php_doc_comment(c.file.id, h.span().start.offset).unwrap_or_default();
        let pure = effects::purity(raw);
        let unless_callable = declared::tagged_params(
            raw,
            &[
                "@pure-unless-callable-is-impure",
                "@phpstan-pure-unless-callable-is-impure",
            ],
        );
        let unless_passed = declared::tagged_params(
            raw,
            &[
                "@pure-unless-parameter-passed",
                "@phpstan-pure-unless-parameter-passed",
            ],
        );
        if pure.is_none() && unless_callable.is_empty() && unless_passed.is_empty() {
            continue; // an unspecified hook is not private: no void.pure
        }
        let set = h.name.value.eq_ignore_ascii_case(b"set");
        let mut hook_params = params(h.parameter_list.as_ref());
        if set && h.parameter_list.is_none() {
            hook_params.push(("value".into(), false, false));
        }
        let name = if set { "Set" } else { "Get" };
        let d = Declaration {
            span: h.span(),
            description: format!(
                "{name} hook for property {}::{}",
                c.display(context),
                String::from_utf8_lossy(property)
            ),
            kind: "PropertyHook",
            params: hook_params,
            param_types: Vec::new(),
            pure,
            unless_callable,
            unless_passed,
            void: set,
            throws_void: !throws_non_void(raw),
            asserts: false,
            constructor: false,
            method: Some(
                h.modifiers.iter().any(|m| matches!(m, Modifier::Final(_)))
                    || context.flags.is_final()
                    || has_final_tag(&class_raw),
            ),
            raw,
        };
        let points = match body {
            PropertyHookConcreteBody::Block(b) => {
                purity_points::body(c, b.statements.as_slice(), false, raw)
            }
            PropertyHookConcreteBody::Expression(e) => {
                purity_points::expression(c, e.expression, raw)
            }
        };
        check(c, &d, points);
    }
}

/// FunctionPurityCheck::check() of a declaration with a purity or a pure-unless tag.
fn check(c: &mut Checker, d: &Declaration, points: Points) {
    let lower = lcfirst(&d.description);
    for (name, _, _) in &d.params {
        if !d.unless_callable.contains(name) {
            continue;
        }
        let Some((_, ty)) = d.param_types.iter().find(|(n, _)| n == name) else {
            continue;
        };
        // the acceptors of a pure-callable are all pure
        if matches!(
            ty,
            pun_phpstan::types::Type::Callable(_) | pun_phpstan::types::Type::Closure(_)
        ) && pure_callable_param(d.raw, name)
        {
            c.report(
                d.span,
                &format!("pure{}.redundantUnlessCallable", d.kind),
                format!(
                    "{} is marked @pure-unless-callable-is-impure for parameter ${name}, but ${name} is already a pure callable, so {lower} can be marked @phpstan-pure instead.",
                    d.description
                ),
                None,
            );
        }
    }
    for (name, optional, by_ref) in &d.params {
        if !d.unless_passed.contains(name) {
            continue;
        }
        if !optional {
            c.report(
                d.span,
                &format!("pure{}.nonOptionalParameterPassed", d.kind),
                format!(
                    "{} is marked @pure-unless-parameter-passed for parameter ${name}, but ${name} is not optional, so {lower} is never pure.",
                    d.description
                ),
                None,
            );
        }
        if !by_ref {
            c.report(
                d.span,
                &format!("pure{}.parameterPassedNotByRef", d.kind),
                format!(
                    "{} is marked @pure-unless-parameter-passed for parameter ${name}, but ${name} is not passed by reference.",
                    d.description
                ),
                None,
            );
        }
    }
    if d.pure == Some(true) {
        for (name, _, by_ref) in &d.params {
            if *by_ref {
                c.report(
                    d.span,
                    &format!("pure{}.parameterByRef", d.kind),
                    format!(
                        "{} is marked as pure but parameter ${name} is passed by reference.",
                        d.description
                    ),
                    None,
                );
            }
        }
        if d.void && !d.constructor && d.throws_void && !d.asserts {
            c.report(
                d.span,
                &format!("pure{}.void", d.kind),
                format!("{} is marked as pure but returns void.", d.description),
                None,
            );
        }
        report_points(c, &points, &d.unless_callable, &lower);
    } else if !d.unless_callable.is_empty() || !d.unless_passed.is_empty() {
        report_points(c, &points, &d.unless_callable, &lower);
    } else if d.pure == Some(false)
        && points.complete
        && !points.throws
        && points.points.is_empty()
        && !d.asserts
        && d.method != Some(false)
    {
        c.report(
            d.span,
            &format!("impure{}.pure", d.kind),
            format!(
                "{} is marked as impure but does not have any side effects.",
                d.description
            ),
            None,
        );
    }
}

/// FunctionPurityCheck::reportImpurePoints(), without the invocations of the pure-unless-callable parameters.
fn report_points(c: &mut Checker, points: &Points, unless_callable: &[String], lower: &str) {
    for p in &points.points {
        if p.invoked
            .as_ref()
            .is_some_and(|name| unless_callable.contains(name))
        {
            continue;
        }
        let (adjective, prefix) = if p.certain {
            ("Impure", "impure")
        } else {
            ("Possibly impure", "possiblyImpure")
        };
        c.report(
            p.span,
            &format!("{prefix}.{}", p.identifier),
            format!("{adjective} {} in pure {lower}.", p.description),
            None,
        );
    }
}

fn pure_callable_param(raw: &[u8], name: &str) -> bool {
    String::from_utf8_lossy(raw).lines().any(|l| {
        let l = l.trim_start().trim_start_matches('*').trim_start();
        ["@param ", "@phpstan-param ", "@psalm-param "]
            .iter()
            .any(|t| {
                l.strip_prefix(t).is_some_and(|rest| {
                    let lower = rest.trim_start().to_ascii_lowercase();
                    (lower.starts_with("pure-callable") || lower.starts_with("pure-closure"))
                        && !rest.contains('|')
                        && rest.contains(&format!("${name}"))
                })
            })
    })
}

fn lcfirst(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_ascii_lowercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// The unspecified-function `void.pure` branch.
fn void_pure_function(c: &mut Checker, f: &Function, phpdoc: &[pun_phpstan::phpdoc::Tag]) {
    if !assert_tags::parameters(phpdoc).is_empty() {
        return;
    }
    // PhpDocsResolver also applies the enclosing class's policy to a named function declared in a method.
    if let Some(class) = c.class_scope() {
        let raw = declared::doc_at(c, class.span)
            .map(|(raw, _, _)| raw)
            .unwrap_or_default();
        if declarations::doc_tags(&raw)
            .iter()
            .any(|tag| tag == "@phpstan-all-methods-impure")
        {
            return;
        }
    }
    let signature = declared::with_classes(c, || declared::function(c, f));
    if !signature.ret.is_void().yes() {
        return;
    }
    let name = c.resolved_span(f.name.span);
    void_pure(
        c,
        &f.body,
        &f.parameter_list,
        f.span(),
        &format!("Function {name}()"),
    );
}

/// The unspecified-method `void.pure` branch (private, not a constructor).
fn void_pure_method(c: &mut Checker, m: &Method, body: &Block) {
    if !m.modifiers.contains_private() || m.name.value.eq_ignore_ascii_case(b"__construct") {
        return;
    }
    let context = c
        .class_scope()
        .expect("method is walked in a class context");
    let declaring =
        missing_types::declaring(c, m.span()).expect("method belongs to its class or trait");
    // Inherited conditional-parameter maps and trait adaptations need complete reflection metadata.
    if declaring.kind == SymbolKind::Trait
        || !context.used_traits.is_empty()
        || context.direct_parent_class.is_some()
        || !context.direct_parent_interfaces.is_empty()
    {
        return;
    }
    let signature = declared::with_classes(c, || declared::method(c, m, declaring, context));
    if !signature.ret.is_void().yes() || !signature.asserts.is_empty() {
        return;
    }
    let name = String::from_utf8_lossy(m.name.value);
    let description = format!("Method {}::{name}()", c.display(context));
    void_pure(c, body, &m.parameter_list, m.span(), &description);
}

fn void_pure(
    c: &mut Checker,
    body: &Block,
    params: &FunctionLikeParameterList,
    span: Span,
    description: &str,
) {
    // User declarations' `&` parameters use PassedByReference::createCreatesNewVariable(), including variadics.
    if params.parameters.iter().any(|p| p.ampersand.is_some()) {
        return;
    }
    let substantive = body.statements.iter().any(|s| match s {
        Statement::Noop(_) => false,
        Statement::Expression(s) => !effects::testing(c, s.expression),
        _ => true,
    });
    if substantive && effects::body(c, body).is_some_and(|effects| effects.empty()) {
        c.report(
            span,
            "void.pure",
            format!("{description} returns void but does not have any side effects."),
            None,
        );
    }
}
