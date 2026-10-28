//! Rector's own fixtures for the ported docblock `types` rules (Rector 2.6.1, rules-tests/TypeDeclarationDocblocks and
//! the docblock rules of rules-tests/TypeDeclaration), copied verbatim into tests/rector/<Rector class>/Fixture with
//! the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.6.1 (the rule alone,
//! `phpVersion(80400)`) wrote for them. Each must give the expected bytes, and the same after the laravel preset's
//! format.

mod lint_c;

use lint_c::run_class;
use mago_syntax::cst::Node;
use pun_lint::Rule;
use pun_lint::rules::{
    AddGetterReturnPhpdoc, AddParamPhpdocFromCallableCall, AddParamPhpdocFromDimFetch, AddParamPhpdocFromLocalCalls,
    AddParamPhpdocFromReferenceAssign,
    AddReturnPhpdocCommonObject, AddReturnPhpdocDimAssignedObject, AddReturnPhpdocDirectArray, AddReturnPhpdocJsonArray,
    AddReturnPhpdocScalarArray, AddVarPhpdocFromConstructorParam, AddVarPhpdocFromDefault, AddVarPhpdocFromDimFetch,
    AddVarPhpdocFromGetter, PreferGenericCollectionPhpdoc,
};

/// A rule called on every node, not only on its `targets()`.
struct Everywhere(Box<dyn Rule>);

impl Rule for Everywhere {
    fn id(&self) -> &'static str {
        self.0.id()
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &pun_lint::Context<'_>,
        out: &mut Vec<pun_lint::Report>,
    ) {
        self.0.visit(node, ancestors, ctx, out);
    }
}

/// The fixtures of `class` through the rule, and through the rule called on every node: `targets()` only spares the
/// engine work, so each rule must leave the other nodes alone on its own.
fn check(rule: fn() -> Box<dyn Rule>, class: &str) {
    run_class(vec![rule()], class);
    run_class(vec![Box::new(Everywhere(rule()))], class);
}

#[test]
fn docblock_return_array_from_direct_array_instance() {
    check(|| Box::new(AddReturnPhpdocDirectArray),
        "DocblockReturnArrayFromDirectArrayInstanceRector",
    );
}

#[test]
fn docblock_getter_return_array_from_property_docblock_var() {
    check(|| Box::new(AddGetterReturnPhpdoc), "DocblockGetterReturnArrayFromPropertyDocblockVarRector");
}

#[test]
fn add_param_array_docblock_based_on_callable_native_func_call() {
    check(|| Box::new(AddParamPhpdocFromCallableCall), "AddParamArrayDocblockBasedOnCallableNativeFuncCallRector");
}

#[test]
fn add_param_array_docblock_from_dim_fetch_access() {
    check(|| Box::new(AddParamPhpdocFromDimFetch), "AddParamArrayDocblockFromDimFetchAccessRector");
}

#[test]
fn add_param_array_docblock_from_assigns_param_to_param_reference() {
    check(|| Box::new(AddParamPhpdocFromReferenceAssign), "AddParamArrayDocblockFromAssignsParamToParamReferenceRector");
}

#[test]
fn add_return_docblock_for_common_object_denominator() {
    check(|| Box::new(AddReturnPhpdocCommonObject), "AddReturnDocblockForCommonObjectDenominatorRector");
}

#[test]
fn add_return_docblock_for_array_dim_assigned_object() {
    check(|| Box::new(AddReturnPhpdocDimAssignedObject), "AddReturnDocblockForArrayDimAssignedObjectRector");
}

#[test]
fn add_return_docblock_for_json_array() {
    check(|| Box::new(AddReturnPhpdocJsonArray), "AddReturnDocblockForJsonArrayRector");
}

#[test]
fn add_return_docblock_for_scalar_array_from_assigns() {
    check(|| Box::new(AddReturnPhpdocScalarArray), "AddReturnDocblockForScalarArrayFromAssignsRector");
}

#[test]
fn docblock_var_from_param_docblock_in_constructor() {
    check(|| Box::new(AddVarPhpdocFromConstructorParam), "DocblockVarFromParamDocblockInConstructorRector");
}

#[test]
fn docblock_var_array_from_property_defaults() {
    check(|| Box::new(AddVarPhpdocFromDefault), "DocblockVarArrayFromPropertyDefaultsRector");
}

#[test]
fn add_var_array_docblock_from_dim_fetch_assign() {
    check(|| Box::new(AddVarPhpdocFromDimFetch), "AddVarArrayDocblockFromDimFetchAssignRector");
}

#[test]
fn docblock_var_array_from_getter_return() {
    check(|| Box::new(AddVarPhpdocFromGetter), "DocblockVarArrayFromGetterReturnRector");
}

#[test]
fn narrow_array_collection_union_return_docblock() {
    check(|| Box::new(PreferGenericCollectionPhpdoc), "NarrowArrayCollectionUnionReturnDocblockRector");
}

#[test]
fn class_method_array_docblock_param_from_local_calls() {
    check(|| Box::new(AddParamPhpdocFromLocalCalls),
        "ClassMethodArrayDocblockParamFromLocalCallsRector",
    );
}
