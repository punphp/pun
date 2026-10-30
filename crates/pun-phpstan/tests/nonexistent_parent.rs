//! NonexistentParentClassType's identity and inherited operations, captured from PHPStan 2.3.1.

use std::cell::Cell;

use pun_phpstan::resolve::{resolve_late, resolve_str};
use pun_phpstan::types::{
    Accessory, Cast, Level, NeverKind, StrictMixed, Template, Tri, Type, accepts, cast_is_error,
    infer_template_types, intersect, is_super_type_of, mixed, to_phpdoc, union,
};
use serde_json::{Value, json};

fn capture() -> Value {
    serde_json::from_str(include_str!("fixtures/nonexistent-parent/expected.json")).unwrap()
}

fn descriptions(t: &Type) -> Value {
    json!([Level::Precise, Level::Value, Level::TypeOnly].map(|level| t.describe(level)))
}

fn tri(t: Tri) -> &'static str {
    match t {
        Tri::Yes => "Yes",
        Tri::No => "No",
        Tri::Maybe => "Maybe",
    }
}

fn ty(name: &str) -> Type {
    let p = Type::NonexistentParent;
    match name {
        "parent-sentinel" => p,
        "named-parent" => Type::Object("parent".into(), vec![], false),
        "stdClass" => resolve_str("stdClass", None),
        "object" => Type::ObjectWithoutClass,
        "generic-object" => Type::Object("ArrayObject".into(), vec![Type::Int], false),
        "shape-empty" => resolve_str("object{}", None),
        "shape-optional" => resolve_str("object{foo?: int}", None),
        "shape-required" => resolve_str("object{foo: int}", None),
        "callable" => resolve_str("callable", None),
        "iterable" => resolve_str("iterable", None),
        "error" => Type::Error,
        "mixed-implicit" => mixed(),
        "mixed-explicit" => Type::Mixed { explicit: true },
        "strict-mixed" => Type::StrictMixed,
        "never-implicit" => Type::Never(NeverKind::Implicit),
        "never-explicit" => Type::Never(NeverKind::Explicit),
        "never-nonaccepting" => Type::Never(NeverKind::NonAccepting),
        "null" => Type::Null,
        "union-null" => Type::Union(vec![p, Type::Null]),
        "benevolent-null" => Type::Benevolent(vec![p, Type::Null]),
        "intersection-method" => {
            Type::Intersection(vec![p, Type::Accessory(Accessory::HasMethod("foo".into()))])
        }
        "union-object" => Type::Union(vec![p, ty("stdClass")]),
        "benevolent-object" => Type::Benevolent(vec![p, ty("stdClass")]),
        "template-union" => Type::Template(Box::new(Template {
            name: "T".into(),
            scope: "function fixture()".into(),
            bound: ty("union-null"),
            default: None,
            argument: false,
        })),
        _ => panic!("unknown capture type {name}"),
    }
}

#[test]
fn parent_is_a_distinct_leaf_with_compound_delegation_and_reverse_relations() {
    let p = Type::NonexistentParent;
    for (name, expected) in capture()["relations"].as_object().unwrap() {
        let t = ty(name);
        assert_eq!(
            json!({
                "equals": [p == t, t == p],
                "super": [tri(is_super_type_of(&p, &t)), tri(is_super_type_of(&t, &p))],
                "accepts": ([false, true].map(|strict| [tri(accepts(&p, &t, strict)), tri(accepts(&t, &p, strict))])),
                "names": t.object_class_names(),
                "isObject": tri(t.is_object()),
            }),
            *expected,
            "{name}"
        );
    }
}

