//! NoUnnecessaryEnumerableToArrayCallsRule: `->toArray()` on an Enumerable whose values are never Arrayable; and
//! NoUnnecessaryCollectionCallRule: a collection method called on the result of a query that the query could do.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::get_specialized_template_type;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_word::word;

use super::super::Checker;
use super::{is_a, is_object_of};

const ENUMERABLE: &[u8] = b"illuminate\\support\\enumerable";
const ARRAYABLE: &[u8] = b"illuminate\\contracts\\support\\arrayable";

/// The `->toArray()` calls whose object the rule reads.
pub(super) fn to_array<'a, 'arena>(
    call: &'a MethodCall<'arena>,
) -> Option<&'a ClassLikeMemberSelector<'arena>> {
    matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value == b"toArray")
        .then_some(&call.method)
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some(name) = to_array(call) else {
        return;
    };
    let Some(ty) = c.type_of(call.object) else {
        return;
    };
    if !is_object_of(c, ty, ENUMERABLE) || !value_type(c, ty).is_some_and(|v| not_arrayable(c, &v))
    {
        return;
    }
    c.report(
        name,
        "larastan.unnecessaryEnumerableToArrayCall",
        "Called [toArray()] on an Enumerable which does not contain any Arrayables.".to_string(),
        Some("Use [all()] to get the items as an array."),
    );
}

/// `getTemplateType(Enumerable::class, 'TValue')`: each part's TValue.
fn value_type(c: &Checker, ty: &TUnion) -> Option<TUnion> {
    template(c, ty, ENUMERABLE)
}

/// Each part's `TValue` of the ancestor `class`.
fn template(c: &Checker, ty: &TUnion, class: &[u8]) -> Option<TUnion> {
    let mut types = Vec::new();
    for t in ty.types.iter() {
        let TAtomic::Object(TObject::Named(n)) = t else {
            return None;
        };
        let k = c.codebase.get_class_like(n.name.as_bytes())?;
        let value = get_specialized_template_type(
            c.codebase,
            word("TValue"),
            word(class),
            k,
            n.type_parameters.as_deref(),
        )?;
        types.extend(value.types.iter().cloned());
    }
    Some(TUnion::from_vec(types))
}

/// `(new ObjectType(Arrayable::class))->isSuperTypeOf($type)->no()`.
fn not_arrayable(c: &Checker, ty: &TUnion) -> bool {
    !ty.types.is_empty()
        && ty.types.iter().all(|t| match t {
            TAtomic::Scalar(_)
            | TAtomic::Null
            | TAtomic::Array(_)
            | TAtomic::Resource(_)
            | TAtomic::Never => true,
            TAtomic::Object(TObject::Named(n)) => {
                !is_a(c, n.name.as_bytes(), ARRAYABLE)
                    && c.codebase
                        .get_class_like(n.name.as_bytes())
                        .is_some_and(|k| k.flags.is_final())
            }
            TAtomic::Object(TObject::Enum(_)) => true,
            _ => false,
        })
}

const COLLECTION: &[u8] = b"illuminate\\support\\collection";
const QUERY_BUILDER: &[u8] = b"illuminate\\database\\query\\builder";
const ELOQUENT_BUILDER: &[u8] = b"illuminate\\database\\eloquent\\builder";
const RELATION: &[u8] = b"illuminate\\database\\eloquent\\relations\\relation";

/// The methods that can be applied on a Collection, but should be applied on a Builder.
const RISKY_METHODS: &[&str] = &[
    "count",
    "isempty",
    "isnotempty",
    "last",
    "pop",
    "skip",
    "slice",
    "take",
    "diff",
    "except",
    "find",
    "modelkeys",
    "only",
];

/// The methods that should in some cases be applied on a builder, depending on their parameter.
const RISKY_PARAM_METHODS: &[&str] = &[
    "average",
    "avg",
    "firstwhere",
    "max",
    "min",
    "pluck",
    "sum",
    "where",
    "wherestrict",
];

/// NoUnnecessaryCollectionCallRule's `$shouldHandle`: `noUnnecessaryCollectionCallOnly`, else every method, without
/// `noUnnecessaryCollectionCallExcept`.
pub(super) fn should_handle(only: &[String], except: &[String]) -> Vec<String> {
    let handled: Vec<String> = if only.is_empty() {
        RISKY_METHODS
            .iter()
            .chain(RISKY_PARAM_METHODS)
            .chain(&["first", "contains", "containsstrict"])
            .map(|m| m.to_string())
            .collect()
    } else {
        only.iter().map(|m| m.to_lowercase()).collect()
    };
    let except: Vec<String> = except.iter().map(|m| m.to_lowercase()).collect();
    handled
        .into_iter()
        .filter(|m| !except.contains(m))
        .collect()
}

