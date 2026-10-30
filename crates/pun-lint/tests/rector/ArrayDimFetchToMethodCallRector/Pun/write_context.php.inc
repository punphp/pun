<?php

// Rector writes `$o->get('a')++`, `--$o->get('a')` and `$o->get('a') = &$b`, which PHP rejects ("Can't use method
// return value in write context"). Pun leaves an access that `++`, `--` or `=&` writes alone.

class SomeClass {}

function f(SomeClass $o, $b) {
    $o['a']++;
    --$o['a'];
    $o['a'] = &$b;
}
