//! Rector's own fixtures for the ported docblock `types` rules (Rector 2.6.1, rules-tests/TypeDeclarationDocblocks and
//! the docblock rules of rules-tests/TypeDeclaration), copied verbatim into tests/rector/<Rector class>/Fixture with
//! the classes they use in Source/. Oracle/ holds extra inputs with what Rector 2.6.1 (the rule alone,
//! `phpVersion(80400)`) wrote for them. Each must give the expected bytes, and the same after the laravel preset's
//! format.

mod lint_c;

use lint_c::run_class;
use pun_lint::Rule;
use pun_lint::rules::AddReturnPhpdocDirectArray;

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
