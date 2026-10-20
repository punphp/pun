//! Level 2's PHPDoc type checks: IncompatiblePhpDocTypeRule (functions, methods) and
//! IncompatiblePropertyHookPhpDocTypeRule over IncompatiblePhpDocTypeCheck, IncompatiblePropertyPhpDocTypeRule and
//! IncompatibleClassConstantPhpDocTypeRule: a PHPDoc type must resolve, fit its native type and name generic
//! classes correctly; `@param` tags must name a parameter, `@param-out` a by-reference one.
//! GenericCallableRuleHelper checks the templates of the PHPDoc types' callables.

use std::collections::HashMap;

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::resolve::resolve_late;
use pun_phpstan::types::{Level, NeverKind, Type, is_super_type_of, mixed};

use super::doc::{Doc, NameScope, doc_before};
use super::{Checker, declared, generics};

pub const IDENTIFIERS: &[&str] = &[
    "parameter.notFound",
    "parameter.unresolvableType",
    "parameter.notByRef",
    "parameter.phpDocType",
    "paramClosureThis.nonClosure",
    "return.unresolvableType",
    "return.phpDocType",
    "property.unresolvableType",
    "property.phpDocType",
    "classConstant.unresolvableType",
    "classConstant.phpDocType",
    "callable.shadowTemplate",
];

