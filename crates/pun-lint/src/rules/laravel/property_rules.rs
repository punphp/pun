//! The rector-laravel rules that move one class property into one class attribute (src/Rector/Class_/
//! `*PropertyTo*AttributeRector`), each a [`Config`] of the steps in [`property_attribute`](super::property_attribute).
//! The comment above each rule names its Rector class.

use super::property_attribute::{
    COMMAND, Check, Config, FORM_REQUEST, JSON_RESOURCE, MODEL, SHOULD_QUEUE, Visibility,
    property_rules,
};

property_rules! {
    // AliasesPropertyToAliasesAttributeRector
    PreferAliasesAttribute = Config {
        id: "lint/laravel/prefer-aliases-attribute",
        message: "Changes the aliases property to use the Aliases attribute.",
        class: COMMAND,
        final_only: false,
        property: "aliases",
        visibility: Visibility::Protected,
        check: Check::MaybeStringArray,
        attribute: "Illuminate\\Console\\Attributes\\Aliases",
        skip_existing: true,
    };
    // AppendsPropertyToAppendsAttributeRector
    PreferAppendsAttribute = Config {
        id: "lint/laravel/prefer-appends-attribute",
        message: "Changes model appends property to use the appends attribute.",
        class: MODEL,
        final_only: false,
        property: "appends",
        visibility: Visibility::Protected,
        check: Check::StringArray,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Appends",
        skip_existing: true,
    };
    // BackoffPropertyToBackoffAttributeRector
    PreferBackoffAttribute = Config {
        id: "lint/laravel/prefer-backoff-attribute",
        message: "Changes the backoff property to use the Backoff attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "backoff",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\Backoff",
        skip_existing: true,
    };
    // CollectsPropertyToCollectsAttributeRector
    PreferCollectsAttribute = Config {
        id: "lint/laravel/prefer-collects-attribute",
        message: "Changes the collects property to use the Collects attribute.",
        class: JSON_RESOURCE,
        final_only: false,
        property: "collects",
        visibility: Visibility::Protected,
        check: Check::ClassConstOrString,
        attribute: "Illuminate\\Http\\Resources\\Attributes\\Collects",
        skip_existing: true,
    };
    // CommandHiddenPropertyToHiddenAttributeRector
    PreferCommandHiddenAttribute = Config {
        id: "lint/laravel/prefer-command-hidden-attribute",
        message: "Changes the hidden property to use the Hidden attribute on console commands.",
        class: COMMAND,
        final_only: false,
        property: "hidden",
        visibility: Visibility::Protected,
        check: Check::TrueOrConstant,
        attribute: "Illuminate\\Console\\Attributes\\Hidden",
        skip_existing: true,
    };
    // ConnectionPropertyToConnectionAttributeRector
    PreferConnectionAttribute = Config {
        id: "lint/laravel/prefer-connection-attribute",
        message: "Changes model connection property to use the Connection attribute.",
        class: MODEL,
        final_only: false,
        property: "connection",
        visibility: Visibility::Protected,
        check: Check::String,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Connection",
        skip_existing: false,
    };
    // DateFormatPropertyToDateFormatAttributeRector
    PreferDateFormatAttribute = Config {
        id: "lint/laravel/prefer-date-format-attribute",
        message: "Changes model dateFormat property to use the DateFormat attribute.",
        class: MODEL,
        final_only: false,
        property: "dateFormat",
        visibility: Visibility::Protected,
        check: Check::String,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\DateFormat",
        skip_existing: true,
    };
    // DelayPropertyToDelayAttributeRector
    PreferDelayAttribute = Config {
        id: "lint/laravel/prefer-delay-attribute",
        message: "Changes the delay property to use the Delay attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "delay",
        visibility: Visibility::Public,
        check: Check::Integer,
        attribute: "Illuminate\\Queue\\Attributes\\Delay",
        skip_existing: true,
    };
    // DeleteWhenMissingModelsPropertyToDeleteWhenMissingModelsAttributeRector
    PreferDeleteWhenMissingModelsAttribute = Config {
        id: "lint/laravel/prefer-delete-when-missing-models-attribute",
        message: "Changes the deleteWhenMissingModels property to use the DeleteWhenMissingModels attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "deleteWhenMissingModels",
        visibility: Visibility::Public,
        check: Check::True,
        attribute: "Illuminate\\Queue\\Attributes\\DeleteWhenMissingModels",
        skip_existing: true,
    };
    // DescriptionPropertyToDescriptionAttributeRector
    PreferDescriptionAttribute = Config {
        id: "lint/laravel/prefer-description-attribute",
        message: "Changes the description property to use the Description attribute.",
        class: COMMAND,
        final_only: false,
        property: "description",
        visibility: Visibility::Protected,
        check: Check::String,
        attribute: "Illuminate\\Console\\Attributes\\Description",
        skip_existing: true,
    };
    // ErrorBagPropertyToErrorBagAttributeRector
    PreferErrorBagAttribute = Config {
        id: "lint/laravel/prefer-error-bag-attribute",
        message: "Changes the errorBag property to use the ErrorBag attribute.",
        class: FORM_REQUEST,
        final_only: false,
        property: "errorBag",
        visibility: Visibility::Protected,
        check: Check::String,
        attribute: "Illuminate\\Foundation\\Http\\Attributes\\ErrorBag",
        skip_existing: true,
    };
    // FailOnTimeoutPropertyToFailOnTimeoutAttributeRector
    PreferFailOnTimeoutAttribute = Config {
        id: "lint/laravel/prefer-fail-on-timeout-attribute",
        message: "Changes the failOnTimeout property to use the FailOnTimeout attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "failOnTimeout",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\FailOnTimeout",
        skip_existing: true,
    };
    // FillablePropertyToFillableAttributeRector
    PreferFillableAttribute = Config {
        id: "lint/laravel/prefer-fillable-attribute",
        message: "Changes model fillable property to use the fillable attribute.",
        class: MODEL,
        final_only: false,
        property: "fillable",
        visibility: Visibility::Protected,
        check: Check::StringArray,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Fillable",
        skip_existing: true,
    };
    // GuardedPropertyToGuardedAttributeRector
    PreferGuardedAttribute = Config {
        id: "lint/laravel/prefer-guarded-attribute",
        message: "Changes model guarded property to use the guarded attribute.",
        class: MODEL,
        final_only: false,
        property: "guarded",
        visibility: Visibility::Protected,
        check: Check::StringArray,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Guarded",
        skip_existing: true,
    };
    // HelpPropertyToHelpAttributeRector
    PreferHelpAttribute = Config {
        id: "lint/laravel/prefer-help-attribute",
        message: "Changes the help property to use the Help attribute.",
        class: COMMAND,
        final_only: false,
        property: "help",
        visibility: Visibility::Protected,
        check: Check::String,
        attribute: "Illuminate\\Console\\Attributes\\Help",
        skip_existing: true,
    };
    // HiddenPropertyToHiddenAttributeRector
    PreferHiddenAttribute = Config {
        id: "lint/laravel/prefer-hidden-attribute",
        message: "Changes model hidden property to use the hidden attribute.",
        class: MODEL,
        final_only: false,
        property: "hidden",
        visibility: Visibility::Protected,
        check: Check::StringArray,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Hidden",
        skip_existing: true,
    };
    // JobConnectionPropertyToJobConnectionAttributeRector
    PreferJobConnectionAttribute = Config {
        id: "lint/laravel/prefer-job-connection-attribute",
        message: "Changes the connection property to use the Connection attribute on queue jobs.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "connection",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\Connection",
        skip_existing: true,
    };
    // MaxExceptionsPropertyToMaxExceptionsAttributeRector
    PreferMaxExceptionsAttribute = Config {
        id: "lint/laravel/prefer-max-exceptions-attribute",
        message: "Changes the maxExceptions property to use the MaxExceptions attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "maxExceptions",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\MaxExceptions",
        skip_existing: true,
    };
    // PreserveKeysPropertyToPreserveKeysAttributeRector
    PreferPreserveKeysAttribute = Config {
        id: "lint/laravel/prefer-preserve-keys-attribute",
        message: "Changes the preserveKeys property to use the PreserveKeys attribute.",
        class: JSON_RESOURCE,
        final_only: false,
        property: "preserveKeys",
        visibility: Visibility::Protected,
        check: Check::TrueOrConstant,
        attribute: "Illuminate\\Http\\Resources\\Attributes\\PreserveKeys",
        skip_existing: true,
    };
    // QueuePropertyToQueueAttributeRector
    PreferQueueAttribute = Config {
        id: "lint/laravel/prefer-queue-attribute",
        message: "Changes the queue property to use the Queue attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "queue",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\Queue",
        skip_existing: true,
    };
    // SignaturePropertyToSignatureAttributeRector
    PreferSignatureAttribute = Config {
        id: "lint/laravel/prefer-signature-attribute",
        message: "Changes the signature property to use the Signature attribute.",
        class: COMMAND,
        final_only: false,
        property: "signature",
        visibility: Visibility::Protected,
        check: Check::String,
        attribute: "Illuminate\\Console\\Attributes\\Signature",
        skip_existing: true,
    };
    // StopOnFirstFailurePropertyToStopOnFirstFailureAttributeRector
    PreferStopOnFirstFailureAttribute = Config {
        id: "lint/laravel/prefer-stop-on-first-failure-attribute",
        message: "Changes the stopOnFirstFailure property to use the StopOnFirstFailure attribute.",
        class: FORM_REQUEST,
        final_only: false,
        property: "stopOnFirstFailure",
        visibility: Visibility::Protected,
        check: Check::TrueOrConstant,
        attribute: "Illuminate\\Foundation\\Http\\Attributes\\StopOnFirstFailure",
        skip_existing: true,
    };
    // TimeoutPropertyToTimeoutAttributeRector
    PreferTimeoutAttribute = Config {
        id: "lint/laravel/prefer-timeout-attribute",
        message: "Changes the timeout property to use the Timeout attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "timeout",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\Timeout",
        skip_existing: true,
    };
    // TouchesPropertyToTouchesAttributeRector
    PreferTouchesAttribute = Config {
        id: "lint/laravel/prefer-touches-attribute",
        message: "Changes model touches property to use the touches attribute.",
        class: MODEL,
        final_only: false,
        property: "touches",
        visibility: Visibility::Protected,
        check: Check::StringArray,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Touches",
        skip_existing: true,
    };
    // TriesPropertyToTriesAttributeRector
    PreferTriesAttribute = Config {
        id: "lint/laravel/prefer-tries-attribute",
        message: "Changes the tries property to use the Tries attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "tries",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\Tries",
        skip_existing: true,
    };
    // UniqueForPropertyToUniqueForAttributeRector
    PreferUniqueForAttribute = Config {
        id: "lint/laravel/prefer-unique-for-attribute",
        message: "Changes the uniqueFor property to use the UniqueFor attribute.",
        class: SHOULD_QUEUE,
        final_only: true,
        property: "uniqueFor",
        visibility: Visibility::Public,
        check: Check::Any,
        attribute: "Illuminate\\Queue\\Attributes\\UniqueFor",
        skip_existing: true,
    };
    // VisiblePropertyToVisibleAttributeRector
    PreferVisibleAttribute = Config {
        id: "lint/laravel/prefer-visible-attribute",
        message: "Changes model visible property to use the Visible attribute.",
        class: MODEL,
        final_only: false,
        property: "visible",
        visibility: Visibility::Protected,
        check: Check::TypedStringArray,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\Visible",
        skip_existing: true,
    };
    // WithoutIncrementingPropertyToWithoutIncrementingAttributeRector
    PreferWithoutIncrementingAttribute = Config {
        id: "lint/laravel/prefer-without-incrementing-attribute",
        message: "Changes model incrementing = false property to use the WithoutIncrementing attribute.",
        class: MODEL,
        final_only: false,
        property: "incrementing",
        visibility: Visibility::Public,
        check: Check::FalseConstant,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\WithoutIncrementing",
        skip_existing: true,
    };
    // WithoutTimestampsPropertyToWithoutTimestampsAttributeRector
    PreferWithoutTimestampsAttribute = Config {
        id: "lint/laravel/prefer-without-timestamps-attribute",
        message: "Changes model timestamps = false property to use the WithoutTimestamps attribute.",
        class: MODEL,
        final_only: false,
        property: "timestamps",
        visibility: Visibility::Public,
        check: Check::FalseConstant,
        attribute: "Illuminate\\Database\\Eloquent\\Attributes\\WithoutTimestamps",
        skip_existing: true,
    };
}
