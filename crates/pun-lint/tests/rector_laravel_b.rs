//! rector-laravel's own fixtures for the `laravel` rules that read types, copied verbatim from driftingly/rector-laravel
//! 2.7.3 (its source at 699f25b, tests/Rector/...) into tests/rector/<Rector class>/Fixture, with rector-laravel's
//! stubs/ (tests/rector-laravel-stubs, copied verbatim; its test suite autoloads them as a classmap) known beside each
//! class's Source/. Oracle/ holds extra inputs with what Rector 2.8.4 and rector-laravel 2.7.3
//! (`.oracle/rector-ecosystem`, the rule alone, `phpVersion(80400)`, the stubs and Source/ on `withAutoloadPaths`)
//! wrote for them; Pun/ holds the cases where Pun keeps the code valid or the meaning where Rector does not, its
//! expected output derived by hand. A fixture is `input -----\n expected`, or only the input when the rule must not
//! change it. Each one must pass as `format(pun lint --fix) == format(expected)` with format being the laravel preset,
//! and the text edits must give the expected bytes before format too.

#[allow(dead_code)]
mod lint_c;

use std::path::Path;

use lint_c::{run_class_paths, run_dirs};
use pun_lint::Rule;
use pun_lint::rules::{
    AddFactoryExtendsPhpdoc, AddHasFactoryTrait, AddQueryClosureType, AddRelationGenerics,
    AddRelationQueryClosureType, AddTapClosureType, ContainerService, NoAppArrayAccess,
    NoCollectionAliases, NoNegatedContains, NoNegatedFilter, NoNegatedWhen, NoUselessValueCall,
    NoUselessWithCall, PreferClassContainerKey, PreferClosureOnlyBind, PreferCollectionAll,
    PreferDispatchHelper, PreferEnvironmentCheck, PreferLatestOldest, PreferProtectedModelMethods,
    PreferQueryBuilderCall, PreferRequestValidate, PreferStatusAssertionMethod,
    PreferTypedAssertClosure, PreferValidationRuleArray,
};

fn check(rule: impl Rule + 'static, class: &str) {
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector-laravel-stubs");
    run_class_paths(vec![Box::new(rule)], class, 80400, &[&stubs]);
}

#[test]
fn avoid_negated_collection_contains_or_doesnt_contain() {
    check(
        NoNegatedContains,
        "AvoidNegatedCollectionContainsOrDoesntContainRector",
    );
}

#[test]
fn unalias_collection_methods() {
    check(NoCollectionAliases, "UnaliasCollectionMethodsRector");
}

#[test]
fn avoid_negated_collection_filter_or_reject() {
    check(
        NoNegatedFilter,
        "AvoidNegatedCollectionFilterOrRejectRector",
    );
}

#[test]
fn reverse_conditionable_method_call() {
    check(NoNegatedWhen, "ReverseConditionableMethodCallRector");
}

#[test]
fn assert_status_to_assert_method() {
    check(
        PreferStatusAssertionMethod,
        "AssertStatusToAssertMethodRector",
    );
}

#[test]
fn app_environment_comparison_to_parameter() {
    check(
        PreferEnvironmentCheck,
        "AppEnvironmentComparisonToParameterRector",
    );
}

#[test]
fn assert_with_class_string_to_type_hinted_closure() {
    check(
        PreferTypedAssertClosure,
        "AssertWithClassStringToTypeHintedClosureRector",
    );
}

#[test]
fn remove_redundant_value_calls() {
    check(NoUselessValueCall, "RemoveRedundantValueCallsRector");
}

#[test]
fn remove_redundant_with_calls() {
    check(NoUselessWithCall, "RemoveRedundantWithCallsRector");
}

#[test]
fn dispatch_to_helper_functions() {
    check(PreferDispatchHelper, "DispatchToHelperFunctionsRector");
}

