//! The level-2 parts of MethodTagCheck, PropertyTagCheck and MixinCheck: MethodTagRule, PropertyTagRule and
//! MixinRule on a class, interface or enum (InClassNode), MixinTraitRule on a trait declaration, and
//! MethodTagTraitUseRule, PropertyTagTraitUseRule and MixinTraitUseRule on a trait in the context of a class
//! using it (InTraitNode). The `checkMissingTypehints` parts are level 6 (missing_types.rs).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_span::Span;
use pun_phpstan::phpdoc::{self, TagValue};
use pun_phpstan::resolve::{const_expr, resolve_in};
use pun_phpstan::symbols;
use pun_phpstan::types::{Level, NeverKind, Type, mixed};

use super::declared::{self, original_name};
use super::doc::NameScope;
use super::generics::{self, GenericMessages};
use super::{Checker, DISCOVERING_SYMBOLS_TIP};

pub(super) const IDENTIFIERS: &[&str] = &[
    "methodTag.trait",
    "methodTag.unresolvableType",
    "propertyTag.trait",
    "propertyTag.unresolvableType",
    "mixin.nonObject",
    "mixin.unresolvableType",
    "mixin.trait",
];

/// A class-like's `@method`, `@property` and `@mixin` tags, resolved (PhpDocNodeResolver).
#[derive(Default)]
struct Tags {
    /// (method, [(parameter description, type)], return type); a later tag for a method replaces an earlier one
    methods: Vec<(String, Vec<(String, Type)>)>,
    /// (property, tag name, types)
    properties: Vec<(String, &'static str, Vec<Type>)>,
    mixins: Vec<Type>,
}

/// PHP's `$array[$key] = $value`: replaced in place, or appended.
fn set<K: PartialEq, V>(list: &mut Vec<(K, V)>, key: K, value: V) {
    match list.iter_mut().find(|(k, _)| *k == key) {
        Some(entry) => entry.1 = value,
        None => list.push((key, value)),
    }
}

fn tags(c: &Checker, k: &ClassLikeMetadata, scope: &NameScope) -> Tags {
    let Some((_, entry)) = c.class_doc(k) else {
        return Tags::default();
    };
    let tags = &entry.phpdoc;
    let named = |names: &[&str]| -> Vec<&TagValue> {
        names
            .iter()
            .flat_map(|name| tags.iter().filter(move |t| t.name == *name))
            .map(|t| &t.value)
            .collect()
    };
    let mut out = Tags::default();
    let mut methods = Vec::new();
    for value in named(&["@method", "@phan-method", "@psalm-method", "@phpstan-method"]) {
        let TagValue::Method {
            ret,
            name,
            params,
            templates,
            ..
        } = value
        else {
            continue;
        };
        let scope = if templates.is_empty() || scope.class.is_none() {
            scope.clone()
        } else {
            let templates: Vec<_> = templates
                .iter()
                .map(|t| super::doc::TemplateTag {
                    name: t.name.clone(),
                    bound: t.bound.clone(),
                    default: t.default.clone(),
                })
                .collect();
            declared::with_method_templates(scope, k, name, &templates)
        };
        let mut types = Vec::new();
        for (i, p) in params.iter().enumerate() {
            let description = format!("parameter #{} ${}", i + 1, &p.name[1..]);
            let mut ty = p.ty.as_ref().map_or_else(mixed, |n| resolve_in(n, &scope));
            if matches!(p.default, Some(phpdoc::ConstExpr::Null)) {
                ty = ty.add_null();
            }
            types.push((description.clone(), ty));
            if let Some(default) = &p.default {
                types.push((format!("{description} default value"), const_expr(default, &scope)));
            }
        }
        let ret = ret.as_ref().map_or_else(mixed, |n| resolve_in(n, &scope));
        types.push(("return type".to_string(), ret));
        set(&mut methods, name.clone(), types);
    }
    out.methods = methods;
    // (name, readable, writable)
    let mut properties: Vec<(String, (Option<Type>, Option<Type>))> = Vec::new();
    for (names, read, write) in [
        (&["@property", "@phpstan-property"][..], true, true),
        (&["@property-read", "@phpstan-property-read"][..], true, false),
        (&["@property-write", "@phpstan-property-write"][..], false, true),
    ] {
        for value in named(names) {
            let TagValue::TypedName(node, name) = value else {
                continue;
            };
            let ty = resolve_in(node, scope);
            let name = name[1..].to_string();
            let old = properties
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, v)| v.clone());
            let (old_read, old_write) = old.unwrap_or((None, None));
            let entry = match (read, write) {
                (true, true) => (Some(ty.clone()), Some(ty)),
                (true, false) => (Some(ty), old_write),
                _ => (old_read, Some(ty)),
            };
            set(&mut properties, name, entry);
        }
    }
    out.properties = properties
        .into_iter()
        .map(|(name, (read, write))| match (read, write) {
            (Some(r), Some(w)) if r == w => (name, "@property", vec![r]),
            (Some(r), Some(w)) => (name, "@property", vec![r, w]),
            (Some(r), None) => (name, "@property-read", vec![r]),
            (r, w) => (name, "@property-write", w.into_iter().chain(r).collect()),
        })
        .collect();
    out.mixins = named(&["@mixin"])
        .into_iter()
        .filter_map(|v| match v {
            TagValue::Type(node) => Some(resolve_in(node, scope)),
            _ => None,
        })
        .collect();
    out
}

