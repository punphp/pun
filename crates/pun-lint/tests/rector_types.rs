//! Rector's own fixtures for the ported `types` rules, copied verbatim from rector-src 2.8.4 (`.oracle/rector`,
//! rules-tests/...) into tests/rector/<Rector class>/Fixture, with the classes they use in Source/. Oracle/ holds extra
//! inputs with what Rector 2.8.4 (`.oracle/rector-bin`, the rule alone, `phpVersion(80400)`) wrote for them; Pun/
//! holds the cases where Pun keeps the meaning but not Rector's bytes, its expected output derived by hand. Each one
//! must pass as `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits
//! must give the expected bytes before format too.
//!
//! ReturnTypeFromSymfonySerializerRector/Pun/serializer_broken.php.inc holds the class arguments where Rector writes a
//! return type that does not parse or names no class: `'\App\Holder'` (Rector: `\\App\Holder`), `Holder::CLASS`
//! (`\App\Holder::CLASS`), `Holder::{$c}` (`\App\Holder::c`) and `self::class` in an anonymous class (Rector's
//! internal `\AnonymousClass<hash>`). Pun writes `\App\Holder` for the first two and leaves the other two alone.
//!
//! ReturnTypeFromStrictNativeCallRector/Pun/unreachable_call.php.inc and ReturnTypeFromStrictTernaryRector/Pun/
//! ternary_unreached.php.inc: Rector types a `return strlen($s)` or `return $x ? 1 : 2` after an `exit` with the
//! scope before it and writes `int`; Mago's analysis does not reach it, so Pun leaves the method alone.
//!
//! TypedPropertyFromStrictSetUpRector/Pun/aliased_var.php.inc: Rector writes a `@var` that names a class that does
//! not exist where the `@var` names an imported alias (`@var Clock` for `use DateTime as Clock` becomes
//! `(\PHPUnit\Framework\MockObject\MockObject & \Clock)`, and an alias of `MockObject` becomes `& \Mock`), and
//! `private int $x = null;` for a `null` default, which PHP rejects. Pun resolves the alias (and leaves an alias of
//! `MockObject` alone), and leaves a property with a `null` default untyped when its type does not allow `null`.
//!
//! AddReturnTypeDeclarationBasedOnParentClassMethodRector/Pun/parent_cycle.php.inc: two classes that extend each
//! other, which Rector stops on ("Circular reference to class"); Pun leaves both alone.
//!
//! ReturnTypeFromReturnDirectArrayRector/Pun/byref_dynamic.php.inc: `$matches` of a `preg_match()` whose pattern is not
//! a constant string, natively `array<string>|null` (the signature map's type), so `b()` stays as Rector leaves it.
//! After `array_shift($keys)` PHPStan 2.3's native scope types it `*ERROR*` and Rector leaves `getExplicitKeys()` alone
//! too; Pun's native scope has `array` there and writes `: array` as for the constant pattern of `c()`.
//!
//! ClosureReturnTypeRector/Pun/iterable_member.php.inc: a `\Countable` returned under `is_iterable()`. PHPStan types it
//! `Countable&iterable` and Rector 2.8.4 writes `: \Countable` (`NativePhpTypeNodeHelper::intersection()` drops the
//! `iterable`); Mago narrows it to `never`, so Pun leaves the closure alone.

mod lint_c;