#[test]
fn parent_rejects_members_and_iteration_and_does_not_traverse_or_infer() {
    let p = Type::NonexistentParent;
    let calls = Cell::new(0);
    let traversed = p.traverse(&|_| {
        calls.set(calls.get() + 1);
        Type::String
    });
    let actual = json!({
        "queries": {
            "isObject": tri(p.is_object()), "isString": tri(p.is_string()),
            "isInteger": tri(p.is_integer()), "isFloat": tri(p.is_float()),
            "isBoolean": tri(p.is_boolean()), "isArray": tri(p.is_array()),
            "isList": tri(p.is_list()), "isIterable": tri(p.is_iterable()),
            "isIterableAtLeastOnce": tri(p.is_iterable_at_least_once()),
            "isCloneable": tri(p.is_cloneable()), "isCallable": tri(p.is_callable()),
            "isNull": tri(p.is_null()), "isVoid": tri(p.is_void()),
        },
        "descriptions": descriptions(&p), "phpdoc": to_phpdoc(&p),
        "constantStrings": p.constant_strings(), "constantArrays": p.constant_arrays().len(),
        "hasMethod": tri(p.has_method("foo")),
        "iterableKey": descriptions(&p.iterable_key_type()),
        "iterableValue": descriptions(&p.iterable_value_type()),
        "traverse": [json!(traversed == p), json!(calls.get())],
        "generalize": p.generalize() == p,
        "inference": infer_template_types(&p, &ty("template-union")).types().len(),
    });
    assert_eq!(actual, capture()["leaf"]);
}

#[test]
fn parent_casts_fail_except_boolean_without_turning_into_mixed_or_never() {
    for (name, expected) in capture()["casts"].as_object().unwrap() {
        let actual = [
            Cast::String,
            Cast::Number,
            Cast::Integer,
            Cast::Float,
            Cast::Boolean,
            Cast::BitwiseNot,
        ]
        .map(|cast| cast_is_error(&ty(name), cast, StrictMixed::default()));
        assert_eq!(json!(actual), *expected, "{name}");
    }
}

#[test]
fn parent_combinators_keep_object_disjointness_and_number_equal_descriptions() {
    for (name, rows) in capture()["combinators"].as_object().unwrap() {
        for (reverse, expected) in rows.as_array().unwrap().iter().enumerate() {
            let mut pair = vec![Type::NonexistentParent, ty(name)];
            if reverse == 1 {
                pair.reverse();
            }
            let actual = json!({
                "union": descriptions(&union(pair.clone())),
                "intersection": descriptions(&intersect(pair)),
            });
            assert_eq!(actual, *expected, "{name}, reverse={reverse}");
        }
    }
    let Type::Template(t) = intersect(vec![ty("template-union"), Type::NonexistentParent]) else {
        panic!("template union must retain its template strategy");
    };
    assert_eq!(
        json!([
            descriptions(&t.bound),
            json!(matches!(t.bound, Type::Mixed { explicit: true }))
        ]),
        capture()["templateBound"]
    );
}

#[test]
fn phpdoc_parent_is_a_sentinel_only_in_a_parentless_class_scope() {
    for (operation, late) in [("resolution", false), ("lateResolution", true)] {
        for row in capture()[operation].as_array().unwrap() {
            let class = row["class"].as_str();
            let input = row["input"].as_str().unwrap();
            let raw = resolve_str(input, class);
            let result = if late { resolve_late(&raw) } else { raw };
            assert_eq!(
                json!({
                    "class": class, "input": input, "descriptions": descriptions(&result),
                    "names": result.object_class_names(), "sentinel": matches!(result, Type::NonexistentParent),
                }),
                *row,
                "{operation}: {input}, class={class:?}"
            );
        }
    }
    // Native unresolved `parent` is deliberately still constructed as a named object by its producer.
    assert_eq!(
        json!(ty("named-parent").object_class_names()),
        capture()["nativeParent"]
    );
}

#[test]
fn object_predicate_is_not_generic_object_containment() {
    for (input, expected) in capture()["objectQueries"].as_object().unwrap() {
        let t = match input.as_str() {
            "mixed~object" => Type::mixed_subtracted(false, Type::ObjectWithoutClass),
            "hasMethod" => Type::Accessory(Accessory::HasMethod("foo".into())),
            input => resolve_str(input, None),
        };
        assert_eq!(json!(tri(t.is_object())), *expected, "{input}");
    }
}
