//! TemplateTypeTrait::equals() at PHPStan 2.3.1 cb3b7e99 ignores the inference strategy, not template identity.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{Accessory, Level, Template, Type, is_super_type_of};
use serde_json::Value;

fn template(scope: &str, name: &str, bound: &str, default: Option<&str>, argument: bool) -> Type {
    Type::Template(Box::new(Template {
        scope: scope.into(),
        name: name.into(),
        bound: resolve_str(bound, None),
        default: default.map(|t| resolve_str(t, None)),
        argument,
    }))
}

#[test]
fn template_identity_ignores_strategy_but_keeps_scope_name_bound_and_default() {
    let rows: Value =
        serde_json::from_str(include_str!("fixtures/type-template-identity.json")).unwrap();
    let base = template("class Box", "T", "int", Some("1"), false);
    for row in rows.as_array().unwrap() {
        let other = template(
            row["scope"].as_str().unwrap(),
            row["name"].as_str().unwrap(),
            row["bound"].as_str().unwrap(),
            row["default"].as_str(),
            row["argument"].as_bool().unwrap(),
        );
        assert_eq!(base == other, row["equals"].as_bool().unwrap(), "{row}");
        if row["equals"].as_bool().unwrap() {
            // A generic's invariant arguments must not reject the same template after toArgument().
            let object = |t| Type::Object("ArrayObject".into(), vec![Type::Int, t]);
            assert_eq!(
                format!(
                    "{:?}",
                    is_super_type_of(&object(base.clone()), &object(other))
                ),
                row["genericSuper"].as_str().unwrap()
            );
        }
    }
}

/// PHPStan 2.3.1's TemplateTypeHelper::toArgument() preserves this change through IntersectionType::traverse().
#[test]
fn traversal_retains_strategy_changes_even_when_template_identity_is_equal() {
    let array = |argument| {
        Type::array(
            Type::Int,
            template("function fixture()", "T", "int", None, argument),
        )
    };
    let original = Type::Intersection(vec![
        array(false),
        Type::Accessory(Accessory::NonEmptyArray),
    ]);
    let mapped = original.traverse(&|inner| match inner {
        Type::Array(..) => array(true),
        _ => inner.clone(),
    });
    assert_eq!(
        original.describe(Level::Precise),
        "non-empty-array<int, T of int (function fixture(), parameter)>"
    );
    assert_eq!(
        mapped.describe(Level::Precise),
        "non-empty-array<int, T of int (function fixture(), argument)>"
    );
}
