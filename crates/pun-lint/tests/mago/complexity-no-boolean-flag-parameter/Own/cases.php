<?php
abstract class A {
    public function __construct(bool $a, public bool $p) { if ($a) {} }
    public function setFlag(bool $a) { if ($a) {} }
    public function set(bool $a) { if ($a) {} }
    abstract public function m(bool $a);
    public function n(?bool $a, bool|int $b, int $c, bool $d) { if ($a || $b || $c) {} $f = fn(bool $e) => 1; $g = function (bool $h) { return $h ? 1 : 2; }; if ($d): elseif ($d): endif; }
    public function o(bool $a, bool $b) { if ($x): elseif ($a): endif; if ($y) {} elseif ($b) {} }
}
interface I { public function m(bool $a); }
function f(bool $a) { $g = function () use ($a) { if ($a) {} }; }
function g(bool $a) { if ($a && $a) {} }
