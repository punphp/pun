//! Rector's own fixtures for the ported dead-code class-method rules, copied verbatim from rector-src 2.6.1
//! (`.oracle/rector`, rules-tests/DeadCode/Rector/ClassMethod/...) into tests/rector/<Rector class>/Fixture, with the
//! classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.6.1 (`.oracle/rector-ecosystem`, the rule
//! alone, `phpVersion(80400)`) wrote for them; Pun/ holds the cases where Pun cannot see what PHPStan sees, its
//! expected output derived by hand, each with its reason. Each one must pass as
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits must give the
//! expected bytes before format too.
//!
//! `RemoveParentDelegatingConstructorRector/Source/vendor/String_.php` is not upstream: it declares the constructor of
//! nikic/php-parser 5's `PhpParser\Node\Scalar\String_`, which the fixtures extend and Rector's run loads from its own
//! vendor directory.

mod lint_c;

use lint_c::run_class;
use pun_lint::Rule;
use pun_lint::rules::{
    NoEmptyMethod, NoRedundantDefaultArgument, NoUnusedPrivateMethod, NoUselessConstructor,
    NoUselessOverride,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn remove_empty_class_method() {
    check(NoEmptyMethod, "RemoveEmptyClassMethodRector");
}

#[test]
fn remove_unused_private_method() {
    check(NoUnusedPrivateMethod, "RemoveUnusedPrivateMethodRector");
}

#[test]
fn remove_parent_delegating_constructor() {
    check(
        NoUselessConstructor,
        "RemoveParentDelegatingConstructorRector",
    );
}

#[test]
fn remove_parent_delegating_class_method() {
    check(NoUselessOverride, "RemoveParentDelegatingClassMethodRector");
}

#[test]
fn remove_argument_from_default_parent_call() {
    check(
        NoRedundantDefaultArgument,
        "RemoveArgumentFromDefaultParentCallRector",
    );
}
