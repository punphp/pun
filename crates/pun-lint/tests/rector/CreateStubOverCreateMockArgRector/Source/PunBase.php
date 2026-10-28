<?php
namespace Pun\Ns4;
abstract class Base extends \PHPUnit\Framework\TestCase { }
class Plain { public function __construct($m = null) {} }
class Child extends Plain {
    public function make() { }
}
