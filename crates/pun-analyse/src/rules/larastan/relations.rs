//! RelationExistenceRule: the relation names given to `has()`, `with()`, `whereHas()` & co. are relation methods of
//! the model the builder, relation or model is of (ModelRuleHelper).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::{Checker, members};
use super::{MODEL, is_a, is_object_of};

const QUERY_BUILDER: &[u8] = b"illuminate\\database\\query\\builder";
const ELOQUENT_BUILDER: &[u8] = b"illuminate\\database\\eloquent\\builder";
const RELATION: &[u8] = b"illuminate\\database\\eloquent\\relations\\relation";

const METHODS: &[&[u8]] = &[
    b"has",
    b"with",
    b"orHas",
    b"doesntHave",
    b"orDoesntHave",
    b"whereHas",
    b"withWhereHas",
    b"orWhereHas",
    b"whereDoesntHave",
    b"orWhereDoesntHave",
    b"whereRelation",
];

/// The first argument of a call the rule reads, and what it is called on (`None` for a class name).
pub(super) fn relation_call<'a, 'arena>(
    method: &ClassLikeMemberSelector<'arena>,
    arguments: &'a ArgumentList<'arena>,
) -> Option<&'a Expression<'arena>> {
    let ClassLikeMemberSelector::Identifier(m) = method else {
        return None;
    };
    if !METHODS.contains(&m.value) {
        return None;
    }
    Some(arguments.arguments.first()?.value())
}

/// The type a call is made on.
enum CalledOn {
    Class(String),
    Type(TUnion),
}

pub(super) fn method_call<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some(arg) = relation_call(&call.method, &call.argument_list) else {
        return;
    };
    let Some(on) = c.type_of(call.object).cloned() else {
        return;
    };
    check(c, call, arg, CalledOn::Type(on));
}

pub(super) fn static_method_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &StaticMethodCall<'arena>,
) {
    let Some(arg) = relation_call(&call.method, &call.argument_list) else {
        return;
    };
    let on = match call.class {
        Expression::Identifier(id) => {
            CalledOn::Class(String::from_utf8_lossy(c.names.get(&id.span())).into_owned())
        }
        e => match c.type_of(e) {
            Some(t) => CalledOn::Type(t.clone()),
            None => return,
        },
    };
    check(c, call, arg, on);
}

/// The relation names in the argument: a constant array's string keys and values, else its constant strings.
fn relations(ty: &TUnion) -> Vec<String> {
    let strings = |t: &TUnion| -> Vec<String> {
        t.types
            .iter()
            .map(|a| {
                a.get_literal_string_value()
                    .map(|v| String::from_utf8_lossy(v).into_owned())
            })
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default()
    };
    let constant_arrays: Option<Vec<&TArray>> = ty
        .types
        .iter()
        .map(|t| match t {
            TAtomic::Array(a @ TArray::Keyed(k)) if k.parameters.is_none() => Some(a),
            TAtomic::Array(a @ TArray::List(l)) if l.element_type.is_never() => Some(a),
            _ => None,
        })
        .collect();
    let Some(arrays) = constant_arrays.filter(|a| !a.is_empty()) else {
        return strings(ty);
    };
    let mut names = Vec::new();
    for array in arrays {
        let (keys, values): (Vec<String>, Vec<&TUnion>) = match array {
            TArray::Keyed(k) => {
                let items = k.known_items.iter().flat_map(|m| m.iter());
                (
                    items
                        .clone()
                        .filter_map(|(key, _)| match key {
                            ArrayKey::String(s) => Some(s.as_str_lossy().into_owned()),
                            _ => None,
                        })
                        .collect(),
                    items.map(|(_, (_, v))| v).collect(),
                )
            }
            TArray::List(l) => (
                Vec::new(),
                l.known_elements
                    .iter()
                    .flat_map(|m| m.values().map(|(_, v)| v))
                    .collect(),
            ),
        };
        names.extend(keys);
        names.extend(values.into_iter().flat_map(strings));
    }
    names
}

fn check<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: &impl HasSpan,
    arg: &Expression<'arena>,
    on: CalledOn,
) {
    let Some(arg) = c.type_of(arg).cloned() else {
        return;
    };
    let mut errors = Vec::new();
    for relation in relations(&arg) {
        let name = relation.split(':').next().unwrap_or_default();
        if !name.contains('.') {
            errors.extend(missing(c, &on, name));
            continue;
        }
        // nested relations: the first missing one is the call's only error, else it has none
        errors.clear();
        let mut on = on;
        for part in name.split('.') {
            if let Some(error) = missing(c, &on, part) {
                errors.push(error);
                break;
            }
            let Some(next) = model(c, &on).and_then(|k| relation_type(c, k, part)) else {
                break;
            };
            on = CalledOn::Type(next);
        }
        break;
    }
    for message in errors {
        c.report(at, "larastan.relationExistence", message, None);
    }
}

/// The error when the model `on` is of has no relation `name`.
fn missing(c: &Checker, on: &CalledOn, name: &str) -> Option<String> {
    let k = model(c, on)?;
    relation_type(c, k, name).is_none().then(|| {
        format!(
            "Relation '{name}' is not found in {} model.",
            k.original_name.as_str_lossy()
        )
    })
}

/// The return type of the model's method `name` when it is a relation.
fn relation_type(c: &Checker, k: &ClassLikeMetadata, name: &str) -> Option<TUnion> {
    let members::Method::User { f, .. } = members::method(c, k, name.as_bytes())? else {
        return None;
    };
    let ty = &f.return_type_metadata.as_ref()?.type_union;
    is_object_of(c, ty, RELATION).then(|| ty.clone())
}

/// ModelRuleHelper::findModelReflectionFromType().
fn model<'a>(c: &Checker<'a, '_>, on: &CalledOn) -> Option<&'a ClassLikeMetadata> {
    let model = match on {
        CalledOn::Class(name) => {
            if !is_a(c, name.as_bytes(), MODEL) {
                return None;
            }
            name.clone()
        }
        CalledOn::Type(ty) => {
            if ![QUERY_BUILDER, ELOQUENT_BUILDER, RELATION, MODEL]
                .iter()
                .any(|class| is_object_of(c, ty, class))
            {
                return None;
            }
            let [TAtomic::Object(TObject::Named(n))] = ty.types.as_ref() else {
                return None;
            };
            match n.type_parameters.as_deref() {
                Some([first, ..]) => {
                    let names: Vec<_> = first
                        .types
                        .iter()
                        .filter(|t| !matches!(t, TAtomic::Null))
                        .filter_map(|t| match t {
                            TAtomic::Object(TObject::Named(m)) => {
                                Some(m.name.as_str_lossy().into_owned())
                            }
                            _ => None,
                        })
                        .collect();
                    match names.as_slice() {
                        [one] => one.clone(),
                        _ => return None,
                    }
                }
                _ if is_a(c, n.name.as_bytes(), MODEL) => n.name.as_str_lossy().into_owned(),
                _ => return None,
            }
        }
    };
    if model.eq_ignore_ascii_case("Illuminate\\Database\\Eloquent\\Model") {
        return None;
    }
    c.codebase
        .get_class_like(model.as_bytes())
        .filter(|k| !k.flags.is_abstract())
}