type Errors = Vec<(&'static str, String, Option<String>)>;

fn report(c: &mut Checker, at: Span, errors: Errors) {
    for (identifier, message, tip) in errors {
        c.report(at, identifier, message, tip.as_deref());
    }
}

/// `UnresolvableTypeHelper::getUnresolvableType() !== null`.
fn unresolvable(t: &Type) -> bool {
    let mut found = false;
    super::type_aliases::each(t, &mut |t| {
        found |= matches!(t, Type::Error | Type::Never(NeverKind::Implicit));
    });
    found
}

/// GenericObjectTypeCheck with the messages of `what` (e.g. `PHPDoc tag @return`).
fn generic(t: &Type, what: &str, errors: &mut Errors) {
    let s = generics::escape(what);
    let messages = generics::GenericMessages::at(&s);
    let mut found = generics::Found::default();
    generics::generic_object_check(t, &messages, &mut found);
    errors.extend(found.errors.into_iter().map(|(id, m, tip)| (id, m, tip.map(String::from))));
}

/// The `is incompatible with` / `is not subtype of` error of a PHPDoc type against its native type.
fn native_check(
    identifier: &'static str,
    what: &str,
    phpdoc: &Type,
    native: &Type,
    tip: bool,
    errors: &mut Errors,
) {
    // (a late-resolvable type compares as what it resolves to, and describes as itself)
    let is = is_super_type_of(native, &resolve_late(phpdoc));
    let (described, native_described) = (phpdoc.describe(Level::TypeOnly), native.describe(Level::TypeOnly));
    if is.no() {
        errors.push((
            identifier,
            format!("{what} with type {described} is incompatible with native type {native_described}."),
            None,
        ));
    } else if is.maybe() {
        let tip = match phpdoc {
            Type::Template(t) if tip => Some(format!("Write @template {} of {native_described} to fix this.", t.name)),
            _ => None,
        };
        errors.push((
            identifier,
            format!("{what} with type {described} is not subtype of native type {native_described}."),
            tip,
        ));
    }
}

/// A function-like's native parameters (name, type, by reference) and native return type, and where it is for
/// GenericCallableRuleHelper.
struct Native {
    params: Vec<(String, Type, bool)>,
    ret: Type,
    cx: generics::CallableContext,
}

/// GenericCallableRuleHelper::check() at `location`.
fn callables(t: &Type, location: &str, cx: &generics::CallableContext, errors: &mut Errors) {
    let mut found = generics::Found::default();
    generics::generic_callables(t, location, cx, &mut found);
    errors.extend(found.errors.into_iter().map(|(id, m, tip)| (id, m, tip.map(String::from))));
}

/// The class part of a [`generics::CallableContext`]: name, parent, display name and `@template` names.
fn class_context(c: &Checker, k: &mago_codex::metadata::class_like::ClassLikeMetadata, display: String) -> (String, Option<String>, String, Vec<String>) {
    let templates = declared::class_templates(c, k).into_iter().map(|t| t.name).collect();
    (declared::original_name(k), declared::parent_name(c.codebase, k), display, templates)
}

/// A late-resolvable type Pun keeps unresolved: a `key-of` / `value-of` of a template (kept as a class name) or an
/// OffsetAccessType [`resolve_late()`] leaves.
// ponytail: PHPStan checks these too (e.g. a `T['k']` against the native type); Pun skips the PHPDoc type
fn late(t: &Type) -> bool {
    let found = std::cell::Cell::new(
        matches!(t, Type::Object(n, _) if n == "key-of" || n == "value-of") || matches!(t, Type::OffsetAccess(..)),
    );
    t.traverse(&|i| {
        found.set(found.get() || late(i));
        i.clone()
    });
    found.get()
}

/// IncompatiblePhpDocTypeCheck::check() on the PHPDoc `doc` resolved in `scope`.
fn check(raw: &[u8], doc: &Doc, scope: &NameScope, native: &Native) -> Errors {
    let mut errors = Errors::new();
    let params: HashMap<String, (Type, bool)> = doc.param_types(scope);
    let outs = Doc::named_types(&doc.outs, scope);
    let closure_this = Doc::named_types(&doc.closure_this, scope);
    let mut tags: Vec<(&str, String, Type, bool)> = Vec::new();
    tags.extend(params.into_iter().map(|(n, (t, variadic))| ("@param", n, t, variadic)));
    tags.extend(outs.into_iter().map(|(n, t)| ("@param-out", n, t, false)));
    tags.extend(closure_this.into_iter().map(|(n, t)| ("@param-closure-this", n, t, false)));
    for (tag, name, mut ty, variadic) in tags {
        if late(&resolve_late(&ty)) {
            continue;
        }
        let Some((_, native_type, by_ref)) = native.params.iter().find(|p| p.0 == name) else {
            errors.push(("parameter.notFound", format!("PHPDoc tag {tag} references unknown parameter: ${name}"), None));
            continue;
        };
        // (the tag's type is intersected with `object`, so a non-object one is never)
        let non_object = tag == "@param-closure-this" && is_super_type_of(&Type::ObjectWithoutClass, &ty).no();
        if non_object || unresolvable(&ty) {
            errors.push((
                "parameter.unresolvableType",
                format!("PHPDoc tag {tag} for parameter ${name} contains unresolvable type."),
                None,
            ));
            continue;
        }
        if variadic && ty.is_array().yes() && native_type.is_array().no() {
            ty = ty.iterable_value_type();
        }
        let what = format!("PHPDoc tag {tag} for parameter ${name}");
        generic(&ty, &what, &mut errors);
        callables(&ty, &format!("{tag} for parameter ${name}"), &native.cx, &mut errors);
        match tag {
            "@param-out" => {
                if !by_ref {
                    errors.push((
                        "parameter.notByRef",
                        format!("Parameter ${name} for PHPDoc tag {tag} is not passed by reference."),
                        None,
                    ));
                }
            }
            "@param" => native_check("parameter.phpDocType", &what, &ty, native_type, true, &mut errors),
            _ => {
                if is_super_type_of(&Type::Object("Closure".into(), vec![]), native_type).no() {
                    errors.push((
                        "paramClosureThis.nonClosure",
                        format!(
                            "PHPDoc tag {tag} is for parameter ${name} with non-Closure type {}.",
                            native_type.describe(Level::TypeOnly)
                        ),
                        None,
                    ));
                }
            }
        }
    }
    // (a `@return` whose value does not parse is an InvalidTagValueNode, no return tag)
    // ponytail: Doc::parse's approximate reader still reads it; PLAN.md "move doc::Doc::parse ... onto phpdoc::tags"
    let invalid_return = pun_phpstan::phpdoc::tags(&String::from_utf8_lossy(raw))
        .unwrap_or_default()
        .iter()
        .any(|t| t.name.ends_with("return") && matches!(t.value, pun_phpstan::phpdoc::TagValue::Invalid(..)));
    if let Some(ret) = doc.return_type(scope).filter(|t| !invalid_return && !late(&resolve_late(t))) {
        if unresolvable(&ret) {
            errors.push(("return.unresolvableType", "PHPDoc tag @return contains unresolvable type.".into(), None));
        } else {
            generic(&ret, "PHPDoc tag @return", &mut errors);
            callables(&ret, "@return", &native.cx, &mut errors);
            native_check("return.phpDocType", "PHPDoc tag @return", &ret, &native.ret, true, &mut errors);
        }
    }
    errors
}

/// `getFunctionType($param->type, isParameterValueNullable($param), false)` of each parameter.
fn native_params(c: &Checker, list: &FunctionLikeParameterList, class: Option<&mago_codex::metadata::class_like::ClassLikeMetadata>) -> Vec<(String, Type, bool)> {
    list.parameters
        .iter()
        .map(|p| {
            let mut ty = p.hint.as_ref().map_or_else(mixed, |h| declared::native(c, h, class));
            if declared::default_is_null(p) {
                ty = ty.add_null();
            }
            (String::from_utf8_lossy(&p.variable.name[1..]).into_owned(), ty, p.ampersand.is_some())
        })
        .collect()
}

fn native_ret(c: &Checker, ret: Option<&FunctionLikeReturnTypeHint>, class: Option<&mago_codex::metadata::class_like::ClassLikeMetadata>) -> Type {
    ret.map_or_else(mixed, |r| declared::native(c, &r.hint, class))
}

pub fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let Some(raw) = doc_before(&c.file.contents, f.span().start.offset as usize) else {
        return;
    };
    let doc = Doc::parse(raw);
    let errors = declared::with_classes(c, || {
        let scope = declared::function_scope(c, f, &doc);
        let templates = doc.templates.iter().map(|t| t.name.clone()).collect();
        let native = Native {
            params: native_params(c, &f.parameter_list, None),
            ret: native_ret(c, f.return_type_hint.as_ref(), None),
            cx: generics::CallableContext {
                function: Some((format!("function {}", c.resolved_span(f.name.span)), templates)),
                class: None,
            },
        };
        check(raw, &doc, &scope, &native)
    });
    report(c, f.span(), errors);
}

