<?php

$a = max(0, min($v, 99999999999999999999));
$b = max(0, min($v, 0x8000000000000000));
$c = max(0, min($v, 0b11111111111111111111111111111111111111111111111111111111111111111));
$d = max(0, min($v, 0o7777777777777777777777));
$e = max(0, min($v, 07777777777777777777777));
$f = max(0, min($v, 9_999_999_999_999_999_999));
$g = max(false, min($v, 10));
$h = max(null, min($v, 10));
$i = max(-1.5, min($v, 10));
$j = max(-'a', min($v, 10));
$k = max(+1, min($v, 10));
$l = max(PHP_FLOAT_EPSILON, min($v, PHP_FLOAT_MAX));
$m = max(PHP_FLOAT_MIN, min($v, PHP_FLOAT_DIG));
$n = max(PHP_INT_SIZE, min($v, \PHP_INT_MAX));
$o = max(-PHP_INT_MIN, min($v, 10));
$p = max(0, min($v, ...$rest));
$q = max(0, min(value: $v, max: 10));
$r = max(0, min($v, "a$b"));
$s = max(0, min($v, ~5));
$t = max(0, min($v, ~1.5));
$u = max(0, min($v, ~'a'));
$w = max(0, min($v, ~true));
-----
<?php

$a = clamp($v, 0, 99999999999999999999);
$b = clamp($v, 0, 0x8000000000000000);
$c = clamp($v, 0, 0b11111111111111111111111111111111111111111111111111111111111111111);
$d = clamp($v, 0, 0o7777777777777777777777);
$e = clamp($v, 0, 07777777777777777777777);
$f = clamp($v, 0, 9_999_999_999_999_999_999);
$g = clamp($v, false, 10);
$h = clamp($v, null, 10);
$i = clamp($v, -1.5, 10);
$j = clamp($v, -'a', 10);
$k = clamp($v, +1, 10);
$l = max(PHP_FLOAT_EPSILON, min($v, PHP_FLOAT_MAX));
$m = max(PHP_FLOAT_MIN, min($v, PHP_FLOAT_DIG));
$n = clamp($v, PHP_INT_SIZE, \PHP_INT_MAX);
$o = clamp($v, -PHP_INT_MIN, 10);
$p = max(0, min($v, ...$rest));
$q = max(0, min(value: $v, max: 10));
$r = max(0, min($v, "a$b"));
$s = clamp($v, 0, ~5);
$t = clamp($v, 0, ~1.5);
$u = clamp($v, 0, ~'a');
$w = max(0, min($v, ~true));