/// The expressions NoUnnecessaryCollectionCallRule reads the types of: the call's object, and the previous call's.
pub(super) fn unnecessary_typed(call: &MethodCall) -> Vec<mago_span::Span> {
    let mut spans = vec![call.object.span()];
    if let Expression::Call(Call::Method(previous)) = call.object {
        spans.push(previous.object.span());
        if let Expression::Call(Call::Method(before)) = previous.object {
            spans.push(before.object.span());
        }
    }
    spans
}

/// NoUnnecessaryCollectionCallRule.
pub(super) fn unnecessary<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &MethodCall<'arena>,
    handled: &[String],
) {
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return;
    };
    let lower = String::from_utf8_lossy(name.value).to_lowercase();
    if !handled.contains(&lower) {
        return;
    }
    if !c
        .type_of(call.object)
        .is_some_and(|t| is_object_of(c, t, COLLECTION))
    {
        return;
    }
    let previous = call.object;
    let previous_name = match previous {
        Expression::Call(Call::Method(p)) => {
            if !c.type_of(p.object).is_some_and(|t| is_builder(c, t)) {
                return;
            }
            &p.method
        }
        Expression::Call(Call::StaticMethod(p)) => {
            let Expression::Identifier(class) = p.class else {
                return;
            };
            let is_query = is_a(c, c.names.get(&class.span()), super::MODEL)
                && matches!(&p.method, ClassLikeMemberSelector::Identifier(m)
                    if matches!(m.value.to_ascii_lowercase().as_slice(), b"get" | b"all" | b"pluck"));
            if !is_query {
                return;
            }
            &p.method
        }
        _ => return,
    };
    let ClassLikeMemberSelector::Identifier(previous_name) = previous_name else {
        return;
    };
    let args = call.argument_list.arguments.as_slice();
    let error = if lower == "first" {
        args.is_empty()
    } else if RISKY_METHODS.contains(&lower.as_str()) {
        true
    } else if RISKY_PARAM_METHODS.contains(&lower.as_str()) {
        if args.is_empty() {
            previous_name.value == b"pluck"
                && previous_call(c, previous).is_some_and(|(a, on)| first_arg_is_column(c, a, on))
        } else {
            first_arg_is_column(c, &call.argument_list, On::Object(call.object))
        }
    } else {
        // contains and containsStrict
        (args.len() == 1
            && !matches!(
                args[0].value(),
                Expression::Closure(_) | Expression::ArrowFunction(_)
            ))
            || first_arg_is_column(c, &call.argument_list, On::Object(call.object))
    };
    if error {
        c.report(
            call,
            "larastan.noUnnecessaryCollectionCall",
            format!(
                "Called '{}' on Laravel collection, but could have been retrieved as a query.",
                String::from_utf8_lossy(name.value)
            ),
            None,
        );
    }
}

fn is_builder(c: &Checker, ty: &TUnion) -> bool {
    [ELOQUENT_BUILDER, QUERY_BUILDER, RELATION]
        .iter()
        .any(|class| is_object_of(c, ty, class))
}

/// What a call is made on: a class name or an object.
enum On<'a, 'arena> {
    Class(String),
    Object(&'a Expression<'arena>),
}

/// The previous call's arguments and what it is made on.
fn previous_call<'a, 'arena>(
    c: &Checker,
    e: &'a Expression<'arena>,
) -> Option<(&'a ArgumentList<'arena>, On<'a, 'arena>)> {
    match e {
        Expression::Call(Call::StaticMethod(s)) => match s.class {
            Expression::Identifier(class) => Some((
                &s.argument_list,
                On::Class(String::from_utf8_lossy(c.names.get(&class.span())).into_owned()),
            )),
            _ => None,
        },
        Expression::Call(Call::Method(m)) => Some((&m.argument_list, On::Object(m.object))),
        _ => None,
    }
}

/// `firstArgIsDatabaseColumn()`: the call's first argument is a string literal naming a column of the model.
fn first_arg_is_column(c: &Checker, args: &ArgumentList, on: On) -> bool {
    let Some(Expression::Literal(Literal::String(LiteralString {
        value: Some(column),
        ..
    }))) = args.arguments.first().map(|a| a.value())
    else {
        return false;
    };
    let column = String::from_utf8_lossy(column).into_owned();
    let model = match on {
        On::Class(model) => model,
        On::Object(object) => {
            let Some(values) = c.type_of(object).and_then(|t| iterable_value(c, t)) else {
                return false;
            };
            if is_object_of(c, &values, b"stdclass") {
                // DB::table(..)->get()->max('id')
                return matches!(object, Expression::Call(Call::Method(p))
                    if c.type_of(p.object).is_some_and(|t| is_object_of(c, t, QUERY_BUILDER)));
            }
            if !is_object_of(c, &values, super::MODEL) {
                return false;
            }
            let Some(TAtomic::Object(TObject::Named(n))) = values.types.first() else {
                return false;
            };
            n.name.as_str_lossy().into_owned()
        }
    };
    super::has_database_property(c, &model, &column)
}

/// `getIterableValueType()` of a collection type.
fn iterable_value(c: &Checker, ty: &TUnion) -> Option<TUnion> {
    template(c, ty, COLLECTION)
}
