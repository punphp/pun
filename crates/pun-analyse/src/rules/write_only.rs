//! ReadingWriteOnlyPropertiesRule: reading a property that is not readable (a virtual property without a `get`
//! hook, an `@property-write` tag). Level 0 and 1 run it with `checkThisOnly`, so only `$this->name` is checked;
//! a static property is always readable.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_word::word;

use super::Checker;
use super::property_access::in_expression_assign;

/// A `$this->name` or `$this?->name` fetch.
pub(super) fn read(
    c: &mut Checker,
    span: Span,
    object: &Expression,
    selector: &ClassLikeMemberSelector,
) {
    let Expression::Variable(Variable::Direct(v)) = object else {
        return; // ponytail: checkThisOnly is off from level 2, which needs every expression's type
    };
    let ClassLikeMemberSelector::Identifier(name) = selector else {
        return;
    };
    if v.name != b"$this" || !c.this_defined() || in_expression_assign(c, span) {
        return;
    }
    let Some(class) = c.class_scope() else {
        return;
    };
    let property = [b"$", name.value].concat();
    // PhpClassReflectionExtension first: a native property is readable from its class unless it is virtual
    // without a `get` hook
    let (declaring, readable) = if let Some((declaring, readable)) = c.native_readable(class, &property) {
        (declaring, readable)
    } else if c.allows_dynamic_properties(class)
        && let Some(declaring) = class.magic_property_ids.get(&word(&property))
        && let Some(tag) = c.codebase.get_magic_property(declaring.as_bytes(), &property)
    {
        let declaring = c.codebase.get_class_like(declaring.as_bytes()).map_or_else(String::new, |d| c.display(d));
        (declaring, !tag.flags.is_writeonly())
    } else {
        return;
    };
    if !readable {
        let message = format!("Property {declaring}::${} is not readable.", String::from_utf8_lossy(name.value));
        c.report(span, "property.writeOnly", message, None);
    }
}

/// The node span a nullsafe fetch reports on.
pub(super) fn null_safe(c: &mut Checker, p: &NullSafePropertyAccess) {
    read(c, p.span(), p.object, &p.property);
}
