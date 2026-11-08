//! The oracle every PHP-backed test needs. A missing oracle fails the test: a skipped
//! comparison would pass without checking anything. `scripts/oracle.sh` installs both.
#![allow(dead_code, reason = "each test binary uses a subset")]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// The PHP the oracle scripts run on: PHP 8.5, which `token_get_all` and tools/api.php need (8.5 syntax such as a
/// `final` promoted parameter). `PUN_PHP` names it; else the first `php`, `php8.5` (Debian) or `php85` (Herd) on
/// PATH that reports 8.5. Anything else fails the test with the versions it found.
pub(crate) fn php_path() -> PathBuf {
    static PHP: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    PHP.get_or_init(find_php)
        .clone()
        .unwrap_or_else(|e| panic!("{e}"))
}

/// `php_path()` as a command.
pub(crate) fn php() -> Command {
    Command::new(php_path())
}

fn find_php() -> Result<PathBuf, String> {
    if let Some(php) = std::env::var_os("PUN_PHP") {
        let php = PathBuf::from(php);
        return match version(&php) {
            Some(v) if v == "8.5" => Ok(php),
            v => Err(format!(
                "PUN_PHP={} is PHP {}, the oracle needs PHP 8.5",
                php.display(),
                v.as_deref().unwrap_or("(does not run)")
            )),
        };
    }
    let mut found = Vec::new();
    for dir in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        for name in ["php", "php8.5", "php85"] {
            let php = dir.join(name);
            if !php.is_file() {
                continue;
            }
            match version(&php) {
                Some(v) if v == "8.5" => return Ok(php),
                v => found.push(format!(
                    "{} ({})",
                    php.display(),
                    v.as_deref().unwrap_or("does not run")
                )),
            }
        }
    }
    Err(format!(
        "the oracle needs PHP 8.5 on PATH (as php, php8.5 or php85) or in PUN_PHP; found {}",
        if found.is_empty() {
            "no php".to_string()
        } else {
            found.join(", ")
        }
    ))
}

/// `MAJOR.MINOR` of the PHP at `php`, or `None` when it does not run.
fn version(php: &Path) -> Option<String> {
    let out = Command::new(php)
        .args(["-r", "echo PHP_MAJOR_VERSION, '.', PHP_MINOR_VERSION;"])
        .output()
        .ok()
        .filter(|o| o.status.success())?;
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The PHP-CS-Fixer checkout the `tools/*.php` scripts load: `PUN_PHP_CS_FIXER`, else `.oracle/php-cs-fixer`.
pub(crate) fn require_php_cs_fixer() {
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
