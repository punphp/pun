//! Level 3's InvalidKeyInArrayItemRule (`array.invalidKey`), ArrayDestructuringRule (`offsetAccess.nonArray`) and
//! NonexistentOffsetInArrayDimFetchRule (`offsetAccess.nonOffsetAccessible`, `class.notFound`), with the
//! NonexistentOffsetInArrayDimFetchCheck (`offsetAccess.notFound`) the last two share.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{ConstArray, Level, ShapeBuilder, Tri, Type, intersect, is_super_type_of, union};

use super::super::{Checker, declared, property_access, unset};
use super::flow::unparen;
use super::offsets::accessible;

pub const IDENTIFIERS: &[&str] = &[
    "array.invalidKey",
    "offsetAccess.nonArray",
    "offsetAccess.nonOffsetAccessible",
    "offsetAccess.notFound",
];

/// `$dimType->describe(...)` of the offset messages: a constant string precisely, anything else by value.
fn describe_dim(t: &Type) -> String {
    t.describe(if t.constant_strings().is_empty() { Level::Value } else { Level::Precise })
}

/// `isUndefinedExpressionAllowed()`.
fn undefined_allowed(c: &Checker, at: Span) -> bool {
    property_access::undefined_allowed(c, at) || c.variables.allowed.contains(&at)
}

/// `reportMaybes` (conf/config.level7.neon).
fn report_maybes(c: &Checker) -> bool {
    c.level >= 7
}

/// InvalidKeyInArrayItemRule on a keyed item of an array literal or a destructuring.
pub(in super::super) fn array_item(c: &mut Checker, item: &KeyValueArrayElement) {
    if c.level < 3 {
        return;
    }
    let allowed = super::values::allowed_keys(c.php_version);
    let message = declared::with_classes(c, || {
        let key = c.find_type_to_check(item.key, &|t| is_super_type_of(&allowed, t).yes()).ty?;
        let fits = is_super_type_of(&allowed, &key);
        (!fits.yes()).then(|| {
            format!(
                "{} array key type {}.",
                if fits.no() { "Invalid" } else { "Possibly invalid" },
                key.describe(Level::TypeOnly)
            )
        })
    });
    if let Some(message) = message {
        c.report(item, "array.invalidKey", message, None);
    }
}

/// `isOffsetAccessLegal()`: what is not offset-accessible is still legal to read an offset of (a warning, not an
/// error), but for closures and strict mixed.
fn legal(t: &Type) -> Tri {
    match t {
        Type::Union(ts) => Tri::extreme_identity(ts.iter().map(legal)),
        Type::Benevolent(ts) => ts.iter().map(legal).fold(Tri::No, Tri::or),
        Type::Intersection(ts) => Tri::max_min(ts.iter().map(legal)),
        Type::Template(tt) if !matches!(&tt.bound, Type::SubtractedMixed { .. }) => legal(&tt.bound),
        Type::Closure(_) | Type::StrictMixed | Type::NonexistentParent => Tri::No,
        Type::SubtractedMixed { excluded, .. } if is_super_type_of(excluded, &Type::ObjectWithoutClass).yes() => Tri::Yes,
        Type::Object(..) | Type::Static(..) | Type::This(_) | Type::EnumCase(..) | Type::SubtractedObject { .. } => {
            accessible(t)
        }
        _ if accessible(t).maybe() => Tri::Maybe,
        _ => Tri::Yes,
    }
}

/// The holder's type as NullsafeOperatorHelper::getNullsafeShortcircuitedExprRespectingScope() reads it: a holder
/// whose chain short-circuits on a `?->` is read without the `null` it adds.
fn shortcircuited(c: &Checker, holder: &Expression) -> Option<Type> {
    fn nullsafe(e: &Expression) -> bool {
        match unparen(e) {
            Expression::Access(Access::NullSafeProperty(_)) | Expression::Call(Call::NullSafeMethod(_)) => true,
            Expression::Access(Access::Property(p)) => nullsafe(p.object),
            Expression::Call(Call::Method(m)) => nullsafe(m.object),
            Expression::ArrayAccess(a) => nullsafe(a.array),
            _ => false,
        }
    }
    let t = c.phpstan_type(holder)?;
    Some(if nullsafe(holder) && !t.is_null().no() { t.remove_null() } else { t })
}

