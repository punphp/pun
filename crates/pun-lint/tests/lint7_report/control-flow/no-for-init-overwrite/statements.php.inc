<?php

function a() {
    for ($i = 0; rand(); $i++) {}
    for ($i = 0; rand();) {}
}
function b() {
    for ($i = 0; $i < 10; $i++) {}
    for ($i = 0; rand();) {}
}
function c() {
    for ($i = 0, $j = 1; ; ) { if (rand()) break; }
    for ($i = 0, $j = 0; rand();) {}
}
function d() {
    while ($x = rand()) {}
    for ($x = 0; rand();) {}
    do { $y = 1; } while (rand());
    for ($y = 0; rand();) {}
    switch (rand()) { case 1: $z = 1; break; default: $z = 2; }
    for ($z = 0; rand();) {}
    if (rand()) { $e = 1; } elseif (rand()) { $e = 2; } else { $e = 3; }
    for ($e = 0; rand();) {}
    if (rand()) { return; } else { $h = 3; }
    for ($h = 0; rand();) {}
    { $blk = 1; }
    for ($blk = 0; rand();) {}
    $obj->p = 1; $$dyn = 1;
    for ($obj = 0, $$dyn = 1, $obj->p = 2; rand();) {}
    [$a, [$b]] = [1, [2]]; ['k' => $c] = ['k' => 1];
    for ([$a, [$b]] = [1, [2]], ['k' => $c] = ['k' => 2]; rand();) {}
    echo $ech = 1;
    for ($ech = 0; rand();) {}
    $m = match (rand()) { 1 => $mm = 1, default => 2 };
    for ($mm = 0; rand();) {}
    $fn = fn() => $af = 1;
    for ($af = 0; rand();) {}
    $o = $nn ?? ($nc = 1);
    for ($nc = 0; rand();) {}
}
-----
19: For loop initial assignment overwrites variable $y.
21: For loop initial assignment overwrites variable $z.
23: For loop initial assignment overwrites variable $e.
25: For loop initial assignment overwrites variable $h.
31: For loop initial assignment overwrites variable $a.
31: For loop initial assignment overwrites variable $b.
31: For loop initial assignment overwrites variable $c.
33: For loop initial assignment overwrites variable $ech.
