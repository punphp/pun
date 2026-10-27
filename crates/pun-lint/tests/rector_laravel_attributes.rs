//! rector-laravel's own fixtures for its property-to-attribute rules (src/Rector/Class_/*PropertyTo*AttributeRector) and
//! `ScopeNamedClassMethodToScopeAttributedClassMethodRector`, copied verbatim from driftingly/rector-laravel 2.5.0 (its
//! source at 2bd499d, tests/Rector/...) into tests/rector/<Rector class>/Fixture, laid out as in rector_laravel.rs:
//! Oracle/ holds extra inputs with what Rector 2.6.1 and rector-laravel 2.5.0 (`.oracle/rector-ecosystem`, the rule
//! alone, `phpVersion(80400)`, rector-laravel's stubs/ autoloaded) wrote for them; Pun/ holds the documented cases
//! where Pun differs, their expected output derived by hand.
//!
//! These rules ask PHPStan's reflection whether the class extends a framework class, so rector-laravel runs its
//! fixtures with its stubs/ on the class map. Every fixture here sees those classes: tests/rector-laravel-stubs holds
//! rector-laravel's stubs/Illuminate, verbatim at 2bd499d.
//!
//! Pun/tabbed (`FillablePropertyToFillableAttributeRector`, `CollectedByPropertyToCollectedByAttributeRector`,
//! `TablePropertyToTableAttributeRector`, `ScopeNamedClassMethodToScopeAttributedClassMethodRector`):
//! a tab in the class's or method's indentation. Rector writes the new attribute's line break and the node after it
//! in spaces at its own indentation; Pun leaves the class alone.
//!
//! Pun/new_static (`CollectedByPropertyToCollectedByAttributeRector`): `return new static(...)` makes Rector write
//! `#[CollectedBy(static::class)]`, which PHP rejects ("static::class cannot be used for compile-time class name
//! resolution"); Pun leaves the method alone.
//!
//! Pun/missing_item (`FillablePropertyToFillableAttributeRector`): an array with an empty item (`[, 'a']`), which
//! php-parser does not parse, so Rector fails on the file; Pun leaves it alone.

#[allow(dead_code)]
mod lint_c;

use lint_c::run_class_with;
use pun_lint::Rule;
use pun_lint::rules::{
    PreferAliasesAttribute, PreferAppendsAttribute, PreferBackoffAttribute,
    PreferCollectedByAttribute, PreferCollectsAttribute, PreferCommandHiddenAttribute,
    PreferConnectionAttribute, PreferDateFormatAttribute, PreferDelayAttribute,
    PreferDeleteWhenMissingModelsAttribute, PreferDescriptionAttribute, PreferErrorBagAttribute,
    PreferFailOnTimeoutAttribute, PreferFillableAttribute, PreferGuardedAttribute,
    PreferHelpAttribute, PreferHiddenAttribute, PreferJobConnectionAttribute,
    PreferMaxExceptionsAttribute, PreferPreserveKeysAttribute, PreferQueueAttribute,
    PreferScopeAttribute, PreferSignatureAttribute, PreferStopOnFirstFailureAttribute,
    PreferTableAttribute, PreferTimeoutAttribute, PreferTouchesAttribute, PreferTriesAttribute,
    PreferUniqueForAttribute, PreferVisibleAttribute, PreferWithoutIncrementingAttribute,
    PreferWithoutTimestampsAttribute,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class_with(
        vec![Box::new(rule)],
        class,
        80400,
        &["rector-laravel-stubs"],
    );
}

#[test]
fn aliases_property_to_aliases_attribute() {
    check(
        PreferAliasesAttribute,
        "AliasesPropertyToAliasesAttributeRector",
    );
}

#[test]
fn appends_property_to_appends_attribute() {
    check(
        PreferAppendsAttribute,
        "AppendsPropertyToAppendsAttributeRector",
    );
}

#[test]
fn backoff_property_to_backoff_attribute() {
    check(
        PreferBackoffAttribute,
        "BackoffPropertyToBackoffAttributeRector",
    );
}

#[test]
fn collected_by_property_to_collected_by_attribute() {
    check(
        PreferCollectedByAttribute,
        "CollectedByPropertyToCollectedByAttributeRector",
    );
}

#[test]
fn collects_property_to_collects_attribute() {
    check(
        PreferCollectsAttribute,
        "CollectsPropertyToCollectsAttributeRector",
    );
}

#[test]
fn command_hidden_property_to_hidden_attribute() {
    check(
        PreferCommandHiddenAttribute,
        "CommandHiddenPropertyToHiddenAttributeRector",
    );
}

#[test]
fn connection_property_to_connection_attribute() {
    check(
        PreferConnectionAttribute,
        "ConnectionPropertyToConnectionAttributeRector",
    );
}