/// NonexistentOffsetInArrayDimFetchCheck::check() on a holder of type `holder` (`None`: unknown), whose expression is
/// in an assignment or where an undefined one is allowed when `exempt`.
fn check(c: &Checker, holder: Option<Type>, exempt: bool, dim: &Type) -> Option<String> {
    let has = |t: &Type| unset::has_offset(c, t, dim);
    let t = c.find_type_to_check_of(holder, &|t| has(t).yes())?;
    // PHPStan's arrays only have int and string keys: writing another key (`$a[[]] = ...`) makes the holder an
    // ErrorType, where Mago keeps an array of that key type
    if exempt || (t.is_array().yes() && !is_super_type_of(&union(vec![Type::Int, Type::String]), &t.iterable_key_type()).yes()) {
        return None;
    }
    let found = has(&t);
    if found.yes() {
        return None;
    }
    if found.no() {
        if t.is_array().yes() && matches!(intersect(vec![all_keys(), dim.clone()]), Type::Never(_)) {
            // Already reported by InvalidKeyInArrayDimFetchRule
            return None;
        }
        return Some(format!("Offset {} does not exist on {}.", describe_dim(dim), t.describe(Level::Value)));
    }
    if !report_maybes(c) {
        return None;
    }
    let flattened = match &t {
        Type::Benevolent(_) => vec![t.clone()],
        _ => flatten(&t),
    };
    let valid = if matches!(dim, Type::Mixed { .. } | Type::SubtractedMixed { .. }) {
        dim.clone()
    } else {
        intersect(vec![all_keys(), dim.clone()])
    };
    let report = flattened.iter().any(|inner| {
        let check = if inner.is_array().yes() { &valid } else { dim };
        // (reportPossiblyNonexistentGeneralArrayOffset and reportPossiblyNonexistentConstantArrayOffset are off)
        if inner.is_constant_array().yes() && !unset::has_offset(c, inner, check).yes() && constant_scalar(check) {
            return true;
        }
        let checks = match check {
            Type::Benevolent(_) => vec![check.clone()],
            _ => flatten(check),
        };
        checks.iter().any(|d| unset::has_offset(c, inner, d).no())
    });
    report.then(|| format!("Offset {} might not exist on {}.", describe_dim(dim), t.describe(Level::Value)))
}

/// `AllowedArrayKeysTypes::getType()` without a PHP version.
fn all_keys() -> Type {
    union(vec![Type::Int, Type::String, Type::Bool, Type::Float, Type::Null])
}

/// `TypeUtils::flattenTypes()`: a union's members, and a constant array as each of its arrays.
fn flatten(t: &Type) -> Vec<Type> {
    let members = match t {
        Type::Union(ts) => ts.clone(),
        t => vec![t.clone()],
    };
    members
        .into_iter()
        .flat_map(|t| match &t {
            Type::ConstArray(a) => all_arrays(a),
            _ => vec![t],
        })
        .collect()
}

/// `ConstantArrayType::getAllArrays()`: the array with each combination of its optional keys present.
fn all_arrays(a: &ConstArray) -> Vec<Type> {
    let optional = &a.optional;
    let n = optional.len();
    let combinations: Vec<Vec<usize>> = if n <= 10 {
        // `powerSet()`
        (0..1usize << n)
            .map(|i| (0..n).filter(|j| (i >> (n - 1 - j)) & 1 == 1).map(|j| optional[j]).collect())
            .collect()
    } else {
        vec![vec![], vec![optional[0]], vec![optional[n - 1]], optional.clone()]
    };
    let required: Vec<usize> = (0..a.keys.len()).filter(|i| !optional.contains(i)).collect();
    combinations
        .into_iter()
        .map(|combination| {
            let mut keys: Vec<usize> = required.iter().chain(&combination).copied().collect();
            keys.sort_unstable();
            let mut builder = ShapeBuilder::new();
            for i in keys {
                builder.set(Some(a.keys[i].clone()), a.values[i].clone(), false);
            }
            builder.get()
        })
        .collect()
}

/// `isConstantScalarValue()->yes()`.
fn constant_scalar(t: &Type) -> bool {
    match t {
        Type::Union(ts) => ts.iter().all(constant_scalar),
        _ => matches!(t, Type::ConstBool(_) | Type::ConstInt(_) | Type::ConstFloat(_) | Type::ConstString(_) | Type::Null),
    }
}

/// `isImplicitArrayCreation()`: the chain's root is a variable certainly undefined.
fn implicit_array_creation(c: &Checker, holder: &Expression) -> bool {
    let mut root = unparen(holder);
    while let Expression::ArrayAccess(a) = root {
        root = unparen(a.array);
    }
    match root {
        Expression::Variable(Variable::Direct(v)) => c.variables.reads.get(&v.span()).is_some_and(|r| r.1.no()),
        _ => false,
    }
}

