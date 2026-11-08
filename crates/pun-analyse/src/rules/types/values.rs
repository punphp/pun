//! Level 3's values PHP needs to be of a kind: ThrowExprTypeRule (`throw.notThrowable`), VariableCloningRule
//! (`clone.nonObject`), IterableInForeachRule (`foreach.nonIterable`), UnpackIterableInArrayRule
//! (`arrayUnpacking.nonIterable`), ArrayUnpackingRule (`arrayUnpacking.stringOffset`) and
//! InvalidKeyInArrayDimFetchRule (`offsetAccess.invalidOffset`); level 4's DeadForeachRule (`foreach.emptyArray`).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, Type, is_super_type_of, union};

use super::super::{Checker, declared};

pub(super) const IDENTIFIERS: &[&str] = &[
    "throw.notThrowable",
    "clone.nonObject",
    "foreach.nonIterable",
    "arrayUnpacking.nonIterable",
    "arrayUnpacking.stringOffset",
    "offsetAccess.invalidOffset",
    "foreach.emptyArray",
];

/// The expressions whose type the rules read.
pub(super) fn typed(e: &Expression, level: u8, spans: &mut Vec<Span>) {
    if level < 3 {
        return;
    }
    match e {
        Expression::Throw(t) => spans.push(t.exception.span()),
        Expression::Clone(c) => spans.push(c.object.span()),
        Expression::ArrayAccess(a) => spans.extend([a.array.span(), a.index.span()]),
        Expression::ArrayAppend(a) => spans.push(a.array.span()),
        Expression::Assignment(a)
            if matches!(
                super::flow::unparen(a.lhs),
                Expression::Array(_) | Expression::List(_)
            ) =>
        {
            spans.push(a.rhs.span());
        }
        Expression::Array(Array { elements, .. })
        | Expression::LegacyArray(LegacyArray { elements, .. }) => {
            spans.extend(elements.iter().filter_map(|e| match e {
                ArrayElement::Variadic(v) => Some(v.value.span()),
                _ => None,
            }));
        }
        _ => {}
    }
}

/// `findTypeToCheck()` of `e` and whether the type found is `ok` (`None`: nothing to report), reporting the unknown
/// classes it names with `unknown` (`%s` the class) when there is one.
fn invalid(
    c: &mut Checker,
    e: &Expression,
    ok: &dyn Fn(&Type) -> bool,
    unknown: Option<&str>,
) -> Option<String> {
    let (found, names) = declared::with_classes(c, || {
        let found = c.find_type_to_check(e, &ok);
        let described = found
            .ty
            .filter(|t| !ok(t))
            .map(|t| t.describe(Level::TypeOnly));
        (described, found.unknown)
    });
    for (pattern, name) in unknown
        .iter()
        .flat_map(|p| names.iter().map(move |n| (p, n)))
    {
        c.report(
            e,
            "class.notFound",
            pattern.replace("%s", name),
            Some(super::super::DISCOVERING_SYMBOLS_TIP),
        );
    }
    found
}

fn throwable(t: &Type) -> bool {
    is_super_type_of(&Type::Object("Throwable".into(), vec![], false), t).yes()
}

pub(in super::super) fn throw(c: &mut Checker, t: &Throw) {
    if c.level < 3 {
        return;
    }
    if let Some(described) = invalid(
        c,
        t.exception,
        &throwable,
        Some("Throwing object of an unknown class %s."),
    ) {
        c.report(
            t,
            "throw.notThrowable",
            format!("Invalid type {described} to throw."),
            None,
        );
    }
}

pub(in super::super) fn clone(c: &mut Checker, k: &Clone) {
    if c.level < 3 {
        return;
    }
    let Some(described) = invalid(
        c,
        k.object,
        &|t| t.is_cloneable().yes(),
        Some("Cloning object of an unknown class %s."),
    ) else {
        return;
    };
    // php-parser drops the parentheses of `clone ($a)`
    let message = match super::flow::unparen(k.object) {
        Expression::Variable(Variable::Direct(v)) => format!(
            "Cannot clone non-object variable {} of type {described}.",
            String::from_utf8_lossy(v.name)
        ),
        _ => format!("Cannot clone {described}."),
    };
    c.report(k, "clone.nonObject", message, None);
}

