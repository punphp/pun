<?php

namespace NoopFixture;

const ANSWER = 42;

/** @phpstan-pure */
function pure(int $i): int
{
    return $i;
}

/** @phan-side-effect-free */
function sideEffectFree(int $i): int
{
    return $i;
}

/** @phpstan-impure */
function impure(int $i = 0): int
{
    return 1;
}

/**
 * @phpstan-pure
 * @throws \RuntimeException
 */
function pureThrows(): int
{
    return 1;
}

/**
 * @phpstan-pure
 * @phpstan-assert int $value
 */
function pureAsserts(mixed $value): int
{
    return 1;
}

/** @phpstan-pure */
function pureNever(): never
{
    throw new \RuntimeException();
}

class Box
{
    public int $value = 0;
    public static int $shared = 0;
    public const LIMIT = 3;

    /** @phpstan-pure */
    public function get(int $i = 0): int
    {
        return $this->value;
    }

    /** @phpstan-pure */
    public static function make(int $i = 0): int
    {
        return 1;
    }

    public function set(): void
    {
        $this->value = 1;
    }

    /** @phpstan-pure */
    public function __construct()
    {
    }
}

class Plain
{
}

class Effects
{
    public function __construct()
    {
        echo 'made';
    }
}

/**
 * @param array<int> $list
 */
function run(int $a, string $s, ?Box $maybe, Box $box, array $list, object $o, mixed $m): void
{
    $a;
    $_GET;
    $list[0];
    $box->value;
    $maybe?->value;
    Box::$shared;
    $box::$shared;
    Box::LIMIT;
    $box::LIMIT;
    ANSWER;
    __LINE__;
    1.5;
    null;
    @$a;
    +$a;
    -$a;
    !$a;
    ~$a;
    (string) $a;
    (int) $s;
    (bool) $a;
    (float) $a;
    (array) $a;
    (object) $a;
    isset($a, $list[1]);
    empty($a);
    "value $a and {$box->value}";
    [1, 'k' => 2, ...$list];
    clone $box;
    $a + 1;
    $a . 'x';
    $a instanceof Box;
    $box->{'value'};
    $a xor $s;
    $a and $s;
    $a or $s;
    $x = $a and pure(1);
    $a && $s;
    $a || $s;
    $a ? 1 : 2;
    $a ?: 2;
    match ($a) {
        1 => 'one',
        default => 'other',
    };
    fn() => 1;
    function () {
    };
    $a++;
    ++$a;
    $a += 1;
    [$y, $z] = $list;
    $ref = &$box->value;
    strlen($s);
    \strlen($s);
    printf('%s', $s);
    pure(1);
    sideEffectFree(1);
    impure();
    pureThrows();
    pureAsserts($m);
    pure(...);
    strlen(...);
    array_map(fn(int $i) => $i + 1, $list);
    array_map(function (int $i) {
        return $i;
    }, $list);
    array_map(function (int $i) {
        echo $i;
        return $i;
    }, $list);
    array_map('strlen', $list);
    array_map('NoopFixture\impure', $list);
    array_filter($list);
    array_filter($list, null);
    array_filter(callback: fn($i) => $i > 1, array: $list);
    var_export($list, true);
    var_export($list);
    print_r($list, return: true);
    'strlen'($s);
    'nothing'($s);
    $box->get();
    $box->set();
    $maybe?->get();
    Box::make();
    $box::make();
    $box->get(...);
    Box::make(...);
    $o->anything();
    new Box();
    new Plain();
    new Effects();
    new \stdClass();
    new \ArrayObject([]);
    new Missing();
    new class {
    };
    new class (1) extends Plain {
        public function __construct(int $i)
        {
        }
    };
    5 |> pure(...);
    5 |> 'NoopFixture\pure';
    5 |> impure(...);
    5 |> $box->get(...);
    5 |> Box::make(...);
    $_GET['a'] = 1;
    $box->value = 1;
    $GLOBALS;
    $$s;
    ${'a' . 'b'};
    $list[] = 1;
    list('a' => $q) = ['a' => 1];
    Box::{'make'}();
    $box->{$s}();
    $s();
    $a . $s++;
    $a + ++$a;
    $r = &$list[0];
    Box::{'LIMIT'};
    new class (i: 1) extends Plain {
        public function __construct(int $i)
        {
        }
    };
    5 |> var_dump(...);
    ($a + 1) * 2;
    (integer) $a;
    (boolean) $a;
    (double) $a;
    (binary) $a;
    $box->$s;
    $box->{$s . 'x'};
    pureNever();
}