use lint_c::{run_class, run_class_at, run_class_stubs, run_oracle_class};
use pun_lint::Rule;
use pun_lint::rules::{
    AddArrayReturn, AddArrowFunctionReturn, AddBoolReturn, AddBoolReturnFromConst,
    AddClosureNeverReturn, AddClosureReturn, AddClosureVoidReturn, AddDoctrineRepositoryReturnType,
    AddFluentReturn, AddFunctionVoidReturn, AddMagicMethodTypes, AddMethodVoidReturn,
    AddNeverReturn, AddNullableReturn, AddNumericReturn, AddNumericReturnFromScalar,
    AddPropertyTypeAssigns, AddPropertyTypeBehatContext, AddPropertyTypeConstructor,
    AddPropertyTypeContainerGet, AddPropertyTypeCreateMock, AddPropertyTypeGetRepository,
    AddPropertyTypeJmsObject, AddPropertyTypeJmsScalar, AddPropertyTypeSetterGetter,
    AddPropertyTypeSetup, AddPropertyTypeSetupDocblock, AddReturnTypeFromCast,
    AddReturnTypeFromConstant, AddReturnTypeFromNativeCall, AddReturnTypeFromNew,
    AddReturnTypeFromParam, AddReturnTypeFromParent, AddReturnTypeFromProperty,
    AddReturnTypeFromSymfonySerializer, AddReturnTypeFromTernary, AddReturnTypeFromTryCatch, AddReturnTypeFromTypedCall,
    AddReturnTypeFromYield, AddStringReturn, AddStringReturnFromScalar, AddUnionReturn,
    NoNullsafeOnNonNull, PreferIdenticalSameType, PreferInstanceof,
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
fn add_return_type_from_try_catch_type() {
    run_class_at(
        vec![Box::new(AddReturnTypeFromTryCatch)],
        "AddReturnTypeFromTryCatchTypeRector",
        80400,
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

/// ReturnTypeFromStrictTypedCallRectorPhp80/Pun/redundant_object: a function returning a class's call or an
/// `object` one; Rector 2.8.4 joins the two declarations into `\Item|object`, which PHP rejects (a class beside
/// `object` is redundant), and Pun leaves the function alone (lint.md: meaning over bytes).
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
fn return_type_from_strict_typed_call_never_is_not_propagated() {
    // Rector 2.8.4's NeverTypeMapper returns no declaration, even at PHP versions that support `never`.
    // Fresh captures at 7.4, 8.0, 8.1 and 8.4 are identical: only the int-returning control changes.
    let fixture =
        include_str!("rector/ReturnTypeFromStrictTypedCallRector/Oracle/contract_never.php.inc");
    let (input, expected) = fixture.split_once("-----\n").unwrap();
    for version in [80100, 80400] {
        let linter = pun_lint::Linter::new(vec![Box::new(AddReturnTypeFromTypedCall)], version);
        let path = std::path::Path::new("contract_never.php");
        let problems = linter.check(path, input).unwrap();
        assert_eq!(problems.len(), 1, "PHP {version}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(problem.rule, "lint/types/add-return-type-from-typed-call");
        assert_eq!(problem.path, path);
        assert_eq!(&input[problem.span.start..problem.span.end], "numbered");
        let fix = problem.fix.as_ref().unwrap();
        assert_eq!(fix.safety, pun_problem::Safety::Unsafe);
        assert_eq!(fix.edits.len(), 1);
        let edit = &fix.edits[0];
        let at = input.find("numbered()").unwrap() + "numbered()".len();
        assert_eq!(edit.span, pun_problem::Span::new(at, at));
        assert_eq!(edit.text, ": int");
        let fixed = linter.fix(path, input).unwrap();
        assert_eq!(fixed.source, expected, "PHP {version}");
        assert_eq!(fixed.applied, problems);
        assert!(linter.check(path, expected).unwrap().is_empty());
    }
}

#[test]
fn add_return_type_declaration_from_yields() {
    check(
        AddReturnTypeFromYield,
        "AddReturnTypeDeclarationFromYieldsRector",
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
fn cleanup_unneeded_nullsafe_operator() {
    check(NoNullsafeOnNonNull, "CleanupUnneededNullsafeOperatorRector");
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
fn typed_property_from_assigns_native_options() {
    let mut rule = AddPropertyTypeAssigns::default();
    assert_eq!(
        rule.configure(&serde_json::json!({"inline_public": true})),
        Err("unknown option `inline_public` or a value of the wrong type".to_string()),
    );
    assert_eq!(
        rule.configure(&serde_json::json!({"inline-public": true})),
        Ok(()),
    );
    run_class_at(
        vec![Box::new(rule)],
        "TypedPropertyFromAssignsRectorInlinePublic",
        80100,
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
    run_class_at(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRectorPhp82",
        80200,
    );
}

#[test]
fn typed_property_from_assigns_inline_public() {
    // config/inline_public_configured_rule.php: inline_public, INTERSECTION_TYPES
    let mut rule = AddPropertyTypeAssigns::default();
    rule.configure(&serde_json::json!({"inline-public": true}))
        .unwrap();
    run_class_at(
        vec![Box::new(rule)],
        "TypedPropertyFromAssignsRectorInlinePublic",
        80100,
    );
}

#[test]
fn property_type_from_strict_setter_getter() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version
    run_class(
        vec![Box::new(AddPropertyTypeSetterGetter)],
        "PropertyTypeFromStrictSetterGetterRector",
    );
}

#[test]
fn scalar_typed_property_from_jms_serializer_attribute_type() {
    // config/configured_rule.php: ATTRIBUTES
    run_class_at(
        vec![Box::new(AddPropertyTypeJmsScalar)],
        "ScalarTypedPropertyFromJMSSerializerAttributeTypeRector",
        80000,
    );
}

#[test]
fn object_typed_property_from_jms_serializer_attribute_type() {
    // config/configured_rule.php: ATTRIBUTES
    run_class_at(
        vec![Box::new(AddPropertyTypeJmsObject)],
        "ObjectTypedPropertyFromJMSSerializerAttributeTypeRector",
        80000,
    );
}

#[test]
fn typed_static_property_in_behat_context() {
    // No upstream fixtures: Oracle/ only, Rector run at PHP 8.0 with Source/ autoloaded.
    run_oracle_class(
        vec![Box::new(AddPropertyTypeBehatContext)],
        "TypedStaticPropertyInBehatContextRector",
        80000,
    );
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

#[test]
fn typed_property_from_container_get_set_up() {
    // config/configured_rule.php: TYPED_PROPERTIES
    run_class_stubs(
        vec![Box::new(AddPropertyTypeContainerGet)],
        "TypedPropertyFromContainerGetSetUpRector",
        70400,
        &["phpunit-stubs"],
    );
}

#[test]
fn typed_property_from_create_mock_assign() {
    // config/rule_config.php: TYPED_PROPERTIES
    run_class_stubs(
        vec![Box::new(AddPropertyTypeCreateMock)],
        "TypedPropertyFromCreateMockAssignRector",
        70400,
        &["phpunit-stubs"],
    );
}

#[test]
fn typed_property_from_strict_set_up() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version
    run_class_stubs(
        vec![Box::new(AddPropertyTypeSetup)],
        "TypedPropertyFromStrictSetUpRector",
        80400,
        &["phpunit-stubs"],
    );
}

#[test]
fn typed_property_from_docblock_set_up_defined() {
    // config/rule_config.php: TYPED_PROPERTIES
    run_class_stubs(
        vec![Box::new(AddPropertyTypeSetupDocblock)],
        "TypedPropertyFromDocblockSetUpDefinedRector",
        70400,
        &["phpunit-stubs"],
    );
}

#[test]
fn child_doctrine_repository_class_type() {
    // config/configured_rule.php: the rule alone, Rector's default PHP version
    run_class_stubs(
        vec![Box::new(AddDoctrineRepositoryReturnType)],
        "ChildDoctrineRepositoryClassTypeRector",
        80400,
        &["doctrine-stubs"],
    );
}

#[test]
fn property_contracts_assignment_flows() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRector",
        &["Oracle/property_contracts_assignment_flows"],
        70400,
        &[],
    );
}

#[test]
fn property_contracts_trait_writes() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRector",
        &["Oracle/property_contracts_trait_writes"],
        70400,
        &[],
    );
}

#[test]
fn property_contracts_doc_narrowing() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRector",
        &["Oracle/property_contracts_doc_narrowing"],
        70400,
        &[],
    );
}

#[test]
fn property_contracts_doc_unions() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRector",
        &["Oracle/property_contracts_doc_unions"],
        70400,
        &[],
    );
}

