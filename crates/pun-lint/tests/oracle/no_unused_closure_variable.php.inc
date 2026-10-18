<?php
$f = function () use ($a, $b, $c,) { return $b; };
$g = function () use (
    $a,
    $b
) { return $a; };
$h = function () use ($a, &$b, $c) { return $c; };
$i = function () use ($a): int { return 1; };
$j = static function () use ($a, $b) { return $$name; };
$k = function () use ($a) { include 'x.php'; };
$l = function () use ($a, $b) { return compact('b'); };
$m = function () use ($A) { return $a; };
$n = function () use ($a) { return fn () => $a; };
$o = function () use ($a) { return function () use ($a) { return $a; }; };
$p = function ($x) use ($a) { return $x; };
$q = function () use ($a) {};
-----
<?php
$f = function () use ($b,) { return $b; };
$g = function () use (
    $a
) { return $a; };
$h = function () use ($c) { return $c; };
$i = function (): int {
    return 1;
};
$j = static function () use ($a, $b) { return $$name; };
$k = function () use ($a) { include 'x.php'; };
$l = function () use ($b) { return compact('b'); };
$m = function () use ($A) { return $a; };
$n = function () use ($a) { return fn () => $a; };
$o = function () use ($a) { return function () use ($a) { return $a; }; };
$p = function ($x) {
    return $x;
};
$q = function () {
};
