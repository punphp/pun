<?php
class A {
    public $p { get => 1; set => 1; }
    public $q { get => 1; }
    public $r;
    public function a() {}
    public function getA() { return $this->a; }
    public function setA($a) { $this->a = $a; }
}
interface I { function a(); function b(); }
trait T { function a() {} function b() {} }
enum E { case X; function a() {} function b() {} }
$x = new class { function a() {} function b() {} };
