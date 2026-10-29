//! Public MixedType contracts from pinned PHPStan 2.3.1, shared with the Analyzer's oracle.

use std::collections::BTreeMap;

use pun_phpstan::types::{
    ConstArray, Level, NeverKind, Tri, Type, accepts, intersect, is_super_type_of, mixed, union,
};
use serde_json::Value;

fn oracle() -> Value {
    serde_json::from_str(include_str!(
        "../../pun-analyse/tests/fixtures/mixed-subtraction/types.json"
    ))
    .unwrap()
}

fn phpstan_types() -> BTreeMap<&'static str, Type> {
    let mut types = BTreeMap::from([
        ("mixed", mixed()),
        ("explicit-mixed", Type::Mixed { explicit: true }),
        ("error", Type::Error),
        ("zero", Type::ConstInt(0)),
        ("one", Type::ConstInt(1)),
        ("thirteen", Type::ConstInt(13)),
        ("null", Type::Null),
        ("int", Type::Int),
        ("string", Type::String),
        ("literal", Type::ConstString("x".into())),
        ("empty", Type::ConstArray(Box::new(ConstArray::empty()))),
        ("never", Type::Never(NeverKind::Implicit)),
        ("explicit-never", Type::Never(NeverKind::Explicit)),
        ("range", Type::int_range(Some(-1), Some(1))),
    ]);
    for (name, excluded) in [
        ("not-zero", Type::ConstInt(0)),
        ("not-one", Type::ConstInt(1)),
        ("not-null", Type::Null),
        ("not-int", Type::Int),
        ("not-string", Type::String),
        ("not-range", types["range"].clone()),
        ("not-empty", types["empty"].clone()),
        ("not-zero-null", union(vec![Type::ConstInt(0), Type::Null])),
        (
            "not-zero-empty",
            union(vec![Type::ConstInt(0), types["empty"].clone()]),
        ),
    ] {
        types.insert(name, Type::mixed_subtracted(false, excluded));
    }
    types
}

fn canonical(description: &str) -> String {
    if let Some(inner) = description
        .strip_prefix("mixed~(")
        .and_then(|s| s.strip_suffix(')'))
    {
        let mut parts: Vec<_> = inner.split('|').collect();
        parts.sort();
        format!("mixed~({})", parts.join("|"))
    } else {
        let mut parts: Vec<_> = description.split('|').collect();
        parts.sort();
        parts.join("|")
    }
}

