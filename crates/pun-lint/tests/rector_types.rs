//! Rector's own fixtures for the ported `types` rules, copied verbatim from rector-src 2.6.1 (`.oracle/rector`,
//! rules-tests/...) into tests/rector/<Rector class>/Fixture, with the classes they use in Source/. Oracle/ holds extra
//! inputs with what Rector 2.6.1 (`.oracle/rector-bin`, the rule alone, `phpVersion(80400)`) wrote for them; Pun/
//! holds the cases where Pun keeps the meaning but not Rector's bytes, its expected output derived by hand. Each one
//! must pass as `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits
//! must give the expected bytes before format too.

mod lint_c;

use lint_c::{run_class, run_class_at, run_class_stubs, run_oracle_class};
use pun_lint::Rule;
use pun_lint::rules::{
    AddArrayReturn, AddArrowFunctionReturn, AddBoolReturn, AddBoolReturnFromConst,
    AddClosureNeverReturn, AddClosureReturn, AddClosureVoidReturn, AddFluentReturn,
    AddFunctionVoidReturn, AddMagicMethodTypes, AddMethodVoidReturn, AddNeverReturn,
    AddNullableReturn, AddNumericReturn, AddDoctrineRepositoryReturnType, AddPropertyTypeAssigns,
    AddPropertyTypeBehatContext, AddPropertyTypeGetRepository, AddPropertyTypeConstructor,
    AddPropertyTypeJmsObject, AddPropertyTypeJmsScalar, AddPropertyTypeSetterGetter, AddNumericReturnFromScalar, AddReturnTypeFromCast,
    AddReturnTypeFromConstant, AddReturnTypeFromNativeCall, AddReturnTypeFromNew,
    AddReturnTypeFromParam, AddReturnTypeFromParent, AddReturnTypeFromProperty,
    AddReturnTypeFromSymfonySerializer, AddReturnTypeFromTernary, AddReturnTypeFromTypedCall,
    AddReturnTypeFromYield, AddStringReturn, AddStringReturnFromScalar, AddUnionReturn,
    AddReturnTypeFromConstant, AddReturnTypeFromNew, AddReturnTypeFromParam,
    AddReturnTypeFromProperty, AddReturnTypeFromTernary, AddStringReturn,
    AddStringReturnFromScalar, AddUnionReturn, NoNullsafeOnNonNull, PreferIdenticalSameType,
    PreferInstanceof,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn add_function_void_return_type_where_no_return() {
    check(
        AddFunctionVoidReturn,
        "AddFunctionVoidReturnTypeWhereNoReturnRector",
    );
}

#[test]
fn add_closure_void_return_type_where_no_return() {
    check(
        AddClosureVoidReturn,
        "AddClosureVoidReturnTypeWhereNoReturnRector",
    );
}

#[test]
fn add_void_return_type_where_no_return() {
    check(AddMethodVoidReturn, "AddVoidReturnTypeWhereNoReturnRector");
}

#[test]
fn return_never_type() {
    check(AddNeverReturn, "ReturnNeverTypeRector");
}

#[test]
fn add_closure_never_return_type() {
    check(AddClosureNeverReturn, "AddClosureNeverReturnTypeRector");
}

#[test]
fn return_type_from_return_cast() {
    check(AddReturnTypeFromCast, "ReturnTypeFromReturnCastRector");
}

#[test]
fn known_magic_class_method_type() {
    check(AddMagicMethodTypes, "KnownMagicClassMethodTypeRector");
}

#[test]
fn bool_return_type_from_boolean_const_returns() {
    check(
        AddBoolReturnFromConst,
        "BoolReturnTypeFromBooleanConstReturnsRector",
    );
}

#[test]
fn return_type_from_return_new() {
    check(AddReturnTypeFromNew, "ReturnTypeFromReturnNewRector");
}

#[test]
fn return_type_from_return_new_php74() {
    run_class_at(
        vec![Box::new(AddReturnTypeFromNew)],
        "ReturnTypeFromReturnNewRectorPhp74",
        70400,
    );
}

#[test]
fn string_return_type_from_strict_scalar_returns() {
    check(
        AddStringReturnFromScalar,
        "StringReturnTypeFromStrictScalarReturnsRector",
    );
}

#[test]
fn numeric_return_type_from_strict_scalar_returns() {
    check(
        AddNumericReturnFromScalar,
        "NumericReturnTypeFromStrictScalarReturnsRector",
    );
}

#[test]
fn bool_return_type_from_boolean_strict_returns() {
    check(
        AddBoolReturn,
        "BoolReturnTypeFromBooleanStrictReturnsRector",
    );
}

#[test]
fn return_type_from_return_direct_array() {
    check(AddArrayReturn, "ReturnTypeFromReturnDirectArrayRector");
}

#[test]
fn string_return_type_from_strict_string_returns() {
    check(
        AddStringReturn,
        "StringReturnTypeFromStrictStringReturnsRector",
    );
}

#[test]
fn numeric_return_type_from_strict_returns() {
    check(AddNumericReturn, "NumericReturnTypeFromStrictReturnsRector");
}

#[test]
fn return_type_from_strict_fluent_return() {
    check(AddFluentReturn, "ReturnTypeFromStrictFluentReturnRector");
}

#[test]
fn return_type_from_strict_fluent_return_php74() {
    run_class_at(
        vec![Box::new(AddFluentReturn)],
        "ReturnTypeFromStrictFluentReturnRectorPhp74",
        70400,
    );
}

#[test]
fn return_type_from_strict_param() {
    check(AddReturnTypeFromParam, "ReturnTypeFromStrictParamRector");
}

#[test]
fn return_type_from_strict_typed_property() {
    check(
        AddReturnTypeFromProperty,
        "ReturnTypeFromStrictTypedPropertyRector",
    );
}

#[test]
fn return_type_from_strict_constant_return() {
    check(
        AddReturnTypeFromConstant,
        "ReturnTypeFromStrictConstantReturnRector",
    );
    for (dir, version) in [
        ("ReturnTypeFromStrictConstantReturnRectorPhp80100", 80100),
        ("ReturnTypeFromStrictConstantReturnRectorPhp70400", 70400),
    ] {
        run_class_at(vec![Box::new(AddReturnTypeFromConstant)], dir, version);
    }
}

#[test]
fn closure_return_type() {
    run_class_at(
        vec![Box::new(AddClosureReturn)],
        "ClosureReturnTypeRector",
        80200,
    );
}

#[test]
fn add_arrow_function_return_type() {
    run_class_at(
        vec![Box::new(AddArrowFunctionReturn)],
        "AddArrowFunctionReturnTypeRector",
        80000,
    );
}

#[test]
fn return_type_from_strict_ternary() {
    run_class_at(
        vec![Box::new(AddReturnTypeFromTernary)],
        "ReturnTypeFromStrictTernaryRector",
        70100,
    );
}

#[test]
fn return_nullable_type() {
    for (dir, version) in [
        ("ReturnNullableTypeRector", 70100),
        ("ReturnNullableTypeRectorPhp82", 80200),
    ] {
        run_class_at(vec![Box::new(AddNullableReturn)], dir, version);
    }
}

#[test]
fn return_union_type() {
    for (dir, version) in [
        ("ReturnUnionTypeRector", 80000),
        ("ReturnUnionTypeRectorPhp81", 80100),
        ("ReturnUnionTypeRectorTrueInUnion", 80200),
        ("ReturnUnionTypeRectorUnionIntersection", 80200),
    ] {
        run_class_at(vec![Box::new(AddUnionReturn)], dir, version);
    }
}

#[test]
fn return_type_from_strict_native_call() {
    run_class_at(
        vec![Box::new(AddReturnTypeFromNativeCall)],
        "ReturnTypeFromStrictNativeCallRector",
        80200,
    );
}

#[test]
fn return_type_from_strict_typed_call() {
    for (dir, version) in [
        ("ReturnTypeFromStrictTypedCallRector", 70400),
        ("ReturnTypeFromStrictTypedCallRectorPhp80", 80000),
    ] {
        // rector-src runs these with PHPUnit installed (Fixture/return_in_test_method.php.inc)
        run_class_stubs(
            vec![Box::new(AddReturnTypeFromTypedCall)],
            dir,
            version,
            &["phpunit-stubs"],
        );
    }
}

#[test]
fn add_return_type_declaration_from_yields() {
    check(
        AddReturnTypeFromYield,
        "AddReturnTypeDeclarationFromYieldsRector",
fn inline_is_a_instance_of() {
    check(PreferInstanceof, "InlineIsAInstanceOfRector");
}

#[test]
fn use_identical_over_equal_with_same_type() {
    check(
        PreferIdenticalSameType,
        "UseIdenticalOverEqualWithSameTypeRector",
    );
}

#[test]
fn add_return_type_declaration_based_on_parent_class_method() {
    run_class_at(
        vec![Box::new(AddReturnTypeFromParent)],
        "AddReturnTypeDeclarationBasedOnParentClassMethodRector",
        80000,
    );
}

#[test]
fn return_type_from_symfony_serializer() {
    run_class_at(
        vec![Box::new(AddReturnTypeFromSymfonySerializer)],
        "ReturnTypeFromSymfonySerializerRector",
        70000,
fn cleanup_unneeded_nullsafe_operator() {
    check(
        NoNullsafeOnNonNull,
        "CleanupUnneededNullsafeOperatorRector",
    );
}

#[test]
fn closure_return_type_intersection_below_dnf() {
    // Oracle/closure_dnf.php.inc below PHP 8.2: .oracle/rector-bin (ClosureReturnTypeRector alone) leaves the closure
    // alone at 8.1 (no DNF types) and at 8.0 (no intersection types).
    let input = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/rector/ClosureReturnTypeRector/Oracle/closure_dnf.php.inc"
    ))
    .unwrap();
    let input = input.split_once("-----\n").unwrap().0;
    for version in [80000, 80100] {
        let linter = pun_lint::Linter::new(vec![Box::new(AddClosureReturn)], version);
        let fixed = linter
            .fix(std::path::Path::new("closure_dnf.php"), input)
            .unwrap();
        assert_eq!(fixed.source, input, "PHP {version}");
    }
}

#[test]
fn left_alone_at_other_versions() {
    // Inputs .oracle/rector-bin (the one Rector rule alone) leaves unchanged at a version the fixture directories do
    // not run: an anonymous class below PHP 7.2 (no `object`), and a ternary with a `mixed` branch at PHP 8.0.
    let cases: [(Box<dyn Rule>, u32, &str); 3] = [
        (
            Box::new(AddReturnTypeFromNew),
            70100,
            "<?php\n\nfunction make()\n{\n    return new class {\n    };\n}\n",
        ),
        (
            Box::new(AddClosureReturn),
            70100,
            "<?php\n\n$f = function () {\n    return new class {\n    };\n};\n",
        ),
        (
            Box::new(AddReturnTypeFromTernary),
            80000,
            "<?php\n\nfunction f(bool $x, mixed $a)\n{\n    return $x ? $a : $a;\n}\n",
        ),
    ];
    for (rule, version, input) in cases {
        let linter = pun_lint::Linter::new(vec![rule], version);
        let fixed = linter.fix(std::path::Path::new("f.php"), input).unwrap();
        assert_eq!(fixed.source, input, "PHP {version}");
    }
}

#[test]
fn typed_property_from_strict_constructor() {
    // INTERSECTION_TYPES, as the fixtures' config sets
    run_class_stubs(
        vec![Box::new(AddPropertyTypeConstructor)],
        "TypedPropertyFromStrictConstructorRector",
        80100,
        &["doctrine-stubs"],
    );
}

#[test]
fn typed_property_from_assigns() {
    // config/configured_rule.php: TYPED_PROPERTIES
    run_class_stubs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRector",
        70400,
        &["php-parser-stubs", "phpunit-stubs"],
    );
}

#[test]
fn typed_property_from_assigns_complex_types() {
    // config/complex_types.php: INTERSECTION_TYPES
    run_class_stubs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRectorComplexTypes",
        80100,
        &["phpunit-stubs"],
    );
}

#[test]
fn typed_property_from_assigns_php82() {
    // config/configured_rule_php82.php: NULL_FALSE_TRUE_STANDALONE_TYPE
    run_class_at(vec![Box::new(AddPropertyTypeAssigns::default())], "TypedPropertyFromAssignsRectorPhp82", 80200);
}

#[test]
fn typed_property_from_assigns_inline_public() {
    // config/inline_public_configured_rule.php: inline_public, INTERSECTION_TYPES
    let mut rule = AddPropertyTypeAssigns::default();
    rule.configure(&serde_json::json!({"inline-public": true})).unwrap();
    run_class_at(vec![Box::new(rule)], "TypedPropertyFromAssignsRectorInlinePublic", 80100);
}

#[test]
fn property_type_from_strict_setter_getter() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version
    run_class(vec![Box::new(AddPropertyTypeSetterGetter)], "PropertyTypeFromStrictSetterGetterRector");
}

#[test]
fn scalar_typed_property_from_jms_serializer_attribute_type() {
    // config/configured_rule.php: ATTRIBUTES
    run_class_at(vec![Box::new(AddPropertyTypeJmsScalar)], "ScalarTypedPropertyFromJMSSerializerAttributeTypeRector", 80000);
}

#[test]
fn object_typed_property_from_jms_serializer_attribute_type() {
    // config/configured_rule.php: ATTRIBUTES
    run_class_at(vec![Box::new(AddPropertyTypeJmsObject)], "ObjectTypedPropertyFromJMSSerializerAttributeTypeRector", 80000);
}

#[test]
fn typed_static_property_in_behat_context() {
    // No upstream fixtures: Oracle/ only, Rector run at PHP 8.0 with Source/ autoloaded.
    run_oracle_class(vec![Box::new(AddPropertyTypeBehatContext)], "TypedStaticPropertyInBehatContextRector", 80000);
}

#[test]
fn typed_property_from_get_repository_set_up() {
    // config/configured_rule.php: TYPED_PROPERTIES
    run_class_stubs(
        vec![Box::new(AddPropertyTypeGetRepository)],
        "TypedPropertyFromGetRepositorySetUpRector",
        70400,
        &["phpunit-stubs"],
    );
}
