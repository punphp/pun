<?php
use A\{function f, const C, D};
use function g;
function h() {} const K = 1;
// trailing spaces   
# hash trailing	
$a = "{$a->{'b'}} {$x[f("{$y}")]}";
class P {
    public $a { #[A] get => 1; #[B] set(int $v) { $this->a = $v; } }
    public $b { set($v) => $v; }
    public function __construct($noPromotion) { if (1) { get(); } { set + 1; } { foo; } }
    public function m() { return new class(1, new class {}) { public $x; }; }
    public function n() { $c = new class { public $y; }; $d = A::class; }
}
A::{'X'}; A::{'X'}::{'Y'}; A::{'X'}[0]; $z = A::{'X'} ?>
<?php
[$k => [$x]] = $y; ['a' => [$x]] = $y; foreach ($a as $k => [$x]) {} foreach ($a as [$x]) {} [[$x]] = $y;
foo(new A); (new A)->b(); [new A]; $f(new A); if (new A) {} (new A());
$g = fn(int ...$a) => $a | $b; function i(A&B &$x, (A&B)|C $y, C|(A&B) $z, ?int $n = null) {}
$h = fn(A|B &$x) => 1; $m = A | B; function j(A|B ...$c) {} $j = $a & $b; try {} catch (A|B $e) {}
enum E { const X = 1; public function f() {} case B; }
class Q { public function __construct(public readonly int $a, final public $b = 1, public private(set) int $c = 2, protected(set) int $d = 3) {} }
interface I { const ?int X = null; } abstract class R { static ?int $s; abstract ?int $t { get; } }
$k = $x instanceof static ? 1 : 2; f(a: 1, b: 2); $l = function () use ($a): ?int {};
trait T { use U; public function f() { return function () use ($x) {}; } }
class S { const F = static function () {}; public $g = 1; }
