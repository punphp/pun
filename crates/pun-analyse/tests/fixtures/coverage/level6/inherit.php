<?php

namespace CoverageInherit;

/**
 * @template T
 */
interface Box
{
    /**
     * @param T $value
     * @param-out list<int> $out
     * @param-closure-this \stdClass $callback
     * @phpstan-self-out static
     * @return list<T>
     */
    public function put($value, &$out, $callback);
}

class Parent_
{
    /**
     * @param array<int, string> $items
     * @return array<string, int>
     */
    public function items($items)
    {
        return [];
    }

    /** @param array<int, int> $hidden */
    private function hidden($hidden): void
    {
    }

    public function __call($name, $args)
    {
    }
}

/**
 * @implements Box<int>
 */
class Child extends Parent_ implements Box
{
    public function put($value, &$out, $callback)
    {
        return [];
    }

    public function items($renamed)
    {
        return [];
    }

    public function hidden($hidden): void
    {
    }

    public function magic($x)
    {
    }
}

trait Abstracted
{
    /**
     * @param array<int, int> $values
     * @return array<int, int>
     */
    abstract public function values($values);
}

class UsesAbstract
{
    use Abstracted;

    public function values($values)
    {
        return $values;
    }
}

/**
 * @template T
 */
trait Holds
{
    /** @var T */
    public $held;

    /**
     * @param T $value
     * @return array<T>
     */
    public function hold($value): array
    {
        return [$value];
    }
}

class Holder
{
    /** @use Holds<array> */
    use Holds;
}

class Holder2
{
    use Holds;
}

/**
 * @template T
 * @param T $value
 * @return array<T>
 */
function wrap($value): array
{
    return [$value];
}

/**
 * @template K of array-key
 * @template V = int
 * @param array<K, V> $a
 * @return array
 */
function keys(array $a): array
{
    return $a;
}
