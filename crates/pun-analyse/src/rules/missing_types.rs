//! Level 6: MissingTypehintCheck and the rules that report with it: MissingFunctionParameterTypehintRule,
//! MissingFunctionReturnTypehintRule, MissingMethodParameterTypehintRule, MissingMethodReturnTypehintRule,
//! MissingMethodSelfOutTypeRule, MissingPropertyTypehintRule and MissingClassConstantTypehintRule.

use std::cell::RefCell;

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::symbols;
use pun_phpstan::types::{Level, Type};

use super::Checker;
use super::declared;

pub const IDENTIFIERS: &[&str] = &[
    "missingType.parameter",
    "missingType.return",
    "missingType.property",
    "missingType.iterableValue",
    "missingType.generics",
    "missingType.callable",
];

/// `MissingTypehintCheck::MISSING_ITERABLE_VALUE_TYPE_TIP`
const TIP: &str =
    "See: https://phpstan.org/blog/solving-phpstan-no-value-type-specified-in-iterable-type";

/// `featureToggles.skipCheckGenericClasses`
const SKIP_GENERIC: [&str; 1] = ["DOMNamedNodeMap"];

/// `MissingTypehintCheck::ITERABLE_GENERIC_CLASS_NAMES`
const ITERABLE_GENERIC: [&str; 4] = ["Traversable", "Iterator", "IteratorAggregate", "Generator"];

/// `getIterableTypesWithMissingValueTypehint()`: "iterable type X" for each iterable with an implicit `mixed`
/// value type.
pub fn iterables_without_value(ty: &Type) -> Vec<String> {
    let out = RefCell::new(Vec::new());
    iterable_walk(ty, &out);
    out.into_inner()
}

fn iterable_walk(ty: &Type, out: &RefCell<Vec<String>>) {
    match ty {
        Type::Template(_) | Type::Accessory(_) => return,
        Type::Intersection(members) if ty.is_callable().yes() && ty.is_array().yes() => {
            let rest: Vec<Type> = members
                .iter()
                .filter(|m| !m.is_array().yes())
                .cloned()
                .collect();
            match rest.as_slice() {
                [one] => iterable_walk(one, out),
                _ => iterable_walk(&Type::Intersection(rest), out),
            }
            return;
        }
        Type::Conditional(c) => {
            out.borrow_mut().extend(iterables_without_value(&c.then));
            out.borrow_mut()
                .extend(iterables_without_value(&c.otherwise));
            return;
        }
        _ => {}
    }
    if ty.is_iterable().yes() {
        let dropped;
        let ty = if is_constant_array(ty) {
            dropped = drop_unsealed(ty);
            &dropped
        } else {
            ty
        };
        let value = ty.iterable_value_type();
        if value.is_implicit_mixed() {
            out.borrow_mut()
                .push(format!("iterable type {}", ty.describe(Level::TypeOnly)));
        }
        if matches!(ty, Type::Intersection(_)) {
            if ty.is_list().yes() {
                iterable_walk(&value, out);
            }
            return;
        }
    }
    ty.traverse(&|inner| {
        iterable_walk(inner, out);
        inner.clone()
    });
}

/// `isConstantArray()->yes()`
fn is_constant_array(ty: &Type) -> bool {
    match ty {
        Type::ConstArray(_) => true,
        Type::Intersection(members) => members.iter().any(is_constant_array),
        Type::Union(members) => members.iter().all(is_constant_array),
        _ => false,
    }
}

/// `ty` with each constant array's unsealed `...<K, V>` dropped (`dropUnsealedTypes()`).
fn drop_unsealed(ty: &Type) -> Type {
    let ty = match ty {
        Type::ConstArray(a) if a.unsealed.is_some() => {
            let mut a = a.clone();
            a.unsealed = None;
            Type::ConstArray(a)
        }
        t => t.clone(),
    };
    ty.traverse(&drop_unsealed)
}

/// `getNonGenericObjectTypesWithGenericClass()`: ("class Foo", "T, U") for each generic class used without its
/// type arguments.
pub fn generics_without_args(ty: &Type, check_iterable_classes: bool) -> Vec<(String, String)> {
    let out = RefCell::new(Vec::new());
    generic_walk(ty, check_iterable_classes, &out);
    out.into_inner()
}

