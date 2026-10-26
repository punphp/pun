<?php
class A { function m() { switch ($a) {} $x = match ($a) { default => 1 }; while ($a) {} do {} while ($a); foreach ($a as $b) {} if ($a) {} } }
trait T { function m() { while ($a) {} while ($a) {} } }
interface I { function m(); }
enum E { case A; function m() { foreach ($a as $b) {} foreach ($a as $b) {} } }
$c = new class { function m() { while ($a) {} while ($a) {} } };
$d = function () { while ($a) {} while ($a) {} };
function f() { while ($a) {} while ($a) {} if ($a) {} }
function g() { while ($a) {} }
