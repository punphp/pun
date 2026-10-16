<?php
namespace App;
trait T { public function list() {} public function print() {} }
interface I { const DEFAULT = 1; const array LIST = [], CLASS_ = 2; public static function new(): static; }
enum E: string { case Default = 'd'; case Class = 'c'; const NEW = self::Default; }
enum F { case Public; case Readonly; }
class A implements I {
    use T { list as protected match; print as fn; T::list insteadof U; list as public; }
    public readonly int $x;
    public function &function() {}
    public static function new(): static { return new static(); }
    public function __construct(public readonly (A&B)|null $dnf = null) {}
}
$a = A::class . A::new() . A::DEFAULT . E::Default->value . $o::class;
foo(default: 1, class: 2, array: [3], list: 4, static: 5);
foreach ($xs as list($p, $q)) {}
foreach ($xs as $k => static::$v) {}
switch ($x) { case static::X: break; case A::B; default: }
$r = readonly(1);
$m = match ($x) { default => 1 };
$f = static fn() => $x ? self::A : static::B;
