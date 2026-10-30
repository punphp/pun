<?php

namespace App;

function f(string $m, bool $q)
{
    $z = [1, 2];
    $z->$m();
    $w = $q ? [1] : [1, 2];
    $w->$m();
    $v = $q ? [] : [1 => 1];
    $v->$m();
    $u = $q ? [] : ['a', 'b'];
    $u->$m();
    $t = $q ? [] : [0 => 'a', 'k' => 2];
    $t->$m();
}
-----
8: Variable method call on array<int, int>.
10: Variable method call on array<int, int>.
12: Variable method call on array<int, int>.
14: Variable method call on list<string>.
16: Variable method call on array<int|string, int|string>.
