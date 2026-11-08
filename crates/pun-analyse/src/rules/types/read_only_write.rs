//! WritingToReadOnlyPropertiesRule (`assign.propertyReadOnly`): writing a property that is not writable (a virtual
//! property without a `set` hook, an `@property-read` tag). Level 0 and 1 run it with `checkThisOnly`, so only
//! `$this->name` is checked.

use mago_syntax::cst::*;
use mago_word::word;

use super::super::Checker;
use super::super::overriding_property::hook_names;
use super::flow::unparen;

/// The property fetches an assignment writes (`$this->a = `, `$this->a .= `, `[$this->a] = `).
pub(in super::super) fn assignment(c: &mut Checker, a: &Assignment) {
    targets(c, a.lhs);
}

fn targets(c: &mut Checker, e: &Expression) {
    match unparen(e) {
        Expression::Access(Access::Property(p)) => property(c, p),
        Expression::List(l) => {
            for el in &l.elements {
                element(c, el);
            }
        }
        Expression::Array(l) => {
            for el in &l.elements {
                element(c, el);
            }
        }
        _ => {}
    }
}

fn element(c: &mut Checker, el: &ArrayElement) {
    match el {
        ArrayElement::KeyValue(kv) => targets(c, kv.value),
        ArrayElement::Value(v) => targets(c, v.value),
        _ => {}
    }
}

fn property(c: &mut Checker, p: &PropertyAccess) {
    let Expression::Variable(Variable::Direct(v)) = p.object else {
        return; // ponytail: checkThisOnly is off from level 2, which needs every expression's type
    };
    let ClassLikeMemberSelector::Identifier(name) = &p.property else {
        return;
    };
    if v.name != b"$this" || !c.this_defined() {
        return;
    }
    let Some(class) = c.class_scope() else {
        return;
    };
    let property = [b"$", name.value].concat();
    let (declaring, writable) = if let Some((declaring, _, meta)) = c.declared_property(class, &property)
        && !(meta.read_visibility == mago_codex::visibility::Visibility::Private && declaring.name != class.name)
    {
        let virtual_ = c.property_virtual(declaring, meta, &property);
        let hooks = c.property_hooks(declaring, hook_names(meta), &property);
        (c.display(declaring), meta.flags.is_static() || !virtual_ || hooks.1)
    } else if c.allows_dynamic_properties(class)
        && let Some(declaring) = class.magic_property_ids.get(&word(&property))
        && let Some(tag) = c.codebase.get_magic_property(declaring.as_bytes(), &property)
    {
        let declaring = c.codebase.get_class_like(declaring.as_bytes()).map_or_else(String::new, |d| c.display(d));
        (declaring, !tag.flags.is_readonly())
    } else {
        return;
    };
    if !writable {
        let message = format!("Property {declaring}::${} is not writable.", String::from_utf8_lossy(name.value));
        c.report(name, "assign.propertyReadOnly", message, None);
    }
}
