//! Level 3's DefaultValueTypesAssignedToPropertiesRule (`property.defaultValue`): a property's default value its type
//! does not accept.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::recommended_level;

use super::super::{Checker, declared};

pub(super) const IDENTIFIERS: &[&str] = &["property.defaultValue"];

fn defaults<'p, 'arena>(p: &'p Property<'arena>) -> Vec<&'p PropertyConcreteItem<'arena>> {
    let items = match p {
        Property::Plain(p) => p.items.iter().collect(),
        Property::Hooked(h) => vec![&h.item],
    };
    items
        .into_iter()
        .filter_map(|i| match i {
            PropertyItem::Concrete(c) => Some(c),
            PropertyItem::Abstract(_) => None,
        })
        .collect()
}

/// The default values, whose types the rule reads.
pub(super) fn typed(p: &Property, spans: &mut Vec<Span>) {
    spans.extend(defaults(p).iter().map(|i| i.value.span()));
}

pub(in super::super) fn property(c: &mut Checker, p: &Property) {
    if c.level < 3 {
        return;
    }
    let Some(context) = c.class_scope() else { return };
    let display = c.display(context);
    let kind = if p.modifiers().iter().any(|m| m.is_static()) { "Static property" } else { "Property" };
    for item in defaults(p) {
        // without a native type, `= null` is the implicit default
        if p.hint().is_none() && matches!(item.value, Expression::Literal(Literal::Null(_))) {
            continue;
        }
        let Some(default) = c.phpstan_type(item.value) else { continue };
        let name = &item.variable.name[1..];
        let levels = c.levels();
        let message = declared::with_classes(c, || {
            let ty = declared::property(c, p.hint(), p.span(), name, context);
            if levels.accepts(&ty, &default, true) {
                return None;
            }
            let level = recommended_level(&ty, Some(&default));
            Some(format!(
                "{kind} {display}::${} ({}) does not accept default value of type {}.",
                String::from_utf8_lossy(name),
                ty.describe(level),
                default.describe(level)
            ))
        });
        if let Some(message) = message {
            c.report(item, "property.defaultValue", message, None);
        }
    }
}
