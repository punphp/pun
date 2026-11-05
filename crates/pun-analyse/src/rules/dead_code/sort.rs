//! SortWithoutEffectRule (level 5, `featureToggles.sortWithoutEffect`): a sort call that cannot change its array.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, Type};

use super::super::{Checker, declared};
use super::noop::{self, function_of};

/// Sort functions that keep the original keys: they cannot change an array with at most one element.
const KEY_PRESERVING: &[&str] = &["arsort", "asort", "krsort", "ksort", "natcasesort", "natsort", "uasort", "uksort"];

/// Sort functions that reindex the array: a no-op on lists only.
const REINDEXING: &[&str] = &["rsort", "shuffle", "sort", "usort"];

const TIP: &str = "Because the type is coming from a PHPDoc, you can turn off this check by setting treatPhpDocTypesAsCertain: false in your %configurationFile%.";

pub(in super::super) fn call(c: &mut Checker, call: &FunctionCall) {
    if c.level < 5 || !c.on(crate::rules::Switches::SORT_WITHOUT_EFFECT) {
        return;
    }
    let Some(noop::Function::Builtin(f)) = function_of(c, call.function) else { return };
    let name = f.name.to_ascii_lowercase();
    let key_preserving = KEY_PRESERVING.contains(&name.as_str());
    if !key_preserving && !REINDEXING.contains(&name.as_str()) { return }
    // ArgumentsNormalizer::reorderFuncArguments(): `$array` and `$flags` by position or by name
    let argument = |position: usize, parameter: &str| {
        let arguments = &call.argument_list.arguments;
        arguments.iter().find_map(|a| match a {
            Argument::Named(n) if n.name.value == parameter.as_bytes() => Some(n.value),
            _ => None,
        })
        .or_else(|| arguments.get(position).filter(|a| matches!(a, Argument::Positional(_))).map(|a| a.value()))
    };
    let Some(array) = argument(0, "array") else { return };
    let Some(t) = c.phpstan_type(array) else { return };
    // hasFlagsKeepingListOrder(): SORT_REGULAR (the default) or SORT_NUMERIC
    let keys_sorted = name == "ksort"
        && argument(1, "flags").is_none_or(|flags| match c.phpstan_type(flags) {
            Some(Type::ConstInt(0 | 1)) => true,
            Some(Type::Union(ts)) => ts.iter().all(|t| matches!(t, Type::ConstInt(0 | 1))),
            _ => false,
        });
    let Some(reason) = reason(&t, key_preserving, keys_sorted) else { return };
    let described = declared::with_classes(c, || t.describe(Level::Value));
    let message = match reason {
        "empty" => format!("Parameter #1 $array ({described}) of function {name} is empty, call has no effect."),
        "list" => format!("Parameter #1 $array ({described}) of function {name} is a list, call has no effect."),
        _ => format!("Parameter #1 $array ({described}) of function {name} has at most 1 element, call has no effect."),
    };
    let tip = (native(c, array).and_then(|n| self::reason(&n, key_preserving, keys_sorted)) != Some(reason)).then_some(TIP);
    c.report(call.span(), &format!("sortArray.{reason}"), message, tip);
}

/// findNoEffectReason().
fn reason(t: &Type, key_preserving: bool, keys_sorted: bool) -> Option<&'static str> {
    if !t.is_array().yes() {
        return None;
    }
    if t.is_iterable_at_least_once().no() {
        return Some("empty");
    }
    let list = t.is_list().yes();
    if keys_sorted && list {
        return Some("list");
    }
    if !key_preserving && !list {
        return None;
    }
    at_most_one(t).then_some("noop")
}

/// `int<0, 1>` is a supertype of the array's size.
fn at_most_one(t: &Type) -> bool {
    match t {
        Type::Union(ts) => ts.iter().all(at_most_one),
        Type::ConstArray(a) => a.keys.len() <= 1 && a.unsealed.is_none(),
        _ => false,
    }
}

/// The argument's native type: a parameter's declared type, a local variable's type, or unknown.
// ponytail: a parameter is its native declaration even after the body assigns it, and a property or offset has no
// native type known (its PHPDoc type always gets the tip)
pub(crate) fn native(c: &Checker, e: &Expression) -> Option<Type> {
    let Expression::Variable(Variable::Direct(v)) = e else {
        return None;
    };
    let span = c.var_tag_functions.last();
    let parameter = span.and_then(|span| {
        let f = c.codebase.function_likes.values().find(|f| f.span == *span)?;
        f.parameters.iter().find(|p| p.name.0.as_bytes() == v.name)
    });
    match parameter {
        Some(p) => Some(p.type_declaration_metadata.as_ref().map_or_else(pun_phpstan::types::mixed, |t| {
            declared::from_tunion_in(c, &t.type_union, c.class_scope())
        })),
        None => c.phpstan_type(e),
    }
}
