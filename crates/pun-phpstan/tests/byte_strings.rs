//! PHPStan ConstantStringType stores arbitrary bytes, independently of UTF-8 diagnostic presentation.

use pun_phpstan::types::{Accessory, Level, Type, accepts, is_super_type_of, union};
use serde_json::{Value, json};

fn bytes(hex: &Value) -> Vec<u8> {
    hex.as_str()
        .unwrap()
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn byte_identity_extraction_accessories_and_unions_match_phpstan() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/byte-strings/expected.json")).unwrap();
    for row in oracle["values"].as_array().unwrap() {
        let ty = Type::ConstString(bytes(&row["hex"]));
        let quoted = format!(
            "\"{}\"",
            bytes(&row["hex"])
                .iter()
                .map(|byte| format!("\\x{byte:02x}"))
                .collect::<String>()
        );
        assert_eq!(pun_phpstan::resolve::resolve_str(&quoted, None), ty);
        assert_eq!(
            json!(
                ty.constant_strings()
                    .iter()
                    .map(|s| hex(s))
                    .collect::<Vec<_>>()
            ),
            row["constants"]
        );
        let queries: Vec<_> = [
            Accessory::NonEmptyString,
            Accessory::NonFalsyString,
            Accessory::NumericString,
            Accessory::LowercaseString,
            Accessory::UppercaseString,
        ]
        .into_iter()
        .map(|accessory| is_super_type_of(&Type::Accessory(accessory), &ty).yes())
        .collect();
        assert_eq!(json!(queries), row["queries"], "{}", row["hex"]);
        assert_eq!(
            ty.describe(Level::Precise),
            row["description"].as_str().unwrap()
        );
        assert_eq!(
            ty.describe(Level::Value),
            row["value_description"].as_str().unwrap()
        );
    }
    for row in oracle["pairs"].as_array().unwrap() {
        let a = Type::ConstString(bytes(&row[0]));
        let b = Type::ConstString(bytes(&row[1]));
        assert_eq!(json!(a == b), row[2]);
        assert_eq!(json!(is_super_type_of(&a, &b).yes()), row[3]);
        assert_eq!(json!(accepts(&a, &b, true).yes()), row[4]);
        let constants: Vec<_> = union(vec![a, b])
            .constant_strings()
            .iter()
            .map(|s| hex(s))
            .collect();
        assert_eq!(json!(constants), row[5]);
    }
    for row in oracle["escapes"].as_array().unwrap() {
        let quoted = String::from_utf8(bytes(&row[0])).unwrap();
        assert_eq!(
            pun_phpstan::resolve::resolve_str(&quoted, None),
            Type::ConstString(bytes(&row[1])),
            "{quoted}"
        );
    }
    // ThisType keeps textual class identity when ConstantStringType switches to byte identity.
    for row in oracle["identity_neighbors"].as_array().unwrap() {
        let a = Type::This(row[0].as_str().unwrap().to_owned());
        let b = Type::This(row[1].as_str().unwrap().to_owned());
        assert_eq!(json!(a == b), row[2]);
    }
    // A non-string alternative must not become a constant method name during byte conversion.
    for row in oracle["callable_neighbors"].as_array().unwrap() {
        let callable =
            pun_phpstan::resolve::resolve_str(row[0].as_str().unwrap(), None).is_callable();
        assert_eq!(
            json!([callable.yes(), callable.maybe(), callable.no()]),
            row[1]
        );
    }
}
