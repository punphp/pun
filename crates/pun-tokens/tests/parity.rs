//! Edge-case fixtures must tokenize identically to PHP in all three modes (needs `php`;
//! the fixer mode also needs .oracle/php-cs-fixer). Skipped when unavailable.

#[path = "../examples/parity.rs"]
#[allow(dead_code)]
mod parity;

#[test]
fn fixtures_match_php() {
    if std::process::Command::new("php").arg("-v").output().is_err() {
        return;
    }
    let fixtures = vec![concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures").to_string()];
    let oracle = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.oracle/php-cs-fixer/vendor/autoload.php");
    for mode in ["raw", "parse", "fixer"] {
        if mode == "fixer" && std::env::var("PUN_PHP_CS_FIXER").is_err() && !std::path::Path::new(oracle).exists() {
            continue;
        }
        assert_eq!(parity::run(mode, &fixtures, false), 0, "mode {mode}");
    }
}
