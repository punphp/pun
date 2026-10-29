<?php

namespace Probe;

/**
 * @template TKey of array-key
 * @template TValue
 */
class Iter
{
    /**
     * @param  iterable<TKey, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     */
    public function through(callable $callback): void
    {
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 */
class ArrKey
{
    /**
     * @param  array<TKey, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     */
    public function through(callable $callback): void
    {
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 */
class ArrInt
{
    /**
     * @param  array<int, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     */
    public function through(callable $callback): void
    {
    }
}

function iter_one()
{
    (new Iter([['id' => 4]]))->through(function ($item) {
        return $item['id'];
    });
}

function iter_two()
{
    (new Iter([['id' => 4], ['id' => 5]]))->through(function ($item) {
        return $item['id'];
    });
}

function iter_same()
{
    (new Iter([['id' => 4], ['id' => 4]]))->through(function ($item) {
        return $item['id'];
    });
}

function arrkey_one()
{
    (new ArrKey([['id' => 4]]))->through(function ($item) {
        return $item['id'];
    });
}

function arrkey_two()
{
    (new ArrKey([['id' => 4], ['id' => 5]]))->through(function ($item) {
        return $item['id'];
    });
}

function arrkey_same()
{
    (new ArrKey([['id' => 4], ['id' => 4]]))->through(function ($item) {
        return $item['id'];
    });
}

function arrint_one()
{
    (new ArrInt([['id' => 4]]))->through(function ($item) {
        return $item['id'];
    });
}

function arrint_two()
{
    (new ArrInt([['id' => 4], ['id' => 5]]))->through(function ($item) {
        return $item['id'];
    });
}

function arrint_same()
{
    (new ArrInt([['id' => 4], ['id' => 4]]))->through(function ($item) {
        return $item['id'];
    });
}
-----
<?php

namespace Probe;

/**
 * @template TKey of array-key
 * @template TValue
 */
class Iter
{
    /**
     * @param  iterable<TKey, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     */
    public function through(callable $callback): void
    {
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 */
class ArrKey
{
    /**
     * @param  array<TKey, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     */
    public function through(callable $callback): void
    {
    }
}

/**
 * @template TKey of array-key
 * @template TValue
 */
class ArrInt
{
    /**
     * @param  array<int, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue, TKey): mixed  $callback
     */
    public function through(callable $callback): void
    {
    }
}

function iter_one()
{
    (new Iter([['id' => 4]]))->through(function (array $item) {
        return $item['id'];
    });
}

function iter_two()
{
    (new Iter([['id' => 4], ['id' => 5]]))->through(function (array $item) {
        return $item['id'];
    });
}

function iter_same()
{
    (new Iter([['id' => 4], ['id' => 4]]))->through(function (array $item) {
        return $item['id'];
    });
}

function arrkey_one()
{
    (new ArrKey([['id' => 4]]))->through(function (array $item) {
        return $item['id'];
    });
}

function arrkey_two()
{
    (new ArrKey([['id' => 4], ['id' => 5]]))->through(function (array $item) {
        return $item['id'];
    });
}

function arrkey_same()
{
    (new ArrKey([['id' => 4], ['id' => 4]]))->through(function (array $item) {
        return $item['id'];
    });
}

function arrint_one()
{
    (new ArrInt([['id' => 4]]))->through(function (array $item) {
        return $item['id'];
    });
}

function arrint_two()
{
    (new ArrInt([['id' => 4], ['id' => 5]]))->through(function (array $item) {
        return $item['id'];
    });
}

function arrint_same()
{
    (new ArrInt([['id' => 4], ['id' => 4]]))->through(function (array $item) {
        return $item['id'];
    });
}
