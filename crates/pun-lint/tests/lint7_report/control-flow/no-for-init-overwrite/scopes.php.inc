<?php

function a(int $n) {
    for ($i = 0; rand(); $i++) {}
    for ($i = 0, $j = $i; rand(); $i++) {}
    if (rand()) { $k = 1; } else { $k = 2; }
    for ($k = 0; rand();) {}
    if (rand()) { $m = 1; }
    for ($m = 0; rand();) {}
    $u = 1; unset($u);
    for ($u = 0; rand();) {}
    foreach ([1] as $key => [$v, $w]) {
        for ($key = 0, [$v, $z] = [1, 2]; $v < 1;) {}
    }
    for ($w = 0; rand();) {}
    rand() && ($q = 1);
    for ($q = 0; rand();) {}
    $r = rand() ? ($s = 1) : 2;
    for ($s = 0, $r = 1; rand();) {}
    $arr['x'] = 1;
    for ($arr = 0; rand();) {}
    $t .= 'x';
    for ($t = 0; rand();) {}
    $f = function ($p) use ($n) { for ($p = 0, $n = 1, $a = 2; rand();) {} };
    try { foo(); } catch (\Exception $e) { for ($e = 0; rand();) {} }
    if ($cc = rand()) {} else { for ($cc = 0; rand();) {} }
    while ($ww = rand()) { for ($ww = 0; rand();) {} }
    global $g; static $st = 1;
    for ($g = 0, $st = 0, $_GET = 1, $this2 = 1; rand();) {}
    for ($cnt = 0; rand(); $cnt++) { $inner = 1; }
    for ($inner = 0; rand();) {}
    for ($n = 0; rand(); $n++) {}
    list($l1, $l2) = [1, 2];
    for ([$l1] = [1], list(, $l2) = [1, 2]; rand();) {}
    foo($fa = 1);
    for ($fa = 0; rand();) {}
    $x1 = $x2 = 1;
    for ($x2 = 0; rand();) {}
    for ($x1 += 0; rand();) {}
}
$top = 1;
for ($top = 0; rand();) {}
-----
7: For loop initial assignment overwrites variable $k.
13: For loop initial assignment overwrites variable $key.
13: For loop initial assignment overwrites variable $v.
19: For loop initial assignment overwrites variable $r.
21: For loop initial assignment overwrites variable $arr.
23: For loop initial assignment overwrites variable $t.
24: For loop initial assignment overwrites variable $n.
24: For loop initial assignment overwrites variable $p.
25: For loop initial assignment overwrites variable $e.
26: For loop initial assignment overwrites variable $cc.
27: For loop initial assignment overwrites variable $ww.
29: For loop initial assignment overwrites variable $_GET.
29: For loop initial assignment overwrites variable $g.
29: For loop initial assignment overwrites variable $st.
32: For loop initial assignment overwrites variable $n.
34: For loop initial assignment overwrites variable $l1.
34: For loop initial assignment overwrites variable $l2.
36: For loop initial assignment overwrites variable $fa.
38: For loop initial assignment overwrites variable $x2.
42: For loop initial assignment overwrites variable $top.
