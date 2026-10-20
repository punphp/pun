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
pub(super) const TIP: &str =
    "See: https://phpstan.org/blog/solving-phpstan-no-value-type-specified-in-iterable-type";

/// `featureToggles.skipCheckGenericClasses`
const SKIP_GENERIC: [&str; 1] = ["DOMNamedNodeMap"];

/// `MissingTypehintCheck::ITERABLE_GENERIC_CLASS_NAMES`
const ITERABLE_GENERIC: [&str; 4] = ["Traversable", "Iterator", "IteratorAggregate", "Generator"];

/// `getIterableTypesWithMissingValueTypehint()`: "iterable type X" for each iterable with an implicit `mixed`
/// value type, each once.
pub fn iterables_without_value(ty: &Type) -> Vec<String> {
    let out = RefCell::new(Vec::new());
    iterable_walk(ty, &out);
    unique(out.into_inner())
}

/// PHPStan 2.3 keys the found descriptions by themselves: the first of equal ones stays, in its place.
fn unique<T: PartialEq>(found: Vec<T>) -> Vec<T> {
    let mut out = Vec::new();
    for f in found {
        if !out.contains(&f) {
            out.push(f);
        }
    }
    out
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
/// type arguments, each once.
pub fn generics_without_args(ty: &Type, check_iterable_classes: bool) -> Vec<(String, String)> {
    let out = RefCell::new(Vec::new());
    generic_walk(ty, check_iterable_classes, &out);
    unique(out.into_inner())
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
                // an enum's `@template` is ignored (enum.generic; oracle-checked)
                || c.is_enum
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
            let kind = if c.is_interface { "interface" } else { "class" };
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
    let (context, display) = c.member_scope();
    let declaring =
        declaring(c, m.span()).expect("a method is declared by its class or a used trait");
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
pub(super) fn declaring<'a>(
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
    let (context, display) = c.member_scope();
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
    let (context, display) = c.member_scope();
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

/// The class-level checks of InClassNode (or a trait's own node): MethodTagCheck, PropertyTagCheck and MixinCheck
/// (`checkMissingTypehints`, in the trait definition context for a trait) and GenericAncestorsCheck's generic
/// ancestors named without type arguments (ClassAncestorsRule, InterfaceAncestorsRule, EnumAncestorsRule).
fn class_level(
    c: &mut Checker,
    span: mago_span::Span,
    name: &str,
    kind: &str,
    natives: &[(String, &str)],
) {
    if c.level < 6 {
        return;
    }
    let k = c
        .codebase
        .get_class_like(name.as_bytes())
        .expect("a declared class-like is in the codebase");
    let display = declared::original_name(k);
    let iterables = c.generic_iterable_classes;
    let doc =
        super::doc::doc_before(&c.file.contents, span.start.offset as usize).map(<[u8]>::to_vec);
    let found = declared::with_classes(c, || {
        let mut out: Found = Vec::new();
        let generics_of = |ty: &Type, out: &mut Found, text: &dyn Fn(&str, &str) -> String| {
            for (inner, list) in generics_without_args(ty, iterables) {
                out.push(("missingType.generics", text(&inner, &list), None));
            }
        };
        let iterables_of = |ty: &Type, out: &mut Found, text: &dyn Fn(&str) -> String| {
            for iterable in iterables_without_value(ty) {
                out.push(("missingType.iterableValue", text(&iterable), Some(TIP)));
            }
        };
        let mut described: Vec<String> = Vec::new();
        if let Some(doc) = &doc {
            let parsed = super::doc::Doc::parse(doc);
            let scope = declared::class_scope(c, span, k);
            for m in &parsed.methods {
                let scope = declared::with_method_templates(&scope, k, &m.name, &m.templates);
                let check_one = |what: &str, ty: &Type, out: &mut Found| {
                    generics_of(ty, out, &|inner, list| {
                        format!(
                            "PHPDoc tag @method for method {display}::{}() {what} contains generic {inner} but does not specify its types: {list}",
                            m.name
                        )
                    });
                    iterables_of(ty, out, &|iterable| {
                        format!(
                            "{kind} {display} has PHPDoc tag @method for method {}() {what} with no value type specified in {iterable}.",
                            m.name
                        )
                    });
                };
                for (i, (pname, node, null_default)) in m.params.iter().enumerate() {
                    let mut ty = node.as_ref().map_or_else(pun_phpstan::types::mixed, |n| {
                        pun_phpstan::resolve::resolve_in(n, &scope)
                    });
                    if *null_default {
                        ty = ty.add_null();
                    }
                    check_one(&format!("parameter #{} ${pname}", i + 1), &ty, &mut out);
                }
                let ret = m.ret.as_ref().map_or_else(pun_phpstan::types::mixed, |n| {
                    pun_phpstan::resolve::resolve_in(n, &scope)
                });
                check_one("return type", &ret, &mut out);
            }
            for (pname, read, write) in &parsed.properties {
                let read = read
                    .as_ref()
                    .map(|n| pun_phpstan::resolve::resolve_in(n, &scope));
                let write = write
                    .as_ref()
                    .map(|n| pun_phpstan::resolve::resolve_in(n, &scope));
                let (tag, types) = match (read, write) {
                    (Some(r), Some(w)) if r == w => ("@property", vec![r]),
                    (Some(r), Some(w)) => ("@property", vec![r, w]),
                    (Some(r), None) => ("@property-read", vec![r]),
                    (r, w) => ("@property-write", w.into_iter().chain(r).collect()),
                };
                for ty in &types {
                    generics_of(ty, &mut out, &|inner, list| {
                        format!(
                            "PHPDoc tag {tag} for property {display}::${pname} contains generic {inner} but does not specify its types: {list}"
                        )
                    });
                    iterables_of(ty, &mut out, &|iterable| {
                        format!(
                            "{kind} {display} has PHPDoc tag {tag} for property ${pname} with no value type specified in {iterable}."
                        )
                    });
                }
            }
            for node in &parsed.mixins {
                let ty = pun_phpstan::resolve::resolve_in(node, &scope);
                // (a non-object mixin is mixin.nonObject instead)
                if !matches!(
                    ty,
                    Type::Object(..)
                        | Type::ObjectWithoutClass
                        | Type::Template(_)
                        | Type::Intersection(_)
                ) {
                    continue;
                }
                iterables_of(&ty, &mut out, &|iterable| {
                    format!(
                        "{kind} {display} has PHPDoc tag @mixin with no value type specified in {iterable}."
                    )
                });
                generics_of(&ty, &mut out, &|inner, list| {
                    format!(
                        "PHPDoc tag @mixin contains generic {inner} but does not specify its types: {list}"
                    )
                });
            }
            for node in &parsed.ancestors {
                if let Type::Object(name, args) = pun_phpstan::resolve::resolve_in(node, &scope)
                    && !args.is_empty()
                {
                    described.push(name);
                }
            }
        }
        // GenericAncestorsCheck: the native ancestors no `@extends` / `@implements` describes
        for (native, message) in natives {
            if described.iter().any(|d| d == native) {
                continue;
            }
            let Some(class) = symbols::lookup(native) else {
                continue;
            };
            if SKIP_GENERIC.contains(&class.name.as_str()) || class.templates.is_empty() {
                continue;
            }
            let total = class.templates.len();
            let required = class
                .templates
                .iter()
                .filter(|t| t.default.is_none())
                .count();
            if required == 0 {
                continue;
            }
            let mut list = class
                .templates
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            if required != total {
                list.push_str(&format!(" ({required}-{total} required)"));
            }
            out.push((
                "missingType.generics",
                format!("{message} {native} but does not specify its types: {list}"),
                None,
            ));
        }
        out
    });
    report(c, span, found);
}

fn names<'a>(c: &Checker, ids: impl Iterator<Item = &'a Identifier<'a>>) -> Vec<String> {
    ids.map(|id| String::from_utf8_lossy(c.names.get(&id.span())).into_owned())
        .collect()
}

pub fn class(c: &mut Checker, class: &Class) {
    let name = String::from_utf8_lossy(c.names.get(&class.name.span)).into_owned();
    let extends = format!("Class {name} extends generic class");
    let implements = format!("Class {name} implements generic interface");
    let mut natives: Vec<(String, &str)> = Vec::new();
    if let Some(e) = &class.extends {
        natives.extend(
            names(c, e.types.iter())
                .into_iter()
                .map(|n| (n, extends.as_str())),
        );
    }
    if let Some(i) = &class.implements {
        natives.extend(
            names(c, i.types.iter())
                .into_iter()
                .map(|n| (n, implements.as_str())),
        );
    }
    class_level(c, class.span(), &name, "Class", &natives);
}

pub fn interface(c: &mut Checker, i: &Interface) {
    let name = String::from_utf8_lossy(c.names.get(&i.name.span)).into_owned();
    let extends = format!("Interface {name} extends generic interface");
    let natives: Vec<(String, &str)> = i
        .extends
        .as_ref()
        .map(|e| {
            names(c, e.types.iter())
                .into_iter()
                .map(|n| (n, extends.as_str()))
                .collect()
        })
        .unwrap_or_default();
    class_level(c, i.span(), &name, "Interface", &natives);
}

pub fn enum_(c: &mut Checker, e: &Enum) {
    let name = String::from_utf8_lossy(c.names.get(&e.name.span)).into_owned();
    let implements = format!("Enum {name} implements generic interface");
    let natives: Vec<(String, &str)> = e
        .implements
        .as_ref()
        .map(|i| {
            names(c, i.types.iter())
                .into_iter()
                .map(|n| (n, implements.as_str()))
                .collect()
        })
        .unwrap_or_default();
    class_level(c, e.span(), &name, "Enum", &natives);
}

/// A trait's own node (MethodTagTraitRule, PropertyTagTraitRule, MixinTraitRule in the definition context).
pub fn trait_(c: &mut Checker, t: &Trait) {
    let name = String::from_utf8_lossy(c.names.get(&t.name.span)).into_owned();
    class_level(c, t.span(), &name, "Trait", &[]);
}
