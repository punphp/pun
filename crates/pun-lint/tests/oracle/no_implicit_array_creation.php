<?php
function f1() { $a[] = 1; }
function f2($c) { if ($c) { $b = []; } $b['k'] = 1; }
function f3() { $x = []; $x['a']['b'] = 1; }
function f4() { $n['a']['b'][] = 1; }
function f5(array $p) { $p[] = 1; }
function f6($c) { foreach ($c as $v) { $acc[$v] = 1; } }
function f7() { $s = 1; $s[] = 2; $_SESSION['a'] = 1; $z .= 'x'; $y[0] .= 'x'; }
function f8($c) { while ($c) { $w[] = 1; $w = []; } }
function f9() { try { $t = []; } catch (\Exception $e) {} $t[] = 1; }
$top[] = 1;
$g = 1;
$g[] = 2;
function f10() { static $st; $st[] = 1; global $gl; $gl[] = 1; }
function f11() { $u = null; $u[] = 1; }
