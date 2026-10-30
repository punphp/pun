//! Rector's own fixtures for `lint/php-8.5/no-null-array-key` (`ArrayKeyExistsNullToEmptyStringRector`),
//! `lint/statements/add-safe-strict-types` (`SafeDeclareStrictTypesRector`) and `lint/classes/add-dynamic-properties`
//! (`CompleteDynamicPropertiesRector`), copied verbatim from rector-src 2.6.1 (`.oracle/rector`, rules-tests/...) into
//! tests/rector/<Rector class>/Fixture. Oracle/ holds extra inputs with what Rector 2.6.1 (`.oracle/rector-ecosystem`,
//! the rule alone, `phpVersion(80500)`, 80400 for the strict types rule, 79999 for the dynamic properties rule) wrote
//! for them. Each must pass as `format(pun lint --fix) == format(expected)` with the laravel preset, and the text edits
//! must give the expected bytes before format too.
//!
//! ArrayKeyExistsNullToEmptyStringRector/Pun/cast_parentheses.php.inc: Rector prints the new `(string)` cast without
//! parentheses around an assignment (`(string) $k = $m`, which does not parse) and around a ternary whose condition
//! is no cast (`(string) $k ? $s : $m . 'a'`, which casts only the condition). Pun writes `(string) ($k = $m)` and
//! `(string) ($k ? $s : $m . 'a')`, the cast of the whole key that Rector's tree holds.
//!
//! CompleteDynamicPropertiesRector/Pun/unreached_closure.php.inc: a closure assigned after a `return`, code Mago's
//! analysis does not reach, so it gives the closure no type. Rector (PHPStan types unreachable code too) writes
//! `@var \Closure(): void`; Pun declares the property without a `@var`, as for any value of unknown type.

mod lint_c;

use lint_c::run_class_at;
use pun_lint::rules::{AddDynamicProperties, AddSafeStrictTypes, NoNullArrayKey};

#[test]
fn array_key_exists_null_to_empty_string() {
    run_class_at(
        vec![Box::new(NoNullArrayKey)],
        "ArrayKeyExistsNullToEmptyStringRector",
        80500,
    );
}

#[test]
fn safe_declare_strict_types() {
    run_class_at(
        vec![Box::new(AddSafeStrictTypes)],
        "SafeDeclareStrictTypesRector",
        80400,
    );
}

#[test]
fn complete_dynamic_properties() {
    // upstream's Fixture runs at `PhpVersionFeature::UNION_TYPES - 1`, FixtureUnionTypes at Rector's default
    run_class_at(
        vec![Box::new(AddDynamicProperties)],
        "CompleteDynamicPropertiesRector",
        79999,
    );
    run_class_at(
        vec![Box::new(AddDynamicProperties)],
        "CompleteDynamicPropertiesRectorUnionTypes",
        80400,
    );
}
