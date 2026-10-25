//! Reason metadata is invisible to PHPStan type identity, queries, acceptance and descriptions.

use pun_phpstan::resolve::{Scope, resolve_in, resolve_str};
use pun_phpstan::types::{
    Cast, Level, NeverKind, ShapeBuilder, StrictMixed, Template, TemplateMap, Tri, Type, accepts,
    accepts_reasons, cast_is_error, infer_template_types, intersect, is_super_type_of, mixed,
    resolve_template_types, to_phpdoc, union,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[test]
fn error_and_never_reasons_preserve_phpstan_type_semantics() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/unresolvable-reasons.json")).unwrap();
    let types = BTreeMap::from([
        (
            "error",
            Type::Error.with_unresolvable_reason(
                "Type stdClass cannot be used as an array shape key.".into(),
            ),
        ),
        (
            "never",
            Type::Never(NeverKind::Implicit).with_unresolvable_reason("A missing property.".into()),
        ),
        (
            "explicit-never",
            Type::Never(NeverKind::Explicit)
                .with_unresolvable_reason("An explicit declaration.".into()),
        ),
        ("string", Type::String),
    ]);
    let tri = |t: Tri| format!("{t:?}");
    for (name, ty) in &types {
        let queries = json!({
            "isString": tri(ty.is_string()), "isArray": tri(ty.is_array()),
            "isIterable": tri(ty.is_iterable()), "isCloneable": tri(ty.is_cloneable()),
            "isCallable": tri(ty.is_callable()), "isFloat": tri(ty.is_float()),
            "isBoolean": tri(ty.is_boolean()), "isNull": tri(ty.is_null()), "isVoid": tri(ty.is_void()),
            "isList": tri(ty.is_list()), "isIterableAtLeastOnce": tri(ty.is_iterable_at_least_once()),
        });
        assert_eq!(queries, oracle[name]["queries"], "{name}");
        assert_eq!(json!(ty.unresolvable_reason()), oracle[name]["reason"]);
        assert_eq!(to_phpdoc(ty), oracle[name]["phpdoc"]);
        for (level, label) in [
            (Level::TypeOnly, "typeOnly"),
            (Level::Value, "value"),
            (Level::Precise, "precise"),
        ] {
            assert_eq!(ty.describe(level), oracle[name]["describe"][label]);
        }
        for (other_name, other) in &types {
            assert_eq!(
                tri(is_super_type_of(ty, other)),
                oracle[name]["super"][other_name]
            );
            assert_eq!(
                tri(accepts(ty, other, true)),
                oracle[name]["accepts"][other_name]
            );
            assert_eq!(
                json!(accepts_reasons(ty, other, true, StrictMixed::default())),
                oracle[name]["acceptsReasons"][other_name]
            );
        }
        for (label, cast) in [
            ("toString", Cast::String),
            ("toNumber", Cast::Number),
            ("toInteger", Cast::Integer),
            ("toFloat", Cast::Float),
            ("toBitwiseNotType", Cast::BitwiseNot),
        ] {
            assert_eq!(
                cast_is_error(ty, cast, StrictMixed::default()),
                oracle[name]["casts"][label],
                "{name} {label}"
            );
        }
        for (operation, want) in oracle[name]["operations"].as_object().unwrap() {
            let actual = match operation.as_str() {
                "union-string" => union(vec![ty.clone(), Type::String]),
                "union-self" => union(vec![ty.clone()]),
                "intersection-string" => intersect(vec![ty.clone(), Type::String]),
                "iterable-key" => ty.iterable_key_type(),
                "iterable-value" => ty.iterable_value_type(),
                _ => unreachable!(),
            };
            assert_eq!(
                json!([
                    actual.describe(Level::Precise),
                    actual.unresolvable_reason()
                ]),
                *want,
                "{name} {operation}"
            );
        }
    }
    assert_eq!(
        types["error"] == Type::Error,
        oracle["equality"]["error-reasonless"]
    );
    assert_eq!(
        types["error"] == Type::Error.with_unresolvable_reason("Another reason.".into()),
        oracle["equality"]["error-other-reason"]
    );
    assert_eq!(
        types["never"] == Type::Never(NeverKind::Implicit),
        oracle["equality"]["never-reasonless"]
    );
    assert_eq!(
        types["never"]
            == Type::Never(NeverKind::Implicit).with_unresolvable_reason("Another reason.".into()),
        oracle["equality"]["never-other-reason"]
    );
    assert_eq!(
        types["error"]
            .clone()
            .with_unresolvable_reason("Changed.".into())
            .unresolvable_reason(),
        Some("Changed.")
    );
    assert_eq!(
        Type::String.with_unresolvable_reason("Not an unresolvable type.".into()),
        Type::String
    );
    for row in oracle["parsed-equality"].as_array().unwrap() {
        let a = resolve_str(row[0].as_str().unwrap(), None);
        let b = resolve_str(row[1].as_str().unwrap(), None);
        assert_eq!(json!(a == b), row[2], "{row}");
    }
    for row in oracle["mixed-equality"].as_array().unwrap() {
        let a = Type::mixed_subtracted(
            row[0].as_bool().unwrap(),
            resolve_str(row[2].as_str().unwrap(), None),
        );
        let b = Type::mixed_subtracted(
            row[1].as_bool().unwrap(),
            resolve_str(row[3].as_str().unwrap(), None),
        );
        assert_eq!(json!(a == b), row[4], "{row}");
    }
}

