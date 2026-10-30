<?php

namespace App;

/**
 * @param array<string, int> $a
 * @param list<int> $b
 * @param non-empty-array<int, string> $c
 * @param array{a: 1, b: 'x'} $d
 * @param array-key|null $k
 * @param numeric|null $n
 * @param non-empty-list<\stdClass> $f
 * @param array<int, array<string, bool>> $g
 * @param array<string, int>|null $h
 */
function f(string $m, array $arr, $a, $b, $c, $d, $k, $n, $f, $g, $h)
{
    $arr->$m();
    $a->$m();
    $b->$m();
    $c->$m();
    $d->$m();
    $k->$m();
    $n->$m();
    $f->$m();
    $g->$m();
    $h->$m();
    $z = [];
    $z->$m();
    $y = ['a' => 1, 'b' => 'x'];
    $y->$m();
}
-----
18: Variable method call on array.
19: Variable method call on array<string, int>.
20: Variable method call on list<int>.
21: Variable method call on array<int, string>.
22: Variable method call on array<string, int|string>.
23: Variable method call on int|string|null.
24: Variable method call on float|int|string|null.
25: Variable method call on list<stdClass>.
26: Variable method call on array<int, array<string, bool>>.
27: Variable method call on array<string, int>|null.
29: Variable method call on array.
31: Variable method call on array<string, int|string>.