pub fn method(c: &mut Checker, m: &Method) {
    if c.level < 2 {
        return;
    }
    let Some(raw) = doc_before(&c.file.contents, m.span().start.offset as usize) else {
        return;
    };
    let doc = Doc::parse(raw);
    let Some(context) = c.class_scope() else { return };
    let declaring = super::missing_types::declaring(c, m.span()).expect("a method is declared by its class or a used trait");
    let display = c.member_scope().1;
    let errors = declared::with_classes(c, || {
        let scope = declared::method_scope(c, m, &doc, declaring, context);
        let templates = doc.templates.iter().map(|t| t.name.clone()).collect();
        let native = Native {
            params: native_params(c, &m.parameter_list, Some(context)),
            ret: native_ret(c, m.return_type_hint.as_ref(), Some(context)),
            cx: generics::CallableContext {
                function: Some((format!("method {display}::{}", String::from_utf8_lossy(m.name.value)), templates)),
                class: Some(class_context(c, context, display.clone())),
            },
        };
        check(raw, &doc, &scope, &native)
    });
    report(c, m.span(), errors);
}

/// IncompatiblePropertyHookPhpDocTypeRule: a hook's parameters are never by reference; a `get` hook returns the
/// property's native type, a `set` hook `void`.
pub fn hook(c: &mut Checker, h: &PropertyHook, hooked: &[u8], hint: Option<&Hint>) {
    if c.level < 2 {
        return;
    }
    let Some(raw) = doc_before(&c.file.contents, h.span().start.offset as usize) else {
        return;
    };
    let doc = Doc::parse(raw);
    let Some(context) = c.class_scope() else { return };
    let display = c.member_scope().1;
    let errors = declared::with_classes(c, || {
        let scope = declared::scope_at(c, h.span(), Some(context));
        let property = hint.map_or_else(mixed, |h| declared::native(c, h, Some(context)));
        let get = h.name.value.eq_ignore_ascii_case(b"get");
        let params = match &h.parameter_list {
            Some(list) => native_params(c, list, Some(context))
                .into_iter()
                .zip(list.parameters.iter())
                .map(|((n, t, _), p)| (n, if p.hint.is_some() { t } else { property.clone() }, false))
                .collect(),
            None if get => Vec::new(),
            None => vec![("value".to_string(), property.clone(), false)],
        };
        let templates = doc.templates.iter().map(|t| t.name.clone()).collect();
        let function = format!("method {display}::{}::{}", String::from_utf8_lossy(hooked), String::from_utf8_lossy(h.name.value));
        let native = Native {
            params,
            ret: if get { property } else { Type::Void },
            cx: generics::CallableContext {
                function: Some((function, templates)),
                class: Some(class_context(c, context, display.clone())),
            },
        };
        check(raw, &doc, &scope, &native)
    });
    report(c, h.span(), errors);
}