struct Templates;

impl Scope for Templates {
    fn parent(&self) -> Option<String> {
        None
    }
    fn template(&self, name: &str) -> Option<Type> {
        ["K", "L"].contains(&name).then(|| {
            Type::Template(Box::new(Template {
                name: name.into(),
                scope: "function shape()".into(),
                bound: mixed(),
                default: None,
                argument: false,
            }))
        })
    }
}

#[test]
fn deferred_array_shape_keys_resolve_before_their_error_is_reported() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/unresolvable-reasons.json")).unwrap();
    for row in oracle["late"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let original = resolve_in(&pun_phpstan::phpdoc::parse(input).unwrap(), &Templates);
        let tri = |t: Tri| format!("{t:?}");
        assert_eq!(
            json!({"isArray":tri(original.is_array()), "isString":tri(original.is_string()), "isCloneable":tri(original.is_cloneable()), "isIterable":tri(original.is_iterable())}),
            row["queries"],
            "{row}"
        );
        assert_eq!(
            tri(is_super_type_of(&original, &Type::String)),
            row["superString"]
        );
        assert_eq!(
            tri(is_super_type_of(&Type::String, &original)),
            row["stringSuper"]
        );
        assert_eq!(
            tri(accepts(&original, &Type::String, true)),
            row["acceptsString"]
        );
        assert_eq!(
            tri(accepts(&Type::String, &original, true)),
            row["stringAccepts"]
        );
        assert_eq!(
            json!(accepts_reasons(
                &original,
                &Type::String,
                true,
                StrictMixed::default()
            )),
            row["acceptsStringReasons"]
        );
        assert_eq!(
            cast_is_error(&original, Cast::String, StrictMixed::default()),
            row["castStringError"]
        );
        // ErrorType's pre-existing iterable getters are outside the late-shape array contract.
        if row["queries"]["isArray"] == "Yes" {
            assert_eq!(
                original.iterable_key_type().describe(Level::Precise),
                row["iterableKey"],
                "{row}"
            );
        }
        let map = row["substitutions"].as_object().unwrap().iter().fold(
            TemplateMap::default(),
            |map, (name, ty)| {
                map.union(infer_template_types(
                    &Templates.template(name).unwrap(),
                    &resolve_str(ty.as_str().unwrap(), None),
                ))
            },
        );
        let resolved = resolve_template_types(&original, &map);
        assert_eq!(resolved.describe(Level::Precise), row["resolved"], "{row}");
        assert_eq!(to_phpdoc(&resolved), row["phpdoc"], "{row}");
        assert_eq!(
            json!(resolved.unresolvable_reason()),
            row["reason"],
            "{row}"
        );
        assert_eq!(original.describe(Level::Precise), row["original"]);
        assert_eq!(to_phpdoc(&original), row["originalPhpdoc"]);
    }
}

/// The analyser does not yet produce mixed~int for !is_int guards. Test the owning public type contract with
/// PHPStan's actual narrowed source type instead of silently discarding the missing flow information.
#[test]
fn offset_comparison_reasons_are_not_the_direct_mixed_intersection_fast_path() {
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/unresolvable-reasons.json")).unwrap();
    let not_int = Type::mixed_subtracted(false, Type::Int);
    let mut builder = ShapeBuilder::new();
    builder.set(
        Some(Type::ConstString("value".into())),
        not_int.clone(),
        false,
    );
    let offset = Type::OffsetAccess(
        Box::new(builder.get()),
        Box::new(Type::ConstString("value".into())),
    );
    let object_offset = Type::OffsetAccess(
        Box::new(Type::Object("ArrayObject".into(), vec![])),
        Box::new(Type::ConstString("value".into())),
    );
    for (label, members) in [
        ("direct-first", vec![not_int.clone(), Type::Int]),
        ("direct-last", vec![Type::Int, not_int]),
        ("offset-first", vec![offset.clone(), Type::Int]),
        ("offset-last", vec![Type::Int, offset]),
        (
            "object-offset-first",
            vec![object_offset.clone(), Type::Int],
        ),
        ("object-offset-last", vec![Type::Int, object_offset]),
    ] {
        let actual = intersect(members);
        assert_eq!(
            json!([
                actual.describe(Level::Precise),
                actual.unresolvable_reason()
            ]),
            oracle["mixed-intersections"][label],
            "{label}"
        );
    }
}
