<?php
function f($a, $b) {
    $x = (int) $a + (string) $b * 2.5 - 'x';
    if ($a) { return $b; } elseif ($b) { return 1; }
    if ($a): elseif ($b): endif;
    for ($i = 0; $i < 10; $i++) { $x .= $i; }
    switch ($a) { default: $x = 1; }
    try { $x = 2; } catch (E $e) { $x = 3; }
    while ($a) { $a = $a - 1; }
    do { $b = $b - 1; } while ($b);
    $g = function ($q) { return $q + 1; };
    $h = fn($q) => $q * 2 + $a - $b / 3;
    $y = -$a;
    class Inner { public function m() { return $zz + $yy; } }
    return $x;
}
class A {
    public $p { get { $a = $b + $c * $d - $e; return $a / $b + 7; } }
    public function m() { return 1; }
    public function n() { }
}
