<?php
class A {
    public function __construct($a, $b, $c) { $f = function ($a, $b, $c) {}; $g = fn($a, $b, $c) => 1; }
    public function __CONSTRUCT2($a, $b, $c) {}
    abstract function m($a, $b, $c);
}
function f($a, $b, $c) {}
function g($a, $b) {}
