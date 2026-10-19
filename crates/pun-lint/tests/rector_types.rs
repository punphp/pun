//! Rector's own fixtures for the ported `types` rules, copied verbatim from rector-src 2.6.1 (`.oracle/rector`,
//! rules-tests/...) into tests/rector/<Rector class>/Fixture, with the classes they use in Source/. Oracle/ holds extra
//! inputs with what Rector 2.6.1 (`.oracle/rector-bin`, the rule alone, `phpVersion(80400)`) wrote for them; Pun/
//! holds the cases where Pun keeps the meaning but not Rector's bytes, its expected output derived by hand. Each one
//! must pass as `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits
//! must give the expected bytes before format too.

mod lint_c;

use lint_c::{run_class, run_class_at};
use pun_lint::Rule;
use pun_lint::rules::{
    AddArrayReturn, AddArrowFunctionReturn, AddBoolReturn, AddBoolReturnFromConst,
    AddClosureNeverReturn, AddClosureReturn, AddClosureVoidReturn, AddFluentReturn,
    AddFunctionVoidReturn, AddMagicMethodTypes, AddMethodVoidReturn, AddNeverReturn,
    AddNullableReturn, AddNumericReturn, AddNumericReturnFromScalar, AddReturnTypeFromCast,
    AddReturnTypeFromConstant, AddReturnTypeFromNew, AddReturnTypeFromParam,
    AddReturnTypeFromProperty, AddReturnTypeFromTernary, AddStringReturn,
    AddStringReturnFromScalar, AddUnionReturn,
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
