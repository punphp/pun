<?php

namespace Probe;

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
class Cov
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
class Inv
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

function cov_chained()
{
    (new Cov([['id' => 4], ['id' => 5]]))->through(function ($item) {
        return $item['id'];
    });
}

function cov_variable()
{
    $p = new Cov([['id' => 4], ['id' => 5]]);
    $p->through(function ($item) {
        return $item['id'];
    });
}

function inv_chained()
{
    (new Inv([['id' => 4], ['id' => 5]]))->through(function ($item) {
        return $item['id'];
    });
}

function inv_variable()
{
    $p = new Inv([['id' => 4], ['id' => 5]]);
    $p->through(function ($item) {
        return $item['id'];
    });
}
-----
<?php

namespace Probe;

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
class Cov
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
class Inv
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

function cov_chained()
{
    (new Cov([['id' => 4], ['id' => 5]]))->through(function ($item) {
        return $item['id'];
    });
}

function cov_variable()
{
    $p = new Cov([['id' => 4], ['id' => 5]]);
    $p->through(function ($item) {
        return $item['id'];
    });
}

function inv_chained()
{
    (new Inv([['id' => 4], ['id' => 5]]))->through(function (array $item) {
        return $item['id'];
    });
}

function inv_variable()
{
    $p = new Inv([['id' => 4], ['id' => 5]]);
    $p->through(function (array $item) {
        return $item['id'];
    });
}
