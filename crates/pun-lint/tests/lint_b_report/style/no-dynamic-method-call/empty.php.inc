<?php

namespace App;

/**
 * @param array{}|list<int> $a
 * @param list<string>|array{} $b
 * @param array{}|array<string, int> $c
 * @param array<string, int>|array{} $d
 */
function f(string $m, bool $q, $a, $b, $c, $d)
{
    $a->$m();
    $b->$m();
    $c->$m();
    $d->$m();
    $x = $q ? [] : [1, 2];
    $x->$m();
    $y = $q ? ['a' => 1] : [];
    $y->$m();
}
-----
13: Variable method call on list<int>.
14: Variable method call on list<string>.
15: Variable method call on array<string, int>.
16: Variable method call on array<string, int>.
18: Variable method call on list<int>.
20: Variable method call on array<string, int>.