fn generic_walk(ty: &Type, iterables: bool, out: &RefCell<Vec<(String, String)>>) {
    match ty {
        Type::Template(_) => return,
        Type::Object(class, args) if args.is_empty() => {
            let Some(c) = symbols::lookup(class) else {
                return;
            };
            if c.name == "Traversable"
                || (!iterables && ITERABLE_GENERIC.contains(&c.name.as_str()))
                || SKIP_GENERIC.contains(&c.name.as_str())
                || c.is_trait
                || c.templates.is_empty()
            {
                return;
            }
            let total = c.templates.len();
            let required = c.templates.iter().filter(|t| t.default.is_none()).count();
            if required == 0 {
                return;
            }
            let mut list = c
                .templates
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            if required != total {
                list.push_str(&format!(" ({required}-{total} required)"));
            }
            let kind = if c.is_interface {
                "interface"
            } else if c.is_trait {
                "trait"
            } else if c.is_enum {
                "enum"
            } else {
                "class"
            };
            out.borrow_mut().push((format!("{kind} {}", c.name), list));
            return;
        }
        _ => {}
    }
    ty.traverse(&|inner| {
        generic_walk(inner, iterables, out);
        inner.clone()
    });
}

/// What one check found: (identifier, message, tip).
type Found = Vec<(&'static str, String, Option<&'static str>)>;

/// The messages of one checked type: `{subject} has {parameter} with ...` for a parameter, else
/// `{subject}{owner} ...` (`Method C::m() return type ...`, `Property C::$p type ...`).
struct Messages<'s> {
    subject: &'s str,
    parameter: Option<&'s str>,
    owner: &'s str,
    /// the identifier and message for an implicit `mixed`
    missing: (&'static str, String),
}

/// The checks of one type (the type model must know the project's classes).
fn check(ty: &Type, m: Messages, iterables: bool, out: &mut Found) {
    if ty.is_implicit_mixed() {
        out.push((m.missing.0, m.missing.1, None));
        return;
    }
    for iterable in iterables_without_value(ty) {
        let message = match m.parameter {
            Some(p) => format!(
                "{} has {p} with no value type specified in {iterable}.",
                m.subject
            ),
            None => format!(
                "{}{} has no value type specified in {iterable}.",
                m.subject, m.owner
            ),
        };
        out.push(("missingType.iterableValue", message, Some(TIP)));
    }
    for (name, list) in generics_without_args(ty, iterables) {
        let message = match m.parameter {
            Some(p) => format!(
                "{} has {p} with generic {name} but does not specify its types: {list}",
                m.subject
            ),
            None => format!(
                "{}{} with generic {name} does not specify its types: {list}",
                m.subject,
                if m.owner == " type" { "" } else { m.owner }
            ),
        };
        out.push(("missingType.generics", message, None));
    }
}

/// The checks of a function-like's parameters and (unless `skip_return`) return type.
fn signature(
    subject: &str,
    sig: &declared::Signature,
    skip_return: bool,
    iterables: bool,
) -> Found {
    let mut out = Vec::new();
    for (param, ty, param_out, closure_this) in &sig.params {
        let checks = [
            (format!("parameter ${param}"), Some(ty)),
            (
                format!("@param-closure-this PHPDoc tag for parameter ${param}"),
                closure_this.as_ref(),
            ),
            (
                format!("@param-out PHPDoc tag for parameter ${param}"),
                param_out.as_ref(),
            ),
        ];
        for (what, ty) in checks {
            let Some(ty) = ty else {
                continue;
            };
            let missing = format!("{subject} has {what} with no type specified.");
            check(
                ty,
                Messages {
                    subject,
                    parameter: Some(&what),
                    owner: "",
                    missing: ("missingType.parameter", missing),
                },
                iterables,
                &mut out,
            );
        }
    }
    if let Some(self_out) = &sig.self_out {
        let tag = "PHPDoc tag @phpstan-self-out";
        let mut found = Vec::new();
        check(
            self_out,
            Messages {
                subject,
                parameter: Some(tag),
                owner: "",
                missing: ("", String::new()),
            },
            iterables,
            &mut found,
        );
        // (an implicit `mixed` self-out type is not reported)
        out.extend(found.into_iter().filter(|(id, _, _)| !id.is_empty()));
    }
    if !skip_return {
        check(
            &sig.ret,
            Messages {
                subject,
                parameter: None,
                owner: " return type",
                missing: (
                    "missingType.return",
                    format!("{subject} has no return type specified."),
                ),
            },
            iterables,
            &mut out,
        );
    }
    out
}

fn report(c: &mut Checker, at: mago_span::Span, found: Found) {
    for (identifier, message, tip) in found {
        c.report(at, identifier, message, tip);
    }
}

