//! Rector's own fixtures for the ported dead-code class-method rules, copied verbatim from rector-src 2.8.4
//! (`.oracle/rector`, rules-tests/DeadCode/Rector/{Class_,ClassMethod}/...) into tests/rector/<Rector class>/Fixture, with the
//! classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.8.4 (`.oracle/rector-ecosystem`, the rule
//! alone, `phpVersion(80400)`) wrote for them; Pun/ holds the cases where Pun cannot see what PHPStan sees, its
//! expected output derived by hand, each with its reason. Each one must pass as
//! `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text edits must give the
//! expected bytes before format too.

mod lint_c;

use lint_c::run_class;
use pun_lint::Rule;
use pun_lint::rules::{
    NoEmptyMethod, NoRedundantDefaultArgument, NoUnusedPrivateMethod, NoUselessOverride,
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
fn unused_private_method_after_facade_fix_rounds() {
    use pun_lint::Linter;
    use pun_lint::rules::PreferFacadeOverApp;
    use std::path::Path;

    let input = "<?php

final class FacadeFixRounds
{
    public function run(): void
    {
        app(Firewall::class, [app('request')])->isBot();
        $this->called();
    }

    private function called(): void {}
    private function unused(): void {}
}
";
    // Rector 2.8.4, PHP 8.4: RemoveUnusedPrivateMethodRector alone, then with
    // HelperFuncCallToFacadeClassRector. Nested helpers force another fix round.
    let expected = "<?php

final class FacadeFixRounds
{
    public function run(): void
    {
        app(Firewall::class, [app('request')])->isBot();
        $this->called();
    }

    private function called(): void {}
}
";
    let path = Path::new("facade-fix-rounds.php");
    let alone = Linter::new(vec![Box::new(NoUnusedPrivateMethod)], 80400)
        .fix(path, input)
        .unwrap();
    assert_eq!(alone.source, expected);

    let combined = Linter::new(
        vec![
            Box::new(NoUnusedPrivateMethod),
            Box::new(PreferFacadeOverApp),
        ],
        80400,
    )
    .fix(path, input)
    .unwrap();
    let expected = expected.replace("app(", "\\Illuminate\\Support\\Facades\\App::make(");
    assert_eq!(combined.source, expected);
    assert!(combined.remaining.is_empty());
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
