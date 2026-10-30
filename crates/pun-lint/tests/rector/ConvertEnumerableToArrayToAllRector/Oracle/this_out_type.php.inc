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

/**
 * @param Bag<int, int> $bag
 */
function put_arrayable($bag)
{
    $bag->put(1, new Item);
    $bag->toArray();
}

/**
 * @param Bag<int, int> $bag
 * @param Bag<int, int> $other
 */
function other_receiver($bag, $other)
{
    $other->put(1, new Item);
    $bag->toArray();
}

/**
 * @param Bag<int, int> $bag
 */
function untagged($bag)
{
    $bag->count();
    $bag->toArray();
}
-----
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

/**
 * @param Bag<int, int> $bag
 */
function put_arrayable($bag)
{
    $bag->put(1, new Item);
    $bag->toArray();
}

/**
 * @param Bag<int, int> $bag
 * @param Bag<int, int> $other
 */
function other_receiver($bag, $other)
{
    $other->put(1, new Item);
    $bag->all();
}

/**
 * @param Bag<int, int> $bag
 */
function untagged($bag)
{
    $bag->count();
    $bag->all();
}
