<?php

$a = preg_quote($s);
$b = preg_quote($s, '/');
$c = PREG_QUOTE($s);
$d = preg_quote();
$e = \preg_quote($s);
$f = other($s);
$g = preg_quote(...$args);
-----
3:6: Missing delimiter argument in `preg_quote()` call
5:6: Missing delimiter argument in `preg_quote()` call
6:6: Missing delimiter argument in `preg_quote()` call
7:6: Missing delimiter argument in `preg_quote()` call
9:6: Missing delimiter argument in `preg_quote()` call
