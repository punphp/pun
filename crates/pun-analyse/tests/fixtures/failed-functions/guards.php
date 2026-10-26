<?php
if (function_exists('pun_guarded_missing_function')) {
    \PHPStan\dumpType(pun_guarded_missing_function());
}
if (function_exists('PUN_GUARDED_CASE_FUNCTION')) {
    \PHPStan\dumpType(pun_guarded_case_function());
}
if (function_exists('pun_guarded_qualified_function')) {
    \PHPStan\dumpType(\pun_guarded_qualified_function());
}
