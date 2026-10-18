//! Edge-case fixtures must tokenize identically to PHP in all four modes (needs `php`; the fixer and
//! classy modes also need .oracle/php-cs-fixer or PUN_PHP_CS_FIXER).

mod common;

#[path = "../examples/parity.rs"]
#[expect(dead_code, reason = "the example's `main` is not called here")]
mod parity;

#[test]
fn fixtures_match_php() {
    common::php();
    common::require_php_cs_fixer();
    let fixtures = vec![concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures").to_string()];
    for mode in ["raw", "parse", "fixer", "classy"] {
        assert_eq!(parity::run(mode, &fixtures, false), 0, "mode {mode}");
    }
}
