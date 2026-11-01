<?php

namespace Illuminate\Database\Concerns;

if (trait_exists('Illuminate\Database\Concerns\BuildsQueries')) {
    return;
}

/**
 * The part of laravel/framework's trait the Eloquent builder gets its `each()` from.
 */
trait BuildsQueries
{
    public function each(callable $callback, $count = 1000)
    {
        return $this;
    }
}
