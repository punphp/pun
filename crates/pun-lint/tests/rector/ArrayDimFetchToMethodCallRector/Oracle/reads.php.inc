<?php

class SomeClass {}

function f(SomeClass $o, $a) {
    $o[ $a+1 ];
    $o[$a] = array( 1,2 );
    $o['a'] = $o['b'];
    $o['a']['b'];
    $o['a']['b'] = 1;
    $x = $o['k'] ?? null;
    $o[] = 1;
    $o['a'] ??= 1;
    [$o['a']] = [1];
    foo($o['a']);
    isset($o['a']['b']);
    unset($o['a']['b']);
    $r = &$o['a'];
    echo "{$o['a']}";
}
-----
<?php

class SomeClass {}

function f(SomeClass $o, $a) {
    $o->get($a+1);
    $o->set($a, array( 1,2 ));
    $o->set('a', $o->get('b'));
    $o->get('a')['b'];
    $o->get('a')['b'] = 1;
    $x = $o->get('k') ?? null;
    $o[] = 1;
    $o['a'] ??= 1;
    [$o['a']] = [1];
    foo($o->get('a'));
    isset($o->get('a')['b']);
    unset($o->get('a')['b']);
    $r = &$o->get('a');
    echo "{$o->get('a')}";
}
