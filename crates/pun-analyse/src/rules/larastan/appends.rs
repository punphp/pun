//! ModelAppendsRule: each string in a model's `$appends` must be a computed property, an accessor that is not a
//! database column.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::Checker;
use super::{MODEL, class_is};

/// ModelAppendsRule on a property declaration.
pub(crate) fn property<'arena>(c: &mut Checker<'_, 'arena>, p: &Property<'arena>) {
    let Some(larastan) = c.larastan.filter(|l| {
        l.switches
            .contains(crate::LarastanSwitches::CHECK_MODEL_APPENDS)
    }) else {
        return;
    };
    let first = match p {
        Property::Plain(p) => p.items.first(),
        Property::Hooked(h) => Some(&h.item),
    };
    let Some(PropertyItem::Concrete(item)) = first else {
        return;
    };
    if item.variable.name != b"$appends" {
        return;
    }
    let Some(k) = c.class_scope().filter(|k| class_is(c, k, MODEL)) else {
        return;
    };
    let elements = match item.value {
        Expression::Array(a) => &a.elements,
        Expression::LegacyArray(a) => &a.elements,
        _ => return,
    };
    let class = k.name.as_str_lossy().into_owned();
    let columns = larastan
        .database_properties
        .get(&class.to_ascii_lowercase());
    for element in elements.iter() {
        let value = match element {
            ArrayElement::KeyValue(kv) => kv.value,
            ArrayElement::Value(v) => v.value,
            _ => continue,
        };
        let Expression::Literal(Literal::String(LiteralString {
            value: Some(name), ..
        })) = value
        else {
            continue;
        };
        let name = String::from_utf8_lossy(name).into_owned();
        let database = columns.is_some_and(|p| p.contains(&name)) && !property_tag(k, &name);
        let message = if database {
            format!("Property '{name}' is not a computed property, remove from $appends.")
        } else if !crate::laravel::has_accessor(c.codebase, &class, &name) {
            format!("Property '{name}' does not exist in model.")
        } else {
            continue;
        };
        c.report(element.span(), "rules.modelAppends", message, None);
    }
}

/// `ReflectionHelper::hasPropertyTag()`: a `@property` tag of the class or an ancestor.
pub(super) fn property_tag(
    k: &mago_codex::metadata::class_like::ClassLikeMetadata,
    name: &str,
) -> bool {
    k.magic_property_ids
        .contains_key(&mago_word::word(format!("${name}").as_bytes()))
}
