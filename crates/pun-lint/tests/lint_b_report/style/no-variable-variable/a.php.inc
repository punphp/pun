<?php

function f($name, $x) {
    $a = $$name;
    $b = $$$name;
    $c = ${'x'};
    $d = ${$x . 'y'};
    $e = $x;
    $s = "a {$x} ${x} ${x[1]} ${$x}";
    return [$a, $b, $c, $d, $e, $s];
}
-----
4: Variable variables are not allowed.
5: Variable variables are not allowed.
5: Variable variables are not allowed.
6: Variable variables are not allowed.
7: Variable variables are not allowed.
9: Variable variables are not allowed.
