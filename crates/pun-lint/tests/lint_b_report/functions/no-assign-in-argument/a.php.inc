<?php

foo($a = 1);
foo(($b .= 'x'), x: ($c = 2));
foo($a == 1, $b);
$o->m($d = 3);
$f = foo($e = 1, ?);
$g = foo(x: $h = 1, y: ?);
new Bar($i ??= 2);
foo(bar($j = 1));
-----
3:5: Avoid assignments in function call arguments.
4:6: Avoid assignments in function call arguments.
4:22: Avoid assignments in function call arguments.
6:7: Avoid assignments in function call arguments.
7:10: Avoid assignments in function call arguments.
8:13: Avoid assignments in function call arguments.
9:9: Avoid assignments in function call arguments.
10:9: Avoid assignments in function call arguments.
