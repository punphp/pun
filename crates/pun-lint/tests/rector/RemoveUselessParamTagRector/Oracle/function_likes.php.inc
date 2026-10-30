<?php

namespace App;

/**
 * @param int $a
 * @param string $b
 */
function plain(int $a, $b): void
{
}

interface Shape
{
    /**
     * @param int $a
     */
    public function area(int $a): void;
}

abstract class Base
{
    /**
     * @phpstan-param int $a
     * @psalm-param int $b
     * @param int $c
     */
    abstract public function run(int $a, int $b, int $c): void;

    /**
     * @param int $a
     */
    public function __construct(private int $a)
    {
    }
}

$arrow = /** @param int $a */ fn (int $a) => $a;

$closure = (new class {
    /**
     * @param \stdClass $s
     * @param iterable $i
     * @param callable $c
     * @param object $o
     * @param float $f
     */
    public function run(\stdClass $s, iterable $i, callable $c, object $o, float $f): void
    {
    }
});
-----
<?php

namespace App;

/**
 * @param string $b
 */
function plain(int $a, $b): void
{
}

interface Shape
{
    public function area(int $a): void;
}

abstract class Base
{
    /**
     * @phpstan-param int $a
     * @psalm-param int $b
     */
    abstract public function run(int $a, int $b, int $c): void;

    public function __construct(private int $a)
    {
    }
}

$arrow = /** @param int $a */ fn (int $a) => $a;

$closure = (new class {
    public function run(\stdClass $s, iterable $i, callable $c, object $o, float $f): void
    {
    }
});
