//! Public offset operations for objects that do not implement ArrayAccess.

use pun_phpstan::resolve::{resolve_late, resolve_str};
use pun_phpstan::types::{Level, Type};
use serde_json::{Value, json};

#[test]
fn non_array_access_objects_use_reflection_offset_certainty() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/config-object-offsets.json")).unwrap();
    for name in [
        "stdClass",
        "DateTimeInterface",
        "Generator",
        "SensitiveParameterValue",
        "ConfigShapeMissingObject",
        "DOMNodeList",
        "FFI\\CData",
    ] {
        for offset in [Type::ConstInt(0), Type::ConstString(b"key".to_vec())] {
            let value = resolve_late(&Type::OffsetAccess(
                Box::new(resolve_str(name, None)),
                Box::new(offset.clone()),
            ));
            let actual = json!({
                "type": name,
                "offset": offset.describe(Level::Precise),
                "value": value.describe(Level::Precise),
                "array": format!("{:?}", value.is_array()),
                "explicit": match value {
                    Type::Mixed { explicit } => Some(explicit),
                    Type::Error => Some(false),
                    _ => None,
                },
            });
            let expected = oracle["rows"]
                .as_array()
                .unwrap()
                .iter()
                .find(|row| row["type"] == name && row["offset"] == actual["offset"])
                .unwrap();
            assert_eq!(actual, *expected);
        }
    }
}

#[test]
fn unresolved_array_access_keeps_its_receiver_and_offset() {
    // The retained oracle also records the four ArrayAccess/ArrayObject observations. Resolving
    // their generic offsetGet signatures is a separate reflection prerequisite, not mixed.
    for name in ["ArrayAccess<int, string>", "ArrayObject<int, string>"] {
        for offset in [Type::ConstInt(0), Type::ConstString(b"key".to_vec())] {
            let input = Type::OffsetAccess(Box::new(resolve_str(name, None)), Box::new(offset));
            let value = resolve_late(&input);
            println!(
                "{}",
                json!({"unsupported": name, "value": value.describe(Level::Precise),
                    "array": format!("{:?}", value.is_array())})
            );
            assert_eq!(value, input);
        }
    }
}