/// `canCallMethods()->yes() && canAccessProperties()->yes()`
fn object_like(t: &Type) -> bool {
    match t {
        Type::Mixed { .. }
        | Type::Error
        | Type::Never(_)
        | Type::Object(..)
        | Type::ObjectWithoutClass
        | Type::Static(..)
        | Type::This(_)
        | Type::EnumCase(..)
        | Type::Closure(_)
        | Type::ObjectShape(_) => true,
        Type::Union(types) | Type::Benevolent(types) => types.iter().all(object_like),
        Type::Intersection(types) => types.iter().any(object_like),
        Type::Template(t) => object_like(&t.bound),
        _ => false,
    }
}

/// UnresolvableTypeHelper: an error type or an implicit `never` anywhere in `ty`.
fn unresolvable(ty: &Type) -> bool {
    let mut found = false;
    super::type_aliases::each(ty, &mut |t| {
        found |= matches!(t, Type::Error | Type::Never(NeverKind::Implicit));
    });
    found
}

/// What the checks found: errors with an optional tip, and the class names ClassNameCheck reads.
#[derive(Default)]
struct Found {
    errors: Vec<(&'static str, String, Option<&'static str>)>,
    names: Vec<String>,
}

impl Found {
    /// The use-context checks of one type: its referenced classes, unresolvability and generic types. `what` is
    /// the subject of the messages ("PHPDoc tag @mixin"), `trait_id` the identifier of a trait reference.
    fn type_checks(&mut self, ty: &Type, what: &str, trait_id: &'static str, generic: &GenericMessages) {
        for class in generics::referenced_classes(ty) {
            match symbols::lookup(&class) {
                None => self.errors.push((
                    "class.notFound",
                    format!("{what} contains unknown class {class}."),
                    Some(DISCOVERING_SYMBOLS_TIP),
                )),
                Some(k) if k.is_trait => self.errors.push((
                    trait_id,
                    format!("{what} contains invalid type {class}."),
                    None,
                )),
                Some(_) => self.names.push(class),
            }
        }
        let mut found = generics::Found::default();
        generics::generic_object_check(ty, generic, &mut found);
        self.errors
            .extend(found.errors);
    }
}

fn messages(subject: &str) -> GenericMessages {
    let s = generics::escape(subject);
    GenericMessages::at(&s)
}