#[test]
fn validation_rule_array_string_value_to_array() {
    check(
        PreferValidationRuleArray,
        "ValidationRuleArrayStringValueToArrayRector",
    );
}

#[test]
fn eloquent_where_type_hint_closure_parameter() {
    check(
        AddQueryClosureType,
        "EloquentWhereTypeHintClosureParameterRector",
    );
}

#[test]
fn eloquent_where_relation_type_hinting_parameter() {
    check(
        AddRelationQueryClosureType,
        "EloquentWhereRelationTypeHintingParameterRector",
    );
}

#[test]
fn make_model_attributes_and_scopes_protected() {
    check(
        PreferProtectedModelMethods,
        "MakeModelAttributesAndScopesProtectedRector",
    );
}

/// The fixtures run with the patterns of rector-laravel's config/configured_rule.php, Oracle/ and Pun/ with the default
/// ones, Configured/ (Rector's output with this configuration) with `$c?l`, `[!a-c]_date` and `x\_at`, and
/// Unpatterned/ (Rector's output) with no pattern, which allows every column.
#[test]
fn eloquent_order_by_to_latest_or_oldest() {
    let class = "EloquentOrderByToLatestOrOldestRector";
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector-laravel-stubs");
    let configured = |patterns: serde_json::Value| -> Vec<Box<dyn Rule>> {
        let mut rule = PreferLatestOldest::default();
        rule.configure(&serde_json::json!({ "allowed-patterns": patterns }))
            .unwrap();
        vec![Box::new(rule)]
    };
    let fixture = serde_json::json!([
        "created_at",
        "submitted_a*",
        "*tested_at",
        "$allowed_variable_name"
    ]);
    run_dirs(configured(fixture), class, &["Fixture"], 80400, &[&stubs]);
    let own = serde_json::json!(["$c?l", "[!a-c]_date", "x\\_at"]);
    run_dirs(configured(own), class, &["Configured"], 80400, &[&stubs]);
    run_dirs(
        configured(serde_json::json!([])),
        class,
        &["Unpatterned"],
        80400,
        &[&stubs],
    );
    let default: Vec<Box<dyn Rule>> = vec![Box::new(PreferLatestOldest::default())];
    run_dirs(default, class, &["Oracle", "Pun"], 80400, &[&stubs]);
}

#[test]
fn eloquent_order_by_to_latest_or_oldest_options() {
    let mut rule = PreferLatestOldest::default();
    assert!(rule.configure(&serde_json::json!(true)).is_err());
    assert!(rule.configure(&serde_json::json!({"other": []})).is_err());
    assert!(
        rule.configure(&serde_json::json!({"allowed-patterns": [1]}))
            .is_err()
    );
    rule.configure(&serde_json::json!(null)).unwrap();
    assert_eq!(rule.allowed_patterns, ["*_at", "*_date", "*_on"]);
    // Rector's `configure` without the key leaves no pattern, which allows every column
    rule.configure(&serde_json::json!({})).unwrap();
    assert!(rule.allowed_patterns.is_empty());
}

#[test]
fn request_static_validate_to_inject() {
    check(PreferRequestValidate, "RequestStaticValidateToInjectRector");
}

#[test]
fn convert_enumerable_to_array_to_all() {
    check(PreferCollectionAll, "ConvertEnumerableToArrayToAllRector");
}

#[test]
fn container_bind_concrete_with_closure_only() {
    check(
        PreferClosureOnlyBind,
        "ContainerBindConcreteWithClosureOnlyRector",
    );
}

