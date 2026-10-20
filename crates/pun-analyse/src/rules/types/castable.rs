//! Level 5's function rules on argument values: ParameterCastableToStringRule, ImplodeParameterCastableToStringRule
//! and SortParameterCastableToStringRule over ParameterCastableToStringCheck, RandomIntParametersRule (all
//! `argument.type`), ArrayFilterRule (`arrayFilter.*`) and ArrayValuesRule (`arrayValues.*`).

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::{Cast, Level, Type, cast_is_error, is_super_type_of, union};

pub const IDENTIFIERS: &[&str] = &[
    "arrayFilter.empty",
    "arrayFilter.same",
    "arrayFilter.alwaysEmpty",
    "arrayValues.empty",
    "arrayValues.list",
];

use super::super::params::{self, Acceptor};
use super::super::{Checker, declared};

/// The arguments as written: (name, value, unpacked).
fn written<'a, 'arena>(call: &'a FunctionCall<'arena>) -> Vec<(Option<&'a [u8]>, &'a Expression<'arena>, bool)> {
    call.argument_list
        .arguments
        .iter()
        .map(|a| match a {
            Argument::Named(n) => (Some(n.name.value), n.value, false),
            Argument::Positional(p) => (None, p.value, p.ellipsis.is_some()),
        })
        .collect()
}

