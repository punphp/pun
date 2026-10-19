<?php

function &a(): void {}
function &b(): int { static $x = 1; return $x; }
function c(): void {}
function &d() { static $x; return $x; }
class C {
    public function &m(): void {}
    public function n(): void {}
    public function &o(): array { return $this->p; }
    public function &q() { return $this->p; }
    public int $p { &get => $this->p; }
    public int $r { &set => $value; }
    public int $s { set => $value; }
}
$f = function &(): void {};
$g = function &(): int {};
$h = function &() {};
$i = function (): void {};
$j = fn &(): void => null;
$k = fn &(): int => 1;
$l = fn &() => 1;
$m = fn (): void => null;
-----
3:10: Returning by reference from a void function is deprecated since PHP 8.0.
8:21: Returning by reference from a void method is deprecated since PHP 8.0.
13:21: Returning by reference from a set property hook is deprecated since PHP 8.0.
16:15: Returning by reference from a void closure is deprecated since PHP 8.0.
20:9: Returning by reference from a void arrow function is deprecated since PHP 8.0.
