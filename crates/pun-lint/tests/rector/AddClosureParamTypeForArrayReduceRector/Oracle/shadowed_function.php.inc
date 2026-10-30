<?php

namespace Probe;

function array_reduce(array $array, callable $callback, mixed $initial = null): mixed
{
    return $initial;
}

final class Shadowed
{
    /**
     * @param  list<int>  $items
     */
    public function run(array $items)
    {
        return array_reduce($items, function ($carry, $item) {
            return (int) $item;
        }, 0);
    }
}
