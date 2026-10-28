//! Rector's own fixtures for the ported docblock `types` rules (Rector 2.6.1, rules-tests/TypeDeclarationDocblocks and
//! the docblock rules of rules-tests/TypeDeclaration), copied verbatim into tests/rector/<Rector class>/Fixture with
//! the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.6.1 (the rule alone,
//! `phpVersion(80400)`) wrote for them. Each must give the expected bytes, and the same after the laravel preset's
//! format.

mod lint_c;

use lint_c::run_class;
use pun_lint::Rule;
use pun_lint::rules::{
    AddGetterReturnPhpdoc, AddParamPhpdocFromCallableCall, AddParamPhpdocFromDimFetch, AddParamPhpdocFromLocalCalls,
    AddParamPhpdocFromReferenceAssign,
    AddReturnPhpdocCommonObject, AddReturnPhpdocDimAssignedObject, AddReturnPhpdocDirectArray, AddReturnPhpdocJsonArray,
    AddReturnPhpdocScalarArray, AddVarPhpdocFromConstructorParam, AddVarPhpdocFromDefault, AddVarPhpdocFromDimFetch,
    AddVarPhpdocFromGetter, PreferGenericCollectionPhpdoc,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn docblock_return_array_from_direct_array_instance() {
    check(
        AddReturnPhpdocDirectArray,
        "DocblockReturnArrayFromDirectArrayInstanceRector",
    );
}

#[test]
fn docblock_getter_return_array_from_property_docblock_var() {
    check(AddGetterReturnPhpdoc, "DocblockGetterReturnArrayFromPropertyDocblockVarRector");
}

#[test]
fn add_param_array_docblock_based_on_callable_native_func_call() {
    check(AddParamPhpdocFromCallableCall, "AddParamArrayDocblockBasedOnCallableNativeFuncCallRector");
}

#[test]
fn add_param_array_docblock_from_dim_fetch_access() {
    check(AddParamPhpdocFromDimFetch, "AddParamArrayDocblockFromDimFetchAccessRector");
}

#[test]
fn add_param_array_docblock_from_assigns_param_to_param_reference() {
    check(AddParamPhpdocFromReferenceAssign, "AddParamArrayDocblockFromAssignsParamToParamReferenceRector");
}

#[test]
fn add_return_docblock_for_common_object_denominator() {
    check(AddReturnPhpdocCommonObject, "AddReturnDocblockForCommonObjectDenominatorRector");
}

#[test]
fn add_return_docblock_for_array_dim_assigned_object() {
    check(AddReturnPhpdocDimAssignedObject, "AddReturnDocblockForArrayDimAssignedObjectRector");
}

#[test]
fn add_return_docblock_for_json_array() {
    check(AddReturnPhpdocJsonArray, "AddReturnDocblockForJsonArrayRector");
}

#[test]
fn add_return_docblock_for_scalar_array_from_assigns() {
    check(AddReturnPhpdocScalarArray, "AddReturnDocblockForScalarArrayFromAssignsRector");
}

#[test]
fn docblock_var_from_param_docblock_in_constructor() {
    check(AddVarPhpdocFromConstructorParam, "DocblockVarFromParamDocblockInConstructorRector");
}

#[test]
fn docblock_var_array_from_property_defaults() {
    check(AddVarPhpdocFromDefault, "DocblockVarArrayFromPropertyDefaultsRector");
}

#[test]
fn add_var_array_docblock_from_dim_fetch_assign() {
    check(AddVarPhpdocFromDimFetch, "AddVarArrayDocblockFromDimFetchAssignRector");
}

#[test]
fn docblock_var_array_from_getter_return() {
    check(AddVarPhpdocFromGetter, "DocblockVarArrayFromGetterReturnRector");
}

#[test]
fn narrow_array_collection_union_return_docblock() {
    check(PreferGenericCollectionPhpdoc, "NarrowArrayCollectionUnionReturnDocblockRector");
}

#[test]
fn class_method_array_docblock_param_from_local_calls() {
    check(
        AddParamPhpdocFromLocalCalls,
        "ClassMethodArrayDocblockParamFromLocalCallsRector",
    );
}
