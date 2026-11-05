//! NewStaticInAbstractClassStaticMethodRule (`featureToggles.newStaticInAbstractClassStaticMethod`, on with
//! bleedingEdge).

use mago_syntax::cst::*;

use super::declared::original_name;
use super::{Checker, FunctionScopeKind};

/// `new static` in a static method of an abstract class.
pub fn instantiation(c: &mut Checker, new: &Instantiation) {
    if !c.on(crate::rules::Switches::NEW_STATIC_IN_ABSTRACT_CLASS_STATIC_METHOD) || !matches!(new.class, Expression::Static(_)) {
        return;
    }
    let Some(k) = c.class_scope() else {
        return;
    };
    let Some(method) = c
        .functions
        .last()
        .filter(|f| f.kind == FunctionScopeKind::Method { is_static: true })
        .map(|f| f.name.clone())
    else {
        return;
    };
    if !k.flags.is_abstract() {
        return;
    }
    let message = format!(
        "Unsafe usage of new static() in abstract class {} in static method {method}().",
        c.display(k)
    );
    let tip = format!(
        "Direct call to {}::{method}() would crash because an abstract class cannot be instantiated.",
        original_name(k)
    );
    c.report(new, "new.staticInAbstractClassStaticMethod", message, Some(&tip));
}