/// `ArgumentsNormalizer::reorderFuncArguments()`'s argument for parameter `k`: (value, passed by name, unpacked).
// ponytail: an unpacked argument ends the positional ones
fn normalized<'a, 'arena>(
    args: &[(Option<&'a [u8]>, &'a Expression<'arena>, bool)],
    acceptor: &Acceptor,
    k: usize,
) -> Option<(&'a Expression<'arena>, bool, bool)> {
    let name = acceptor.names.get(k)?;
    if let Some((_, v, _)) = args.iter().find(|(n, ..)| n.is_some_and(|n| n == name.as_bytes())) {
        return Some((v, true, false));
    }
    let positional: Vec<_> = args.iter().take_while(|(n, ..)| n.is_none()).collect();
    positional.get(k).map(|(_, v, unpack)| (*v, false, *unpack))
}

/// `getParameterName()`.
fn parameter_name(acceptor: &Acceptor, idx: usize, named: bool) -> String {
    match acceptor.names.get(idx) {
        None => format!("#{}", idx + 1),
        Some(name) if named => format!("${name}"),
        Some(name) => format!("#{} ${name}", idx + 1),
    }
}

/// ParameterCastableToStringCheck::checkParameter(): the array argument's type when a value of it fails `cast`.
fn check_parameter(c: &Checker, value: &Expression, unpack: bool, cast: &dyn Fn(&Type) -> bool) -> Option<String> {
    if unpack {
        return None;
    }
    declared::with_classes(c, || {
        let array = c.find_type_to_check(value, &|t| t.is_array().yes()).ty?;
        if !array.is_array().yes() {
            return None;
        }
        let item = c.find_type_to_check_of(Some(array.iterable_value_type()), &|t| !cast(t))?;
        cast(&item).then(|| array.describe(Level::TypeOnly))
    })
}

fn function(c: &Checker, call: &FunctionCall) -> Option<(String, Acceptor)> {
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    let (name, variants) = std::iter::once(c.names.get(&id.span())).chain(fallback).find_map(|n| c.function(n))?;
    let counts = params::count(&call.argument_list);
    Some((name, params::select(c, variants, &counts)))
}

pub(in super::super) fn function_call(c: &mut Checker, call: &FunctionCall) {
    if c.level < 5 {
        return;
    }
    let Some((name, acceptor)) = function(c, call) else {
        return;
    };
    let args = written(call);
    let sm = c.levels().strict_mixed();
    let to_string = |t: &Type| cast_is_error(t, Cast::String, sm);
    let mut errors = Vec::new();
    match name.as_str() {
        "array_intersect" | "array_intersect_assoc" | "array_diff" | "array_diff_assoc" => {
            for (i, (n, v, unpack)) in args.iter().enumerate() {
                if let Some(given) = check_parameter(c, v, *unpack, &to_string) {
                    let p = parameter_name(&acceptor, i, n.is_some());
                    errors.push(format!(
                        "Parameter {p} of function {name} expects an array of values castable to string, {given} given."
                    ));
                }
            }
        }
        "array_combine" | "natcasesort" | "natsort" | "array_count_values" | "array_fill_keys" => {
            if let Some((v, named, unpack)) = normalized(&args, &acceptor, 0)
                && let Some(given) = check_parameter(c, v, unpack, &to_string)
            {
                let p = parameter_name(&acceptor, 0, named);
                errors.push(format!(
                    "Parameter {p} of function {name} expects an array of values castable to string, {given} given."
                ));
            }
        }
        "implode" | "join" => {
            let named = |n: &str| args.iter().any(|(a, _, unpack)| !unpack && a.is_some_and(|a| a == n.as_bytes()));
            let normalized: Vec<_> = (0..2).map_while(|k| normalized(&args, &acceptor, k)).collect();
            let idx = match normalized.len() {
                1 => 0,
                2 => 1,
                _ => return,
            };
            let (v, _, unpack) = normalized[idx];
            let p = if named("array") {
                "$array".to_string()
            } else if named("separator") && args.len() == 1 {
                "$separator".to_string()
            } else {
                format!("#{} $array", idx + 1)
            };
            if let Some(given) = check_parameter(c, v, unpack, &to_string) {
                errors.push(format!("Parameter {p} of function {name} expects array<string>, {given} given."));
            }
        }
        "array_unique" | "sort" | "rsort" | "asort" | "arsort" => {
            let Some((v, named, unpack)) = normalized(&args, &acceptor, 0) else {
                return;
            };
            let flags = match normalized(&args, &acceptor, 1) {
                Some((f, ..)) => c.phpstan_type(f),
                None => acceptor.param_types.get(1).map(|_| Type::ConstInt(default_of(&name))),
            };
            const SORT_REGULAR: i64 = 0;
            let Some(flags) = flags.filter(|f| *f != Type::ConstInt(SORT_REGULAR)) else {
                return;
            };
            let constants: Vec<i64> = match &flags {
                Type::ConstInt(v) => vec![*v],
                Type::Union(ts) => ts.iter().filter_map(|t| if let Type::ConstInt(v) = t { Some(*v) } else { None }).collect(),
                _ => Vec::new(),
            };
            let (mut string, mut float) = (constants.is_empty(), constants.is_empty());
            for f in constants {
                // SORT_NUMERIC; SORT_STRING, SORT_LOCALE_STRING, SORT_NATURAL (with or without SORT_FLAG_CASE)
                if f == 1 {
                    float = true;
                } else if [2, 5, 6].contains(&(f & !8)) {
                    string = true;
                }
            }
            let (what, cast): (&str, Box<dyn Fn(&Type) -> bool>) = match (string, float) {
                (true, false) => ("string", Box::new(to_string)),
                (true, true) => (
                    "string and float",
                    Box::new(move |t: &Type| cast_is_error(t, Cast::Float, sm) || cast_is_error(t, Cast::String, sm)),
                ),
                (false, true) => ("float", Box::new(move |t: &Type| cast_is_error(t, Cast::Float, sm))),
                (false, false) => return,
            };
            if let Some(given) = check_parameter(c, v, unpack, &*cast) {
                let p = parameter_name(&acceptor, 0, named);
                errors.push(format!(
                    "Parameter {p} of function {name} expects an array of values castable to {what}, {given} given."
                ));
            }
        }
        "random_int" => {
            if let [(_, min, _), (_, max, _), ..] = args.as_slice()
                && let Some(message) = random_int(c, min, max)
            {
                errors.push(message);
            }
        }
        "array_filter" | "array_values" => {
            let Some((v, ..)) = normalized(&args, &acceptor, 0) else {
                return;
            };
            let count = (0..acceptor.names.len()).map_while(|k| normalized(&args, &acceptor, k)).count();
            let filter = name == "array_filter";
            if filter && count != 1 {
                return;
            }
            if let Some((identifier, message)) = array_function(c, v, filter) {
                c.report(call.span(), identifier, message, None);
            }
        }
        _ => {}
    }
    for message in errors {
        c.report(call.span(), "argument.type", message, None);
    }
}

/// ArrayFilterRule (without a callback) and ArrayValuesRule, with treatPhpDocTypesAsCertain.
// ponytail: the treatPhpDocTypesAsCertain tip needs the native type, which Mago does not tell apart
fn array_function(c: &Checker, value: &Expression, filter: bool) -> Option<(&'static str, String)> {
    let t = c.phpstan_type(value)?;
    declared::with_classes(c, || {
        let described = t.describe(Level::Value);
        let function = if filter { "array_filter" } else { "array_values" };
        if t.is_iterable_at_least_once().no() {
            let identifier = if filter { "arrayFilter.empty" } else { "arrayValues.empty" };
            let message = format!("Parameter #1 $array ({described}) to function {function} is empty, call has no effect.");
            return Some((identifier, message));
        }
        if !filter {
            return t.is_list().yes().then(|| {
                let message = format!("Parameter #1 $array ({described}) of array_values is already a list, call has no effect.");
                ("arrayValues.list", message)
            });
        }
        // StaticTypeFactory::falsey()
        let falsey = union(vec![
            Type::Null,
            Type::ConstBool(false),
            Type::ConstInt(0),
            Type::ConstFloat(0.0),
            Type::ConstString(String::new()),
            Type::ConstString("0".to_string()),
            Type::ConstArray(Box::new(pun_phpstan::types::ConstArray::empty())),
        ]);
        let sup = is_super_type_of(&falsey, &t.iterable_value_type());
        if sup.no() {
            let message = format!(
                "Parameter #1 $array ({described}) to function array_filter does not contain falsy values, the array will always stay the same."
            );
            return Some(("arrayFilter.same", message));
        }
        sup.yes().then(|| {
            let message = format!(
                "Parameter #1 $array ({described}) to function array_filter contains falsy values only, the result will always be an empty array."
            );
            ("arrayFilter.alwaysEmpty", message)
        })
    })
}

/// The `$flags` default of the sort functions (`getDefaultValue()`): SORT_STRING for `array_unique()`.
fn default_of(function: &str) -> i64 {
    if function == "array_unique" { 2 } else { 0 }
}

/// RandomIntParametersRule: `$max` certainly (or, with reportMaybes, possibly) below `$min`.
// ponytail: only integer constants and ranges, not `toInteger()` of other constants
fn random_int(c: &Checker, min: &Expression, max: &Expression) -> Option<String> {
    let bounds = |t: &Type| match t {
        Type::ConstInt(v) => Some((Some(*v), Some(*v))),
        Type::IntRange(a, b) => Some((*a, *b)),
        _ => None,
    };
    let (min_t, max_t) = (c.phpstan_type(min)?, c.phpstan_type(max)?);
    let ((min_lo, min_hi), (max_lo, max_hi)) = (bounds(&min_t)?, bounds(&max_t)?);
    // `$maxType->isSmallerThan($minType)`
    let yes = matches!((max_hi, min_lo), (Some(a), Some(b)) if a < b);
    let no = matches!((max_lo, min_hi), (Some(a), Some(b)) if a >= b);
    let report_maybes = c.level >= 7;
    if !yes && !no && report_maybes && [min_lo, min_hi, max_lo, max_hi].iter().any(Option::is_none) {
        return None;
    }
    (yes || !no && report_maybes).then(|| {
        format!(
            "Parameter #1 $min ({}) of function random_int expects lower number than parameter #2 $max ({}).",
            min_t.describe(Level::Value),
            max_t.describe(Level::Value)
        )
    })
}
