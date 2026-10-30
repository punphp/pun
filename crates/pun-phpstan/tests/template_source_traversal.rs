//! PHPStan 2.3.1 cb3b7e99: equal replacements are not retained source nodes.

use pun_phpstan::types::{Level, Template, Traversal, Type};
use serde_json::Value;

#[test]
fn traversal_keeps_explicit_source_decisions_without_using_equality() {
    let oracle: Value = serde_json::from_str(include_str!(
        "fixtures/template-source-identity/transformations.json"
    ))
    .unwrap();
    let original = Type::Template(Box::new(Template {
        name: "T".into(),
        scope: "function fixture()".into(),
        bound: Type::Union(vec![Type::Int, Type::String]),
        default: Some(Type::ConstInt(1)),
        argument: false,
    }));
    for row in oracle.as_array().unwrap() {
        let operation = row["operation"].as_str().unwrap();
        if !matches!(
            operation,
            "traverse-identity"
                | "traverse-fresh-equal-bound"
                | "traverse-fresh-equal-default"
                | "traverse-changed-bound"
        ) {
            continue;
        }
        let mapped = original.traverse_source(&|inner| match (operation, inner) {
            ("traverse-fresh-equal-bound", Type::Union(_)) => {
                Traversal::Replace(Type::Union(vec![Type::Int, Type::String]))
            }
            ("traverse-changed-bound", Type::Union(_)) => Traversal::Replace(Type::String),
            ("traverse-fresh-equal-default", Type::ConstInt(_)) => {
                Traversal::Replace(Type::ConstInt(1))
            }
            _ => Traversal::Retain,
        });
        assert_eq!(
            matches!(mapped, Traversal::Retain),
            row["sameInstance"].as_bool().unwrap(),
            "{row}"
        );
        assert_eq!(
            mapped.into_type(&original) == original,
            row["equal"].as_bool().unwrap(),
            "{row}"
        );
    }
    let scalar = Type::Int;
    let mapped = scalar.traverse_source(&|_| Traversal::Replace(Type::String));
    assert!(matches!(mapped, Traversal::Retain));
    assert_eq!(mapped.into_type(&scalar), Type::Int);

    let legacy = original.traverse(&|inner| match inner {
        Type::Union(_) => Type::String,
        t => t.clone(),
    });
    let changed = oracle
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["operation"] == "traverse-changed-bound")
        .unwrap();
    assert_eq!(legacy.describe(Level::Precise), changed["candidate"][0]);
}
