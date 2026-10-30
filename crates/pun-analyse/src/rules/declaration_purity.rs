//! The conservative level-2, unspecified-purity `void.pure` consumer of FunctionPurityCheck.
//! This does not port or activate the complete PureFunctionRule/PureMethodRule mappings.

use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::{
    Checker, assert_tags, declaration_effects as effects, declarations, declared, missing_types,
};

pub(super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let raw = declarations::doc_before(&c.file.contents, f.span().start.offset as usize)
        .unwrap_or_default();
    if effects::purity(raw).is_some()
        || effects::conditional(raw)
        || !assert_tags::parameters(raw).is_empty()
    {
        return;
    }
    // PhpDocsResolver also applies the enclosing class's policy to a named function declared in a method.
    if let Some(class) = c.class_scope() {
        let raw = declared::doc_at(c, class.span)
            .map(|(raw, _)| raw)
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
    check(
        c,
        &f.body,
        &f.parameter_list,
        f.span(),
        &format!("Function {name}()"),
    );
}

pub(super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 2
        || !m
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)))
        || m.name.value.eq_ignore_ascii_case(b"__construct")
    {
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
    // Inherited conditional-parameter maps and trait adaptations need complete reflection metadata.
    if declaring.kind == SymbolKind::Trait
        || !context.used_traits.is_empty()
        || context.direct_parent_class.is_some()
        || !context.direct_parent_interfaces.is_empty()
    {
        return;
    }
    let raw = declarations::doc_before(&c.file.contents, m.span().start.offset as usize)
        .unwrap_or_default();
    if effects::conditional(raw) {
        return;
    }
    let (purity, unless_callable, unless_passed) =
        declared::method_purity(c, context, m.name.value);
    if purity.is_some() || unless_callable || unless_passed {
        return;
    }
    let signature = declared::with_classes(c, || declared::method(c, m, declaring, context));
    if !signature.ret.is_void().yes() || !signature.asserts.is_empty() {
        return;
    }
    let name = String::from_utf8_lossy(m.name.value);
    let description = format!("Method {}::{name}()", c.display(context));
    check(c, body, &m.parameter_list, m.span(), &description);
}

fn check(
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
