//! Level 2: PHPStan's Generics rules. TemplateTypeCheck (ClassTemplateTypeRule, InterfaceTemplateTypeRule,
//! TraitTemplateTypeRule, FunctionTemplateTypeRule, MethodTemplateTypeRule, MethodTagTemplateTypeRule and
//! MethodTagTemplateTypeTraitRule), EnumTemplateTypeRule, and GenericAncestorsCheck (ClassAncestorsRule,
//! InterfaceAncestorsRule, EnumAncestorsRule, UsedTraitsRule), both through GenericObjectTypeCheck.
//!
//! ponytail: unresolvable ancestor types and CrossCheckInterfacesHelper are not
//! ported yet.

use std::cell::RefCell;

use mago_names::kind::NameKind;
use mago_names::scope::NamespaceScope;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::resolve::{Scope, has_global_alias, resolve_in, resolve_str};
use pun_phpstan::symbols::{self, Class as Reflected};
use pun_phpstan::types::{Level, Template, Type, ancestor_args, is_super_type_of};

use super::Checker;
use super::class_names::{self, Pair};
use super::declared;
use super::doc::{Doc, NameScope, TemplateTag};
use super::facts::DocEntry;

pub(super) const IDENTIFIERS: &[&str] = &[
    "generics.existingClass",
    "generics.existingTypeAlias",
    "generics.traitBound",
    "generics.notSupportedBound",
    "generics.requiredTypeAfterOptional",
    "generics.templateDefaultOutOfBounds",
    "generics.notGeneric",
    "generics.lessTypes",
    "generics.moreTypes",
    "generics.notSubtype",
    "generics.notCompatible",
    "generics.noParent",
    "generics.wrongParent",
    "generics.trait",
    "method.shadowTemplate",
    "methodTag.shadowTemplate",
    "enum.generic",
    "generics.callSiteVarianceConflict",
    "generics.callSiteVarianceRedundant",
    "generics.callSiteVarianceNotAllowed",
];

/// What a check found: (identifier, message, tip); and the class names ClassNameCheck reads.
#[derive(Default)]
pub(super) struct Found {
    /// (identifier, message, tip)
    pub(super) errors: Vec<(&'static str, String, Option<&'static str>)>,
    names: Vec<String>,
}

impl Found {
    fn push(&mut self, identifier: &'static str, message: String) {
        self.errors.push((identifier, message, None));
    }
    fn push_tip(&mut self, identifier: &'static str, message: String, tip: &'static str) {
        self.errors.push((identifier, message, Some(tip)));
    }
}

/// PHP's `sprintf()` with `%s`, `%d` and `%%`: GenericObjectTypeCheck's callers pass formats whose placeholders
/// do not always line up with its arguments, and PHP's output is what PHPStan prints.
fn sprintf(format: &str, args: &[String]) -> String {
    let mut out = String::new();
    let mut args = args.iter();
    let mut chars = format.chars();
    while let Some(ch) = chars.next() {
        if ch != '%' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('s') => out.push_str(args.next().map_or("", String::as_str)),
            Some('d') => out.push_str(&intval(args.next().map_or("", String::as_str))),
            // `%%` (the callers' formats have no other)
            other => out.extend(other),
        }
    }
    out
}

/// `(int) $s` for `%d`: the leading integer, else 0.
fn intval(s: &str) -> String {
    let t = s.trim_start();
    let sign = usize::from(t.starts_with(['-', '+']));
    let digits = t[sign..].bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return "0".to_string();
    }
    t[..sign + digits].trim_start_matches('+').to_string()
}

/// `SprintfHelper::escapeFormatString()`
pub(super) fn escape(s: &str) -> String {
    s.replace('%', "%%")
}

/// `strtolower(getClassTypeDescription())`
fn kind_of(c: &Reflected) -> &'static str {
    if c.is_interface {
        "interface"
    } else if c.is_trait {
        "trait"
    } else if c.is_enum {
        "enum"
    } else {
        "class"
    }
}

/// `getTemplateTypeMap()->getTypes()` of a class.
fn class_templates(c: &Reflected) -> Vec<Type> {
    c.templates
        .iter()
        .map(|t| {
            Type::Template(Box::new(Template {
                name: t.name.clone(),
                scope: format!("class {}", c.name),
                bound: resolve_str(&t.bound, Some(&c.name)),
                default: t.default.as_ref().map(|d| resolve_str(d, Some(&c.name))),
                argument: false,
            }))
        })
        .collect()
}

/// `getReferencedClasses()`
pub(super) fn referenced_classes(ty: &Type) -> Vec<String> {
    fn walk(t: &Type, out: &RefCell<Vec<String>>) {
        // Template subclasses inherit getReferencedClasses() from the bound, not TemplateTypeTrait::traverse().
        // Defaults participate in type traversal, but are not referenced classes of this type.
        if let Type::Template(t) = t {
            walk(&t.bound, out);
            return;
        }
        if let Type::Object(name, _, _)
        | Type::Static(name, _)
        | Type::This(name)
        | Type::EnumCase(name, _) = t
        {
            out.borrow_mut().push(name.clone());
        }
        t.traverse(&|inner| {
            walk(inner, out);
            inner.clone()
        });
    }
    let out = RefCell::new(Vec::new());
    walk(ty, &out);
    out.into_inner()
}

/// A `TemplateTypeMap`: (name, scope, type), the first entry of a template winning.
type TemplateMap = Vec<(String, String, Type)>;

/// `TemplateTypeHelper::resolveTemplateTypes()`: the map's templates replaced.
fn apply(t: &Type, map: &TemplateMap) -> Type {
    if let Type::Template(tt) = t
        && let Some((_, _, with)) = map
            .iter()
            .find(|(name, scope, _)| *name == tt.name && *scope == tt.scope)
    {
        return with.clone();
    }
    t.traverse(&|inner| apply(inner, map))
}

/// `inferTemplateTypes()` of a template (or a generic object over templates) from the type it receives.
fn infer(pattern: &Type, received: &Type, out: &mut TemplateMap) {
    match pattern {
        Type::Template(t) => {
            let mut inner = Vec::new();
            infer(&t.bound, received, &mut inner);
            if is_super_type_of(&apply(&t.bound, &inner).resolve_to_bounds(), received).yes() {
                out.push((t.name.clone(), t.scope.clone(), received.clone()));
                out.extend(inner);
            }
        }
        Type::Object(class, args, _) if !args.is_empty() => {
            let Type::Object(other, other_args, _) = received else {
                return;
            };
            let ancestor = if other.eq_ignore_ascii_case(class) {
                Some(other_args.clone())
            } else {
                symbols::lookup(other).and_then(|c| ancestor_args(&c, class, other_args))
            };
            for (a, o) in args.iter().zip(ancestor.unwrap_or_default().iter()) {
                infer(a, o, out);
            }
        }
        _ => {}
    }
}

