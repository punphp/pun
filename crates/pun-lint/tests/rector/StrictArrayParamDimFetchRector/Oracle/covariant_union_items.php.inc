<?php

namespace Probe;

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
interface Listable
{
}

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
class Pages
{
    /**
     * @param  Listable<TKey, TValue>|iterable<TKey, TValue>|null  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @template TThroughValue
     *
     * @param  callable(TValue, TKey): TThroughValue  $callback
     * @return $this
     */
    public function through(callable $callback)
    {
        return $this;
    }
}

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
class Plain
{
    /**
     * @param  iterable<TKey, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     * @return $this
     */
    public function through(callable $callback)
    {
        return $this;
    }
}

function unionParam()
{
    $p = new Pages([['id' => 4], ['id' => 5]]);
    $p->through(function ($item) {
        return $item['id'];
    });
}

function assignedArg()
{
    $p = new Plain($array = [['id' => 4], ['id' => 5]]);
    $p->through(function ($item) {
        return $item['id'];
    });
}

function plain()
{
    $p = new Plain([['id' => 4], ['id' => 5]]);
    $p->through(function ($item) {
        return $item['id'];
    });
}
