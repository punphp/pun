//! TypeCombinator's runtime-class scalar buckets, captured from pinned PHPStan 2.3.1.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{Level, Template, Type, sort_types, union};
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
    for row in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row.get("sort").is_none())
    {
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

#[test]
fn union_sort_matches_phpstans_zend_order_and_stability() {
    let rows: Value =
        serde_json::from_str(include_str!("fixtures/template-scalar-unions.json")).unwrap();
    let mut failures = Vec::new();
    for row in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row.get("sort").is_some())
    {
        let spec = &row["sort"];
        let pattern = spec["pattern"].as_array().unwrap();
        let mut inputs: Vec<_> = (0..spec["count"].as_u64().unwrap())
            .map(|i| {
                let mut entry = pattern[i as usize % pattern.len()].clone();
                if let Some(template) = entry.as_object_mut() {
                    template.insert(
                        "name".into(),
                        format!("{}{i}", template["name"].as_str().unwrap()).into(),
                    );
                }
                input(&entry, false)
            })
            .collect();
        if spec["reverse"].as_bool().unwrap() {
            inputs.reverse();
        }
        let got: Vec<_> = sort_types(&inputs)
            .iter()
            .map(|t| t.describe(Level::Precise))
            .collect();
        let want: Vec<_> = row["order"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| inputs[i.as_u64().unwrap() as usize].describe(Level::Precise))
            .collect();
        if got != want {
            let differences: Vec<_> = got
                .iter()
                .zip(&want)
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .take(8)
                .collect();
            failures.push(format!(
                "{spec}\nlengths: {}/{}; first differing slots: {differences:?}",
                got.len(),
                want.len()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