/// GenericObjectTypeCheck's messages.
pub(super) struct GenericMessages {
    pub(super) not_generic: String,
    pub(super) less: String,
    pub(super) more: String,
    pub(super) not_subtype: String,
    /// a call-site variance in conflict with the template's, the same as the template's
    pub(super) conflict: String,
    pub(super) redundant: String,
}

impl GenericMessages {
    /// The messages of a PHPDoc type at `s` (escaped, e.g. `PHPDoc tag @var`).
    pub(super) fn at(s: &str) -> GenericMessages {
        GenericMessages {
            not_generic: format!("{s} contains generic type %s but %s %s is not generic."),
            less: format!(
                "Generic type %s in {s} does not specify all template types of %s %s: %s"
            ),
            more: format!(
                "Generic type %s in {s} specifies %d template types, but %s %s supports only %d: %s"
            ),
            not_subtype: format!(
                "Type %s in generic type %s in {s} is not subtype of template type %s of %s %s."
            ),
            conflict: format!(
                "Call-site variance of %s in generic type %s in {s} is in conflict with %s template type %s of %s %s."
            ),
            redundant: format!(
                "Call-site variance of %s in generic type %s in {s} is redundant, template type %s of %s %s has the same variance."
            ),
        }
    }
}

/// `getGenericTypes()`: each generic object or `static` type, resolved to its bounds, outermost first: the type,
/// its class and its arguments.
fn generic_types(ty: &Type) -> Vec<(Type, String, Vec<Type>)> {
    fn walk(t: &Type, out: &RefCell<Vec<(Type, String, Vec<Type>)>>) {
        if let Type::Object(name, args, _) | Type::Static(name, args) = t
            && !args.is_empty()
        {
            let args = args.iter().map(Type::resolve_to_bounds).collect();
            out.borrow_mut()
                .push((t.resolve_to_bounds(), name.clone(), args));
        }
        t.traverse(&|inner| {
            walk(inner, out);
            inner.clone()
        });
    }
    let out = RefCell::new(Vec::new());
    walk(ty, &out);
    out.into_inner()
}

/// GenericObjectTypeCheck::check()
pub(super) fn generic_object_check(ty: &Type, m: &GenericMessages, out: &mut Found) {
    for (generic, name, args) in generic_types(ty) {
        let Some(class) = symbols::lookup(&name) else {
            continue;
        };
        let kind = kind_of(&class).to_string();
        let described = generic.describe(Level::TypeOnly);
        if class.templates.is_empty() {
            out.push(
                "generics.notGeneric",
                sprintf(&m.not_generic, &[described, kind, class.name.clone()]),
            );
            continue;
        }
        let mut templates = class_templates(&class);
        let total = templates.len();
        let required = class
            .templates
            .iter()
            .filter(|t| t.default.is_none())
            .count();
        let names = class
            .templates
            .iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        if required > args.len() {
            let mut list = names;
            if required != total {
                list.push_str(&format!(" ({required}-{total} required)."));
            }
            out.push(
                "generics.lessTypes",
                sprintf(
                    &m.less,
                    &[described.clone(), kind.clone(), class.name.clone(), list],
                ),
            );
        } else if total < args.len() {
            let mut list = names;
            if required != total {
                list.push_str(&format!(" ({required}-{total} required)"));
            }
            out.push(
                "generics.moreTypes",
                sprintf(
                    &m.more,
                    &[
                        described.clone(),
                        args.len().to_string(),
                        kind.clone(),
                        class.name.clone(),
                        total.to_string(),
                        list,
                    ],
                ),
            );
        }
        for i in 0..total {
            let Some(arg) = args.get(i) else {
                continue;
            };
            let template = templates[i].clone();
            if let Type::Projection(covariant, inner) = arg {
                let projection = arg.describe(Level::TypeOnly);
                let variance = class.templates[i].variance;
                if variance == pun_phpstan::types::variance_name(*covariant) {
                    if class.name != "ReflectionClass" || class.templates[i].name != "T" {
                        let args = [
                            projection,
                            described.clone(),
                            template.describe(Level::TypeOnly),
                            kind.clone(),
                            class.name.clone(),
                        ];
                        out.push_tip(
                            "generics.callSiteVarianceRedundant",
                            sprintf(&m.redundant, &args),
                            "You can safely remove the call-site variance annotation.",
                        );
                    }
                } else if variance != "invariant" {
                    let args = [
                        projection,
                        described.clone(),
                        variance.to_string(),
                        template.describe(Level::TypeOnly),
                        kind.clone(),
                        class.name.clone(),
                    ];
                    out.push(
                        "generics.callSiteVarianceConflict",
                        sprintf(&m.conflict, &args),
                    );
                }
                let arg = &**inner;
                if is_super_type_of(&template.resolve_to_bounds(), arg).yes() {
                    let mut map = Vec::new();
                    infer(&template, arg, &mut map);
                    for (j, other) in templates.iter_mut().enumerate() {
                        if j != i {
                            *other = apply(other, &map);
                        }
                    }
                    continue;
                }
                out.push(
                    "generics.notSubtype",
                    sprintf(
                        &m.not_subtype,
                        &[
                            arg.describe(Level::TypeOnly),
                            described.clone(),
                            template.describe(Level::TypeOnly),
                            kind.clone(),
                            class.name.clone(),
                        ],
                    ),
                );
                continue;
            }
            if is_super_type_of(&template.resolve_to_bounds(), arg).yes() {
                let mut map = Vec::new();
                infer(&template, arg, &mut map);
                for (j, other) in templates.iter_mut().enumerate() {
                    if j != i {
                        *other = apply(other, &map);
                    }
                }
                continue;
            }
            // a bivariant `*` argument
            if matches!(arg, Type::Wildcard) {
                continue;
            }
            out.push(
                "generics.notSubtype",
                sprintf(
                    &m.not_subtype,
                    &[
                        arg.describe(Level::TypeOnly),
                        described.clone(),
                        template.describe(Level::TypeOnly),
                        kind.clone(),
                        class.name.clone(),
                    ],
                ),
            );
        }
    }
}

