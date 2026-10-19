//! SetPropertyHookParameterRule (InPropertyHookNode of a `set` hook with a parameter list; without one the parameter
//! takes the property's type). checkPhpDocMethodSignatures (propertySetHook.parameterType and the missing-typehint
//! checks) is off below level 3.

use mago_syntax::cst::*;
use pun_phpstan::types::{Level, NeverKind, Type, is_super_type_of};

use super::Checker;
use super::declared;
use super::signatures;

/// The hooks `list` of property `name` (with its `$`) declared with the native type `hint`.
pub fn hooks<'arena>(
    c: &mut Checker<'_, 'arena>,
    name: &[u8],
    hint: Option<&Hint<'arena>>,
    list: &PropertyHookList<'arena>,
) {
    let k = c.class_scope().expect("a property is walked inside its class-like");
    let class = c.display(k);
    let property = String::from_utf8_lossy(&name[1..]).into_owned();
    for hook in list.hooks.iter().filter(|h| h.name.value == b"set") {
        let Some(parameter) = hook.parameter_list.as_ref().and_then(|l| l.parameters.first()) else {
            continue;
        };
        let parameter_name = String::from_utf8_lossy(&parameter.variable.name[1..]).into_owned();
        let message = match (hint, &parameter.hint) {
            (None, None) => continue,
            (None, Some(_)) => format!(
                "Parameter ${parameter_name} of set hook has a native type but the property {class}::${property} does not."
            ),
            (Some(_), None) => format!(
                "Parameter ${parameter_name} of set hook does not have a native type but the property {class}::${property} does."
            ),
            (Some(theirs), Some(ours)) => {
                // an intersection TypeCombinator::intersect() turns into an implicit never
                let resolve = |c: &Checker, hint: &Hint| {
                    if signatures::unresolvable(c, hint) {
                        Type::Never(NeverKind::Implicit)
                    } else {
                        declared::native(c, hint, Some(k))
                    }
                };
                let theirs = resolve(c, theirs);
                let ours = resolve(c, ours);
                if is_super_type_of(&ours, &theirs).yes() {
                    continue;
                }
                format!(
                    "Native type {} of set hook parameter ${parameter_name} is not contravariant with native type {} of property {class}::${property}.",
                    ours.describe(Level::TypeOnly),
                    theirs.describe(Level::TypeOnly)
                )
            }
        };
        c.report_non_ignorable(hook, "propertySetHook.nativeParameterType", message, None);
    }
}
