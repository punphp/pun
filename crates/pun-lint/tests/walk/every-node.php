<?php
declare(strict_types=1);
namespace App\Models;
use Foo\Bar, Foo\Baz as Qux;
use function Foo\f;
use const Foo\{C, D};
use Foo\{function g, const H, Klass};

#[Attr(1, name: 2)]
abstract class A extends B implements I, J
{
    use T1, T2 {
        T1::m insteadof T2;
        T2::m as protected n;
        m as o;
    }
    use T3;
    const int X = 1, Y = 2;
    public static ?int $p = null, $q;
    var $v;
    #[P] public int $attributed = 1;
    #[P] var int $old { get => 1; }
    public string $hooked {
        get => $this->hooked;
        set(string $value) { $this->hooked = $value; }
    }
    public int $virtual { get; }
    abstract public function abs(): void;
    public function __construct(private readonly int $x = 1, int ...$rest) {}
    public function m(int|string $a, A&B $b, (A&B)|null $c, ?int $d, &$e): static|self
    {
        echo $a, "b $a {$b->c} ${d}", <<<EOT
            x $a
            EOT, <<<'EOT'
            raw
            EOT;
        print `ls $a`;
        global $g;
        static $s = 1, $t;
        unset($a[0], $b->c);
        if ($a) { } elseif ($b) { } else { }
        if ($a): elseif ($b): else: endif;
        while ($a) { break; }
        while ($a): continue 1; endwhile;
        do { } while ($a);
        for ($i = 0, $j = 1; $i < 3; $i++) { }
        for (;;): endfor;
        foreach ($a as $k => &$v) { }
        foreach ($a as $v): endforeach;
        switch ($a) { case 1: break; default: }
        switch ($a): case 1; endswitch;
        try { } catch (E|F $e) { } finally { }
        label:
        goto label;
        declare(ticks=1) { }
        declare(ticks=1): echo 1; enddeclare;
        ;
        { }
        $a = [1, 'k' => 2, ...$b, [, $x] = $y];
        $a = array(1);
        list($a, $b) = $c;
        [$a] = $b;
        $a[] = 1;
        $a += -1 + !$b . (int) $c ?: $d ? 1.5 : true ?? false ?? null;
        $a++;
        $a = ($b);
        $a = new readonly class(1) extends B implements C { };
        $a = new #[X] class { };
        $a = #[X] function () { };
        $a = static function ($x) use ($b, &$c): int { return $x; };
        $a = static fn($x): int => $x;
        $a = ${'a'} . $$b;
        $a = foo() . Foo\bar() . \baz();
        $a = match ($a) { 1, 2 => 'x', default => 'y' };
        $a = isset($a) && empty($b) && eval('1') && include 'a' && include_once 'b' && require 'c' && require_once 'd';
        $a = print 1;
        $a = throw new E();
        $a = clone $b;
        $a = $o->m(1, x: 2) . $o?->m() . A::m() . parent::m() . static::m() . self::m();
        $a = f(...) . $o->m(...) . A::m(...) . f(?, x: ?);
        $a = $o->p . $o?->p . A::$p . A::C . $o->{'p'} . A::{'C'} . FOO . __LINE__;
        $a = $a |> f(...);
        $a = new A(1);
        $a = function () { yield; yield $a; yield $k => $v; yield from $a; };
        exit(1);
        die;
        return $a;
    }
}

interface I extends J
{
    public function f();
}

trait T
{
}

enum E: string implements I
{
    case A = 'a';
    case B;
    const C = self::A;
}

function f(): void
{
}

const K = 1;
?>
inline
<?= $a ?>
<?php
__halt_compiler();
data
