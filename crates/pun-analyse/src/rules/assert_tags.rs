//! FunctionAssertRule and MethodAssertRule (level 2, over AssertRuleHelper): a function's or method's
//! `@phpstan-assert*` tags must name a parameter (or `$this`, its property or method) and assert a type that is
//! resolvable, known, and neither impossible nor already certain for it.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::{Function, Method, ModifierSequenceExt};
use pun_phpstan::phpdoc::{self, TagValue};
use pun_phpstan::resolve::resolve_in;
use pun_phpstan::types::{Level, NeverKind, Type, is_super_type_of};

use super::doc::NameScope;
use super::{Checker, declared, generics, missing_types};

pub(super) const IDENTIFIERS: &[&str] = &[
    "parameter.notFound",
    "assert.unknownExpr",
    "assert.unresolvableType",
    "assert.impossibleType",
    "assert.alreadyNarrowedType",
    "assert.trait",
];

pub(super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let Some((_, entry)) = c.doc_before(c.file.id, f.span().start.offset) else {
        return;
    };
    let scope = declared::function_scope(c, f, &entry.doc);
    let errors = declared::with_classes(c, || {
        check(c, &entry.phpdoc, &scope, &declared::function(c, f), None)
    });
    report(c, f.span(), errors);
}

pub(super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 2 {
        return;
    }
    let Some(context) = c.class_scope() else {
        return;
    };
    let Some(declaring) = missing_types::declaring(c, m.span()) else {
        return;
    };
    let this = (!m.modifiers.contains_static())
        .then(|| Type::Object(declared::original_name(context), vec![], false));
    let own = c
        .doc_before(c.file.id, m.span().start.offset)
        .map(|(_, entry)| entry)
        .filter(|entry| !asserts(&entry.phpdoc).is_empty());
    let errors = declared::with_classes(c, || {
        let sig = declared::method(c, m, declaring, context);
        match own {
            Some(entry) => check(
                c,
                &entry.phpdoc,
                &declared::method_scope(c, m, &entry.doc, declaring, context),
                &sig,
                this,
            ),
            // the inherited (implicit) asserts: only their parameter is checked
            None => sig
                .asserts
                .iter()
                .filter(|name| {
                    !sig.params.iter().any(|p| p.0 == **name)
                        && !(this.is_some() && *name == "this")
                })
                .map(|name| {
                    (
                        "parameter.notFound",
                        format!("Assert references unknown parameter ${name}."),
                        None,
                        None,
                    )
                })
                .collect(),
        }
    });
    report(c, m.span(), errors);
}

