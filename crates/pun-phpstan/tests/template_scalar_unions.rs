//! TypeCombinator's runtime-class scalar buckets, captured from pinned PHPStan 2.3.1.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{Level, Template, Type, union};
use serde_json::Value;

fn input(spec: &Value, argument: bool) -> Type {
    if let Some(ty) = spec.as_str() {
        return resolve_str(ty, None);
    }
    let argument = argument || spec["argument"].as_bool().unwrap();
    Type::Template(Box::new(Template {
        name: spec["name"].as_str().unwrap().into(),
        scope: format!(
            "function {}()",
            spec["function"].as_str().unwrap_or("fixture")
        ),
        bound: input(&spec["bound"], argument),
        default: None,
        argument,
    }))
}

#[test]
fn constant_template_buckets_keep_identity_and_factory_subclass_distinctions() {
    let rows: Value =
        serde_json::from_str(include_str!("fixtures/template-scalar-unions.json")).unwrap();
    let mut failures = Vec::new();
    for row in rows.as_array().unwrap() {
        let result = union(
            row["inputs"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| input(v, false))
                .collect(),
        );
        let descriptions =
            [Level::Precise, Level::Value, Level::TypeOnly].map(|level| result.describe(level));
        let want: Vec<_> = row["descriptions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        let members = match &result {
            Type::Union(parts) | Type::Benevolent(parts) => parts.len(),
            _ => 1,
        };
        if descriptions != want.as_slice() || members != row["members"].as_u64().unwrap() as usize {
            failures.push(format!(
                "{row}\nactual: {descriptions:?}; members: {members}"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