/// The bound types TemplateTypeCheck supports (`get_class()` of the bound, or a union, intersection or template).
fn supported_bound(t: &Type) -> bool {
    matches!(
        t,
        Type::Mixed { .. }
            | Type::ConstArray(_)
            | Type::Array(..)
            | Type::ConstString(_)
            | Type::String
            | Type::ConstInt(_)
            | Type::Int
            | Type::Float
            | Type::Bool
            | Type::ObjectWithoutClass
            | Type::Object(..)
            | Type::ObjectShape(_)
            | Type::Iterable(..)
            | Type::Null
            | Type::Union(_)
            | Type::Benevolent(_)
            | Type::Intersection(_)
            | Type::Template(_)
    )
}

/// One `@template` tag (`TemplateTag`): its name as written, bound and default.
#[derive(PartialEq)]
struct TemplateInput {
    name: String,
    bound: Type,
    default: Option<Type>,
}

/// `resolveTemplateTags()` keyed by name: a later tag of the same name replaces an earlier one in its place. `scope`
/// has the PHPDoc's templates unbounded; a bound naming another of them sees that one's bound.
fn template_inputs(tags: &[TemplateTag], scope: &NameScope) -> Vec<TemplateInput> {
    let resolve_all = |scope: &NameScope| {
        let mut out: Vec<TemplateInput> = Vec::new();
        for t in tags {
            let input = TemplateInput {
                name: t.name.clone(),
                bound: t
                    .bound
                    .as_ref()
                    .map_or(Type::Mixed { explicit: false }, |b| resolve_in(b, scope)),
                default: t.default.as_ref().map(|d| resolve_in(d, scope)),
            };
            match out.iter_mut().find(|x| x.name == t.name) {
                Some(slot) => *slot = input,
                None => out.push(input),
            }
        }
        out
    };
    let mut bounded = scope.clone();
    let mut resolved = resolve_all(scope);
    // (each pass resolves one more level of bounds naming bounds)
    for _ in 0..tags.len() {
        for input in &resolved {
            if let Some(Type::Template(t)) = bounded.templates.get_mut(&input.name) {
                t.bound = input.bound.clone();
                t.default.clone_from(&input.default);
            }
        }
        let next = resolve_all(&bounded);
        if next == resolved {
            break;
        }
        resolved = next;
    }
    resolved
}

/// What a TemplateTypeCheck call says: `@template` or `@method template`, and the subject (`class Foo`,
/// `method Foo::bar()`, `anonymous class`).
struct TemplateSubject<'s> {
    subject: &'s str,
    method_tag: bool,
}

/// TemplateTypeCheck::check()'s message formats (`%s` placeholders): the name is an existing class, an existing
/// type alias; the bound has an invalid class, is not supported; the default has an invalid class, is not in the
/// bound; a required template follows an optional one.
pub(super) struct TemplateMessages {
    pub(super) existing_class: String,
    pub(super) existing_alias: String,
    pub(super) invalid_bound: String,
    pub(super) not_supported: String,
    pub(super) invalid_default: String,
    pub(super) default_not_subtype: String,
    pub(super) required_after_optional: String,
}

impl TemplateSubject<'_> {
    fn messages(&self) -> TemplateMessages {
        let subject = escape(self.subject);
        let tag = if self.method_tag {
            "@method template"
        } else {
            "@template"
        };
        let end = if self.method_tag { "" } else { "." };
        TemplateMessages {
            existing_class: format!(
                "PHPDoc tag {tag} for {subject} cannot have existing class %s as its name."
            ),
            existing_alias: format!(
                "PHPDoc tag {tag} for {subject} cannot have existing type alias %s as its name."
            ),
            invalid_bound: format!("PHPDoc tag {tag} %s for {subject} has invalid bound type %s."),
            not_supported: format!(
                "PHPDoc tag {tag} %s for {subject} with bound type %s is not supported."
            ),
            invalid_default: format!(
                "PHPDoc tag {tag} %s for {subject} has invalid default type %s{end}"
            ),
            default_not_subtype: format!(
                "Default type %s in PHPDoc tag {tag} %s for {subject} is not subtype of bound type %s{end}"
            ),
            required_after_optional: format!(
                "PHPDoc tag @template %s for {subject} does not have a default type but follows an optional @template %s."
            ),
        }
    }
}