/// `checkInTraitUseContext()` of the three checks on `tags`, `display` naming the class-like that declares them.
fn use_context(tags: &Tags, display: &str, out: &mut Found) {
    for (method, types) in &tags.methods {
        for (description, ty) in types {
            let what = format!("PHPDoc tag @method for method {display}::{method}() {description}");
            out.type_checks(ty, &what, "methodTag.trait", &messages(&what));
            if unresolvable(ty) {
                out.errors.push((
                    "methodTag.unresolvableType",
                    format!("{what} contains unresolvable type."),
                    None,
                ));
            }
        }
    }
    for (property, tag, types) in &tags.properties {
        for ty in types {
            let what = format!("PHPDoc tag {tag} for property {display}::${property}");
            out.type_checks(ty, &what, "propertyTag.trait", &messages(&what));
            if unresolvable(ty) {
                out.errors.push((
                    "propertyTag.unresolvableType",
                    format!("{what} contains unresolvable type."),
                    None,
                ));
            }
        }
    }
    for ty in &tags.mixins {
        if unresolvable(ty) {
            out.errors.push((
                "mixin.unresolvableType",
                "PHPDoc tag @mixin contains unresolvable type.".to_string(),
                None,
            ));
            continue;
        }
        out.type_checks(ty, "PHPDoc tag @mixin", "mixin.trait", &messages("PHPDoc tag @mixin"));
    }
}

/// MixinCheck::checkInTraitDefinitionContext() at level 2: a mixin that is no object.
fn definition_context(tags: &Tags, out: &mut Found) {
    for ty in &tags.mixins {
        if !object_like(ty) {
            out.errors.push((
                "mixin.nonObject",
                format!(
                    "PHPDoc tag @mixin contains non-object type {}.",
                    ty.describe(Level::TypeOnly)
                ),
                None,
            ));
        }
    }
}

fn report(c: &mut Checker, at: Span, found: Found) {
    let pairs: Vec<_> = found
        .names
        .into_iter()
        .map(|name| super::class_names::Pair::node(name, at))
        .collect();
    let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
    super::class_names::check(c, &pairs, case);
    for (identifier, message, tip) in found.errors {
        c.report(at, identifier, message, tip);
    }
}

/// MethodTagRule, PropertyTagRule and MixinRule on a class, interface or enum declared at `at`.
pub(super) fn class_like(c: &mut Checker, at: Span, k: &ClassLikeMetadata) {
    if c.level < 2 {
        return;
    }
    let mut found = Found::default();
    declared::with_classes(c, || {
        let tags = tags(c, k, &declared::class_scope(c, k.span, k));
        definition_context(&tags, &mut found);
        use_context(&tags, &original_name(k), &mut found);
    });
    report(c, at, found);
}

/// MixinTraitRule on trait `name` declared at `at`.
pub(super) fn trait_definition(c: &mut Checker, at: Span, name: &str) {
    if c.level < 2 {
        return;
    }
    let k = c
        .codebase
        .get_class_like(name.as_bytes())
        .expect("a declared trait is in the codebase");
    let mut found = Found::default();
    declared::with_classes(c, || {
        definition_context(&tags(c, k, &declared::class_scope(c, k.span, k)), &mut found);
    });
    report(c, at, found);
}

/// MethodTagTraitUseRule, PropertyTagTraitUseRule and MixinTraitUseRule on trait `name` in the context of
/// `class`, the class using it (which `self` and `static` mean).
pub(super) fn trait_use(c: &mut Checker, at: Span, name: &str, class: &str) {
    if c.level < 2 {
        return;
    }
    // (the class's walk put both in the codebase)
    let k = c.codebase.get_class_like(name.as_bytes()).expect("a trait");
    let using = c.codebase.get_class_like(class.as_bytes()).expect("a class");
    let mut found = Found::default();
    declared::with_classes(c, || {
        let mut scope = declared::class_scope(c, k.span, k);
        scope.class = Some(original_name(using));
        scope.parent = using
            .direct_parent_class
            .and_then(|p| c.codebase.get_class_like(p.as_bytes()))
            .map(original_name);
        use_context(&tags(c, k, &scope), &original_name(k), &mut found);
    });
    report(c, at, found);
}
