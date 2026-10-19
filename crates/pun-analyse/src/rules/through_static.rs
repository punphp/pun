//! The level-2 `static::` rules on private members of a class that is not final: AccessPrivatePropertyThroughStaticRule,
//! CallPrivateMethodThroughStaticRule and AccessPrivateConstantThroughStaticRule (a subclass reaching the member
//! through `static::` would fail).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_word::word;

use super::Checker;

/// The class `static::` names, when the rule applies: level 2, inside a class that is not final (`isFinal()`: the
/// keyword or `@final`; an anonymous class is final).
fn static_class<'a>(c: &Checker<'a, '_>, class: &Expression) -> Option<&'a ClassLikeMetadata> {
    if c.level < 2 || !matches!(class, Expression::Static(_)) {
        return None;
    }
    let k = c.class_scope()?;
    let is_final =
        k.flags.is_final() || k.original_name.as_bytes().starts_with(b"{anonymous-class:");
    (!is_final).then_some(k)
}

/// The display name of a member's declaring class: a trait's member belongs to the class using it.
fn declaring_display(c: &Checker, k: &ClassLikeMetadata, declaring: &[u8]) -> String {
    match c.codebase.get_class_like(declaring) {
        Some(d) if d.kind != SymbolKind::Trait => c.display(d),
        _ => c.display(k),
    }
}

/// AccessPrivatePropertyThroughStaticRule on `static::$name`.
pub fn property(c: &mut Checker, p: &StaticPropertyAccess) {
    let Variable::Direct(v) = &p.property else {
        return;
    };
    let Some(k) = static_class(c, p.class) else {
        return;
    };
    let Some(declaring) = c
        .codebase
        .get_declaring_property_class(k.name.as_bytes(), v.name)
    else {
        return;
    };
    let Some(property) = c.codebase.get_declaring_property(k.name.as_bytes(), v.name) else {
        return;
    };
    let declaring_class = c.codebase.get_class_like(declaring.as_bytes());
    // `hasStaticProperty()`: a parent's private property is not the class's
    let own = declaring.as_bytes().eq_ignore_ascii_case(k.name.as_bytes())
        || declaring_class.is_some_and(|d| d.kind == SymbolKind::Trait);
    if !own || !property.flags.is_static() || !property.read_visibility.is_private() {
        return;
    }
    let display = declaring_display(c, k, declaring.as_bytes());
    let name = String::from_utf8_lossy(&v.name[1..]);
    c.report(
        p.property.span(),
        "staticClassAccess.privateProperty",
        format!("Unsafe access to private property {display}::${name} through static::."),
        None,
    );
}

/// CallPrivateMethodThroughStaticRule on `static::name()`.
pub fn method(c: &mut Checker, call: &StaticMethodCall) {
    let ClassLikeMemberSelector::Identifier(id) = &call.method else {
        return;
    };
    let Some(k) = static_class(c, call.class) else {
        return;
    };
    // `getMethod()` finds a parent's private method too
    let Some(m) = super::members::method(c, k, id.value) else {
        return;
    };
    if !m.visibility().is_private() {
        return;
    }
    let display = declaring_display(c, k, m.declaring_name().as_bytes());
    let name = m.name();
    c.report(
        id.span,
        "staticClassAccess.privateMethod",
        format!("Unsafe call to private method {display}::{name}() through static::."),
        None,
    );
}

/// AccessPrivateConstantThroughStaticRule on `static::NAME`.
pub fn constant(c: &mut Checker, a: &ClassConstantAccess) {
    let ClassLikeConstantSelector::Identifier(id) = &a.constant else {
        return;
    };
    let Some(k) = static_class(c, a.class) else {
        return;
    };
    // the class's own constants and its traits' (a parent's private constant is not the class's)
    let constant = k.constants.get(&word(id.value)).or_else(|| {
        let t = k.trait_constant_ids.get(&word(id.value))?;
        c.codebase
            .get_class_like(t.as_bytes())?
            .constants
            .get(&word(id.value))
    });
    if !constant.is_some_and(|x| x.visibility.is_private()) {
        return;
    }
    let display = c.display(k);
    let name = String::from_utf8_lossy(id.value);
    c.report(
        a.span(),
        "staticClassAccess.privateConstant",
        format!("Unsafe access to private constant {display}::{name} through static::."),
        None,
    );
}