/// TemplateTypeCheck::check(). `resolve_name` is `$scope->resolveName()`, `has_alias` `hasTypeAlias()` in the
/// template type scope's class.
fn template_check(
    tags: &[TemplateInput],
    m: &TemplateMessages,
    resolve_name: &dyn Fn(&str) -> String,
    has_alias: &dyn Fn(&str) -> bool,
    out: &mut Found,
) {
    let mut with_default: Option<String> = None;
    for t in tags {
        let name = resolve_name(&t.name);
        if symbols::lookup(&name).is_some() {
            out.push(
                "generics.existingClass",
                sprintf(&m.existing_class, std::slice::from_ref(&name)),
            );
        }
        if has_alias(&name) {
            out.push(
                "generics.existingTypeAlias",
                sprintf(&m.existing_alias, std::slice::from_ref(&name)),
            );
        }
        let referenced = referenced_classes(&t.bound);
        for class in &referenced {
            let message = sprintf(&m.invalid_bound, &[name.clone(), class.clone()]);
            match symbols::lookup(class) {
                None => out.push("class.notFound", message),
                Some(k) if k.is_trait => out.push("generics.traitBound", message),
                Some(_) => {}
            }
        }
        out.names.extend(referenced);
        if !supported_bound(&t.bound) {
            out.push(
                "generics.notSupportedBound",
                sprintf(
                    &m.not_supported,
                    &[name.clone(), t.bound.describe(Level::TypeOnly)],
                ),
            );
        }
        let escaped = escape(&name);
        let bound_messages = GenericMessages {
            not_generic: format!(
                "PHPDoc tag @template {escaped} bound contains generic type %s but %s %s is not generic."
            ),
            less: format!(
                "PHPDoc tag @template {escaped} bound has type %s which does not specify all template types of %s %s: %s"
            ),
            more: format!(
                "PHPDoc tag @template {escaped} bound has type %s which specifies %d template types, but %s %s supports only %d: %s"
            ),
            not_subtype: format!(
                "Type %s in generic type %s in PHPDoc tag @template {escaped} is not subtype of template type %s of %s %s."
            ),
            conflict: format!(
                "Call-site variance of %s in generic type %s in PHPDoc tag @template {escaped} is in conflict with %s template type %s of %s %s."
            ),
            redundant: format!(
                "Call-site variance of %s in generic type %s in PHPDoc tag @template {escaped} is redundant, template type %s of %s %s has the same variance."
            ),
        };
        generic_object_check(&t.bound, &bound_messages, out);
        let Some(default) = &t.default else {
            if let Some(optional) = &with_default {
                out.push(
                    "generics.requiredTypeAfterOptional",
                    sprintf(&m.required_after_optional, &[name, optional.clone()]),
                );
            }
            continue;
        };
        with_default = Some(name.clone());
        let referenced = referenced_classes(default);
        for class in &referenced {
            let message = sprintf(&m.invalid_default, &[name.clone(), class.clone()]);
            match symbols::lookup(class) {
                None => out.push("class.notFound", message),
                Some(k) if k.is_trait => out.push("generics.traitBound", message),
                Some(_) => {}
            }
        }
        out.names.extend(referenced);
        // (these formats name the class kind where the class goes: PHP's sprintf() drops the extra argument)
        let default_messages = GenericMessages {
            not_generic: format!(
                "PHPDoc tag @template {escaped} default contains generic type %s but class %s is not generic."
            ),
            less: format!(
                "PHPDoc tag @template {escaped} default has type %s which does not specify all template types of class %s: %s"
            ),
            more: format!(
                "PHPDoc tag @template {escaped} default has type %s which specifies %d template types, but class %s supports only %d: %s"
            ),
            not_subtype: format!(
                "Type %s in generic type %s in PHPDoc tag @template {escaped} default is not subtype of template type %s of class %s."
            ),
            conflict: format!(
                "Call-site variance of %s in generic type %s in PHPDoc tag @template {escaped} default is in conflict with %s template type %s of %s %s."
            ),
            redundant: format!(
                "Call-site variance of %s in generic type %s in PHPDoc tag @template {escaped} default is redundant, template type %s of %s %s has the same variance."
            ),
        };
        generic_object_check(default, &default_messages, out);
        if !is_super_type_of(&t.bound, default).no() {
            continue;
        }
        out.push(
            "generics.templateDefaultOutOfBounds",
            sprintf(
                &m.default_not_subtype,
                &[
                    default.describe(Level::TypeOnly),
                    name,
                    t.bound.describe(Level::TypeOnly),
                ],
            ),
        );
    }
}

/// Where a PHPDoc type with callables is: the function-like (`function Foo\\bar` / `method Foo::bar`) and its
/// `@template` names, and the class (name, parent, display name, `@template` names).
#[derive(Default)]
pub(super) struct CallableContext {
    pub(super) function: Option<(String, Vec<String>)>,
    pub(super) class: Option<(String, Option<String>, String, Vec<String>)>,
}

/// GenericCallableRuleHelper::check(): each callable's own templates in `ty` (at `location`, e.g. `@param for
/// parameter $x`) through TemplateTypeCheck, and the ones shadowing the function's or the class's.
pub(super) fn generic_callables(ty: &Type, location: &str, cx: &CallableContext, out: &mut Found) {
    let cell = RefCell::new(std::mem::take(out));
    walk_callables(ty, location, cx, &cell);
    *out = cell.into_inner();
}

fn walk_callables(ty: &Type, location: &str, cx: &CallableContext, out: &RefCell<Found>) {
    let sig = match ty {
        Type::Callable(Some(sig)) | Type::Closure(sig) => Some(sig),
        _ => None,
    };
    if let Some(sig) = sig {
        let description = ty.describe(Level::Precise);
        let (loc, desc) = (escape(location), escape(&description));
        let messages = TemplateMessages {
            existing_class: format!(
                "PHPDoc tag {loc} template of {desc} cannot have existing class %s as its name."
            ),
            existing_alias: format!(
                "PHPDoc tag {loc} template of {desc} cannot have existing type alias %s as its name."
            ),
            invalid_bound: format!(
                "PHPDoc tag {loc} template %s of {desc} has invalid bound type %s."
            ),
            not_supported: format!(
                "PHPDoc tag {loc} template %s of {desc} with bound type %s is not supported."
            ),
            invalid_default: format!(
                "PHPDoc tag {loc} template %s of {desc} has invalid default type %s."
            ),
            default_not_subtype: format!(
                "Default type %s in PHPDoc tag {loc} template %s of {desc} is not subtype of bound type %s."
            ),
            required_after_optional: format!(
                "PHPDoc tag {loc} template %s of {desc} does not have a default type but follows an optional template %s."
            ),
        };
        let inputs: Vec<TemplateInput> = sig
            .templates
            .iter()
            .map(|t| TemplateInput {
                name: t.name.clone(),
                bound: t.bound.clone(),
                default: t.default.clone(),
            })
            .collect();
        let resolve_name = |name: &str| match &cx.class {
            Some((class, parent, ..)) => class_resolve_name(class, parent.as_deref())(name),
            None => name.to_string(),
        };
        let out = &mut *out.borrow_mut();
        template_check(
            &inputs,
            &messages,
            &resolve_name,
            &|name| has_global_alias(name),
            out,
        );
        let own = |name: &str| sig.templates.iter().any(|t| t.name == name);
        if let Some((function, templates)) = &cx.function {
            for name in templates.iter().filter(|n| own(n)) {
                out.push(
                    "callable.shadowTemplate",
                    format!("PHPDoc tag {location} template {name} of {description} shadows @template {name} for {function}."),
                );
            }
        }
        if let Some((.., display, templates)) = &cx.class {
            for name in templates.iter().filter(|n| own(n)) {
                out.push(
                    "callable.shadowTemplate",
                    format!("PHPDoc tag {location} template {name} of {description} shadows @template {name} for class {display}."),
                );
            }
        }
    }
    ty.traverse(&|inner| {
        walk_callables(inner, location, cx, out);
        inner.clone()
    });
}

/// `$scope->resolveName()` in a class: `self` and `static` are the class, `parent` its parent.
fn class_resolve_name<'s>(class: &'s str, parent: Option<&'s str>) -> impl Fn(&str) -> String + 's {
    move |name: &str| match name.to_ascii_lowercase().as_str() {
        "self" | "static" => class.to_string(),
        "parent" if parent.is_some() => parent.unwrap_or_default().to_string(),
        _ => name.to_string(),
    }
}

