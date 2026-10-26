<?php
class A { public $a, $b; public $c { get => 1; } const X = 1; function m() {} }
interface I { public $a { get; } public $b { get; } }
trait T { public $a; public $b; }
$x = new class { public $a; public $b; };
enum E { case A; }
