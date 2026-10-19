//! pun_phpstan::phpdoc against phpdoc-parser itself (scripts/export-phpdoc-parser-fixtures.php): every tag of each
//! doc comment with its start line, and the ParserException message of an invalid value or type alias; the
//! TypeStringResolver parse step's error on type strings.

use pun_phpstan::phpdoc::{self, TagValue};
use serde_json::Value;

fn fixture() -> Value {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/phpdoc-tags.json"
    );
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn phpdoc_parser_tags() {
    let fixture = fixture();
    let mut failures = Vec::new();
    for case in fixture["docs"].as_array().unwrap() {
        let doc = case["doc"].as_str().unwrap();
        let got: Value = phpdoc::tags(doc).map_or(Value::Null, |tags| {
            tags.into_iter()
                .map(|tag| {
                    let (value, message) = match tag.value {
                        TagValue::Invalid(value, e) => {
                            (Value::from(value), Value::from(e.message()))
                        }
                        TagValue::TypeAlias(_, Err(e)) => (Value::Null, Value::from(e.message())),
                        _ => (Value::Null, Value::Null),
                    };
                    serde_json::json!([tag.name, tag.line, value, message])
                })
                .collect()
        });
        if got != case["tags"] {
            failures.push(format!("{doc:?}\n  want {}\n  got  {got}", case["tags"]));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failure(s):\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn type_string_errors() {
    for case in fixture()["types"].as_array().unwrap() {
        let input = case["type"].as_str().unwrap();
        let got = phpdoc::parse(input).err().map(|e| e.message());
        assert_eq!(got.as_deref(), case["error"].as_str(), "{input:?}");
    }
}
