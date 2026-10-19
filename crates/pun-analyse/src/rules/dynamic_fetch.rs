//! DynamicClassConstantFetchRule: `Foo::{$name}` before PHP 8.3, and a name that is not a string.

use mago_syntax::cst::*;

use super::Checker;

pub fn class_constant(c: &mut Checker, a: &ClassConstantAccess) {
    let ClassLikeConstantSelector::Expression(selector) = &a.constant else {
        return;
    };
    // PhpVersion::supportsDynamicClassConstantFetch()
    if c.php_version < 80300 {
        let message = "Fetching class constants with a dynamic name is supported only on PHP 8.3 and later.";
        c.report_non_ignorable(a, "classConstant.dynamicFetch", message.to_string(), None);
        return;
    }
    // RuleLevelHelper::findTypeToCheck() with checkThisOnly (levels 0 and 1) checks only `$this`, never a string
    // ponytail: from level 2 the name's type is needed
    let is_this = matches!(selector.expression, Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    if c.level < 2
        && is_this
        && c.this_defined()
        && let Some(class) = c.class_scope()
    {
        let message = format!(
            "Class constant name in dynamic fetch can only be a string, $this({}) given.",
            c.display(class)
        );
        c.report(a, "classConstant.nameType", message, None);
    }
}