/// `ClassReflection::getTypeAliases()`: a class-like's `@phpstan-type` names and the aliases it imports from classes
/// that have them.
fn type_aliases(c: &Checker, class: &str, depth: usize) -> Vec<String> {
    let Some(k) = c.codebase.get_class_like(class.as_bytes()) else {
        return Vec::new();
    };
    let Some((_, entry, names)) = declared::doc_at(c, k.span) else {
        return Vec::new();
    };
    aliases_of(c, &entry.doc, names, depth)
}

fn aliases_of(c: &Checker, doc: &Doc, names: &NamespaceScope, depth: usize) -> Vec<String> {
    let mut out = doc.type_aliases.clone();
    // (circular imports end here)
    if depth > 8 {
        return out;
    }
    for (alias, from, alias_as) in &doc.alias_imports {
        let (from, _) = names.resolve(NameKind::Default, from.as_bytes());
        let from = String::from_utf8_lossy(&from).into_owned();
        if type_aliases(c, &from, depth + 1).contains(alias) {
            out.push(alias_as.clone().unwrap_or_else(|| alias.clone()));
        }
    }
    out
}

fn report(c: &mut Checker, at: Span, found: Found) {
    let pairs: Vec<Pair> = found
        .names
        .into_iter()
        .map(|name| Pair::node(name, at))
        .collect();
    let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
    class_names::check(c, &pairs, case);
    for (identifier, message, tip) in found.errors {
        c.report(at, identifier, message, tip);
    }
}

/// The class-like checks of InClassNode for a class, interface, enum or anonymous class: the TemplateTypeCheck of
/// its own `@template` tags (`kind` and `display` name it; an enum has EnumTemplateTypeRule instead) and
/// MethodTagTemplateTypeRule.
struct ClassLike<'s> {
    at: Span,
    /// the class's name (`getName()`) and display name
    name: &'s str,
    display: &'s str,
    /// `class Foo`, `interface Foo`, `anonymous class`; `None` for an enum
    subject: Option<String>,
    doc: Option<&'s DocEntry>,
    /// the namespace scope of the PHPDoc
    names: &'s std::sync::Arc<NamespaceScope>,
}

fn class_like(c: &Checker, l: &ClassLike) -> Found {
    let mut found = Found::default();
    let Some(entry) = l.doc else {
        return found;
    };
    let parsed = &entry.doc;
    let k = c.codebase.get_class_like(l.name.as_bytes());
    let parent = k.and_then(|k| k.direct_parent_class.as_ref()).map(|p| {
        c.codebase.get_class_like(p.as_bytes()).map_or_else(
            || String::from_utf8_lossy(p.as_bytes()).into_owned(),
            declared::original_name,
        )
    });
    let mut scope = NameScope {
        names: l.names.clone(),
        class: Some(l.name.to_string()),
        parent: parent.clone(),
        templates: Default::default(),
    };
    for t in &parsed.templates {
        scope.templates.insert(
            t.name.clone(),
            Type::Template(Box::new(Template {
                name: t.name.clone(),
                scope: format!("class {}", l.name),
                bound: Type::Mixed { explicit: false },
                default: None,
                argument: false,
            })),
        );
    }
    let aliases = aliases_of(c, parsed, l.names, 0);
    let resolve_name = class_resolve_name(l.name, parent.as_deref());
    let has_alias = |name: &str| has_global_alias(name) || aliases.iter().any(|a| a == name);
    declared::with_classes(c, || {
        let templates = template_inputs(&parsed.templates, &scope);
        match &l.subject {
            Some(subject) => template_check(
                &templates,
                &TemplateSubject {
                    subject,
                    method_tag: false,
                }
                .messages(),
                &resolve_name,
                &has_alias,
                &mut found,
            ),
            None if !templates.is_empty() => found.push(
                "enum.generic",
                format!(
                    "Enum {} has PHPDoc @template tag{} but enums cannot be generic.",
                    l.display,
                    if templates.len() == 1 { "" } else { "s" }
                ),
            ),
            None => {}
        }
        method_tags(
            parsed,
            &scope,
            l.name,
            l.display,
            &resolve_name,
            &has_alias,
            &mut found,
        );
    });
    found
}

/// MethodTagTemplateTypeCheck: the `@template` tags of each `@method` tag of a class-like's PHPDoc.
fn method_tags(
    parsed: &Doc,
    scope: &NameScope,
    class: &str,
    display: &str,
    resolve_name: &dyn Fn(&str) -> String,
    has_alias: &dyn Fn(&str) -> bool,
    found: &mut Found,
) {
    let class_templates = symbols::lookup(class)
        .map(|k| class_templates(&k))
        .unwrap_or_default();
    for m in &parsed.methods {
        let mut method_scope = scope.clone();
        for t in &m.templates {
            method_scope.templates.insert(
                t.name.clone(),
                Type::Template(Box::new(Template {
                    name: t.name.clone(),
                    scope: format!("method {class}::{}()", m.name),
                    bound: Type::Mixed { explicit: false },
                    default: None,
                    argument: false,
                })),
            );
        }
        let templates = template_inputs(&m.templates, &method_scope);
        let subject = format!("method {display}::{}()", m.name);
        template_check(
            &templates,
            &TemplateSubject {
                subject: &subject,
                method_tag: true,
            }
            .messages(),
            resolve_name,
            has_alias,
            found,
        );
        for t in &templates {
            if let Some(shadowed) = class_templates
                .iter()
                .find(|ct| matches!(ct, Type::Template(x) if x.name == t.name))
            {
                found.push(
                    "methodTag.shadowTemplate",
                    format!(
                        "PHPDoc tag @method template {} for method {display}::{}() shadows @template {} for class {display}.",
                        t.name,
                        m.name,
                        shadowed.describe(Level::TypeOnly)
                    ),
                );
            }
        }
    }
}

/// GenericAncestorsCheck's messages for one tag kind (`@extends`, `@implements`, `@use`).
struct AncestorMessages<'s> {
    tag: &'s str,
    incompatible: String,
    no_names: String,
    /// `%s` the described class, `%s` the names
    no_related: String,
    /// `checkMissingTypehints`: `%s` a generic name without a tag, `%s` its templates
    missing: Option<String>,
    /// VarianceCheck's context: `%s` the ancestor type
    variance: String,
}

