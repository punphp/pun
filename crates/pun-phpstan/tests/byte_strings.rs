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

fn shape_keys(ty: &Type) -> Value {
    let arrays = ty.constant_arrays();
    assert_eq!(arrays.len(), 1);
    json!(
        arrays[0]
            .keys
            .iter()
            .zip(&arrays[0].values)
            .map(|(key, value)| {
                let key = match key {
                    Type::ConstInt(i) => json!(["int", i]),
                    Type::ConstString(s) => json!(["string", hex(s)]),
                    _ => panic!("nonconstant shape key: {key:?}"),
                };
                let Type::ConstString(value) = value else {
                    panic!("nonconstant shape value: {value:?}");
                };
                json!([key, hex(value)])
            })
            .collect::<Vec<_>>()
    )
}

#[test]
fn quoted_shape_keys_preserve_bytes_and_overwrite_only_equal_keys() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/byte-strings/expected.json")).unwrap();
    for row in oracle["shape_keys"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let ty = pun_phpstan::resolve::resolve_str(input, None);
        assert_eq!(shape_keys(&ty), row["entries"], "{input}");
    }
}

#[test]
fn late_quoted_shape_keys_preserve_bytes_through_template_substitution() {
    use pun_phpstan::resolve::{Scope, resolve_in};
    use pun_phpstan::types::{Template, infer_template_types, resolve_template_types};

    struct TemplateScope(Type);
    impl Scope for TemplateScope {
        fn parent(&self) -> Option<String> {
            None
        }
        fn template(&self, name: &str) -> Option<Type> {
            (name == "K").then(|| self.0.clone())
        }
    }
    let template = Type::Template(Box::new(Template {
        name: "K".into(),
        scope: "function shapeBytes()".into(),
        bound: Type::String,
        default: None,
        argument: false,
    }));
    let scope = TemplateScope(template.clone());
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/byte-strings/expected.json")).unwrap();
    for row in oracle["late_shape_keys"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let node = pun_phpstan::phpdoc::parse(input).unwrap();
        let ty = resolve_in(&node, &scope);
        assert!(matches!(ty, Type::LateArrayShape(_)), "{input}");
        let replacement = Type::ConstString(bytes(&row["replacement"]));
        let map = infer_template_types(&template, &replacement);
        let resolved = resolve_template_types(&ty, &map);
        assert_eq!(
            shape_keys(&resolved),
            row["entries"],
            "{input}; replacement {}",
            row["replacement"]
        );
    }
}
