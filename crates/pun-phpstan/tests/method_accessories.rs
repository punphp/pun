//! PHPStan 2.3.1 HasMethodType and MethodExistsTypeSpecifyingExtension oracle values. The fixture constructs
//! HasMethodType for `accessory`; otherwise it intersects the base with the extension's truthy specification.

use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{
    Accessory, Cast, Level, StrictMixed, Type, accepts, cast_is_error, is_super_type_of, to_phpdoc,
};
use serde_json::{Value, json};

fn accessory(name: &str) -> Type {
    Type::Accessory(Accessory::HasMethod(name.to_string()))
}

#[test]
fn method_accessories_match_phpstan_type_operations() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/type-method-accessories.json")).unwrap();
    for row in oracle.as_array().unwrap() {
        let base = row["base"].as_str().unwrap();
        let method = row["method"].as_str().unwrap();
        let ty = match base {
            "accessory" => accessory(method),
            "static-string" => {
                Type::ClassString(Some(Box::new(Type::Static("Exception".into(), vec![]))))
                    .require_method(method)
            }
            _ => resolve_str(base, None).require_method(method),
        };
        let describe = [Level::TypeOnly, Level::Value, Level::Precise].map(|l| ty.describe(l));
        let tri = |t| format!("{t:?}");
        let h = accessory("mixed");
        let other = accessory("other");
        let got = json!({
            "base": base, "method": method, "describe": describe, "phpdoc": to_phpdoc(&ty),
            "has": (["mixed", "other"].map(|n| tri(ty.has_method(n)))),
            "queries": ([ty.is_string(), ty.is_array(), ty.is_iterable(), ty.is_cloneable(), ty.is_callable()].map(tri)),
            "super": ([is_super_type_of(&h, &ty), is_super_type_of(&ty, &h), is_super_type_of(&other, &ty)].map(tri)),
            "accepts": ([accepts(&h, &ty, true), accepts(&ty, &h, true)].map(tri)),
            "casts": ([Cast::String, Cast::Number, Cast::Integer, Cast::Float, Cast::Boolean, Cast::BitwiseNot]
                .map(|c| cast_is_error(&ty, c, StrictMixed::default()))),
        });
        assert_eq!(&got, row, "{base}, {method}: {ty:?}");
    }
    // HasMethodType::equals() uses the canonical method name, not the description's spelling.
    assert_eq!(accessory("MiXeD"), accessory("mixed"));
    assert_ne!(accessory("MiXeD"), accessory("other"));

    // PHPStan's IntersectionType([HasMethodType('MiXeD'), ClassStringType]) intersected with
    // HasMethodType('Other'): operand order must not turn the class-string refinement into an object.
    let commuted = Type::Intersection(vec![accessory("MiXeD"), Type::ClassString(None)])
        .require_method("Other");
    assert_eq!(
        commuted.describe(Level::Precise),
        "class-string&hasMethod(MiXeD)&hasMethod(Other)"
    );
}