type Errors = Vec<(&'static str, String, Option<String>, Option<String>)>;

fn report(c: &mut Checker, at: Span, errors: Errors) {
    let mut names = Vec::new();
    for (identifier, message, tip, class) in errors {
        match class {
            Some(class) => names.push(super::class_names::Pair::node(class, at)),
            None => c.report(at, identifier, message, tip.as_deref()),
        }
    }
    let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
    super::class_names::check(c, &names, case);
}

/// An assert tag: its tag name, type, parameter, `->member` (a method with `()`) and negation.
type Assert = (
    &'static str,
    phpdoc::Node,
    String,
    Option<(String, bool)>,
    bool,
);

/// The parameters (without `$`) the asserts of a PHPDoc's `tags` name.
pub(super) fn parameters(tags: &[phpdoc::Tag]) -> Vec<String> {
    asserts(tags)
        .into_iter()
        .map(|a| a.2[1..].to_string())
        .collect()
}

/// `resolveAssertTags()`: the first of the `@phpstan-`, `@psalm-` and `@phan-` prefixes with any assert.
fn asserts(tags: &[phpdoc::Tag]) -> Vec<Assert> {
    for prefix in ["@phpstan", "@psalm", "@phan"] {
        let mut found = Vec::new();
        for (suffix, name) in [
            ("-assert", "@phpstan-assert"),
            ("-assert-if-true", "@phpstan-assert-if-true"),
            ("-assert-if-false", "@phpstan-assert-if-false"),
        ] {
            let tag = format!("{prefix}{suffix}");
            // (by kind: the parameter asserts, then the property ones, then the method ones)
            let mut of: Vec<Assert> = tags
                .iter()
                .filter(|t| t.name == tag)
                .filter_map(|t| match &t.value {
                    TagValue::Assert {
                        ty,
                        parameter,
                        member,
                        negated,
                        ..
                    } => Some((
                        name,
                        ty.clone(),
                        parameter.clone(),
                        member.clone(),
                        *negated,
                    )),
                    _ => None,
                })
                .collect();
            of.sort_by_key(|a| match &a.3 {
                None => 0,
                Some((_, false)) => 1,
                Some((_, true)) => 2,
            });
            found.extend(of);
        }
        if !found.is_empty() {
            return found;
        }
    }
    Vec::new()
}

/// `$scope->getScopeType()` of the asserted expression: a property fetch or method call on the parameter's type,
/// typed by the classes that have the member (an error type when none has it).
fn member_type(c: &Checker, on: &Type, member: &(String, bool)) -> Type {
    let (name, method) = member;
    let found: Vec<Type> = on
        .object_class_names()
        .iter()
        .filter_map(|class| {
            let ty = if *method {
                c.codebase
                    .get_method(class.as_bytes(), name.as_bytes())
                    .map(|m| {
                        m.return_type_metadata
                            .as_ref()
                            .map(|t| c.to_phpstan(&t.type_union))
                    })
            } else {
                c.codebase
                    .get_property(class.as_bytes(), format!("${name}").as_bytes())
                    .map(|p| {
                        p.type_metadata
                            .as_ref()
                            .map(|t| c.to_phpstan(&t.type_union))
                    })
            };
            ty.map(|t| t.unwrap_or(Type::Mixed { explicit: false }))
        })
        .collect();
    if found.is_empty() {
        Type::Error
    } else {
        pun_phpstan::types::union(found)
    }
}

/// AssertRuleHelper::check(); the class names to check for their case come last, as `(.., Some(class))`.
fn check(
    c: &Checker,
    tags: &[phpdoc::Tag],
    scope: &NameScope,
    sig: &declared::Signature,
    this: Option<Type>,
) -> Errors {
    let mut errors: Errors = Vec::new();
    let mut by_name: Vec<(String, Type)> = sig
        .params
        .iter()
        .map(|p| (p.0.clone(), p.1.clone()))
        .collect();
    by_name.extend(this.map(|t| ("this".to_string(), t)));
    for (tag, ty, parameter, member, negated) in asserts(tags) {
        let name = &parameter[1..];
        let Some((_, param_type)) = by_name.iter().find(|(n, _)| n == name) else {
            errors.push((
                "parameter.notFound",
                format!("Assert references unknown parameter ${name}."),
                None,
                None,
            ));
            continue;
        };
        let (expr_type, expr) = match &member {
            None => (param_type.clone(), parameter.clone()),
            Some(m) => (
                member_type(c, param_type, m),
                format!("{parameter}->{}{}", m.0, if m.1 { "()" } else { "" }),
            ),
        };
        if expr_type == Type::Error {
            errors.push((
                "assert.unknownExpr",
                format!("Assert references unknown {expr}."),
                None,
                None,
            ));
            continue;
        }
        let asserted = resolve_in(&ty, scope);
        let mut unresolvable = false;
        super::type_aliases::each(&asserted, &mut |t| {
            unresolvable |= matches!(t, Type::Error | Type::Never(NeverKind::Implicit));
        });
        if unresolvable {
            errors.push((
                "assert.unresolvableType",
                format!("PHPDoc tag {tag} for {expr} contains unresolvable type."),
                None,
                None,
            ));
            continue;
        }
        let is = is_super_type_of(&asserted, &expr_type);
        if !is.maybe() {
            let negation = if negated { "negated " } else { "" };
            let (described, given) = (
                asserted.describe(Level::Precise),
                expr_type.describe(Level::Precise),
            );
            if if negated { is.yes() } else { is.no() } {
                errors.push((
                    "assert.impossibleType",
                    format!("Asserted {negation}type {described} for {expr} with type {given} can never happen."),
                    None,
                    None,
                ));
            } else {
                errors.push((
                    "assert.alreadyNarrowedType",
                    format!("Asserted {negation}type {described} for {expr} with type {given} does not narrow down the type."),
                    None,
                    None,
                ));
            }
        }
        let mut names = Vec::new();
        for class in generics::referenced_classes(&asserted) {
            match pun_phpstan::symbols::lookup(&class) {
                None => errors.push((
                    "class.notFound",
                    format!("PHPDoc tag {tag} for {expr} contains unknown class {class}."),
                    None,
                    None,
                )),
                Some(k) if k.is_trait => errors.push((
                    "assert.trait",
                    format!("PHPDoc tag {tag} for {expr} contains invalid type {class}."),
                    None,
                    None,
                )),
                Some(_) => names.push(("", String::new(), None, Some(class))),
            }
        }
        let s = generics::escape(&format!("PHPDoc tag {tag} for {expr}"));
        let messages = generics::GenericMessages::at(&s);
        let mut found = generics::Found::default();
        generics::generic_object_check(&asserted, &messages, &mut found);
        errors.extend(
            found
                .errors
                .into_iter()
                .map(|(id, m, tip)| (id, m, tip.map(String::from), None)),
        );
        errors.extend(names);
        if c.level < 6 {
            continue;
        }
        for iterable in missing_types::iterables_without_value(&asserted) {
            errors.push((
                "missingType.iterableValue",
                format!("PHPDoc tag {tag} for {expr} has no value type specified in {iterable}."),
                Some(missing_types::TIP.to_string()),
                None,
            ));
        }
        for (inner, list) in missing_types::generics_without_args(
            &asserted,
            c.on(crate::rules::Switches::GENERIC_ITERABLE_CLASSES),
        ) {
            errors.push((
                "missingType.generics",
                format!("PHPDoc tag {tag} for {expr} contains generic {inner} but does not specify its types: {list}"),
                None,
                None,
            ));
        }
        for callable in missing_types::callables_without_signature(
            &asserted,
            c.on(crate::rules::Switches::MISSING_CALLABLE_SIGNATURE),
        ) {
            errors.push((
                "missingType.callable",
                format!("PHPDoc tag {tag} for {expr} has no signature specified for {callable}."),
                None,
                None,
            ));
        }
    }
    errors
}
