<?php

namespace App;

use function array_merge as am;

foreach ($a as $b) {
    $r = array_merge($r, $b);
    $r = array_merge_recursive($r, $b);
    $r = array_unique($r);
    $r = array_values($r);
    $r = [...$r, ...$b];
    $r = [$r, $b];
    $r = am($r, $b);
    $r = \array_values($r);
    $r = foo($r);
    $r = $f($r);
    $r[] = $b;
    $r += array_values($r);
    foreach ($b as $c) { $r = array_merge($r, $c); }
    $f = function () { $r = array_merge($r, $c); };
    $g = fn () => $r = array_merge($r, $c);
    if ($x) { $r = array_merge($r, $b); }
    new class { public function m() { $r = array_merge($r, $c); } };
    function inner() { $r = array_merge($r, $c); }
}
for (;;) $r = array_merge($r, $b);
for (;;): $r = array_unique($r); endfor;
foreach ($a as $b): $r = [...$r]; endforeach;
while (true) $r = array_values($r);
while (true): $r = array_values($r); endwhile;
do { $r = array_merge($r, $b); } while (true);
do $r = array_merge($r, $b); while (true);
while (true) { for (;;) {} while (1) {} do {} while (1); }
-----
8:10: `array_merge()` called inside a loop causes O(n²) array copying.
9:10: `array_merge_recursive()` called inside a loop causes O(n²) array copying.
10:10: `array_unique()` called inside a loop reprocesses the entire array every iteration.
11:10: `array_values()` called inside a loop reprocesses the entire array every iteration.
12:5: Array spread in reassignment inside a loop causes O(n²) array copying.
14:10: `array_merge()` called inside a loop causes O(n²) array copying.
15:10: `array_values()` called inside a loop reprocesses the entire array every iteration.
19:11: `array_values()` called inside a loop reprocesses the entire array every iteration.
20:31: `array_merge()` called inside a loop causes O(n²) array copying.
23:20: `array_merge()` called inside a loop causes O(n²) array copying.
27:15: `array_merge()` called inside a loop causes O(n²) array copying.
28:16: `array_unique()` called inside a loop reprocesses the entire array every iteration.
29:21: Array spread in reassignment inside a loop causes O(n²) array copying.
30:19: `array_values()` called inside a loop reprocesses the entire array every iteration.
31:20: `array_values()` called inside a loop reprocesses the entire array every iteration.
32:11: `array_merge()` called inside a loop causes O(n²) array copying.
33:9: `array_merge()` called inside a loop causes O(n²) array copying.
