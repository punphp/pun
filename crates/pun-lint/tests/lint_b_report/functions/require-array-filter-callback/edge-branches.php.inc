<?php

namespace App;

/**
 * @param array<true|false|null> $a
 * @param array<int<-5, -1>|0> $b
 * @param array<int<2, 5>|0> $c
 * @param array<int<-5, -2>|0> $d
 * @param array<1|0|null> $e
 */
function g(array $a, array $b, array $c, array $d, array $e, callable $fn)
{
    array_filter($a);
    array_filter($b);
    array_filter($c);
    array_filter($d);
    array_filter($e);
    array_filter(callback: null);
    array_filter($a, null, 1, 2);
    array_filter($a, array: $b);
    $fn($a);
    \strlen('x');
    array_filter(...[$a]);
    array_filter();
    array_filter(foo: 1);
    array_filter(foo: [1, 0]);
    array_filter($a, mode: 1, callback: null, array: $b);
    array_filter(array: [0, 1], callback: null);
}
-----
14: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
15: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
20: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
21: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
26: Call to function array_filter() requires parameter #2 to be passed to avoid loose comparison semantics.
28: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
29: Parameter #2 of array_filter() cannot be null to avoid loose comparison semantics (null given).
