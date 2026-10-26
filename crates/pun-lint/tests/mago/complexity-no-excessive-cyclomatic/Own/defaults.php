<?php
interface I { public function a(); public function b(); }
abstract class A {
    abstract public function x();
    public function getA() { return $this->a; }
    public function setA($a) { $this->a = $a; }
    public function withA($a) { $this->a = $a; return $this; }
    public function notSetter($a) { $this->a = $a; return $a; }
    public function notGetter($b) { return $this->a; }
    public function m($a) {
        if ($a) {} elseif ($b) {}
        if ($a): elseif ($b): endif;
        for (;;) {} foreach ($a as $b) {} while ($a) {} do {} while ($a);
        try {} catch (A $e) {} catch (B $e) {}
        $x = $a ? 1 : 2;
        $x = $a && $b || $c and $d or $e xor $f;
        $x = $a ?? $b;
        $x = $a <=> $b;
        switch ($a) { case 1: break; default: break; }
        $f = fn() => $a ? 1 : 2;
    }
}
trait T { public function m() { if ($a) {} if ($b) {} if ($c) {} } }
enum E { case A; public function m() { if ($a) {} if ($b) {} if ($c) {} } }
$c = new class { public function m() { if ($a) {} if ($b) {} if ($c) {} } };
$d = function () { if ($a) {} if ($b) {} if ($c) {} };
function f() { if ($a) {} if ($b) {} if ($c) {} }
function g() {}