/// Fixture/WithoutConfiguration, Oracle/ and Pun/ run without options; Fixture/Configured with the list of
/// rector-laravel's config/configured_rule_with_configuration.php, and Configured/ with `App\A`, `app\b`, `\App\G`,
/// `App\H` and `App\Missing`.
#[test]
fn add_has_factory_to_models() {
    let class = "AddHasFactoryToModelsRector";
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector-laravel-stubs");
    let configured = |items: serde_json::Value| -> Vec<Box<dyn Rule>> {
        let mut rule = AddHasFactoryTrait::default();
        rule.configure(&serde_json::json!({ "items": items }))
            .unwrap();
        vec![Box::new(rule)]
    };
    let fixture = serde_json::json!([
        "RectorLaravel\\Tests\\Rector\\Class_\\AddHasFactoryToModelsRector\\Fixture\\Configured\\User",
        "RectorLaravel\\Tests\\Rector\\Class_\\AddHasFactoryToModelsRectorConfigured\\Fixture\\SkipIfAlreadyPresent",
    ]);
    run_dirs(
        configured(fixture),
        class,
        &["Fixture/Configured"],
        80400,
        &[&stubs],
    );
    let own = serde_json::json!(["App\\A", "app\\b", "\\App\\G", "App\\H", "App\\Missing"]);
    run_dirs(configured(own), class, &["Configured"], 80400, &[&stubs]);
    run_dirs(
        configured(serde_json::json!([])),
        class,
        &["Unpatterned"],
        80400,
        &[&stubs],
    );
    let default: Vec<Box<dyn Rule>> = vec![Box::new(AddHasFactoryTrait::default())];
    run_dirs(
        default,
        class,
        &["Fixture/WithoutConfiguration", "Oracle", "Pun"],
        80400,
        &[&stubs],
    );
}

#[test]
fn add_has_factory_to_models_options() {
    let mut rule = AddHasFactoryTrait::default();
    rule.configure(&serde_json::json!(null)).unwrap();
    assert!(rule.items.is_empty());
    assert!(rule.configure(&serde_json::json!([])).is_err());
    assert!(rule.configure(&serde_json::json!({"other": []})).is_err());
    assert!(
        rule.configure(&serde_json::json!({"items": "App\\A"}))
            .is_err()
    );
    // Rector fails on a name with `*` (no pattern matching in `isNames`)
    assert!(
        rule.configure(&serde_json::json!({"items": ["App\\*"]}))
            .is_err()
    );
}

/// The fixtures run with rector-laravel's config/configured_rule.php (`exclude_methods`); Oracle/ without options.
#[test]
fn eloquent_magic_method_to_query_builder() {
    let class = "EloquentMagicMethodToQueryBuilderRector";
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector-laravel-stubs");
    let mut rule = PreferQueryBuilderCall::default();
    rule.configure(&serde_json::json!({
        "exclude-methods": ["excludablePublicMethodBelongsToEloquentQueryBuilder"]
    }))
    .unwrap();
    run_dirs(vec![Box::new(rule)], class, &["Fixture"], 80400, &[&stubs]);
    let default: Vec<Box<dyn Rule>> = vec![Box::new(PreferQueryBuilderCall::default())];
    run_dirs(default, class, &["Oracle", "Pun"], 80400, &[&stubs]);
}

#[test]
fn eloquent_magic_method_to_query_builder_options() {
    let mut rule = PreferQueryBuilderCall::default();
    rule.configure(&serde_json::json!(null)).unwrap();
    assert!(rule.configure(&serde_json::json!(true)).is_err());
    assert!(rule.configure(&serde_json::json!({})).is_err());
    assert!(rule.configure(&serde_json::json!({"other": []})).is_err());
    assert!(rule.configure(&serde_json::json!({"items": [1]})).is_err());
    rule.configure(&serde_json::json!({"items": ["a"]}))
        .unwrap();
    assert_eq!(rule.exclude_methods, ["a"]);
}

