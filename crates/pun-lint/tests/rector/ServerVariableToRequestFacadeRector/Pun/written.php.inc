<?php

// Rector writes `\Illuminate\Support\Facades\Request::server('a')++;` and `... .= $foo;` (laravel-framework's
// tests), which PHP rejects.
$_SERVER['a']++;
--$_SERVER['b'];
$_SERVER['c'] .= $foo;
$r = &$_SERVER['d'];
[$_SERVER['e']] = [1];
foreach ([1] as $_SERVER['f']) {
}
