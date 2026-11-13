<?php

namespace App;

use function PHPStan\dumpType;

/**
 * @template K of array-key
 * @template V
 */
final class Box
{
    /** @param iterable<K, V> $v */
    public function __construct(public iterable $v) {}
}

/**
 * @template K of array-key
 * @template V
 * @param iterable<K, V> $v
 * @return Box<K, V>
 */
function box(iterable $v): Box
{
    return new Box($v);
}

/** @param Box<int, int|null> $b */
function takes(Box $b): void {}

class B
{
    /** @param array<mixed> $a */
    public function f(array $a): void
    {
        dumpType(box([1, null]));
        dumpType(new Box([1, null]));
        $b = new Box([1, null]);
        takes($b);
        dumpType($b);
        dumpType([...$a, 'x' => 1]);
        $k = fn () => [...$a, 'x' => 1];
        dumpType($k());
    }

    public function g(): void
    {
        $f = function ($x) {
            dumpType($x);
        };
        echo count(array_map($f, [1, 2]));
        $h = fn ($s) => dumpType($s);
        $this->each($h);
    }

    /** @param callable(string): void $c */
    private function each(callable $c): void {}
}
