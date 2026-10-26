<?php

class Foo {
    public function __construct() {}
    public function __destruct() {}
    public function __toString(): string { return ''; }
    public function __get($name) {}
    public function __set($name, $value) {}
}