/// The fixtures and Oracle/ run with rector-laravel's config/configured_rule.php (`encrypter` to the `Encrypter`
/// class, the `Session` class to `session`), Oracle/ with `session` to `store` after them, written as `pun migrate`
/// writes the value objects.
#[test]
fn replace_service_container_call_arg() {
    let constant = |class: &str| {
        serde_json::json!({
            "attributes": [],
            "class": { "attributes": [], "name": class },
            "name": { "attributes": [], "name": "class" },
        })
    };
    let mut items = vec![
        serde_json::json!({
            "old-service": "encrypter",
            "new-service": constant("Illuminate\\Contracts\\Encryption\\Encrypter"),
        }),
        serde_json::json!({
            "old-service": constant("Illuminate\\Contracts\\Session\\Session"),
            "new-service": "session",
        }),
    ];
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector-laravel-stubs");
    let class = "ReplaceServiceContainerCallArgRector";
    let rule = |items: &[serde_json::Value]| -> Vec<Box<dyn Rule>> {
        let mut rule = PreferClassContainerKey::default();
        rule.configure(&serde_json::json!({ "items": items }))
            .unwrap();
        vec![Box::new(rule)]
    };
    run_dirs(rule(&items), class, &["Fixture"], 80400, &[&stubs]);
    items.push(serde_json::json!({ "old-service": "session", "new-service": "store" }));
    run_dirs(rule(&items), class, &["Oracle"], 80400, &[&stubs]);
}

#[test]
fn replace_service_container_call_arg_options() {
    let mut rule = PreferClassContainerKey::default();
    rule.configure(&serde_json::json!(null)).unwrap();
    assert!(rule.items.is_empty());
    for bad in [
        serde_json::json!(true),
        serde_json::json!({"other": []}),
        serde_json::json!({"items": "a"}),
        serde_json::json!({"items": ["a"]}),
        serde_json::json!({"items": [{"old-service": "a"}]}),
        serde_json::json!({"items": [{"old-service": "a", "new-service": "b", "x": 1}]}),
        serde_json::json!({"items": [{"old-service": 1, "new-service": "b"}]}),
        serde_json::json!({"items": [{"old-service": {"class": {"name": "A"}, "name": {"name": "class"}, "x": 1}, "new-service": "b"}]}),
        serde_json::json!({"items": [{"old-service": {"class": {"name": "A"}}, "new-service": "b"}]}),
    ] {
        assert!(rule.configure(&bad).is_err(), "{bad}");
    }
    rule.configure(&serde_json::json!({"items": [{
        "old-service": {"class": {"name": "A"}, "name": {"name": "FOO"}},
        "new-service": "b",
    }]}))
    .unwrap();
    assert_eq!(
        rule.items,
        [(
            ContainerService::Constant("A".into(), "FOO".into()),
            ContainerService::Name("b".into())
        )]
    );
}

#[test]
fn add_extends_annotation_to_model_factories() {
    check(
        AddFactoryExtendsPhpdoc,
        "AddExtendsAnnotationToModelFactoriesRector",
    );
}

#[test]
fn call_on_app_array_access_to_standalone_assign() {
    check(
        NoAppArrayAccess,
        "CallOnAppArrayAccessToStandaloneAssignRector",
    );
}

/// rector-laravel 2.7.3's test of the rule (`AddGenericReturnTypeToRelationsRectorTest`, Fixture/NewGenerics); the
/// rule no longer reads Laravel's version, so every suite runs with the stubs as they are. Fixture/OldGenerics and
/// Fixture/NewGenericsNoPivot are the tests of `AddLegacyGenericReturnTypeToRelationsRector` and
/// `AddNewGenericReturnTypeToRelationsRector`, which Pun has not ported.
#[test]
fn add_generic_return_type_to_relations() {
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/rector-laravel-stubs");
    run_dirs(
        vec![Box::new(AddRelationGenerics)],
        "AddGenericReturnTypeToRelationsRector",
        &["Fixture/NewGenerics", "Oracle/NewGenerics", "Oracle/Laravel12", "Pun/Laravel12"],
        80400,
        &[stubs.as_path()],
    );
}

#[test]
fn type_hint_tappable_call() {
    check(AddTapClosureType, "TypeHintTappableCallRector");
}
