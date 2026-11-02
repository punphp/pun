//! ImpossibleInArrayHaystackFiniteTypesRule (level 4, `featureToggles.finiteTypesInHaystack`): a finite value in a
//! constant array haystack of in_array(), array_search() or array_keys() that can never be the needle.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::{ConstArray, Level, Tri, Type, is_super_type_of, recommended_level};

use super::super::{Checker, declared};
use super::conditions::{finite, loose, scalar};
use super::node_name;

pub(in super::super) fn call(c: &mut Checker, call: &FunctionCall) {
    if !c.toggled.finite_haystack {
        return;
    }
    let Expression::Identifier(id) = call.function else { return };
    let name = String::from_utf8_lossy(node_name(c.names, id)).to_ascii_lowercase();
    let (needle_arg, haystack_arg) = match name.as_str() {
        "in_array" | "array_search" => (0, 1),
        "array_keys" => (1, 0),
        _ => return,
    };
    let args = &call.argument_list.arguments;
    if args.len() <= needle_arg.max(haystack_arg) {
        return;
    }
    let ty = |e: &Expression| {
        if c.treat_php_doc_types_as_certain {
            c.phpstan_type(e)
        } else {
            c.phpstan_type_in(e, &c.native_types).or_else(|| constant_expression(e).then(|| c.phpstan_type(e)).flatten())
        }
    };
    let (Some(needle), Some(haystack)) = (ty(args[needle_arg].value()), ty(args[haystack_arg].value())) else {
        return;
    };
    let arrays = haystack.constant_arrays();
    if arrays.is_empty() {
        return;
    }
    let strict = if args.len() >= 3 {
        let Some(strict) = ty(args[2].value()) else { return };
        is_true(&strict)
    } else {
        Tri::No
    };
    let mut dead: Vec<(String, Type, Vec<String>, bool)> = Vec::new();
    let mut any_possible_match = false;
    declared::with_classes(c, || {
        for value in arrays.iter().flat_map(|a| a.values.iter()) {
            let Some(reasons) = never_matches(c, &needle, value, strict) else {
                any_possible_match = true;
                continue;
            };
            if finite_count(c, value) == 0 {
                continue;
            }
            // isAnotherValueOfNeedleType()
            let another = is_super_type_of(&value.generalize(), &needle).yes();
            let key = value.describe(Level::Precise);
            match dead.iter_mut().find(|d| d.0 == key) {
                Some(d) => *d = (key, value.clone(), reasons, another),
                None => dead.push((key, value.clone(), reasons, another)),
            }
        }
    });
    // A call no haystack value matches is ImpossibleCheckTypeFunctionCallRule's; array_search() has no such rule.
    if !any_possible_match && name == "in_array" {
        return;
    }
    let verb = if strict.no() { "equal to" } else { "identical to" };
    for (_, value, reasons, another) in dead {
        if any_possible_match && another {
            continue;
        }
        let (value, needle) = declared::with_classes(c, || {
            (value.describe(recommended_level(&value, None)), needle.describe(recommended_level(&needle, None)))
        });
        let tip = match reasons.as_slice() {
            [] => None,
            [reason] => Some(reason.clone()),
            reasons => Some(reasons.iter().map(|r| format!("• {r}")).collect::<Vec<_>>().join("\n")),
        };
        c.report(
            call.span(),
            "function.impossibleHaystackValue",
            format!("Value {value} in the haystack passed to {name}() can never be {verb} the needle type {needle}."),
            tip.as_deref(),
        );
    }
}

/// `count(getFiniteTypes())`, with ConstantArrayType's product of its values' finite types (at most 128).
fn finite_count(c: &Checker, t: &Type) -> usize {
    let Type::ConstArray(a) = t else { return finite(c, t).len() };
    if a.unsealed.is_some() {
        return 0;
    }
    let mut count = 1;
    for (i, value) in a.values.iter().enumerate() {
        let values = finite_count(c, value);
        if values == 0 {
            return 0;
        }
        count *= values + usize::from(a.optional.contains(&i));
        if count > 128 {
            return 0;
        }
    }
    count
}

/// An expression whose native type is its type: Mago's native lane does not observe literals and constants.
pub(super) fn constant_expression(e: &Expression) -> bool {
    match super::super::types::flow::unparen(e) {
        Expression::Literal(_) | Expression::ConstantAccess(_) => true,
        Expression::Access(Access::ClassConstant(_)) => true,
        Expression::Array(a) => a.elements.iter().all(|e| match e {
            ArrayElement::KeyValue(kv) => constant_expression(kv.key) && constant_expression(kv.value),
            ArrayElement::Value(v) => constant_expression(v.value),
            _ => false,
        }),
        Expression::LegacyArray(a) => a.elements.iter().all(|e| match e {
            ArrayElement::KeyValue(kv) => constant_expression(kv.key) && constant_expression(kv.value),
            ArrayElement::Value(v) => constant_expression(v.value),
            _ => false,
        }),
        _ => false,
    }
}

