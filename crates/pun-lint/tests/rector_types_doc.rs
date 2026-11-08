//! Rector's own fixtures for the ported docblock `types` rules (Rector 2.8.4, rules-tests/TypeDeclarationDocblocks and
//! the docblock rules of rules-tests/TypeDeclaration), copied verbatim into tests/rector/<Rector class>/Fixture with
//! the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.8.4 (the rule alone,
//! `phpVersion(80400)`) wrote for them. Each must give the expected bytes, and the same after the laravel preset's
//! format.
//!
//! The return type rules that write Rector's docblocks too (`ReturnTypeFromStrictNewArrayRector`,
//! `PrivateMethodReturnTypeFromStrictNewArrayRector`, `NarrowObjectReturnTypeRector`,
//! `ReturnTypeFromGetRepositoryDocblockRector` and `NarrowWideUnionReturnTypeRector`) run here as well, the last at
//! PHP 8.0, as their config sets. NarrowWideUnionReturnTypeRector/Stubs holds the vendor classes the cases name (the
//! monolog/monolog classes rector-src's test vendor has, and the Oracle cases' packages), under a `vendor/` path so they
//! count as vendor code, and the composer.json the rule reads: Source/composer.json's requires, and those packages.
//!
//! Pun/ holds the cases where Pun keeps the meaning but not Rector's bytes, its expected output derived by hand. Rector
//! 2.8.4 prints a changed docblock with phpdoc-parser's format-preserving `Printer`, which Pun does not port (it
//! prints as Rector 2.6.1's `PhpDocInfoPrinter` did; context/plan/lint.md). The two differ on the Pun/ cases of
//! DocblockReturnArrayFromDirectArrayInstanceRector and ClassMethodArrayDocblockParamFromLocalCallsRector: Rector
//! starts a tag it adds with the line break before the doc's last line, so after an empty ` *` line or on lines
//! without a space after `*` it writes `*@return` where Pun writes ` * @return`; it keeps an empty last line after
//! the new tag, where Pun drops it; it keeps the spacing between a changed type and its description (`$items-desc`,
//! `array<...>   the items`), where Pun writes one space; and in a doc without `*` lines it writes ` *  @return`.

mod lint_c;

use lint_c::{run_class, run_class_at, run_class_stubs, run_dirs};
use mago_syntax::cst::Node;
use pun_lint::Rule;
use pun_lint::rules::{
    AddGetterReturnPhpdoc, AddParamPhpdocFromCallableCall, AddParamPhpdocFromDimFetch, AddParamPhpdocFromLocalCalls,
    AddParamPhpdocFromReferenceAssign,
    AddReturnPhpdocCommonObject, AddReturnPhpdocDimAssignedObject, AddReturnPhpdocDirectArray, AddReturnPhpdocJsonArray,
    AddReturnPhpdocScalarArray, AddVarPhpdocFromConstructorParam, AddVarPhpdocFromDefault, AddVarPhpdocFromDimFetch,
    AddVarPhpdocFromGetter, PreferGenericCollectionPhpdoc,
};
use pun_lint::rules::{
    AddPrivateMethodArrayReturnType, AddReturnTypeFromGetRepository, AddReturnTypeFromNewArray, NoUnusedReturnType,
    PreferSpecificReturnType,
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

#[test]
fn return_type_from_strict_new_array() {
    check(|| Box::new(AddReturnTypeFromNewArray), "ReturnTypeFromStrictNewArrayRector");
}

#[test]
fn private_method_return_type_from_strict_new_array() {
    check(|| Box::new(AddPrivateMethodArrayReturnType), "PrivateMethodReturnTypeFromStrictNewArrayRector");
}

#[test]
fn narrow_object_return_type() {
    check(|| Box::new(PreferSpecificReturnType), "NarrowObjectReturnTypeRector");
}

#[test]
fn return_type_from_get_repository_docblock() {
    for rule in [
        Box::new(AddReturnTypeFromGetRepository) as Box<dyn Rule>,
        Box::new(Everywhere(Box::new(AddReturnTypeFromGetRepository))),
    ] {
        run_class_stubs(vec![rule], "ReturnTypeFromGetRepositoryDocblockRector", 80400, &["doctrine-stubs"]);
    }
}

#[test]
fn narrow_wide_union_return_type() {
    // the fixtures' config reads Source/composer.json and sets PHP 8.0; Stubs/composer.json has its requires and the
    // packages of the Oracle cases' vendor classes; Php84/ holds the cases of intersections, run at PHP 8.4
    let class = "NarrowWideUnionReturnTypeRector";
    let composer_json =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector").join(class).join("Stubs/composer.json");
    let rule = || NoUnusedReturnType {
        composer_json: Some(composer_json.clone()),
    };
    run_class_at(vec![Box::new(rule())], class, 80000);
    run_class_at(vec![Box::new(Everywhere(Box::new(rule())))], class, 80000);
    run_dirs(vec![Box::new(rule())], class, &["Php84"], 80400, &[]);
}
