<?php

function f() {
    global $a, $b;
    $x = $GLOBALS['a'];
    $y = $globals;
}
-----
4:5: Unsafe use of `global` keyword.
5:10: Unsafe use of `$GLOBALS` variable.