/// `resolveExtendsTags()` & co.: each tag's type keyed by its class name, a later tag replacing an earlier one.
fn ancestor_types(nodes: &[pun_phpstan::phpdoc::Node], scope: &NameScope) -> Vec<Type> {
    let mut out: Vec<(String, Type)> = Vec::new();
    for node in nodes {
        // phpdoc-parser's parseExtendsTagValue() reads an identifier and its generic arguments, nothing else
        let pun_phpstan::phpdoc::Node::Generic { base, .. } = node else {
            continue;
        };
        let key = scope.resolve_name(base);
        let ty = resolve_in(node, scope);
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1 = ty,
            None => out.push((key, ty)),
        }
    }
    out.into_iter().map(|(_, t)| t).collect()
}

/// GenericAncestorsCheck::check() without the `missingType.generics` of unused names
/// (MissingTypehintCheck's rules report those).
fn ancestors_check(names: &[String], types: &[Type], m: &AncestorMessages, out: &mut Found) {
    let tag = m.tag;
    let mut unused: Vec<&String> = names.iter().collect();
    for ty in types {
        let Type::Object(class, args, _) = ty else {
            out.push(
                "generics.notCompatible",
                sprintf(&m.incompatible, &[ty.describe(Level::TypeOnly)]),
            );
            continue;
        };
        if !names.contains(class) {
            if names.is_empty() {
                out.push("generics.noParent", m.no_names.clone());
            } else {
                out.push(
                    "generics.wrongParent",
                    sprintf(&m.no_related, &[class.clone(), names.join(", ")]),
                );
            }
            continue;
        }
        unused.retain(|n| *n != class);
        let messages = GenericMessages {
            not_generic: format!(
                "PHPDoc tag {tag} contains generic type %s but %s %s is not generic."
            ),
            less: format!(
                "Generic type %s in PHPDoc tag {tag} does not specify all template types of %s %s: %s"
            ),
            more: format!(
                "Generic type %s in PHPDoc tag {tag} specifies %d template types, but %s %s supports only %d: %s"
            ),
            not_subtype: format!(
                "Type %s in generic type %s in PHPDoc tag {tag} is not subtype of template type %s of %s %s."
            ),
            // (GenericAncestorsCheck passes empty ones)
            conflict: String::new(),
            redundant: String::new(),
        };
        generic_object_check(ty, &messages, out);
        for referenced in referenced_classes(ty) {
            let message = format!("PHPDoc tag {tag} has invalid type {referenced}.");
            match symbols::lookup(&referenced) {
                None => out.push("class.notFound", message),
                Some(_) if referenced == *class => {}
                Some(k) if k.is_trait => out.push("generics.trait", message),
                Some(_) => {}
            }
        }
        let mut variance = Vec::new();
        let context = sprintf(&m.variance, &[ty.describe(Level::TypeOnly)]);
        super::variance::check(
            super::variance::Variance::Static,
            ty,
            &context,
            &mut variance,
        );
        for v in variance {
            out.push("generics.variance", v);
        }
        for arg in args
            .iter()
            .filter(|a| matches!(a, Type::Projection(..) | Type::Wildcard))
        {
            let message = format!(
                "Call-site variance annotation of {} in generic type {} in PHPDoc tag {tag} is not allowed.",
                arg.describe(Level::TypeOnly),
                ty.describe(Level::TypeOnly)
            );
            out.push("generics.callSiteVarianceNotAllowed", message);
        }
    }
    let Some(missing) = &m.missing else {
        return;
    };
    for name in unused {
        let Some(k) = symbols::lookup(name) else {
            continue;
        };
        // `featureToggles.skipCheckGenericClasses`
        if k.name == "DOMNamedNodeMap" || k.templates.is_empty() {
            continue;
        }
        let total = k.templates.len();
        let required = k.templates.iter().filter(|t| t.default.is_none()).count();
        if required == 0 {
            continue;
        }
        let mut list = k
            .templates
            .iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        if required != total {
            list.push_str(&format!(" ({required}-{total} required)"));
        }
        out.push(
            "missingType.generics",
            sprintf(missing, &[name.clone(), list]),
        );
    }
}

pub(super) fn resolved_names<'a>(
    c: &Checker,
    ids: impl Iterator<Item = &'a Identifier<'a>>,
) -> Vec<String> {
    ids.map(|id| c.resolved_span(id.span())).collect()
}

/// The PHPDoc before `offset` in the walked file and the namespace scope there.
fn doc_and_names<'a>(
    c: &Checker<'a, '_>,
    span: Span,
) -> (Option<&'a DocEntry>, &'a std::sync::Arc<NamespaceScope>) {
    let doc = c
        .doc_before(c.file.id, span.start.offset)
        .map(|(_, entry)| entry);
    (doc, c.facts.scopes.at(span.start.offset))
}

/// The class-like's own `@extends` and `@implements` tag types (`getExtendsTags()`, `getImplementsTags()`).
fn class_ancestor_tags(c: &Checker, l: &ClassLike) -> (Vec<Type>, Vec<Type>) {
    let Some(entry) = l.doc else {
        return (Vec::new(), Vec::new());
    };
    let parsed = &entry.doc;
    // (a declared class-like is in the codebase)
    let scope = c
        .codebase
        .get_class_like(l.name.as_bytes())
        .map(|k| declared::class_scope(c, l.at, k))
        .unwrap_or_default();
    declared::with_classes(c, || {
        (
            ancestor_types(&parsed.extends, &scope),
            ancestor_types(&parsed.implements, &scope),
        )
    })
}

fn run_ancestors(
    c: &Checker,
    checks: &[(&[String], &[Type], AncestorMessages)],
    found: &mut Found,
) {
    declared::with_classes(c, || {
        for (names, types, messages) in checks {
            ancestors_check(names, types, messages, found);
        }
    });
}

