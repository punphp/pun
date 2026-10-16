<?php
#[Attr, Other(1, name: 'x')]
#[\Fully\Qualified]
final class A extends B implements C, \D\E {
    public private(set) string $name { get => $this->name; set(string $v) { $this->name = $v; } }
    protected(set) int $x = 0;
    PUBLIC(SET) ?Foo $y;
    public function __construct(public readonly int|string $id, private (A&B)|null $dnf = null) {}
    public function &ref(): static { return $this; }
    const int LIST = 1;
    enum $e;
}
enum Suit: string { case Hearts = 'H'; }
enum extends;
function f(?int ...$xs): never { yield from gen(); yield
 from x(); yield /* c */ from y(); yield fromage; }
$f = fn&(&$x) => $x & $y;
$x = $a |> strlen(...);
$o?->p?->q;
$a->class->list;
$v = (int) $a . ( float ) $b . (INTEGER)$c . (real) $d . (void) f() . (unset) $e . (binary) $s . ( bool	) $q;
$n = 0x7FFFFFFFFFFFFFFF + 0x8000000000000000 + 9223372036854775807 + 9223372036854775808 + 0b1_0 + 0o17 + 017 + 089 + 1_000.5e-3 + .5 + 1. + 1e3 + 0777777777777777777777 + 1__2;
namespace\foo(); \strlen('x'); Foo\Bar::baz(); namespace Foo\Bar;
$x = $y <=> $z ?? $w ??= 1; $a **= 2; $b <<= 1; $c >>= 1; $d .= 'x'; $e |= 1; $f ^= 1; $g &= 1;
goto end; end:
"a"; 'b\'c'; @$x; !$y; ~$z; `x`;
