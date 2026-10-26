//! AccessPropertiesRule's AccessPropertiesCheck once `checkThisOnly` is off (level 2): a fetch on any expression,
//! through RuleLevelHelper's `findTypeToCheck()`. Fetches on `$this` stay with property_access.rs.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_word::word;
use pun_phpstan::types::{Level, Tri, Type};

use super::super::reflection::Lookup;
use super::super::{Checker, calls, declared, property_access};

pub const IDENTIFIERS: &[&str] = &["property.nonObject", "property.protected", "staticProperty.nonObject", "staticProperty.protected"];

/// The class a receiver type names, for its properties.
fn class_of(t: &Type) -> Option<&str> {
    match t {
        Type::Object(n, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => Some(n),
        Type::Closure(_) => Some("Closure"),
        _ => None,
    }
}

impl<'a> Checker<'a, '_> {
    /// The classes of `k` and its ancestors and traits.
    fn lineage(&self, k: &'a ClassLikeMetadata) -> impl Iterator<Item = &'a ClassLikeMetadata> + '_ {
        std::iter::once(&k.name)
            .chain(k.all_parent_classes.iter())
            .chain(k.all_parent_interfaces.iter())
            .chain(k.used_traits.iter())
            .filter_map(|n| self.codebase.get_class_like(n.as_bytes()))
    }

    /// The `@mixin` types of `k` and its lineage.
    pub(super) fn mixins(&self, k: &'a ClassLikeMetadata) -> Vec<Type> {
        self.lineage(k).flat_map(|a| a.mixins.iter()).map(|m| self.to_phpstan(&m.type_union)).collect()
    }

    /// The classes `@phpstan-require-extends` and `@phpstan-require-implements` of `k` (and its lineage) name, whose
    /// members it has (RequireExtendsPropertiesClassReflectionExtension, RequireExtendsMethodsClassReflectionExtension).
    pub(super) fn required(&self, k: &'a ClassLikeMetadata) -> Vec<&'a ClassLikeMetadata> {
        self.lineage(k)
            .flat_map(|a| a.require_extends.iter().chain(a.require_implements.iter()))
            .filter_map(|n| self.codebase.get_class_like(n.as_bytes()))
            .collect()
    }

    /// [`Self::has_property()`] of class `n`.
    fn class_has_property(&self, n: &str, property: &[u8], is_static: bool) -> Tri {
        // an unknown class is findTypeToCheck()'s ErrorType, never checked
        let Lookup::Found(k) = self.class_like(n) else { return Tri::Yes };
        let has = if is_static {
            property_access::native_property(self, k, property) == Some(true)
        } else {
            property_access::has_instance_property(self, k, property)
        };
        // SimpleXMLElementProperties: every name is a child element
        let xml = std::iter::once(&k.name).chain(k.all_parent_classes.iter()).any(|n| n.as_bytes().eq_ignore_ascii_case(b"SimpleXMLElement"));
        // the class reflection extensions answer instance properties only
        let extended = || {
            // MixinPropertiesClassReflectionExtension; ponytail: a mixin other than a class (`@mixin T`) is not resolved
            self.mixins(k).iter().any(|t| !matches!(t, Type::Object(..)) || self.has_property(t, property, false).yes())
                || xml
                || self.required(k).iter().any(|r| property_access::has_instance_property(self, r, property))
        };
        if has || !is_static && extended() {
            return Tri::Yes;
        }
        let is_final = k.flags.is_final() || k.original_name.as_bytes().starts_with(b"{anonymous-class:");
        let dynamic = !is_static && property_access::allows_dynamic_properties(self, k);
        if dynamic || !is_final { Tri::Maybe } else { Tri::No }
    }

    /// `Type::hasInstanceProperty()` (`is_static` false) or `hasStaticProperty()`; `property` has its `$`.
    fn has_property(&self, t: &Type, property: &[u8], is_static: bool) -> Tri {
        t.query_members(&|t| match t {
            // (an object shape has no static properties)
            Type::ObjectShape(props) => match props.iter().find(|(n, ..)| !is_static && n.as_bytes() == &property[1..]) {
                Some((_, _, optional)) => if *optional { Tri::Maybe } else { Tri::Yes },
                None => Tri::No,
            },
            Type::ObjectWithoutClass | Type::Iterable(..) | Type::Callable(_) => Tri::Maybe,
            t => class_of(t).map_or(Tri::No, |n| self.class_has_property(n, property, is_static)),
        })
    }

    /// The declaring class and visibility of the instance property a type has.
    // ponytail: a union's UnionTypePropertyReflection is its first member's property
    fn property_of(&self, t: &Type, property: &[u8]) -> Option<(&'a ClassLikeMetadata, mago_codex::visibility::Visibility)> {
        match t {
            Type::Union(ts) | Type::Benevolent(ts) | Type::Intersection(ts) => {
                ts.iter().find_map(|t| self.property_of(t, property))
            }
            Type::Template(tt) => self.property_of(&tt.bound, property),
            t => {
                let k = self.class_like(class_of(t)?).found()?;
                let declaring = self.codebase.get_declaring_property_class(k.name.as_bytes(), property)?;
                let d = self.codebase.get_class_like(declaring.as_bytes())?;
                Some((d, d.properties.get(&word(property))?.read_visibility))
            }
        }
    }
}

/// The receivers whose type the check reads: every property fetch's object but `$this`'s (level 2).
pub(super) fn typed(object: &Expression, spans: &mut Vec<Span>) {
    if !is_this(object) {
        spans.push(object.span());
    }
}

fn is_this(e: &Expression) -> bool {
    matches!(e, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// AccessPropertiesCheck::processSingleProperty() on `$object->name` (`nullsafe` for `?->`, `allowed` when an
/// undefined expression is allowed there).
pub(in super::super) fn property_fetch(
    c: &mut Checker,
    span: Span,
    object: &Expression,
    selector: &ClassLikeMemberSelector,
    nullsafe: bool,
    allowed: bool,
) {
    if c.level < 2 || is_this(object) {
        return;
    }
    let ClassLikeMemberSelector::Identifier(id) = selector else {
        return; // ponytail: a dynamic name's constant strings
    };
    let name = String::from_utf8_lossy(id.value).into_owned();
    let property = format!("${name}");
    let property = property.as_bytes();
    let ty = c
        .phpstan_type(object)
        .map(|t| if nullsafe || super::super::assign::contains_nullsafe(object) { super::receivers::non_null(t) } else { t });
    let found = {
        let c = &*c;
        c.find_type_to_check_of(ty, &|t| {
            c.can_access_properties(t).yes()
                && (c.has_property(t, property, false).yes() || c.has_property(t, property, true).yes())
        })
    };
    let Some(t) = found else {
        return;
    };
    let describe = |c: &Checker, t: &Type| {
        let t = match t {
            Type::Static(n, args) => Type::Object(n.clone(), args.clone()),
            Type::This(n) => Type::Object(n.clone(), vec![]),
            t => t.clone(),
        };
        declared::with_classes(c, || t.describe(Level::TypeOnly))
    };
    let can = c.can_access_properties(&t);
    if can.no() || can.maybe() && !allowed {
        let message = format!("Cannot access property ${name} on {}.", describe(c, &t));
        c.report(id.span(), "property.nonObject", message, None);
        return;
    }
    let has = c.has_property(&t, property, false);
    let has_static = c.has_property(&t, property, true);
    // !checkDynamicProperties
    if has.maybe() && !has_static.yes() && allowed {
        return;
    }
    if !has.yes() {
        if c.has_fetch_type(object, &name) {
            return;
        }
        let names = t.object_class_names();
        if let [n] = names.as_slice()
            && let Lookup::Found(k) = c.class_like(n)
        {
            let mut parent = k.direct_parent_class.and_then(|n| c.codebase.get_class_like(n.as_bytes()));
            while let Some(p) = parent {
                if property_access::has_instance_property(c, p, property) {
                    let refused = p.properties.get(&word(property)).is_some_and(|x| {
                        !calls::can_call(c, c.class_scope(), &String::from_utf8_lossy(p.name.as_bytes()), x.read_visibility)
                    });
                    if refused {
                        let message = format!("Access to private property ${name} of parent class {}.", c.display(p));
                        c.report(id.span(), "property.private", message, None);
                    }
                    return;
                }
                parent = p.direct_parent_class.and_then(|n| c.codebase.get_class_like(n.as_bytes()));
            }
        }
        if has_static.yes() {
            let declaring = names
                .iter()
                .find_map(|n| c.codebase.get_declaring_property_class(n.as_bytes(), property))
                .and_then(|d| c.codebase.get_class_like(d.as_bytes()))
                .map_or_else(|| describe(c, &t), |d| c.display(d));
            let message = format!("Non-static access to static property {declaring}::${name}.");
            c.report(id.span(), "staticProperty.nonStaticAccess", message, None);
            return;
        }
        let message = format!("Access to an undefined property {}::${name}.", describe(c, &t));
        c.report(span, "property.notFound", message, Some(property_access::TIP));
        return;
    }
    // ponytail: a write is checked as a read: asymmetric visibility (`private(set)`, `protected(set)`) is not
    let Some((declaring, visibility)) = c.property_of(&t, property) else {
        return;
    };
    if calls::can_call(c, c.class_scope(), &String::from_utf8_lossy(declaring.name.as_bytes()), visibility) {
        return;
    }
    // ClassReflection::getInstanceProperty(): a class allowing dynamic properties answers an unreadable native
    // property with its `@property` tag
    let tagged = t.object_class_names().iter().any(|n| {
        matches!(c.class_like(n), Lookup::Found(k)
            if k.magic_property_ids.contains_key(&word(property)) && property_access::allows_dynamic_properties(c, k))
    });
    if tagged {
        return;
    }
    if allowed && !declaring.flags.is_final() {
        return;
    }
    let v = if visibility.is_private() { "private" } else { "protected" };
    let described = declared::with_classes(c, || t.describe(Level::TypeOnly));
    let message = format!("Access to {v} property {described}::${name}.");
    c.report(id.span(), &format!("property.{v}"), message, None);
}

/// The class expressions of static property fetches the check reads (`$x::$name`; names and keywords are
/// property_access.rs's).
pub(super) fn typed_class(class: &Expression, spans: &mut Vec<Span>) {
    if !matches!(
        class,
        Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)
    ) {
        spans.push(class.span());
    }
}

/// AccessStaticPropertiesCheck::processSingleProperty() on `$class::$name`. As for a named class (property_access.rs),
/// no allowed-undefined context exempts it (oracle-checked: `isset($class::$x)` on `C|false` is reported).
/// `property` has its `$` (a dynamic name is not checked).
pub(in super::super) fn static_property_fetch(c: &mut Checker, p: &StaticPropertyAccess, property: &[u8]) {
    if c.level < 2 {
        return;
    }
    let name = String::from_utf8_lossy(&property[1..]).into_owned();
    let ty = c
        .phpstan_type(p.class)
        .map(|t| if super::super::assign::contains_nullsafe(p.class) { t.remove_null() } else { t });
    let found = {
        let c = &*c;
        c.find_type_to_check_of(ty, &|t| {
            c.can_access_properties(t).yes() && c.has_property(t, property, true).yes()
        })
    };
    let Some(mut t) = found else {
        return;
    };
    if t.is_string().yes() {
        return;
    }
    let for_describe = match &t {
        Type::This(n) => Type::Object(n.clone(), vec![]),
        t => t.clone(),
    };
    let described = declared::with_classes(c, || for_describe.describe(Level::TypeOnly));
    // TypeCombinator::remove($classType, new StringType())
    if let Type::Union(ts) = &t {
        t = pun_phpstan::types::union(ts.iter().filter(|t| !t.is_string().yes()).cloned().collect());
    }
    let at = p.property.span();
    let can = c.can_access_properties(&t);
    if !can.yes() {
        let message = format!("Cannot access static property ${name} on {described}.");
        c.report(at, "staticProperty.nonObject", message, None);
        return;
    }
    let has = c.has_property(&t, property, true);
    let names = t.object_class_names();
    let codebase = c.codebase;
    let declaring = names
        .iter()
        .find_map(|n| codebase.get_declaring_property_class(n.as_bytes(), property))
        .and_then(|d| codebase.get_class_like(d.as_bytes()));
    if !has.yes() {
        if let [n] = names.as_slice()
            && let Lookup::Found(k) = c.class_like(n)
        {
            let mut parent = k.direct_parent_class.and_then(|n| c.codebase.get_class_like(n.as_bytes()));
            while let Some(p) = parent {
                if property_access::native_property(c, p, property) == Some(true) {
                    let refused = p.properties.get(&word(property)).is_some_and(|x| {
                        !calls::can_call(c, c.class_scope(), &String::from_utf8_lossy(p.name.as_bytes()), x.read_visibility)
                    });
                    if refused {
                        let message = format!("Access to private static property ${name} of parent class {}.", c.display(p));
                        c.report(at, "staticProperty.private", message, None);
                    }
                    return;
                }
                parent = p.direct_parent_class.and_then(|n| c.codebase.get_class_like(n.as_bytes()));
            }
        }
        if c.has_property(&t, property, false).yes() {
            let declaring = declaring.map_or_else(|| described.clone(), |d| c.display(d));
            let message = format!("Static access to instance property {declaring}::${name}.");
            c.report(at, "property.staticAccess", message, None);
            return;
        }
        let message = format!("Access to an undefined static property {described}::${name}.");
        c.report(at, "staticProperty.notFound", message, None);
        return;
    }
    let found = declaring.and_then(|d| Some((d, d.properties.get(&word(property))?.read_visibility)));
    let Some((d, visibility)) = found else { return };
    if calls::can_call(c, c.class_scope(), &String::from_utf8_lossy(d.name.as_bytes()), visibility) {
        return;
    }
    let v = if visibility.is_private() { "private" } else { "protected" };
    let message = format!("Access to {v} property ${name} of class {}.", c.display(d));
    c.report(at, &format!("staticProperty.{v}"), message, None);
}