/// IncompatiblePropertyPhpDocTypeRule (`ClassPropertyNode`) of a declared property: the declaration's single
/// unnamed `@var`, else the one naming the property.
pub fn property(c: &mut Checker, p: &Property) {
    if c.level < 2 {
        return;
    }
    let Some(doc) = doc_before(&c.file.contents, p.span().start.offset as usize) else {
        return;
    };
    let doc = Doc::parse(doc);
    let (context, display) = c.member_scope();
    let hint = p.hint();
    for v in p.variables() {
        let name = String::from_utf8_lossy(&v.name[1..]).into_owned();
        let errors = declared::with_classes(c, || {
            let scope = declared::scope_at(c, p.span(), Some(context));
            let vars = doc.var_types(&scope);
            let phpdoc = match vars.as_slice() {
                [(n, t)] if n.is_empty() => Some(t.clone()),
                _ => vars.into_iter().find(|(n, _)| *n == name).map(|(_, t)| t),
            };
            let native = hint.map(|h| declared::native(c, h, Some(context)));
            phpdoc.filter(|t| !late(&resolve_late(t))).map(|t| {
                let mut errors = Errors::new();
                let cx = generics::CallableContext { function: None, class: Some(class_context(c, context, display.clone())) };
                callables(&t, "@var", &cx, &mut errors);
                errors.extend(property_check(&t, native.as_ref(), "PHPDoc tag @var", &display, &name));
                errors
            })
        });
        report(c, v.span(), errors.unwrap_or_default());
    }
}

/// A promoted constructor parameter's property: its own `@var`, else the constructor's `@param` for it.
pub fn promoted(c: &mut Checker, m: &Method) {
    if c.level < 2 || !m.name.value.eq_ignore_ascii_case(b"__construct") {
        return;
    }
    let Some((context, display)) = c.class_scope().map(|_| c.member_scope()) else { return };
    let declaring = super::missing_types::declaring(c, m.span()).expect("a method is declared by its class or a used trait");
    for p in m.parameter_list.parameters.iter().filter(|p| p.is_promoted_property()) {
        let name = String::from_utf8_lossy(&p.variable.name[1..]).into_owned();
        let errors = declared::with_classes(c, || {
            let own = doc_before(&c.file.contents, p.span().start.offset as usize).and_then(|doc| {
                let doc = Doc::parse(doc);
                let vars = doc.var_types(&declared::scope_at(c, p.span(), Some(context)));
                match vars.as_slice() {
                    [(n, t)] if n.is_empty() => Some(Some(t.clone())),
                    _ => vars.into_iter().find(|(n, _)| *n == name).map(|(_, t)| Some(t)),
                }
            });
            let phpdoc = own.unwrap_or_else(|| {
                let doc = Doc::parse(doc_before(&c.file.contents, m.span().start.offset as usize)?);
                let scope = declared::method_scope(c, m, &doc, declaring, context);
                doc.param_types(&scope).remove(&name).map(|(t, _)| t)
            });
            let native = p.hint.as_ref().map(|h| declared::native(c, h, Some(context)));
            phpdoc.filter(|t| !late(&resolve_late(t))).map(|t| property_check(&t, native.as_ref(), "PHPDoc type", &display, &name))
        });
        report(c, p.span(), errors.unwrap_or_default());
    }
}

fn property_check(phpdoc: &Type, native: Option<&Type>, description: &str, class: &str, name: &str) -> Errors {
    let mut errors = Errors::new();
    let what = format!("{description} for property {class}::${name}");
    if unresolvable(phpdoc) {
        errors.push(("property.unresolvableType", format!("{what} contains unresolvable type."), None));
    }
    if let Some(native) = native {
        native_check("property.phpDocType", &what, phpdoc, native, true, &mut errors);
    }
    generic(phpdoc, &what, &mut errors);
    errors
}

/// IncompatibleClassConstantPhpDocTypeRule (`ClassConst`).
pub fn class_constant(c: &mut Checker, k: &ClassLikeConstant) {
    if c.level < 2 {
        return;
    }
    let (context, display) = c.member_scope();
    let name_of = declared::original_name(context);
    for item in k.items.iter() {
        let constant = String::from_utf8_lossy(item.name.value).into_owned();
        let errors = declared::with_classes(c, || {
            let phpdoc = declared::constant(c, k.span(), item.name.value, context)?;
            let native = k.hint.as_ref().map(|h| declared::native(c, h, Some(context)));
            // resolveConstantVarPhpDocType(): an inherited `@var` the native type does not certainly accept is dropped
            let own = doc_before(&c.file.contents, k.span().start.offset as usize).is_some();
            if !own && native.as_ref().is_some_and(|n| !is_super_type_of(n, &phpdoc).yes()) {
                return None;
            }
            let mut errors = Errors::new();
            if unresolvable(&phpdoc) {
                errors.push((
                    "classConstant.unresolvableType",
                    format!("PHPDoc tag @var for constant {name_of}::{constant} contains unresolvable type."),
                    None,
                ));
            } else if let Some(native) = native {
                let what = format!("PHPDoc tag @var for constant {display}::{constant}");
                native_check("classConstant.phpDocType", &what, &phpdoc, &native, false, &mut errors);
            }
            generic(&phpdoc, &format!("PHPDoc tag @var for constant {display}::{constant}"), &mut errors);
            Some(errors)
        });
        report(c, k.span(), errors.unwrap_or_default());
    }
}
