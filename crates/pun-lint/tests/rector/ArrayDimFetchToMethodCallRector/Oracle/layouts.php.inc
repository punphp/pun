<?php

class SomeClass {}

function f(SomeClass $o, $b, $x) {
    $y = 1;
    // all
    unset($o['a'], $o['b']);
    while ($x) { unset($o['a'], $b); }
    while ($x) { $y = 2; unset($o['a'], $o['b']); }
    while ($x) { unset($o['a']); }
    if ($x) { unset($o['a'], $o['b']); } else { $y = 3; }
}

function h(SomeClass $o) {
    unset($o['a'], $o['b']); // trailing
    $z = $o['c'];
}
-----
<?php

class SomeClass {}

function f(SomeClass $o, $b, $x) {
    $y = 1;
    $o->unset('a');
    $o->unset('b');
    while ($x) { unset($b);
    $o->unset('a'); }
    while ($x) { $y = 2;
    $o->unset('a');
    $o->unset('b'); }
    while ($x) { $o->unset('a'); }
    if ($x) {
        $o->unset('a');
        $o->unset('b');
    } else { $y = 3; }
}

function h(SomeClass $o) {
    $o->unset('a');
    $o->unset('b');
    // trailing
    $z = $o->get('c');
}
