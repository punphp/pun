<?php

class SomeClass {}

function f(SomeClass $o, $a) {
    foreach ($a as [$o['x'], $o['y']]) {}
    foreach ($a as $o['a']['b']) {}
    foreach ($o['a'] as $v) {}
    foreach ($a as $k => [$o['x']]) {}
    $b = [$o['a'], [$o['b']]];
    [[$o['a']]] = $a;
    [$o['a']['b']] = $a;
}
-----
<?php

class SomeClass {}

function f(SomeClass $o, $a) {
    foreach ($a as [$o['x'], $o['y']]) {}
    foreach ($a as $o['a']['b']) {}
    foreach ($o->get('a') as $v) {}
    foreach ($a as $k => [$o['x']]) {}
    $b = [$o->get('a'), [$o->get('b')]];
    [[$o->get('a')]] = $a;
    [$o->get('a')['b']] = $a;
}
