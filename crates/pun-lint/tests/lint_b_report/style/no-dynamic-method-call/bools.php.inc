<?php

namespace App;

/**
 * @param true|null $a
 * @param false|int $b
 * @param true|false|null $c
 * @param list<true|null> $d
 * @param bool|null $e
 */
function f(string $m, $a, $b, $c, $d, $e)
{
    $a->$m();
    $b->$m();
    $c->$m();
    $d->$m();
    $e->$m();
}
-----
14: Variable method call on true|null.
15: Variable method call on int|false.
16: Variable method call on bool|null.
17: Variable method call on list<true|null>.
18: Variable method call on bool|null.
