<?php
namespace A\B;
use X\Y as Z, function f, const C;
abstract class K extends P implements I, J {
    use T { a as protected b; T::c insteadof U; }
    final public const X = 1;
    private readonly int $p;
    protected static ?array $q = null;
    var $v;
    public function __construct(callable $c, iterable $i, int|string ...$r) { parent::__construct(); }
    public function m(&$x, &...$y): static { return new static(); }
}
interface I {} trait T {} enum E: string implements I { case A = 'a'; }
enum F { case B; }
$f = fn($x) => $x; $g = function () use (&$f) { yield 1; yield from [2]; };
if ($a and $b or $c xor $d) {} elseif (1) {} else {}
if (1): elseif (2): else: endif;
while (0): endwhile; for (;;): break; endfor; foreach ([] as $k => $v): continue; endforeach;
switch (1): case 1: default: endswitch; declare(ticks=1): enddeclare;
do {} while (0); try { throw new \Exception; } catch (\E $e) {} finally {}
goto L; L: echo 1; print 2; global $gg; static $ss = 1; unset($x); list($a) = [1];
isset($a); empty($a); eval('1;'); include 'a'; include_once 'a'; require 'a'; require_once 'a';
$m = match (1) { 1 => 2, default => 3 }; clone $a; $a instanceof B; exit; die(1);
__CLASS__; __TRAIT__; __FUNCTION__; __METHOD__; __LINE__; __FILE__; __DIR__; __NAMESPACE__; __PROPERTY__;
IF (1) {} ELSE {} Function g() {} ECHO 1; NEW \A; Exit(0);
(int)$a; (integer)$a; (float)$a; (double)$a; (string)$a; (binary)$a; (array)$a; (object)$a; (bool)$a; (boolean)$a;
( int )$a; (	string	)$a; (INT)$a; (Bool) $a; (unset)$a; (void)f(); (real)$a; (intx)$a; ( int $a;
$a === $b; $a !== $b; $a <=> $b; $a **= 2; f(...$a); $a <<= 1; $a >>= 1; $a ??= 1; $a?->b; $a->b; A::b; $a ?? 1;
[1 => 2]; $a++; $a--; $a == 1; $a != 1; $a <> 1; $a <= 1; $a >= 1; $a += 1; $a -= 1; $a *= 1; $a ** 1; $a /= 1;
$a .= 1; $a %= 1; $a &= 1; $a |= 1; $a ^= 1; $a |> f(...); $a || $b; $a && $b; $a << 1; $a >> 1; $a & $b; &$a;
$a = &$b; f(& $x); f(&/* c */$x); f(&// c
$x); f(&# c
$x); f(&#
$x); f(&# c
...$x); $a & /* */ 1; $a & # x
1; $a &
1;
\A\b(); namespace\x(); \strlen('a'); $a = 0x1F + 0b101 + 0o17 + 017 + 1_000 + 1.5e3 + 1E-3 + .5 + 1. + 0x7FFFFFFFFFFFFFFF + 0x8000000000000000;
$b = 9223372036854775807 + 9223372036854775808 + 0777777777777777777777 + 01777777777777777777777 + 0b111111111111111111111111111111111111111111111111111111111111111 + 0b1111111111111111111111111111111111111111111111111111111111111111;
$c = 0o777777777777777777777 + 0o1777777777777777777777 + 018 + 1e + 1e+ + 0x + 0b2 + 1_ + 0089;
#[Attr] function h() {} # comment ?> <?php // comment ?> <?php /* x */ /** doc */ /**x*/ /** */
$s = 'a\'b\\' . "a\"b $x {$y} ${z} $a[0] $a[x] $a[-1] $a[$i] $a->b $a?->b \{$x} {$a->b()}" . b'x' . B"y $x" . `ls $x`;
$h = <<<EOT
  a $x {$y} \$z \{ $a[1] $a->b
  EOT;
$n = <<<'EOT'
raw $x
EOT;
$q = <<<"EOT"
q
EOT;
$bh = b<<<EOT
x
EOT;
$e = <<<EOT
EOT;
$t = <<< EOT
EOTX
EOT;
$a <<< 1;
__halt_compiler(); raw data ?> <?php
