//! The oracle every PHP-backed test needs. A missing oracle fails the test: a skipped
//! comparison would pass without checking anything. `scripts/oracle.sh` installs both.
#![allow(dead_code, reason = "each test binary uses a subset")]

use std::path::{Path, PathBuf};
use std::process::Command;

/// `php` on PATH (PHP 8.5, like CI's oracle setup).
pub fn php() -> Command {
    let ok = Command::new("php")
        .arg("-v")
        .output()
        .is_ok_and(|o| o.status.success());
    assert!(
        ok,
        "php (the oracle) must be on PATH; run scripts/oracle.sh"
    );
    Command::new("php")
}

/// The PHP-CS-Fixer checkout the `tools/*.php` scripts load: `PUN_PHP_CS_FIXER`, else `.oracle/php-cs-fixer`.
pub fn require_php_cs_fixer() {
    let dir = std::env::var_os("PUN_PHP_CS_FIXER").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/php-cs-fixer"),
        PathBuf::from,
    );
    assert!(
        dir.join("vendor/autoload.php").exists(),
        "{} has no vendor/autoload.php; run scripts/oracle.sh or set PUN_PHP_CS_FIXER",
        dir.display()
    );
}
