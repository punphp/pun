<?php

namespace N;

use Other\Thing;

function f($m, \Unknown $u, ?Thing $t, Missing|int $e, \Unknown&\Countable $i)
{
    $u->$m();
    $t->$m();
    $e->$m();
    $i->$m();
    /** @var \Unknown $x */
    $x->$m();
    $u?->$m();
    \Unknown::$m();
    $u::$m();
}
-----
9: Variable method call on Unknown.
10: Variable method call on Other\Thing|null.
11: Variable method call on int|N\Missing.
12: Variable method call on Countable&Unknown.
14: Variable method call on mixed.
15: Variable method call on Unknown.
