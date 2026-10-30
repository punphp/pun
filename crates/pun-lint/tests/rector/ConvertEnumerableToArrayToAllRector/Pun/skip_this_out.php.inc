<?php

namespace App;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Enumerable;

final class Item implements Arrayable
{
    public function toArray()
    {
        return [];
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 * @implements Enumerable<TKey, TValue>
 */
final class Bag implements Enumerable
{
    /**
     * @template TPutValue
     * @param TPutValue $value
     * @return $this
     * @phpstan-this-out static<TKey, TValue|TPutValue>
     */
    public function put($key, $value)
    {
        return $this;
    }

    /** @return int */
    public function count()
    {
        return 0;
    }
}

// PHPStan gives $bag the put() tag's type (@phpstan-this-out) from the call on; Mago does not model the tag, so Pun
// keeps toArray() whenever the function calls a method with that tag on the same receiver, before the call or not.
/**
 * @param Bag<int, int> $bag
 */
function put_later($bag)
{
    $bag->toArray();
    $bag->put(1, new Item);
}

/**
 * @param Bag<int, int> $bag
 */
function in_closure($bag)
{
    $f = function () use ($bag) {
        $bag->put(1, new Item);
    };
    $bag->toArray();
}
