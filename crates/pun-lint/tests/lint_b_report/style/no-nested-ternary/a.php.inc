<?php

$a = $x ? ($y ? 1 : 2) : 3;
$b = $x ? 1 : ($y ? 2 : 3);
$c = ($x ? 1 : 2) ? 3 : 4;
$d = $x ? ($y ?: 2) : 3;
$e = $x ?: ($y ? 1 : 2);
$f = $x ? ((($y ? 1 : 2))) : 3;
$g = $x ? $y : $z;
$h = $x ? 1 : ($y ? 2 : ($z ? 3 : 4));
-----
3:12: Nested ternary expressions are confusing due to PHP's operator associativity rules.
4:16: Nested ternary expressions are confusing due to PHP's operator associativity rules.
5:7: Nested ternary expressions are confusing due to PHP's operator associativity rules.
8:14: Nested ternary expressions are confusing due to PHP's operator associativity rules.
10:16: Nested ternary expressions are confusing due to PHP's operator associativity rules.
10:26: Nested ternary expressions are confusing due to PHP's operator associativity rules.
