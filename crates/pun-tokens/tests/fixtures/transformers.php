<?php
declare(strict_types=1);
namespace App\Models;

use Foo\{Bar, Baz as Qux};
use function Foo\{fa, fb};
use const Foo\{CA, CB};
use function strlen;
use const PHP_EOL;

#[\Attribute(\Attribute::TARGET_CLASS), Other(name: 'x', list: [1, 2])]
final class Post extends \Base\Model implements \Stringable, Contracts\Thing
{
    use HasFactory, \Traits\Other { HasFactory::make as protected build; }

    public private(set) ?string $title = null {
        get => $this->title;
        set(?string $value) { $this->title = $value; }
    }
    public array $tags = [];
    public ?\DateTimeInterface $at;
    public static int|string|null $u;
    public (A&B)|null $dnf;
    const ?int MAYBE = null;
    const CLASS_NAME = self::class;

    public function __construct(
        public readonly int $id,
        protected ?Foo &$ref = null,
        private Foo&Bar $both = new Foo(),
        #[SensitiveParameter] string $rest = "",
    ) {}

    public function &getRef(): static|null { return $this->ref; }

    public function m(array $a, callable $c, A|B $ab, A&B $ib, ?int $n = null): ?array
    {
        [$x, [$y, $z]] = $a;
        [, $second] = $a;
        ['k' => $v, 'l' => [$w]] = $a;
        list($p, $q) = $a;
        foreach ($a as [$i, $j]) {}
        foreach ($a as $key => [$i, $j]) {}
        $arr = [1, [2, 3], 'k' => [4]];
        $arr[0][1] = $arr['k'][0];
        $fn = function (int $x) use ($arr, &$n): int { return $x; };
        $arrow = fn (?int $x): ?int => $x ?? null;
        $ternary = $n ? $a : $c;
        $short = $n ?: $a;
        $obj->{$name} = $obj->{'p' . $q};
        ${'dyn'} = $$name;
        ${$var} = 1;
        $s = "{$obj->p} ${name} {$arr['k']} $arr[0]";
        $call = strlen(...);
        $m = $this->m(...);
        $static = static::create(...);
        $const = Foo::{$name};
        $const2 = Foo::{'BAR'}::class;
        $new = (new Foo())->bar();
        $new2 = (new class { public function x() {} })->x();
        $isA = $x instanceof static ? 1 : 2;
        $amp = $a & $b;
        $bor = $a | $b;
        try {} catch (FooException|BarException) {} catch (\Throwable $e) {}
        foo(name: 1, other: $x ? 1 : 2);
        $r = namespace\func();
        $r2 = match (true) { default => null };
        goto end;
        end:
        return $a;
    }
}

enum Status: string implements HasLabel
{
    use Labels;
    case Active = 'active';
    const DEFAULT = self::Active;
    public function label(): string { return match ($this) { self::Active => 'A' }; }
}

interface I { public function f(int ...$a): void; }
trait T { abstract public function g(): self; }
$cl = function () use ($x) {};
function &refFn(array &$a): array { return $a; }
function nullable(?Foo $a, ?Foo\Bar $b, ?\Foo $c): ?\Foo { return $c; }
