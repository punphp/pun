<?php

function test(int &$out): void {
    global $out;

    $out = 42;
}
-----
3:20: Parameter `$out` is shadowed by a loop, catch, `global`, or `static` variable.
