//! The array-origin HasOffsetValue accessory, from actual PHPStan 2.3.1 public type operations.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{
    Accessory, ConstArray, Level, Tri, Type, intersect, is_super_type_of, to_phpdoc,
};
use serde_json::{Value, json};

fn input(row: &Value) -> Type {
    if let Some(ty) = row["type"].as_str() {
        if ty == "array<int, *ERROR*>" {
            return Type::array(Type::Int, Type::Error);
        }
        if ty == "array{...<int, string>}" {
            let mut array = ConstArray::empty();
            array.unsealed = Some((Type::Int, Type::String));
            array.is_list = Tri::Maybe;
            return Type::ConstArray(Box::new(array));
        }
        return resolve_str(ty, None);
    }
    let key = row["offset"].as_i64().map_or_else(
        || Type::ConstString(row["offset"].as_str().unwrap().into()),
        Type::ConstInt,
    );
    Type::Accessory(Accessory::HasOffsetValue(
        Box::new(key),
        Box::new(resolve_str(row["value"].as_str().unwrap(), None)),
    ))
}

#[test]
fn array_offset_accessories_preserve_values_and_compare_asymmetrically() {
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/has-offset-value.json")).unwrap();
    let mut types = Vec::new();
    for row in expected["types"].as_array().unwrap() {
        let ty = input(&row["input"]);
        assert_eq!(
            json!([Level::Precise, Level::Value, Level::TypeOnly].map(|level| ty.describe(level))),
            row["describe"],
            "{}",
            row["input"],
        );
        assert_eq!(
            json!(
                [
                    ty.is_array(),
                    ty.is_string(),
                    ty.is_list(),
                    ty.is_iterable(),
                    ty.is_iterable_at_least_once(),
                    ty.is_cloneable()
                ]
                .map(|answer| format!("{answer:?}"))
            ),
            row["queries"],
            "{}",
            row["input"],
        );
        types.push(ty);
    }
    for row in expected["comparisons"].as_array().unwrap() {
        let left = &types[row[0].as_u64().unwrap() as usize];
        let right = &types[row[1].as_u64().unwrap() as usize];
        assert_eq!(
            format!("{:?}", is_super_type_of(left, right)),
            row[2].as_str().unwrap(),
            "{left:?} >= {right:?}"
        );
    }
    let wrapped = intersect(vec![
        types[4].clone(),
        Type::Accessory(Accessory::NonEmptyArray),
        types[0].clone(),
    ]);
    assert_eq!(wrapped.describe(Level::Precise), expected["wrapped"]);
    assert_eq!(
        json!([to_phpdoc(&types[0]), to_phpdoc(&wrapped)]),
        expected["phpdoc"]
    );
    let mapped = types[0].traverse(&|ty| {
        if ty == &Type::String {
            Type::Int
        } else {
            Type::ConstInt(7)
        }
    });
    assert_eq!(mapped.describe(Level::Precise), expected["traverse"]);
}
