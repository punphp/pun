//! Rector's own fixtures for `lint/php-8.5/no-null-array-key` (`ArrayKeyExistsNullToEmptyStringRector`) and
//! `lint/statements/add-safe-strict-types` (`SafeDeclareStrictTypesRector`), copied verbatim from rector-src 2.8.4
//! (`.oracle/rector`, rules-tests/...) into tests/rector/<Rector class>/Fixture. Oracle/ holds extra inputs with what
//! Rector 2.8.4 (`.oracle/rector-ecosystem`, the rule alone, `phpVersion(80500)`, 80400 for the strict types rule)
//! wrote for them. Each must pass as `format(pun lint --fix) == format(expected)` with the laravel preset, and the text edits
//! must give the expected bytes before format too.

mod lint_c;

use lint_c::run_class_at;
use pun_lint::rules::{AddSafeStrictTypes, NoNullArrayKey};

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
