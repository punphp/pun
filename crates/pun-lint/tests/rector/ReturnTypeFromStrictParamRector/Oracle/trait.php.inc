<?php

namespace App;

trait T
{
    private string $name = '';

    public function a() { return strtoupper('x'); }
    public function b() { return $this->name; }
    public function c($x) { return 'a' . $x; }
    public function d() { return []; }
    public function e($x) { return strtoupper($x); }
    public function f() { return $this->g(); }
    public function g(): string { return ''; }
    public function h($x) { $y = 'a'; return $y; }
    public function i() { return static::class; }
    public function j($x) { return $x ? 'a' : 'b'; }
}

class C
{
    private string $name = '';

    public function a() { return strtoupper('x'); }
    public function b() { return $this->name; }
    public function c($x) { return 'a' . $x; }
    public function d() { return []; }
    public function e($x) { return strtoupper($x); }
    public function f() { return $this->g(); }
    public function g(): string { return ''; }
    public function h($x) { $y = 'a'; return $y; }
    public function i() { return static::class; }
    public function j($x) { return $x ? 'a' : 'b'; }
}