pub(super) fn class(c: &mut Checker, class: &Class) {
    if c.level < 2 {
        return;
    }
    let name = c.resolved_span(class.name.span);
    let (doc, names) = doc_and_names(c, class.span());
    let l = ClassLike {
        at: class.span(),
        name: &name,
        display: &name,
        subject: Some(format!("class {}", escape(&name))),
        doc,
        names,
    };
    let mut found = class_like(c, &l);
    let (extends, implements) = class_ancestor_tags(c, &l);
    let parents: Vec<String> = class
        .extends
        .as_ref()
        .map(|e| resolved_names(c, e.types.iter()))
        .unwrap_or_default();
    let interfaces: Vec<String> = class
        .implements
        .as_ref()
        .map(|i| resolved_names(c, i.types.iter()))
        .unwrap_or_default();
    let n = escape(&name);
    run_ancestors(
        c,
        &[
            (
                &parents,
                &extends,
                AncestorMessages {
                    tag: "@extends",
                    incompatible: format!("Class {n} @extends tag contains incompatible type %s."),
                    no_names: format!(
                        "Class {name} has @extends tag, but does not extend any class."
                    ),
                    no_related: format!(
                        "The @extends tag of class {n} describes %s but the class extends %s."
                    ),
                    missing: None,
                    variance: format!("in extended type %s of class {n}"),
                },
            ),
            (
                &interfaces,
                &implements,
                AncestorMessages {
                    tag: "@implements",
                    incompatible: format!(
                        "Class {n} @implements tag contains incompatible type %s."
                    ),
                    no_names: format!(
                        "Class {name} has @implements tag, but does not implement any interface."
                    ),
                    no_related: format!(
                        "The @implements tag of class {n} describes %s but the class implements: %s"
                    ),
                    missing: None,
                    variance: format!("in implemented type %s of class {n}"),
                },
            ),
        ],
        &mut found,
    );
    report(c, class.span(), found);
}

pub(super) fn interface(c: &mut Checker, i: &Interface) {
    if c.level < 2 {
        return;
    }
    let name = c.resolved_span(i.name.span);
    let (doc, names) = doc_and_names(c, i.span());
    let l = ClassLike {
        at: i.span(),
        name: &name,
        display: &name,
        subject: Some(format!("interface {}", escape(&name))),
        doc,
        names,
    };
    let mut found = class_like(c, &l);
    let (extends, implements) = class_ancestor_tags(c, &l);
    let parents: Vec<String> = i
        .extends
        .as_ref()
        .map(|e| resolved_names(c, e.types.iter()))
        .unwrap_or_default();
    let n = escape(&name);
    run_ancestors(
        c,
        &[
            (
                &parents,
                &extends,
                AncestorMessages {
                    tag: "@extends",
                    incompatible: format!(
                        "Interface {n} @extends tag contains incompatible type %s."
                    ),
                    no_names: format!(
                        "Interface {name} has @extends tag, but does not extend any interface."
                    ),
                    no_related: format!(
                        "The @extends tag of interface {n} describes %s but the interface extends: %s"
                    ),
                    missing: None,
                    variance: format!("in extended type %s of interface {n}"),
                },
            ),
            (
                &[],
                &implements,
                AncestorMessages {
                    tag: "@implements",
                    incompatible: format!(
                        "Interface {n} @implements tag contains incompatible type %s."
                    ),
                    no_names: format!(
                        "Interface {name} has @implements tag, but can not implement any interface, must extend from it."
                    ),
                    no_related: String::new(),
                    missing: None,
                    variance: String::new(),
                },
            ),
        ],
        &mut found,
    );
    report(c, i.span(), found);
}

pub(super) fn enum_(c: &mut Checker, e: &Enum) {
    if c.level < 2 {
        return;
    }
    let name = c.resolved_span(e.name.span);
    let (doc, names) = doc_and_names(c, e.span());
    let l = ClassLike {
        at: e.span(),
        name: &name,
        display: &name,
        subject: None,
        doc,
        names,
    };
    let mut found = class_like(c, &l);
    let (extends, implements) = class_ancestor_tags(c, &l);
    let interfaces: Vec<String> = e
        .implements
        .as_ref()
        .map(|i| resolved_names(c, i.types.iter()))
        .unwrap_or_default();
    let n = escape(&name);
    run_ancestors(
        c,
        &[
            (
                &[],
                &extends,
                AncestorMessages {
                    tag: "@extends",
                    incompatible: format!("Enum {n} @extends tag contains incompatible type %s."),
                    no_names: format!("Enum {name} has @extends tag, but cannot extend anything."),
                    no_related: String::new(),
                    missing: None,
                    variance: String::new(),
                },
            ),
            (
                &interfaces,
                &implements,
                AncestorMessages {
                    tag: "@implements",
                    incompatible: format!(
                        "Enum {n} @implements tag contains incompatible type %s."
                    ),
                    no_names: format!(
                        "Enum {name} has @implements tag, but does not implement any interface."
                    ),
                    no_related: format!(
                        "The @implements tag of enum {n} describes %s but the enum implements: %s"
                    ),
                    missing: None,
                    variance: format!("in implemented type %s of enum {n}"),
                },
            ),
        ],
        &mut found,
    );
    report(c, e.span(), found);
}

/// ClassTemplateTypeRule and MethodTagTemplateTypeRule for an anonymous class (named `name`, displayed as
/// `display`); ClassAncestorsRule skips anonymous classes.
pub(super) fn anonymous(c: &mut Checker, a: &AnonymousClass, name: &str, display: &str) {
    if c.level < 2 {
        return;
    }
    // php-parser's Class_ node starts at its attributes, modifiers or `class`, and the PHPDoc is before that
    let start = a
        .attribute_lists
        .first()
        .map(HasSpan::span)
        .or_else(|| a.modifiers.first().map(HasSpan::span))
        .unwrap_or_else(|| a.class.span());
    let at = start.join(a.span());
    let (doc, names) = doc_and_names(c, at);
    let l = ClassLike {
        at,
        name,
        display,
        subject: Some("anonymous class".to_string()),
        doc,
        names,
    };
    let found = class_like(c, &l);
    report(c, at, found);
}

/// TraitTemplateTypeRule and MethodTagTemplateTypeTraitRule (the trait's own node, outside any class).
pub(super) fn trait_(c: &mut Checker, t: &Trait) {
    if c.level < 2 {
        return;
    }
    let name = c.resolved_span(t.name.span);
    let (doc, names) = doc_and_names(c, t.span());
    let (Some(entry), Some(k)) = (doc, c.codebase.get_class_like(name.as_bytes())) else {
        return;
    };
    let parsed = &entry.doc;
    let display = c.display(k);
    let scope = declared::class_scope(c, t.span(), k);
    let aliases = aliases_of(c, parsed, names, 0);
    let has_alias = |n: &str| has_global_alias(n) || aliases.iter().any(|a| a == n);
    // not in a class: names resolve as written
    let resolve_name = |n: &str| n.to_string();
    let mut found = Found::default();
    let subject = format!("trait {}", escape(&name));
    declared::with_classes(c, || {
        let templates = template_inputs(&parsed.templates, &scope);
        template_check(
            &templates,
            &TemplateSubject {
                subject: &subject,
                method_tag: false,
            }
            .messages(),
            &resolve_name,
            &has_alias,
            &mut found,
        );
        method_tags(
            parsed,
            &scope,
            &name,
            &display,
            &resolve_name,
            &has_alias,
            &mut found,
        );
    });
    report(c, t.span(), found);
}

