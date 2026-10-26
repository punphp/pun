//! Checked mixed contracts from PHPStan 2.3.1, not scope/declaration widening.

use pun_phpstan::types::{Cast, Level, StrictMixed, Template, Type, accepts, cast_is_error, intersect, is_super_type_of, to_phpdoc, union};
use serde_json::Value;

fn input(spec: &Value) -> Type {
    if let Some(name) = spec.as_str() {
        return match name {
            "mixed" => Type::Mixed { explicit: false },
            "explicit" => Type::Mixed { explicit: true },
            "strict" => Type::StrictMixed,
            "int" => Type::Int,
            "string" => Type::String,
            "null" => Type::Null,
            "object" => Type::ObjectWithoutClass,
            "error" => Type::Error,
            _ => panic!("unknown input {spec}"),
        };
    }
    if let Some(excluded) = spec.get("excluded") {
        return Type::SubtractedMixed { explicit: spec["explicit"].as_bool().unwrap_or(false), excluded: Box::new(input(excluded)) };
    }
    if let Some(bound) = spec.get("template") {
        return Type::Template(Box::new(Template {
            name: spec["name"].as_str().unwrap().into(),
            scope: format!("function {}()", spec["scope"].as_str().unwrap_or("fixture")),
            bound: if spec["strict"].as_bool().unwrap_or(false) { Type::StrictMixed } else { input(bound) },
            default: spec.get("default").filter(|v| !v.is_null()).map(input),
            argument: spec["argument"].as_bool().unwrap_or(false),
        }));
    }
    let parts = spec["parts"].as_array().unwrap().iter().map(input).collect();
    match spec["kind"].as_str().unwrap() {
        "union" => Type::Union(parts),
        "intersection" => Type::Intersection(parts),
        "combine-union" => union(parts),
        "combine-intersection" => intersect(parts),
        _ => panic!("unknown input {spec}"),
    }
}

#[test]
fn checked_mixed_queries_rendering_identity_and_acceptance() {
    let oracle: Value = serde_json::from_str(include_str!("fixtures/strict-mixed.json")).unwrap();
    for row in oracle["rows"].as_array().unwrap() {
        let t = input(&row["input"]);
        let descriptions = [Level::TypeOnly, Level::Value, Level::Precise].map(|l| t.describe(l));
        assert_eq!(serde_json::json!(descriptions), row["descriptions"], "{row}");
        assert_eq!(to_phpdoc(&t), row["phpdoc"].as_str().unwrap(), "{row}");
        if row.get("queries").is_none() { continue; }
        let queries = [t.is_array(), t.is_callable(), t.is_cloneable(), t.is_iterable(), t.is_iterable_at_least_once(), t.is_string(), t.is_integer(), t.is_boolean(), t.is_null()].map(|v| format!("{v:?}"));
        assert_eq!(serde_json::json!(queries), row["queries"], "{row}");
        let casts = [Cast::String, Cast::Number, Cast::Integer, Cast::Float, Cast::Boolean, Cast::BitwiseNot].map(|c| cast_is_error(&t, c, StrictMixed::default()));
        assert_eq!(serde_json::json!(casts), row["casts"], "{row}");
        let iterable = [t.iterable_key_type(), t.iterable_value_type()].map(|v| v.describe(Level::Precise));
        assert_eq!(serde_json::json!(iterable), row["iterable"], "{row}");
        assert_eq!(t.traverse(&Clone::clone), t, "traversal must retain strict identity: {row}");
    }
    for row in oracle["pairs"].as_array().unwrap() {
        let a = input(&row[0]);
        let b = input(&row[1]);
        let actual = serde_json::json!([row[0], row[1], a == b,
            format!("{:?}", is_super_type_of(&a, &b)),
            format!("{:?}", is_super_type_of(&b, &a)),
            format!("{:?}", accepts(&a, &b, true)),
            format!("{:?}", accepts(&b, &a, true))]);
        assert_eq!(actual, *row);
    }
}
