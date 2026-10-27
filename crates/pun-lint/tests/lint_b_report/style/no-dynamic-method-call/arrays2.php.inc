<?php

namespace App;

/**
 * @param array<int> $a
 * @param array{a: int, ...<string, bool>} $b
 * @param array<array-key, string> $d
 * @param array{} $e
 */
function f(string $m, $a, $b, $d, $e, string $k)
{
    $a->$m();
    $b->$m();
    $d->$m();
    $e->$m();
    $x = [1 => 'a', 5 => true];
    $x->$m();
    $y = ['a' => 1];
    $y[$k] = 2.5;
    $y->$m();
    $z = [1];
    $z[] = 'b';
    $z->$m();
}
-----
13: Variable method call on array<int>.
14: Variable method call on array<string, bool|int>.
15: Variable method call on array<string>.
16: Variable method call on array.
18: Variable method call on array<int, string|true>.
21: Variable method call on array<string, float|int>.
24: Variable method call on array<int, int|string>.
