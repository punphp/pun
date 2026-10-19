<?php

$a = explode($string, ',');
$b = explode(',', $string);
$c = explode('a long separator', ',');
$d = explode(',', ',');
$e = explode(',,', ',');
$f = explode($s, 'a string that is long');
$g = explode(...$args);
$h = explode($s, ...$args);
$i = EXPLODE($s, '-');
$j = \explode($s, '-');
$k = explode(separator: $s, string: '-');
$l = explode($s, "-$x");
$m = explode("$x", '-');
-----
3:23: The arguments to `explode` may be swapped.
5:34: The arguments to `explode` may be swapped.
7:20: The arguments to `explode` may be swapped.
11:18: The arguments to `explode` may be swapped.
12:19: The arguments to `explode` may be swapped.
15:20: The arguments to `explode` may be swapped.
