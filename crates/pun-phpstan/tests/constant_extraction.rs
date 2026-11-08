//! getConstantStrings/getConstantArrays from actual PHPStan 2.3.1, including raw union/intersection inputs.
//! Expected values are fresh oracle operations, not descriptions of Pun's current extraction.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{Accessory, Level, Template, Type};
use serde_json::{Value, json};

fn input(spec: &Value) -> Type {
    if let Some(ty) = spec.as_str() {
        return resolve_str(ty, None);
    }
    if let Some(t) = spec.get("template") {
        return Type::Template(Box::new(Template {
            name: t["name"].as_str().unwrap().into(),
            scope: "function fixture()".into(),
            bound: input(&t["bound"]),
            default: None,
            argument: false,
        }));
    }
    if let Some(offset) = spec.get("offset") {
        return Type::Accessory(Accessory::HasOffsetValue(
            Box::new(Type::ConstInt(offset[0].as_i64().unwrap())),
            Box::new(input(&offset[1])),
        ));
    }
    for (key, construct) in [
        ("union", Type::Union as fn(Vec<Type>) -> Type),
        ("benevolent", Type::Benevolent),
        ("intersection", Type::Intersection),
    ] {
        if let Some(parts) = spec.get(key).and_then(Value::as_array) {
            return construct(parts.iter().map(input).collect());
        }
    }
    panic!("unknown oracle input: {spec}");
}

#[test]
fn constant_extraction_matches_phpstan_domains_templates_and_callable_offsets() {
    let rows: Value =
        serde_json::from_str(include_str!("fixtures/compact-extraction.json")).unwrap();
    let mut failures = Vec::new();
    for row in rows.as_array().unwrap() {
        let ty = input(&row["input"]);
        let arrays: Vec<_> = ty.constant_arrays().into_iter().map(|a| json!({
            "keys": a.keys.iter().map(|t| t.describe(Level::Precise)).collect::<Vec<_>>(),
            "values": a.values.iter().map(|t| t.describe(Level::Precise)).collect::<Vec<_>>(),
            "optional": a.optional,
            "list": format!("{:?}", a.is_list),
            "next": a.next_auto_indexes,
        })).collect();
        let got = json!({"input": row["input"], "strings": ty.constant_strings().into_iter().map(|s| String::from_utf8(s).unwrap()).collect::<Vec<_>>(), "arrays": arrays});
        if got != *row {
            failures.push(format!("{row}\nactual: {got}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
