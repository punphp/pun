//! DynamicClassConstantFetchRule: `Foo::{$name}` before PHP 8.3, and a name that is not a string.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::Level;

use super::Checker;

pub fn class_constant(c: &mut Checker, a: &ClassConstantAccess) {
    let ClassLikeConstantSelector::Expression(selector) = &a.constant else {
        return;
    };
    // PhpVersion::supportsDynamicClassConstantFetch()
    if !c.surely_at_least(80300) {
        let message = "Fetching class constants with a dynamic name is supported only on PHP 8.3 and later.";
        c.report_non_ignorable(a, "classConstant.dynamicFetch", message.to_string(), None);
        return;
    }
    let Some((_, found)) = super::types::dynamic_names::not_string(c, selector.expression.span(), false) else {
        return;
    };
    let given = super::declared::with_classes(c, || found.describe(Level::TypeOnly));
    let message = format!("Class constant name in dynamic fetch can only be a string, {given} given.");
    c.report(a, "classConstant.nameType", message, None);
}
