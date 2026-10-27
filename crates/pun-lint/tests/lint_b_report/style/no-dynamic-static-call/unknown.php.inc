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
16: Variable static method call on Unknown.
17: Variable static method call on Unknown.
