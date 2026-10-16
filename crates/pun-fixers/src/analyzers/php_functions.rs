//! PHP's `get_defined_functions()['internal']` as seen by the oracle (system PHP 8.5.11 with its
//! extension set, see docs/oracle.md), for fixers that call `get_defined_functions` / `function_exists`.
//! Regenerate: `php -r '$f=get_defined_functions()["internal"]; sort($f); echo implode("\n", $f), "\n";'`.
const LIST: &str = include_str!("php_internal_functions.txt");

/// `function_exists($name)` for internal functions (case-insensitive, like PHP).
pub fn is_internal_function(name: &[u8]) -> bool {
    let lower = name.to_ascii_lowercase();
    static SET: std::sync::LazyLock<std::collections::HashSet<&'static [u8]>> =
        std::sync::LazyLock::new(|| LIST.lines().map(str::as_bytes).collect());
    SET.contains(&lower[..])
}