/// MissingMethodParameterTypehintRule and MissingMethodReturnTypehintRule (`InClassMethodNode`).
pub fn method(c: &mut Checker, m: &Method) {
    if c.level < 6 {
        return;
    }
    let (Some(context), Some(display)) = (
        c.class_scope(),
        c.decls.last().and_then(|d| d.display.clone()),
    ) else {
        return;
    };
    let Some(declaring) = declaring(c, m.span()) else {
        return;
    };
    let name = c
        .renamed
        .as_ref()
        .filter(|(span, _)| *span == m.span())
        .map_or_else(
            || String::from_utf8_lossy(m.name.value).into_owned(),
            |(_, n)| n.clone(),
        );
    let subject = format!("Method {display}::{name}()");
    // a trait's constructor has no return type to check (`originalTraitMethodName`)
    let skip_return = declaring.kind == mago_codex::symbol::SymbolKind::Trait
        && m.name.value.eq_ignore_ascii_case(b"__construct");
    let iterables = c.generic_iterable_classes;
    let found = declared::with_classes(c, || {
        let sig = declared::method(c, m, declaring, context);
        signature(&subject, &sig, skip_return, iterables)
    });
    report(c, m.span(), found);
}

/// The class-like the method at `span` is declared in: the class being walked or one of its traits.
fn declaring<'a>(
    c: &Checker<'a, '_>,
    span: mago_span::Span,
) -> Option<&'a mago_codex::metadata::class_like::ClassLikeMetadata> {
    let inside = |k: &&mago_codex::metadata::class_like::ClassLikeMetadata| {
        k.span.file_id == span.file_id
            && k.span.start.offset <= span.start.offset
            && span.end.offset <= k.span.end.offset
    };
    let class = c.class_scope()?;
    if inside(&class) {
        return Some(class);
    }
    class
        .used_traits
        .iter()
        .filter_map(|t| c.codebase.get_class_like(t.as_bytes()))
        .find(inside)
}

/// MissingFunctionParameterTypehintRule and MissingFunctionReturnTypehintRule (`InFunctionNode`).
pub fn function(c: &mut Checker, f: &Function) {
    if c.level < 6 {
        return;
    }
    let name = String::from_utf8_lossy(c.names.get(&f.name.span())).into_owned();
    let subject = format!("Function {name}()");
    let iterables = c.generic_iterable_classes;
    let found = declared::with_classes(c, || {
        let sig = declared::function(c, f);
        signature(&subject, &sig, false, iterables)
    });
    report(c, f.span(), found);
}

/// MissingPropertyTypehintRule (`ClassPropertyNode`, not promoted).
pub fn property(c: &mut Checker, p: &Property) {
    if c.level < 6 {
        return;
    }
    let (Some(context), Some(display)) = (
        c.class_scope(),
        c.decls.last().and_then(|d| d.display.clone()),
    ) else {
        return;
    };
    let iterables = c.generic_iterable_classes;
    let hint = p.hint();
    for v in p.variables() {
        let name = &v.name[1..];
        let subject = format!("Property {display}::${}", String::from_utf8_lossy(name));
        let found = declared::with_classes(c, || {
            let ty = declared::property(c, hint, p.span(), name, context);
            let mut out = Vec::new();
            check(
                &ty,
                Messages {
                    subject: &subject,
                    parameter: None,
                    owner: " type",
                    missing: (
                        "missingType.property",
                        format!("{subject} has no type specified."),
                    ),
                },
                iterables,
                &mut out,
            );
            out
        });
        report(c, p.span(), found);
    }
}

/// MissingClassConstantTypehintRule (`ClassConst`): the PHPDoc type of each constant.
pub fn class_constant(c: &mut Checker, k: &ClassLikeConstant) {
    if c.level < 6 {
        return;
    }
    let (Some(context), Some(display)) = (
        c.class_scope(),
        c.decls.last().and_then(|d| d.display.clone()),
    ) else {
        return;
    };
    let iterables = c.generic_iterable_classes;
    for item in k.items.iter() {
        let name = item.name.value;
        let subject = format!("Constant {display}::{}", String::from_utf8_lossy(name));
        let found = declared::with_classes(c, || {
            let mut out = Vec::new();
            if let Some(ty) = declared::constant(c, k.span(), name, context) {
                check(
                    &ty,
                    Messages {
                        subject: &subject,
                        parameter: None,
                        owner: " type",
                        missing: ("", String::new()),
                    },
                    iterables,
                    &mut out,
                );
            }
            out.retain(|(id, _, _)| !id.is_empty());
            out
        });
        report(c, k.span(), found);
    }
}
