<?php

class SomeClass {}

function g2(SomeClass $o, $x) {
    while ($x) { unset($o['a'], $o['b']); }
}

function g3(SomeClass $o, $x) {
    $y = 1;

    unset($o['a'], $o['b']);
}
-----
<?php

class SomeClass {}

function g2(SomeClass $o, $x) {
    while ($x) {
        $o->unset('a');
        $o->unset('b');
    }
}

function g3(SomeClass $o, $x) {
    $y = 1;
    $o->unset('a');
    $o->unset('b');
}
