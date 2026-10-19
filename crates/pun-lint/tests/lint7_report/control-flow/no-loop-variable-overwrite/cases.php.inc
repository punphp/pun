<?php

function a(array $items, $key) {
    foreach ($items as $key => $value) {}
    $value = 1;
    foreach ($items as $value) {}
    foreach ($items as $k => $v) {
        foreach ($items as $k => [$v, $w]) {}
        foreach ($items as &$v) {}
    }
    $pair = [1, 2]; $x = 1;
    foreach ($items as list($pair, , $x)) {}
    foreach ($items as ['a' => $x, 'b' => $items]) {}
    foreach ($items as $items) {}
    foreach ([1] as $fresh) {}
    foreach ([1] as $fresh) {}
    foreach ($items as $this2->prop) {}
    foreach ($items as $$dyn) {}
    if (rand()) { $m = 1; }
    foreach ($items as $m) {}
}
-----
4: Foreach overwrites $key with its key variable.
6: Foreach overwrites $value with its value variable.
8: Foreach overwrites $k with its key variable.
8: Foreach overwrites $v with its value variable.
9: Foreach overwrites $v with its value variable.
12: Foreach overwrites $pair with its value variable.
12: Foreach overwrites $x with its value variable.
13: Foreach overwrites $items with its value variable.
13: Foreach overwrites $x with its value variable.
14: Foreach overwrites $items with its value variable.