#[test]
fn date_format_property_to_date_format_attribute() {
    check(
        PreferDateFormatAttribute,
        "DateFormatPropertyToDateFormatAttributeRector",
    );
}

#[test]
fn delay_property_to_delay_attribute() {
    check(PreferDelayAttribute, "DelayPropertyToDelayAttributeRector");
}

#[test]
fn delete_when_missing_models_property_to_delete_when_missing_models_attribute() {
    check(
        PreferDeleteWhenMissingModelsAttribute,
        "DeleteWhenMissingModelsPropertyToDeleteWhenMissingModelsAttributeRector",
    );
}

#[test]
fn description_property_to_description_attribute() {
    check(
        PreferDescriptionAttribute,
        "DescriptionPropertyToDescriptionAttributeRector",
    );
}

#[test]
fn error_bag_property_to_error_bag_attribute() {
    check(
        PreferErrorBagAttribute,
        "ErrorBagPropertyToErrorBagAttributeRector",
    );
}

#[test]
fn fail_on_timeout_property_to_fail_on_timeout_attribute() {
    check(
        PreferFailOnTimeoutAttribute,
        "FailOnTimeoutPropertyToFailOnTimeoutAttributeRector",
    );
}

#[test]
fn fillable_property_to_fillable_attribute() {
    check(
        PreferFillableAttribute,
        "FillablePropertyToFillableAttributeRector",
    );
}

#[test]
fn guarded_property_to_guarded_attribute() {
    check(
        PreferGuardedAttribute,
        "GuardedPropertyToGuardedAttributeRector",
    );
}

#[test]
fn help_property_to_help_attribute() {
    check(PreferHelpAttribute, "HelpPropertyToHelpAttributeRector");
}

#[test]
fn hidden_property_to_hidden_attribute() {
    check(
        PreferHiddenAttribute,
        "HiddenPropertyToHiddenAttributeRector",
    );
}

#[test]
fn job_connection_property_to_job_connection_attribute() {
    check(
        PreferJobConnectionAttribute,
        "JobConnectionPropertyToJobConnectionAttributeRector",
    );
}

#[test]
fn max_exceptions_property_to_max_exceptions_attribute() {
    check(
        PreferMaxExceptionsAttribute,
        "MaxExceptionsPropertyToMaxExceptionsAttributeRector",
    );
}

#[test]
fn preserve_keys_property_to_preserve_keys_attribute() {
    check(
        PreferPreserveKeysAttribute,
        "PreserveKeysPropertyToPreserveKeysAttributeRector",
    );
}

#[test]
fn queue_property_to_queue_attribute() {
    check(PreferQueueAttribute, "QueuePropertyToQueueAttributeRector");
}

#[test]
fn signature_property_to_signature_attribute() {
    check(
        PreferSignatureAttribute,
        "SignaturePropertyToSignatureAttributeRector",
    );
}

#[test]
fn stop_on_first_failure_property_to_stop_on_first_failure_attribute() {
    check(
        PreferStopOnFirstFailureAttribute,
        "StopOnFirstFailurePropertyToStopOnFirstFailureAttributeRector",
    );
}

#[test]
fn table_property_to_table_attribute() {
    check(PreferTableAttribute, "TablePropertyToTableAttributeRector");
}

#[test]
fn timeout_property_to_timeout_attribute() {
    check(
        PreferTimeoutAttribute,
        "TimeoutPropertyToTimeoutAttributeRector",
    );
}

#[test]
fn touches_property_to_touches_attribute() {
    check(
        PreferTouchesAttribute,
        "TouchesPropertyToTouchesAttributeRector",
    );
}

#[test]
fn tries_property_to_tries_attribute() {
    check(PreferTriesAttribute, "TriesPropertyToTriesAttributeRector");
}

#[test]
fn unique_for_property_to_unique_for_attribute() {
    check(
        PreferUniqueForAttribute,
        "UniqueForPropertyToUniqueForAttributeRector",
    );
}

#[test]
fn visible_property_to_visible_attribute() {
    check(
        PreferVisibleAttribute,
        "VisiblePropertyToVisibleAttributeRector",
    );
}

#[test]
fn without_incrementing_property_to_without_incrementing_attribute() {
    check(
        PreferWithoutIncrementingAttribute,
        "WithoutIncrementingPropertyToWithoutIncrementingAttributeRector",
    );
}

#[test]
fn without_timestamps_property_to_without_timestamps_attribute() {
    check(
        PreferWithoutTimestampsAttribute,
        "WithoutTimestampsPropertyToWithoutTimestampsAttributeRector",
    );
}

#[test]
fn scope_named_class_method_to_scope_attributed_class_method() {
    check(
        PreferScopeAttribute,
        "ScopeNamedClassMethodToScopeAttributedClassMethodRector",
    );
}
