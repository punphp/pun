<?php

function a() {
    { $blk = 1; }
    for ($blk = 0; rand();) {}
}
function b() {
    $i = 1;
    for ($i = 0; rand(); $i++) {}
    for ($i = 0; rand();) {}
}
function c() {
    foreach ([1] as $v) { $in = 1; }
    for ($in = 0; rand();) {}
    $w = 1;
    while (rand()) { $w2 = 1; }
    for ($w = 0, $w2 = 0; rand();) {}
}
function d() {
    try { $t = rand(); } finally { $f = 1; }
    for ($t = 0, $f = 0; rand();) {}
}
function e() {
    try { $t = rand(); } catch (Exception $e) { $t = 2; }
    for ($t = 0; rand();) {}
    try { $u = rand(); } catch (Exception $e) { return; }
    for ($u = 0; rand();) {}
}
function f() {
    if (rand()) { $a = 1; } elseif (rand()) { throw new Exception(); } else { exit; }
    for ($a = 0; rand();) {}
    if (rand()): $b = 1; else: $b = 2; endif;
    for ($b = 0; rand();) {}
    if (rand()) $c = 1; else $c = 2;
    for ($c = 0; rand();) {}
    switch (rand()) { case 1: $d = 1; break; }
    for ($d = 0; rand();) {}
    switch (rand()) { case 1: $g = 1; default: $h = 1; }
    for ($g = 0, $h = 0; rand();) {}
}
function g() {
    $x = 1;
    if (rand()) { unset($x); }
    for ($x = 0; rand();) {}
    $y = 1;
    unset($y);
    $y = 2;
    for ($y = 0; rand();) {}
    $z = &$foo;
    for ($z = 0; rand();) {}
    $l = [1];
    foreach ($l as $l) {}
    for ($l = 0; rand();) {}
}
-----
9: For loop initial assignment overwrites variable $i.
10: For loop initial assignment overwrites variable $i.
17: For loop initial assignment overwrites variable $w.
21: For loop initial assignment overwrites variable $f.
21: For loop initial assignment overwrites variable $t.
25: For loop initial assignment overwrites variable $t.
27: For loop initial assignment overwrites variable $u.
31: For loop initial assignment overwrites variable $a.
33: For loop initial assignment overwrites variable $b.
35: For loop initial assignment overwrites variable $c.
39: For loop initial assignment overwrites variable $h.
48: For loop initial assignment overwrites variable $y.
50: For loop initial assignment overwrites variable $z.
53: For loop initial assignment overwrites variable $l.
