<?php

namespace TemplateSelfReference;

/**
 * A reduce whose callback takes its own result: TReduceReturnType resolves to a type that names it again.
 *
 * @template TValue
 */
final class Bag
{
    /** @param list<TValue> $items */
    public function __construct(private array $items)
    {
    }

    /**
     * @template TReduceInitial
     * @template TReduceReturnType
     *
     * @param  callable(TReduceInitial|TReduceReturnType, TValue): TReduceReturnType  $callback
     * @param  TReduceInitial  $initial
     * @return TReduceReturnType
     */
    public function reduce(callable $callback, $initial = null)
    {
        $result = $initial;
        foreach ($this->items as $item) {
            $result = $callback($result, $item);
        }

        return $result;
    }
}

function run(): mixed
{
    return (new Bag([1, 2]))->reduce(function ($carry, $item) {
        return $carry;
    });
}