/// FunctionTemplateTypeRule
pub(super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let (Some(entry), _) = doc_and_names(c, f.span()) else {
        return;
    };
    let name = c.resolved_span(f.name.span);
    let parsed = &entry.doc;
    let mut scope = declared::scope_at(c, f.span(), None);
    add_templates(&mut scope, &parsed.templates, &format!("function {name}()"));
    let mut found = Found::default();
    let subject = format!("function {}()", escape(&name));
    declared::with_classes(c, || {
        let templates = template_inputs(&parsed.templates, &scope);
        template_check(
            &templates,
            &TemplateSubject {
                subject: &subject,
                method_tag: false,
            }
            .messages(),
            &|n: &str| n.to_string(),
            &|n: &str| has_global_alias(n),
            &mut found,
        );
    });
    report(c, f.span(), found);
}

/// The PHPDoc's own templates in `scope` (bounds are not needed to resolve the tags' types).
fn add_templates(scope: &mut NameScope, tags: &[TemplateTag], describe: &str) {
    for t in tags {
        scope.templates.insert(
            t.name.clone(),
            Type::Template(Box::new(Template {
                name: t.name.clone(),
                scope: describe.to_string(),
                bound: Type::Mixed { explicit: false },
                default: None,
                argument: false,
            })),
        );
    }
}

/// MethodTemplateTypeRule (`method` is the name the method is analysed under).
pub(super) fn method(c: &mut Checker, m: &Method, method: &str) {
    if c.level < 2 {
        return;
    }
    let (Some(doc), Some(k), Some(display)) = (
        doc_and_names(c, m.span()).0,
        c.class_scope(),
        c.decls.last().and_then(|d| d.display.clone()),
    ) else {
        return;
    };
    let class = declared::original_name(k);
    let parsed = &doc.doc;
    let mut scope = declared::scope_at(c, m.span(), Some(k));
    add_templates(
        &mut scope,
        &parsed.templates,
        &format!("method {class}::{method}()"),
    );
    let parent = scope.parent.clone();
    let aliases = type_aliases(c, &class, 0);
    let has_alias = |n: &str| has_global_alias(n) || aliases.iter().any(|a| a == n);
    let resolve_name = class_resolve_name(&class, parent.as_deref());
    let subject = format!("method {}::{}()", escape(&display), escape(method));
    let mut found = Found::default();
    declared::with_classes(c, || {
        let templates = template_inputs(&parsed.templates, &scope);
        template_check(
            &templates,
            &TemplateSubject {
                subject: &subject,
                method_tag: false,
            }
            .messages(),
            &resolve_name,
            &has_alias,
            &mut found,
        );
        let class_templates = symbols::lookup(&class)
            .map(|k| class_templates(&k))
            .unwrap_or_default();
        for t in &templates {
            if let Some(shadowed) = class_templates
                .iter()
                .find(|ct| matches!(ct, Type::Template(x) if x.name == t.name))
            {
                found.push(
                    "method.shadowTemplate",
                    format!(
                        "PHPDoc tag @template {} for method {display}::{method}() shadows @template {} for class {display}.",
                        t.name,
                        shadowed.describe(Level::TypeOnly)
                    ),
                );
            }
        }
    });
    report(c, m.span(), found);
}

/// UsedTraitsRule
pub(super) fn trait_use(c: &mut Checker, u: &TraitUse) {
    // (a `use` of traits is in a class-like)
    let Some(k) = c.class_scope().filter(|_| c.level >= 2) else {
        return;
    };
    let class = declared::original_name(k);
    let (description, kind) = match c.in_trait() {
        Some(t) => {
            let t = c
                .codebase
                .get_class_like(t.as_bytes())
                .map_or_else(|| t.to_string(), declared::original_name);
            (format!("trait {}", escape(&t)), "trait")
        }
        None => {
            let kind = match c.decls.last().map(|d| d.kind) {
                Some(mago_codex::symbol::SymbolKind::Interface) => "interface",
                Some(mago_codex::symbol::SymbolKind::Enum) => "enum",
                _ => "class",
            };
            (format!("{kind} {}", escape(&class)), kind)
        }
    };
    let names = resolved_names(c, u.trait_names.iter());
    let (doc, _) = doc_and_names(c, u.span());
    let mut types = Vec::new();
    if let Some(entry) = doc {
        let parsed = &entry.doc;
        let scope = declared::scope_at(c, u.span(), Some(k));
        types = declared::with_classes(c, || ancestor_types(&parsed.uses, &scope));
    }
    let mut upper = description.clone();
    if let Some(first) = upper.get_mut(..1) {
        first.make_ascii_uppercase();
    }
    let mut found = Found::default();
    run_ancestors(
        c,
        &[(
            &names,
            &types,
            AncestorMessages {
                tag: "@use",
                incompatible: format!("{upper} @use tag contains incompatible type %s."),
                no_names: format!("{upper} has @use tag, but does not use any trait."),
                no_related: format!(
                    "The @use tag of {description} describes %s but the {kind} uses %s."
                ),
                missing: (c.level >= 6).then(|| {
                    format!("{upper} uses generic trait %s but does not specify its types: %s")
                }),
                variance: format!("in used type %s of {description}"),
            },
        )],
        &mut found,
    );
    report(c, u.span(), found);
}

#[cfg(test)]
mod tests {
    use super::{intval, sprintf};

    /// PHP: `sprintf('a %s b %d c %%', 'x', 'Foo\Bar')` is `a x b 0 c %`; `(int) '12ab'` is 12.
    #[test]
    fn php_sprintf() {
        assert_eq!(
            sprintf("a %s b %d c %%", &["x".into(), "Foo\\Bar".into()]),
            "a x b 0 c %"
        );
        assert_eq!(intval("12ab"), "12");
        assert_eq!(intval("-3"), "-3");
    }
}
