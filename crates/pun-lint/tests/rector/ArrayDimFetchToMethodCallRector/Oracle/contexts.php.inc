<?php

class SomeClass {}

function f(SomeClass $o, $a, $b) {
    foreach ($a as $o['k']) {}
    foreach ($a as $o['k'] => $v) {}
    list($o['a'], $o['b']) = $a;
    [$o['a'], [$o['b']]] = $a;
    [ 'x' => $o['a'] ] = $a;
    $b = &$o['a'];
    sort($o['a']);
    $o['a'] .= 'x';
    $c = $o['a'] = 2;
    $o['a'] = $o['b'] = 3;
    isset($o['a'], $b);
    isset($b, $o['a']);
    isset($o['a']) && true;
    unset($b, $o['a'], $c);
    $fn = fn() => $o['a'];
    $o['a']->m();
    $o['a']::C;
    $o['a'][] = 1;
    empty($o['a']);
    $o?->x['a'];
    (clone $o)['a'];
}
-----
<?php

class SomeClass {}

function f(SomeClass $o, $a, $b) {
    foreach ($a as $o['k']) {}
    foreach ($a as $o['k'] => $v) {}
    list($o['a'], $o['b']) = $a;
    [$o['a'], [$o->get('b')]] = $a;
    [ 'x' => $o['a'] ] = $a;
    $b = &$o->get('a');
    sort($o->get('a'));
    $o['a'] .= 'x';
    $c = $o->set('a', 2);
    $o->set('a', $o->set('b', 3));
    isset($b) && $o->has('a');
    isset($b) && $o->has('a');
    $o->has('a') && true;
    unset($b, $c);
    $o->unset('a');
    $fn = fn() => $o->get('a');
    $o->get('a')->m();
    $o->get('a')::C;
    $o->get('a')[] = 1;
    empty($o->get('a'));
    $o?->x['a'];
    (clone $o)->get('a');
}