/// `isTrue()`.
fn is_true(t: &Type) -> Tri {
    match t {
        Type::ConstBool(b) => Tri::from(*b),
        Type::Union(ts) => ts.iter().map(is_true).reduce(|a, b| if a == b { a } else { Tri::Maybe }).unwrap_or(Tri::No),
        Type::Bool | Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::StrictMixed => Tri::Maybe,
        _ => Tri::No,
    }
}

/// findCanNeverMatchReasons(): the reasons a value never matches, None when it can.
fn never_matches(c: &Checker, needle: &Type, value: &Type, strict: Tri) -> Option<Vec<String>> {
    let decide = |(result, reasons): (Tri, Vec<String>)| result.no().then_some(reasons);
    if strict.yes() {
        return decide(identical(c, needle, value));
    }
    if strict.no() {
        return decide(equal(c, needle, value));
    }
    let mut reasons = decide(identical(c, needle, value))?;
    for reason in decide(equal(c, needle, value))? {
        if !reasons.contains(&reason) {
            reasons.push(reason);
        }
    }
    Some(reasons)
}

/// InitializerExprTypeResolver::resolveIdenticalType(): yes for true, no for false, maybe for bool, with the
/// isSuperTypeOf() reasons of a false result.
fn identical(c: &Checker, left: &Type, right: &Type) -> (Tri, Vec<String>) {
    if matches!(left, Type::Never(_)) || matches!(right, Type::Never(_)) {
        return (Tri::No, Vec::new());
    }
    if scalar(left) && scalar(right) {
        return (Tri::from(left == right), Vec::new());
    }
    if let ([l], [r]) = (finite(c, left).as_slice(), finite(c, right).as_slice()) {
        return (Tri::from(l == r), Vec::new());
    }
    if is_super_type_of(left, right).no() && is_super_type_of(right, left).no() {
        let mut reasons = super_reasons(left, right);
        reasons.extend(super_reasons(right, left));
        return (Tri::No, reasons);
    }
    if let (Type::ConstArray(l), Type::ConstArray(r)) = (left, right) {
        return (arrays(l, r, &|a, b| identical(c, a, b).0), Vec::new());
    }
    (Tri::Maybe, Vec::new())
}

/// resolveEqualType().
fn equal(c: &Checker, left: &Type, right: &Type) -> (Tri, Vec<String>) {
    if matches!(left, Type::Never(_)) || matches!(right, Type::Never(_)) {
        return (Tri::No, Vec::new());
    }
    if (c.is_enum(left) && is_true(right).no()) || (c.is_enum(right) && is_true(left).no()) {
        return identical(c, left, right);
    }
    if let (Type::ConstArray(l), Type::ConstArray(r)) = (left, right) {
        return (arrays(l, r, &|a, b| equal(c, a, b).0), Vec::new());
    }
    (loose(left, right), Vec::new())
}

/// MixedType::isSuperTypeOf()'s reason for a value its subtracted type already removed.
fn super_reasons(a: &Type, b: &Type) -> Vec<String> {
    match a {
        Type::SubtractedMixed { excluded, .. } if is_super_type_of(excluded, b).yes() => vec![format!(
            "Type {} has already been eliminated from {}.",
            excluded.describe(Level::Precise),
            a.describe(Level::TypeOnly)
        )],
        _ => Vec::new(),
    }
}

/// resolveConstantArrayTypeComparison().
fn arrays(left: &ConstArray, right: &ConstArray, compare: &dyn Fn(&Type, &Type) -> Tri) -> Tri {
    let mut result = Tri::Yes;
    let mut remaining: Vec<usize> = (0..right.keys.len()).collect();
    for (i, key) in left.keys.iter().enumerate() {
        let left_optional = left.optional.contains(&i);
        if left_optional {
            result = Tri::Maybe;
        }
        if remaining.is_empty() {
            if !left_optional {
                return Tri::No;
            }
            continue;
        }
        let mut found = None;
        while !remaining.is_empty() {
            let j = remaining.remove(0);
            if right.keys[j] == *key {
                found = Some(j);
                break;
            } else if !right.optional.contains(&j) {
                return Tri::No;
            }
        }
        let Some(j) = found else {
            if !left_optional {
                return Tri::No;
            }
            continue;
        };
        if right.optional.contains(&j) {
            result = Tri::Maybe;
            if left_optional {
                continue;
            }
        }
        let value = compare(&left.values[i], &right.values[j]);
        if value.no() {
            return Tri::No;
        }
        if value != result {
            result = Tri::Maybe;
        }
    }
    if remaining.iter().any(|j| !right.optional.contains(j)) {
        return Tri::No;
    }
    if !remaining.is_empty() {
        result = Tri::Maybe;
    }
    result
}
