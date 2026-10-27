<?php

class SomeClass {}

function f(SomeClass $o, $a, $b, $x) {
    $y = !isset($o['a'], $b);
    $y = isset($o['a'], $o['b']) || $x;
    $y = isset($o['a'], $b) ? 1 : 2;
    $y = $x and isset($o['a'], $b);
    $y = isset($o['a'], $b) == true;
    $y = isset( $b , $o['a'] , $x );
    $y = isset($o['a']);
    $y = $x . isset($o['a'], $b);
    $y = isset($o['a'], $b) xor $x;
    if ($x) unset($o['a'], $o['b']);
    if ($x)
        unset($o['a']);
    // keep
    unset($o['a'], $b);
    /** doc */
    unset( $b , $o['a'] );
    unset($o['a'],);
    while ($x) { unset($o['a'], $o['b']); }
    $fn = function () use ($o) { unset($o['z']); };
}
-----
<?php

class SomeClass {}

function f(SomeClass $o, $a, $b, $x) {
    $y = !(isset($b) && $o->has('a'));
    $y = $o->has('a') && $o->has('b') || $x;
    $y = isset($b) && $o->has('a') ? 1 : 2;
    $y = $x and isset($b) && $o->has('a');
    $y = (isset($b) && $o->has('a')) == true;
    $y = isset( $b , $x ) && $o->has('a');
    $y = $o->has('a');
    $y = $x . (isset($b) && $o->has('a'));
    $y = isset($b) && $o->has('a') xor $x;
    if ($x) {
        $o->unset('a');
        $o->unset('b');
    }
    if ($x)
        $o->unset('a');
    // keep
    unset($b);
    $o->unset('a');
    /** doc */
    unset( $b );
    $o->unset('a');
    $o->unset('a');
    while ($x) {
        $o->unset('a');
        $o->unset('b');
    }
    $fn = function () use ($o) { $o->unset('z'); };
}