/// The name of a call to a function written without a namespace (`Name::toLowerString()` after NameResolver).
fn called<'e, 'arena>(e: &'e Expression<'arena>) -> Option<(String, &'e ArgumentList<'arena>)> {
    let Expression::Call(Call::Function(call)) = unparen(e) else { return None };
    let Expression::Identifier(id) = call.function else { return None };
    let name = String::from_utf8_lossy(id.value()).to_ascii_lowercase();
    Some((name.trim_start_matches('\\').to_string(), &call.argument_list))
}

fn arguments<'e, 'arena>(list: &'e ArgumentList<'arena>) -> Vec<&'e Expression<'arena>> {
    list.arguments.iter().map(|a| a.value()).collect()
}

/// `$array[f($array)]` where `f` surely gives one of the array's keys.
fn known_key(c: &Checker, holder: &Expression, dim: &Expression) -> bool {
    let same = |arg: &Expression| match (unparen(arg), unparen(holder)) {
        (Expression::Variable(Variable::Direct(a)), Expression::Variable(Variable::Direct(h))) => a.name == h.name,
        _ => false,
    };
    let non_empty = |arg: &Expression, list: bool| {
        c.phpstan_type(arg).is_some_and(|t| {
            (if list { t.is_list() } else { t.is_array() }).yes() && t.is_iterable_at_least_once().yes()
        })
    };
    if let Some((name, list)) = called(dim) {
        let args = arguments(list);
        if matches!(name.as_str(), "array_key_first" | "array_key_last")
            && let Some(first) = args.first()
            && same(first)
            && declared::with_classes(c, || non_empty(first, false))
        {
            return true;
        }
        if name == "array_rand"
            && let Some(first) = args.first()
            && same(first)
            && declared::with_classes(c, || {
                non_empty(first, false)
                    && args.get(1).is_none_or(|num| {
                        c.phpstan_type(num).is_some_and(|t| is_super_type_of(&Type::ConstInt(1), &t).yes())
                    })
            })
        {
            return true;
        }
    }
    if let Expression::Binary(Binary { operator: BinaryOperator::Subtraction(_), lhs, rhs }) = unparen(dim)
        && let Expression::Literal(Literal::Integer(one)) = unparen(rhs)
        && one.value == Some(1)
        && let Some((name, list)) = called(lhs)
        && matches!(name.as_str(), "count" | "sizeof")
        && let Some(first) = arguments(list).first()
        && same(first)
        && declared::with_classes(c, || non_empty(first, true))
    {
        return true;
    }
    false
}

/// NonexistentOffsetInArrayDimFetchRule on `$holder[$dim]` (`$holder[]` without a dim).
pub(in super::super) fn dim_fetch(c: &mut Checker, node: Span, holder: &Expression, dim: Option<&Expression>) {
    if c.level < 3 || implicit_array_creation(c, holder) {
        return;
    }
    let dim_type = match dim {
        Some(d) => match c.phpstan_type(d) {
            Some(t) => Some(t),
            None => return,
        },
        None => None,
    };
    let described = dim_type.as_ref().map(|t| declared::with_classes(c, || t.describe(Level::Value)));
    let unknown = |class: &str| match &described {
        Some(d) => format!("Access to offset {d} on an unknown class {class}."),
        None => format!("Access to an offset on an unknown class {class}."),
    };
    let holder_type = shortcircuited(c, holder);
    let (found, names) = declared::with_classes(c, || {
        let mut names = Vec::new();
        let found = holder_type.clone().and_then(|t| {
            c.checks_expression(holder)
                .then(|| c.find_in(c.levels(), t, &|t| accessible(t).yes(), Some(&mut names)))
                .flatten()
        });
        (found, names)
    });
    for name in names {
        c.report(holder, "class.notFound", unknown(&name), Some(super::super::DISCOVERING_SYMBOLS_TIP));
    }
    let Some(found) = found else { return };
    // `$scope->hasExpressionType($node)->yes()`: a fetch the scope narrowed, as array_key_exists() with a key that
    // is not constant does
    if c.defined.get(&node) == Some(&Tri::Yes) {
        return;
    }
    let assigned = property_access::dim_assigned(c, node);
    let allowed = undefined_allowed(c, node);
    let message = declared::with_classes(c, || {
        let is_accessible = accessible(&found);
        if assigned && is_accessible.yes() {
            return None;
        }
        if allowed && legal(&found).yes() {
            return None;
        }
        if !is_accessible.yes() {
            if !is_accessible.no() && !report_maybes(c) {
                return None;
            }
            let message = match &dim_type {
                Some(d) => format!("Cannot access offset {} on {}.", describe_dim(d), found.describe(Level::Value)),
                None => format!("Cannot access an offset on {}.", found.describe(Level::TypeOnly)),
            };
            return Some(("offsetAccess.nonOffsetAccessible", message));
        }
        let (Some(dim), Some(dim_type)) = (dim, &dim_type) else { return None };
        if known_key(c, holder, dim) {
            return None;
        }
        let exempt = property_access::dim_assigned(c, holder.span()) || undefined_allowed(c, holder.span());
        check(c, holder_type, exempt, dim_type).map(|m| ("offsetAccess.notFound", m))
    });
    if let Some((identifier, message)) = message {
        c.report(node, identifier, message, None);
    }
}

