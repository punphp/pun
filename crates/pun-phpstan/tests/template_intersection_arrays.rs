//! Inherited TemplateIntersectionType array/accessory views, captured from PHPStan 2.3.1.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{Accessory, Level, Template, Tri, Type, intersect, union};
use serde_json::{Value, json};

fn template(name: &str, bound: Type, default: &str, argument: bool) -> Type {
    Type::Template(Box::new(Template {
        scope: "function fixture()".into(),
        name: name.into(),
        bound,
        default: Some(resolve_str(default, None)),
        argument,
    }))
}

fn descriptions(ty: &Type) -> Value {
    json!([Level::Precise, Level::Value, Level::TypeOnly].map(|level| ty.describe(level)))
}

#[test]
fn template_intersection_array_unions_match_inherited_views_without_erasing_identity() {
    let rows: Value =
        serde_json::from_str(include_str!("fixtures/template-intersection-arrays.json")).unwrap();
    let mut failures = Vec::new();
    for row in rows.as_array().unwrap() {
        let argument = row["argument"].as_bool().unwrap();
        let bound = if row["elementTemplate"].as_bool().unwrap() {
            intersect(vec![
                Type::array(Type::Int, template("U", Type::Int, "1", argument)),
                Type::Accessory(Accessory::List),
            ])
        } else {
            resolve_str(row["bound"].as_str().unwrap(), None)
        };
        let mut pair = vec![template("T", bound, "array{}", argument)];
        if let Some(other) = row["other"].as_str() {
            pair.push(resolve_str(other, None));
        }
        if row["reverse"].as_bool().unwrap() {
            pair.reverse();
        }
        let result = union(pair);
        let queries = [
            result.is_array(),
            result.is_list(),
            result.is_iterable_at_least_once(),
        ]
        .map(|answer| match answer {
            Tri::Yes => "Yes",
            Tri::No => "No",
            Tri::Maybe => "Maybe",
        });
        let metadata = match &result {
            Type::Template(t) => json!({
                "name": t.name,
                "scope": t.scope,
                "argument": t.argument,
                "bound": descriptions(&t.bound),
                "default": descriptions(t.default.as_ref().unwrap()),
            }),
            _ => Value::Null,
        };
        let actual = json!({
            "descriptions": descriptions(&result),
            "queries": queries,
            "template": metadata,
        });
        let expected = json!({
            "descriptions": row["descriptions"],
            "queries": row["queries"],
            "template": row["template"],
        });
        if actual != expected {
            failures.push(format!("input: {row}\nactual: {actual}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