#[test]
fn pun_mixed_exclusions_describe_compare_and_accept_like_phpstan() {
    let types = phpstan_types();
    let expected = oracle();
    let mut failures = Vec::new();
    for (label, want) in expected["types"].as_object().unwrap() {
        let parts: Vec<_> = label.split(':').collect();
        let result = match parts.as_slice() {
            ["union", a, b] => union(vec![types[a].clone(), types[b].clone()]),
            ["intersect", a, b] => intersect(vec![types[a].clone(), types[b].clone()]),
            [a] => types[a].clone(),
            _ => continue, // Subtraction and unset originate in codex, not the conversion layer.
        };
        for (level, key) in [
            (Level::Precise, "precise"),
            (Level::Value, "value"),
            (Level::TypeOnly, "typeOnly"),
        ] {
            let got = canonical(&result.describe(level));
            if got != canonical(want[key].as_str().unwrap()) {
                failures.push(format!("{label} {key}: {got}, expected {}", want[key]));
            }
        }
    }
    for (label, want) in expected["relations"].as_object().unwrap() {
        let (a, b) = label.split_once(':').unwrap();
        for (key, got) in [
            ("super", is_super_type_of(&types[a], &types[b])),
            ("accepts", accepts(&types[a], &types[b], true)),
        ] {
            let got = match got {
                Tri::Yes => "yes",
                Tri::Maybe => "maybe",
                Tri::No => "no",
            };
            if got != want[key].as_str().unwrap() {
                failures.push(format!("{label} {key}: {got}, expected {}", want[key]));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn mixed_exclusions_keep_public_traversal_cast_and_acceptance_diagnostics() {
    use pun_phpstan::types::{Cast, StrictMixed, accepts_reasons, cast_is_error, to_phpdoc};
    let types = phpstan_types();
    let excluded = &types["not-zero"];
    let expected = oracle();
    let want = &expected["utilities"];
    for (cast, label) in [
        (Cast::String, "toString"),
        (Cast::Integer, "toInteger"),
        (Cast::BitwiseNot, "toBitwiseNotType"),
    ] {
        assert_eq!(
            cast_is_error(excluded, cast, StrictMixed::default()),
            want[label].as_bool().unwrap()
        );
    }
    assert_eq!(to_phpdoc(excluded), want["phpdoc"]);
    assert_eq!(excluded.is_cloneable().yes(), want["cloneable"]);
    assert_eq!(excluded.is_implicit_mixed(), want["implicit"]);
    assert_eq!(
        Type::mixed_subtracted(true, Type::ConstInt(0)).is_implicit_mixed(),
        want["explicit"]
    );
    assert_eq!(
        Type::array(excluded.clone(), Type::Int).describe(Level::Precise),
        want["array-int"]
    );
    assert_eq!(
        Type::array(excluded.clone(), mixed()).describe(Level::Precise),
        want["array-mixed"]
    );
    assert_eq!(
        cast_is_error(
            excluded,
            Cast::Integer,
            StrictMixed {
                implicit: true,
                explicit: false
            }
        ),
        want["strict-toInteger"]
    );
    assert_eq!(
        serde_json::to_value(accepts_reasons(
            excluded,
            &Type::ConstInt(0),
            true,
            StrictMixed::default()
        ))
        .unwrap(),
        want["accepts-reasons"]
    );
    assert_eq!(
        serde_json::to_value(accepts_reasons(
            &Type::ConstInt(0),
            excluded,
            true,
            StrictMixed::default()
        ))
        .unwrap(),
        want["reasons"]
    );
    let shifted = excluded.traverse(&|t| {
        if *t == Type::ConstInt(0) {
            Type::ConstInt(1)
        } else {
            t.clone()
        }
    });
    assert_eq!(shifted.describe(Level::Precise), want["traverse"]);
}

#[test]
fn pun_mixed_queries_distinguish_complete_domains_from_partial_exclusions() {
    use pun_phpstan::types::{Accessory, Conditional, Signature, Subject, Template};
    let domains = BTreeMap::from([
        ("string", Type::String),
        ("literal-string", Type::ConstString("x".into())),
        ("float", Type::Float),
        ("literal-float", Type::ConstFloat(1.5)),
        ("bool", Type::Bool),
        ("false", Type::ConstBool(false)),
        ("array", Type::array(mixed(), mixed())),
        (
            "empty-array",
            Type::ConstArray(Box::new(ConstArray::empty())),
        ),
        ("int-array", Type::array(Type::Int, mixed())),
        ("list", Type::list(mixed())),
        (
            "nonempty-array",
            Type::Intersection(vec![
                Type::array(mixed(), mixed()),
                Type::Accessory(Accessory::NonEmptyArray),
            ]),
        ),
        (
            "iterable",
            Type::Iterable(Box::new(mixed()), Box::new(mixed())),
        ),
        (
            "int-iterable",
            Type::Iterable(Box::new(Type::Int), Box::new(mixed())),
        ),
        ("callable", Type::Callable(None)),
        (
            "partial-callable",
            Type::Callable(Some(Box::new(Signature {
                params: vec![],
                ret: Type::Void,
                templates: vec![],
            }))),
        ),
        ("null", Type::Null),
        ("void", Type::Void),
        ("never", Type::Never(NeverKind::Implicit)),
        ("not-string", Type::mixed_subtracted(false, Type::String)),
        ("string-float", Type::Union(vec![Type::String, Type::Float])),
    ]);
    let expected = oracle();
    let mut failures = vec![];
    for (label, row) in expected["queries"].as_object().unwrap() {
        let ty = if label == "mixed" {
            mixed()
        } else if label == "error" {
            Type::Error
        } else {
            let (kind, name) = label.split_once(':').unwrap();
            let domain = domains[name].clone();
            let excluded = Type::mixed_subtracted(kind == "explicit", domain.clone());
            match kind {
                "exclude" | "explicit" => excluded,
                "union" | "benevolent" => {
                    let mut parts = vec![excluded];
                    if let Type::Union(inner) = domain {
                        parts.extend(inner);
                    } else {
                        parts.push(domain);
                    }
                    if kind == "union" {
                        Type::Union(parts)
                    } else {
                        Type::Benevolent(parts)
                    }
                }
                "intersection" => Type::Intersection(vec![excluded, domain]),
                "template" => Type::Template(Box::new(Template {
                    name: "T".into(),
                    scope: "function mixedQueries()".into(),
                    bound: excluded,
                    default: None,
                    argument: false,
                })),
                "conditional" => Type::Conditional(Box::new(Conditional {
                    subject: Subject::Type(Type::ConstInt(0)),
                    target: Type::Int,
                    then: excluded,
                    otherwise: domain,
                    negated: false,
                })),
                _ => panic!("unknown oracle operation {kind}"),
            }
        };
        for (query, result) in [
            ("isString", ty.is_string()),
            ("isFloat", ty.is_float()),
            ("isBoolean", ty.is_boolean()),
            ("isArray", ty.is_array()),
            ("isList", ty.is_list()),
            ("isIterable", ty.is_iterable()),
            ("isIterableAtLeastOnce", ty.is_iterable_at_least_once()),
            ("isCallable", ty.is_callable()),
            ("isNull", ty.is_null()),
            ("isVoid", ty.is_void()),
        ] {
            let got = match result {
                Tri::Yes => "yes",
                Tri::Maybe => "maybe",
                Tri::No => "no",
            };
            if got != row[query].as_str().unwrap() {
                failures.push(format!("{label} {query}: {got}, expected {}", row[query]));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn subtraction_descriptions_follow_runtime_type_not_rendered_delimiters() {
    use pun_phpstan::resolve::resolve_str;
    use pun_phpstan::types::{Accessory, Template};

    fn input(spec: &Value, argument: bool) -> Type {
        if let Some(ty) = spec.as_str() {
            return resolve_str(ty, None);
        }
        if let Some(bound) = spec.get("template") {
            let argument = argument || spec["argument"].as_bool().unwrap();
            return Type::Template(Box::new(Template {
                name: spec["name"].as_str().unwrap().into(),
                scope: "function subtractionDescriptions()".into(),
                bound: input(bound, argument),
                default: None,
                argument,
            }));
        }
        if let Some(excluded) = spec.get("mixed") {
            return Type::mixed_subtracted(
                spec["explicit"].as_bool().unwrap(),
                input(excluded, argument),
            );
        }
        if let Some(parts) = spec["array"].as_array() {
            return Type::array(input(&parts[0], argument), input(&parts[1], argument));
        }
        if spec["accessory"] == "non-empty-array" {
            return Type::Accessory(Accessory::NonEmptyArray);
        }
        for kind in ["union", "benevolent", "intersection"] {
            if let Some(parts) = spec[kind].as_array() {
                let parts = parts.iter().map(|part| input(part, argument)).collect();
                return match kind {
                    "union" => Type::Union(parts),
                    "benevolent" => Type::Benevolent(parts),
                    _ => Type::Intersection(parts),
                };
            }
        }
        panic!("unknown oracle type input: {spec}");
    }

    let rows: Value =
        serde_json::from_str(include_str!("fixtures/mixed-subtraction-descriptions.json")).unwrap();
    let mut failures = vec![];
    for row in rows.as_array().unwrap() {
        let ty = input(&row["input"], false);
        for (i, level) in [Level::Precise, Level::Value, Level::TypeOnly]
            .into_iter()
            .enumerate()
        {
            let got = ty.describe(level);
            let want = row["descriptions"][i].as_str().unwrap();
            if got != want {
                failures.push(format!(
                    "{} {level:?}: {got}, expected {want}",
                    row["label"]
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

// Exclusions retain the positive identity for iteration, but not for containment or precise unions.
#[test]
fn object_exclusions_keep_identity_through_public_queries_and_unions() {
    use pun_phpstan::resolve::resolve_str;
    use serde_json::json;

    fn input(spec: &Value) -> Type {
        if let Some(name) = spec.as_str() {
            return resolve_str(name, None);
        }
        if let Some(object) = spec.get("object") {
            return Type::object_subtracted(input(object), input(&spec["excluded"]));
        }
        Type::Union(
            spec["union"]
                .as_array()
                .unwrap()
                .iter()
                .map(input)
                .collect(),
        )
    }

    let describe =
        |t: &Type| [Level::Precise, Level::Value, Level::TypeOnly].map(|level| t.describe(level));
    let tri = |result: Tri| match result {
        Tri::Yes => "Yes",
        Tri::Maybe => "Maybe",
        Tri::No => "No",
    };
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/object-subtraction-api.json")).unwrap();
    let types: BTreeMap<_, _> = oracle["inputs"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, spec)| (name.as_str(), input(spec)))
        .collect();
    for (name, expected) in oracle["queries"].as_object().unwrap() {
        let t = &types[name.as_str()];
        assert_eq!(
            json!({
                "descriptions": describe(t), "names": t.object_class_names(),
                "key": describe(&t.iterable_key_type()), "value": describe(&t.iterable_value_type()),
            }),
            *expected,
            "{name}"
        );
    }
    for (pair, expected) in oracle["super"].as_object().unwrap() {
        let (a, b) = pair.split_once(':').unwrap();
        assert_eq!(
            json!(tri(is_super_type_of(&types[a], &types[b]))),
            *expected,
            "{pair}"
        );
    }
    for (pair, expected) in oracle["unions"].as_object().unwrap() {
        let (a, b) = pair.split_once(':').unwrap();
        assert_eq!(
            json!(describe(&union(vec![types[a].clone(), types[b].clone()]))),
            *expected,
            "{pair}"
        );
    }
}
