<?php

function a($items, $key, $e, $g, $s, $other) {
    foreach ($items as $key => $items) {}
    foreach ($other as $key) {}
    try {} catch (Exception $e) {}
    try {} catch (Exception) {}
    global $g, $$name;
    static $s = 1, $t;
    $f = function ($x) use ($items) { foreach ($x as $items) {} };
    $h = fn ($y) => $y;
    foreach ($a as [$key]) {}
    foreach ($a as $o->key) {}
    new class { public function m($z) { foreach ($z as $z) {} } };
}

function b() { foreach ($a as $a) {} }

function c($a, $a) { foreach ($b as $a) {} }

class K {
    abstract public function m($a);
    public function n($a) { foreach ($b as $a) {} }
    public string $p { set($value) { foreach ($v as $value) {} } }
}

$c = function ($a) { if ($x) { while (true) { global $a; } } };
$d = static function ($a) { $a = 1; };
function e($x) {
    function inner() { foreach ($b as $x) {} }
    new class { public $p { set($v) { foreach ($a as $x) {} } } };
}
-----
3:12: Parameter `$items` is shadowed by a loop, catch, `global`, or `static` variable.
3:20: Parameter `$key` is shadowed by a loop, catch, `global`, or `static` variable.
3:26: Parameter `$e` is shadowed by a loop, catch, `global`, or `static` variable.
3:30: Parameter `$g` is shadowed by a loop, catch, `global`, or `static` variable.
3:34: Parameter `$s` is shadowed by a loop, catch, `global`, or `static` variable.
14:35: Parameter `$z` is shadowed by a loop, catch, `global`, or `static` variable.
19:16: Parameter `$a` is shadowed by a loop, catch, `global`, or `static` variable.
23:23: Parameter `$a` is shadowed by a loop, catch, `global`, or `static` variable.
27:16: Parameter `$a` is shadowed by a loop, catch, `global`, or `static` variable.