pub(in super::super) fn foreach(c: &mut Checker, f: &Foreach) {
    if c.level < 3 {
        return;
    }
    if let Some(described) = invalid(
        c,
        f.expression,
        &|t| t.is_iterable().yes(),
        Some("Iterating over an object of an unknown class %s."),
    ) {
        let message = format!(
            "Argument of an invalid type {described} supplied for foreach, only iterables are supported."
        );
        c.report(f.expression, "foreach.nonIterable", message, None);
    }
    if c.level < 4 {
        return;
    }
    let Some(t) = c.phpstan_type(f.expression) else {
        return;
    };
    if !t.is_iterable().no() && t.is_iterable_at_least_once().no() {
        c.report(
            f,
            "foreach.emptyArray",
            "Empty array passed to foreach.".into(),
            None,
        );
    }
}

pub(in super::super) fn array(c: &mut Checker, elements: &[ArrayElement]) {
    if c.level < 3 {
        return;
    }
    for e in elements {
        let ArrayElement::Variadic(v) = e else {
            continue;
        };
        if let Some(described) = invalid(c, v.value, &|t| t.is_iterable().yes(), None) {
            let message = format!("Only iterables can be unpacked, {described} given.");
            c.report(v, "arrayUnpacking.nonIterable", message, None);
        }
        if c.surely_at_least(80100) {
            continue;
        }
        // ArrayUnpackingRule: before PHP 8.1, only int keys
        let Some(t) = c.phpstan_type(v.value) else {
            continue;
        };
        let levels = c.levels();
        let message = declared::with_classes(c, || {
            let key = c.find_in(
                levels,
                t.iterable_key_type(),
                &|k| k.is_string().no(),
                Some(&mut Vec::new()),
            )?;
            let string = key.is_string();
            (!string.no()).then(|| {
                format!(
                    "Array unpacking cannot be used on an array with {}string keys: {}",
                    if string.yes() { "" } else { "potential " },
                    t.describe(Level::Value)
                )
            })
        });
        if let Some(message) = message {
            c.report(v, "arrayUnpacking.stringOffset", message, None);
        }
    }
}

/// `AllowedArrayKeysTypes::getType()` (without `reportNonIntStringArrayKey`).
pub(super) fn allowed_keys(php_version: u32) -> Type {
    let mut types = vec![Type::Int, Type::String, Type::Bool];
    if php_version < 80100 {
        types.push(Type::Float);
    }
    if php_version < 80500 {
        types.push(Type::Null);
    }
    union(types)
}

pub(in super::super) fn array_access(c: &mut Checker, a: &ArrayAccess) {
    if c.level < 3 {
        return;
    }
    let allowed = allowed_keys(c.php_version);
    let report_maybes = c.level >= 7;
    let message = declared::with_classes(c, || {
        let array = c.find_type_to_check(a.array, &|t| t.is_array().no()).ty?;
        let is_array = array.is_array();
        if is_array.no() || (is_array.maybe() && !report_maybes) {
            return None;
        }
        let key = c
            .find_type_to_check(a.index, &|t| is_super_type_of(&allowed, t).yes())
            .ty?;
        let fits = is_super_type_of(&allowed, &key);
        if fits.yes() || (fits.maybe() && !report_maybes) {
            return None;
        }
        let certain = is_array.yes() && fits.no();
        Some(format!(
            "{} array key type {}.",
            if certain {
                "Invalid"
            } else {
                "Possibly invalid"
            },
            key.describe(Level::TypeOnly)
        ))
    });
    if let Some(message) = message {
        c.report(a, "offsetAccess.invalidOffset", message, None);
    }
}