/// The dim fetches whose `hasExpressionType()` [`dim_fetch`] reads.
pub(in super::super) fn watched(program: &Program) -> Vec<Span> {
    struct Fetches(Vec<Span>);
    impl<'ast, 'arena> mago_syntax::walker::MutWalker<'ast, 'arena, ()> for Fetches {
        fn walk_in_array_access(&mut self, a: &'ast ArrayAccess<'arena>, _: &mut ()) {
            self.0.push(a.span());
        }
    }
    let mut fetches = Fetches(Vec::new());
    mago_syntax::walker::MutWalker::walk_program(&mut fetches, program, &mut ());
    fetches.0
}

/// ArrayDestructuringRule on `[...] = $value` (`value` the assigned expression's type), reported at `at`.
pub(in super::super) fn destructuring(c: &mut Checker, at: Span, target: &Expression, value: Option<Type>, value_span: Option<Span>) {
    if c.level < 3 {
        return;
    }
    let mut errors = Vec::new();
    declared::with_classes(c, || destructured(c, target, value, value_span, &mut errors));
    for (identifier, message) in errors {
        c.report(at, identifier, message, None);
    }
}

/// The items of a destructuring target (`short`: a `[...]` in it is a nested destructuring).
fn list_items<'e, 'arena>(target: &'e Expression<'arena>) -> Option<(&'e TokenSeparatedSequence<'arena, ArrayElement<'arena>>, bool)> {
    match unparen(target) {
        Expression::Array(a) => Some((&a.elements, true)),
        Expression::List(l) => Some((&l.elements, false)),
        _ => None,
    }
}

fn destructured(
    c: &Checker,
    target: &Expression,
    value: Option<Type>,
    value_span: Option<Span>,
    errors: &mut Vec<(&'static str, String)>,
) {
    let Some((items, short)) = list_items(target) else { return };
    let array_access = Type::Object("ArrayAccess".into(), vec![], false);
    let destructurable = |t: &Type| t.is_array().yes() || is_super_type_of(&array_access, t).yes();
    let Some(found) = c.find_type_to_check_of(value.clone(), &destructurable) else { return };
    if !destructurable(&found) {
        errors.push((
            "offsetAccess.nonArray",
            format!("Cannot use array destructuring on {}.", found.describe(Level::TypeOnly)),
        ));
        return;
    }
    let exempt = value_span.is_some_and(|s| property_access::dim_assigned(c, s) || undefined_allowed(c, s));
    let mut i = 0;
    for item in items.iter() {
        let (key, item_value) = match item {
            ArrayElement::KeyValue(kv) => (c.phpstan_type(kv.key), kv.value),
            ArrayElement::Value(v) => (Some(Type::ConstInt(i)), v.value),
            ArrayElement::Missing(_) => {
                i += 1;
                continue;
            }
            ArrayElement::Variadic(_) => continue,
        };
        let Some(key) = key else {
            i += 1;
            continue;
        };
        errors.extend(check(c, value.clone(), exempt, &key).map(|m| ("offsetAccess.notFound", m)));
        let nested = match unparen(item_value) {
            Expression::List(_) => true,
            Expression::Array(_) => short,
            _ => false,
        };
        if !nested {
            i += 1;
            continue;
        }
        let offset = value.as_ref().map(|t| offset_value(t, &key));
        destructured(c, item_value, offset, None, errors);
    }
}

/// `getOffsetValueType()`: a union's is its members' that have one.
fn offset_value(t: &Type, key: &Type) -> Type {
    match t {
        Type::Union(ts) => {
            let values: Vec<_> = ts.iter().map(|t| offset_value(t, key)).filter(|v| !matches!(v, Type::Error)).collect();
            if values.is_empty() { Type::Error } else { union(values) }
        }
        _ => unset::late_offset(t, key).unwrap_or(Type::Error),
    }
}
