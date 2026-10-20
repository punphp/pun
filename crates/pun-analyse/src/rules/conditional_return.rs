//! FunctionConditionalReturnTypeRule and MethodConditionalReturnTypeRule (level 2, over
//! ConditionalReturnTypeRuleHelper): a conditional type's subject must involve a template or name a parameter, and
//! its condition must not be decided by the subject's type alone.

use std::cell::RefCell;

use mago_span::HasSpan;
use mago_syntax::cst::{Function, Method};
use pun_phpstan::types::{Conditional, Signature, Subject, Type, is_super_type_of, recommended_level};

use super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &[
    "conditionalType.subjectNotFound",
    "parameter.notFound",
    "conditionalType.alwaysTrue",
    "conditionalType.alwaysFalse",
];

pub fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let errors = declared::with_classes(c, || check(&declared::function(c, f)));
    for (identifier, message) in errors {
        c.report(f.span(), identifier, message, None);
    }
}

pub fn method(c: &mut Checker, m: &Method) {
    if c.level < 2 {
        return;
    }
    let Some(context) = c.class_scope() else { return };
    let declaring = super::missing_types::declaring(c, m.span()).expect("a method is declared by its class or a used trait");
    let errors = declared::with_classes(c, || check(&declared::method(c, m, declaring, context)));
    for (identifier, message) in errors {
        c.report(m.span(), identifier, message, None);
    }
}

/// A callable's conditional return on its own parameter with boolean branches: a type predicate, whose return type
/// is `bool` (`resolveCallableReturnTypeAssertions()`, CallableAssertionsHelper::createAssertionsFromConditional()).
fn predicate(sig: &Signature) -> bool {
    let Type::Conditional(c) = &sig.ret else {
        return false;
    };
    let Subject::Parameter(name) = &c.subject else {
        return false;
    };
    if !sig.params.iter().any(|p| p.name == name[1..]) {
        return false;
    }
    let (yes, no) = if c.negated { (&c.otherwise, &c.then) } else { (&c.then, &c.otherwise) };
    let branch = |t: &Type| match t {
        Type::ConstBool(b) => Some(Some(*b)),
        _ if t.is_boolean().yes() => Some(None),
        _ => None,
    };
    matches!(
        (branch(yes), branch(no)),
        (Some(Some(true)), Some(Some(false)))
            | (Some(Some(false)), Some(Some(true)))
            | (Some(None), Some(Some(_)))
            | (Some(Some(_)), Some(None))
    )
}

/// `TypeTraverser::map()` collecting the conditional types, outer ones first.
fn conditionals(t: &Type, out: &RefCell<Vec<Conditional>>) {
    if let Type::Conditional(c) = t {
        out.borrow_mut().push((**c).clone());
    }
    if let Type::Callable(Some(sig)) | Type::Closure(sig) = t
        && predicate(sig)
    {
        for p in &sig.params {
            conditionals(&p.ty, out);
        }
        return;
    }
    t.traverse(&|i| {
        conditionals(i, out);
        i.clone()
    });
}

fn has_template(t: &Type) -> bool {
    if matches!(t, Type::Template(_)) {
        return true;
    }
    let found = std::cell::Cell::new(false);
    t.traverse(&|i| {
        found.set(found.get() || has_template(i));
        i.clone()
    });
    found.get()
}

/// ConditionalReturnTypeRuleHelper::check().
fn check(sig: &declared::Signature) -> Vec<(&'static str, String)> {
    let found = RefCell::new(Vec::new());
    for (_, ty, out, closure_this) in &sig.params {
        for t in [Some(ty), out.as_ref(), closure_this.as_ref()].into_iter().flatten() {
            conditionals(t, &found);
        }
    }
    conditionals(&sig.ret, &found);
    let mut errors = Vec::new();
    for conditional in found.into_inner() {
        let subject = match &conditional.subject {
            Subject::Type(Type::Static(..) | Type::This(_)) => continue,
            Subject::Type(subject) => {
                if !has_template(subject) {
                    errors.push((
                        "conditionalType.subjectNotFound",
                        format!(
                            "Conditional return type uses subject type {} which is not part of PHPDoc @template tags.",
                            subject.describe(pun_phpstan::types::Level::TypeOnly)
                        ),
                    ));
                    continue;
                }
                subject.clone()
            }
            Subject::Parameter(name) => {
                let name = &name[1..];
                let Some((_, ty, ..)) = sig.params.iter().find(|p| p.0 == name) else {
                    errors.push(("parameter.notFound", format!("Conditional return type references unknown parameter ${name}.")));
                    continue;
                };
                ty.clone()
            }
        };
        let target = &conditional.target;
        // (an OffsetAccessType subject of a template resolves as its bound's offset type)
        let resolved = match &subject {
            Type::OffsetAccess(..) => match pun_phpstan::resolve::resolve_late(&subject.resolve_to_bounds()) {
                // ponytail: an offset Pun's resolver leaves unresolved is not checked
                Type::OffsetAccess(..) => continue,
                t => t,
            },
            _ => subject.clone(),
        };
        let is = is_super_type_of(target, &resolved);
        if is.maybe() {
            continue;
        }
        let level = recommended_level(&subject, Some(target));
        let always = is.yes() != conditional.negated;
        errors.push((
            if always { "conditionalType.alwaysTrue" } else { "conditionalType.alwaysFalse" },
            format!(
                "Condition \"{} {} {}\" in conditional return type is always {}.",
                subject.describe(level),
                if conditional.negated { "is not" } else { "is" },
                target.describe(level),
                if always { "true" } else { "false" }
            ),
        ));
    }
    errors
}