#[test]
fn property_contracts_known_url_components() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeAssigns::default())],
        "TypedPropertyFromAssignsRectorPhp82",
        &["Oracle/property_contracts_known_url_components"],
        80200,
        &[],
    );
}

#[test]
fn property_contracts_documented_constructor_types() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeConstructor)],
        "TypedPropertyFromStrictConstructorRector",
        &["Oracle/property_contracts_documented_constructor_types"],
        80100,
        &[],
    );
}

#[test]
fn property_contracts_partial_defaults() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeConstructor)],
        "TypedPropertyFromStrictConstructorRector",
        &["Oracle/property_contracts_partial_defaults"],
        80100,
        &[],
    );
}

#[test]
fn property_contracts_parent_roundtrip() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeConstructor)],
        "TypedPropertyFromStrictConstructorRector",
        &["Oracle/property_contracts_parent_roundtrip"],
        80100,
        &[],
    );
}

#[test]
fn property_contracts_getter_dnf() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeSetterGetter)],
        "PropertyTypeFromStrictSetterGetterRector",
        &["Oracle/property_contracts_getter_dnf"],
        80400,
        &[],
    );
}

#[test]
fn property_contracts_constructor_dnf_boundaries() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeConstructor)],
        "TypedPropertyFromStrictConstructorRector",
        &["Oracle/property_contracts_constructor_dnf_boundaries"],
        80100,
        &[],
    );
}

#[test]
fn property_contracts_getter_union_boundaries() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeSetterGetter)],
        "PropertyTypeFromStrictSetterGetterRector",
        &["Oracle/property_contracts_getter_union_boundaries"],
        80400,
        &[],
    );
}

#[test]
fn property_contracts_scalar_hint_comparisons() {
    lint_c::run_dirs(
        vec![Box::new(AddPropertyTypeConstructor)],
        "TypedPropertyFromStrictConstructorRector",
        &["Oracle/property_contracts_scalar_hint_comparisons"],
        80100,
        &[],
    );
}
