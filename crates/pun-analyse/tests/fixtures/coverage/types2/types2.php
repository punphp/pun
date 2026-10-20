<?php

namespace Types2;

class Base
{
    private int $hidden = 1;
    private static int $hiddenStatic = 1;
    protected static int $guarded = 1;
    private static function secret(): void {}
    protected static function guardedCall(): void {}
    public function instance(): void {}
}

class Middle extends Base {}

final class Leaf extends Middle
{
    public static int $shared = 1;
    public int $own = 1;
}

/**
 * @template T of Base
 */
final class Holder
{
    /** @param T $t */
    public function __construct(private Base $t) {}

    /** @param T $t */
    public function read(Base $t): void
    {
        $_ = $t->missing;
        $_ = $t->hidden;
        $t->absent();
    }
}

/**
 * @method static void magic()
 */
class Magic {}

class ByRef
{
    /** @var list<int> */
    private static array $store = [];

    public function __construct(int &$x) {}

    public static function &make(): array
    {
        return self::$store;
    }

    public function &get(): array
    {
        return self::$store;
    }

    public static function call(): void
    {
        sort(self::make());
        sort(static::make());
    }
}

#[\Attribute]
final class RefAttr
{
    public function __construct(int &$x) {}
}

#[RefAttr(1)]
final class Attributed {}

/**
 * @param array{int, string} ...$pairs
 */
function variadic(array ...$pairs): void {}

/**
 * @param object{a?: int, b: int} $shape
 * @param int|string $mixedInt
 */
function probe(\Closure $closure, object $shape, Leaf $leaf, Base $base, mixed $m, ByRef $r, string $method, string $name, int $a, int $b, $mixedInt): void
{
    $_ = $closure->foo;
    $_ = [$shape->a, $shape->b, $shape->c];
    $_ = $leaf->hidden;
    $_ = $leaf->shared;
    $_ = $leaf::$hiddenStatic;
    $_ = $leaf::$own;
    $_ = $leaf::$nope;
    $_ = $base::$guarded;
    $_ = $leaf::$$name;
    $base::secret();
    $base::guardedCall();
    $base::instance();
    $leaf::$method();
    $magic = new Magic();
    $magic::magic();
    $doc = new \DOMDocument();
    $doc::loadHTML('<p></p>');
    $_ = $m->x;
    $m->y();
    $m::z();
    new ByRef(1);
    settype($r->$method(), "int");
    settype(ByRef::$method(), "int");
    variadic(1.5);
    echo random_int($a, $b);
    echo mt_rand(1, $mixedInt);
    if (property_exists(\PHP_EOL, 'x')) {
    }
    $_ = $name->{$method};
}

/**
 * @param Leaf|\stdClass $u
 * @param Base&\Countable $i
 * @param ($a is int ? Leaf : Base) $cond
 */
function more($u, $i, $cond, int $a, Leaf|Middle $lb, int|Leaf $il): void
{
    $il->instance();
    $_ = $il->own;
    $lb->instance();
    $_ = $lb->zzz;
    $_ = random_int($a, 5);
    array_filter();
    variadic2(1.5);
    $mx = new Mixer();
    $mx::foo();
    $_ = $mx::$shared;
    $mx->instance();
    $_ = $mx->own;
    $_ = $mx->none;
    $_ = $u->own;
    $i->instance();
    $i->nothing();
    $_ = $i->own;
}

/**
 * @param array{int, string} ...$pairs
 */
function variadic2(...$pairs): void {}

/**
 * @mixin Leaf
 */
final class Mixer {}

class Selfish
{
    public function __construct() {}

    public function run(): void
    {
        $s = $this;
        $_ = $s->nope;
    }

    /** @return static */
    public static function make(): static
    {
        return new static();
    }

    public static function go(): void
    {
        $_ = static::make()->nope2;
    }
}

/**
 * @template T
 * @mixin T
 */
final class Proxy {}

/**
 * @param int<0, max> $r
 */
function ranged(int $r): int
{
    return random_int($r, 5);
}
