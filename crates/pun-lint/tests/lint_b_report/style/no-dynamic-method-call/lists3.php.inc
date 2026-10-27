<?php

namespace App;

function f(string $m, bool $q, bool $r)
{
    $a = $q ? [] : [1];
    $a->$m();
    $b = $q ? [1, 2] : null;
    $b->$m();
    $c = $q ? [1] : null;
    $c->$m();
    $d = $q ? ['a' => 1] : null;
    $d->$m();
    $e = $q ? [] : null;
    $e->$m();
    $f = $q ? [1] : ($r ? [1, 2] : null);
    $f->$m();
    $g = $q ? [1, 2] : 'x';
    $g->$m();
    $h = $q ? [1, 'a'] : [2, 3, 4, 5];
    $h->$m();
    $i = $q ? [2 => 1] : [1, 2];
    $i->$m();
}
-----
8: Variable method call on list<int>.
10: Variable method call on list<int>|null.
12: Variable method call on list<int>|null.
14: Variable method call on array<string, int>|null.
16: Variable method call on array|null.
18: Variable method call on list<int>|null.
20: Variable method call on list<int>|string.
22: Variable method call on list<int|string>.
24: Variable method call on array<int, int>.
