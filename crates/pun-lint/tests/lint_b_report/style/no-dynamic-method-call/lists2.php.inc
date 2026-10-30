<?php

namespace App;

/**
 * @param list<int> $l
 * @param array<int, int> $a
 */
function f(string $m, bool $q, bool $r, $l, $a)
{
    $z = $q ? [1, 2] : $l;
    $z->$m();
    $w = $q ? [] : ($r ? [1] : [1, 2]);
    $w->$m();
    $v = $q ? [1] : [1, 2, 3];
    $v->$m();
    $u = $q ? [] : $l;
    $u->$m();
    $t = $q ? [1, 2] : $a;
    $t->$m();
    $s = $q ? [] : ['a' => 1];
    $s->$m();
    $p = $q ? ['a' => 1] : [1, 2, 3];
    $p->$m();
    $o = $q ? [] : [1, 'a' => 2, 3];
    $o->$m();
}
-----
12: Variable method call on list<int>.
14: Variable method call on list<int>.
16: Variable method call on list<int>.
18: Variable method call on list<int>.
20: Variable method call on array<int, int>.
22: Variable method call on array<string, int>.
24: Variable method call on array<int|string, int>.
26: Variable method call on array<int|string, int>.
