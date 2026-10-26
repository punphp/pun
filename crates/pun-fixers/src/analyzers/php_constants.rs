//! PHP's `get_defined_constants(true)` without the `user` group, as seen by the oracle (PHP 8.5 with the extension set
//! of php_internal_functions.txt, see context/reference/oracle.md), for native_constant_invocation's `fix_built_in`.
//! Regenerate with the oracle's PHP:
//! `php -r '$c=get_defined_constants(true); unset($c["user"]); foreach($c as $x) foreach(array_keys($x) as $k) echo $k, "\n";' | LC_ALL=C sort -u`.
use std::collections::HashSet;
use std::sync::LazyLock;

const LIST: &str = include_str!("php_internal_constants.txt");

/// The constant names (case-sensitive, like the array keys upstream reads).
pub(crate) static INTERNAL_CONSTANTS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| LIST.lines().collect());
