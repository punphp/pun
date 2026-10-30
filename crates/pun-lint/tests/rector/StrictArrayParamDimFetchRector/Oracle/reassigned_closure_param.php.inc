<?php

/**
 * @template TValue
 */
final class Coll
{
    /**
     * @param  iterable<array-key, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue): mixed  $callback
     * @return $this
     */
    public function each(callable $callback)
    {
        return $this;
    }
}

final class Manager
{
    public function a(array $config)
    {
        (new Coll($config['processors'] ?? []))->each(function ($processor) {
            echo $processor['processor'];
        });
    }

    public function b(array $config)
    {
        (new Coll($config['processors']))->each(function ($processor) {
            $processor = $processor['processor'] ?? $processor;
        });
    }

    public function c(array $config)
    {
        (new Coll([]))->each(function ($processor) {
            $processor = $processor['processor'] ?? $processor;
        });
    }
}
-----
<?php

/**
 * @template TValue
 */
final class Coll
{
    /**
     * @param  iterable<array-key, TValue>  $items
     */
    public function __construct($items)
    {
    }

    /**
     * @param  callable(TValue): mixed  $callback
     * @return $this
     */
    public function each(callable $callback)
    {
        return $this;
    }
}

final class Manager
{
    public function a(array $config)
    {
        (new Coll($config['processors'] ?? []))->each(function (array $processor) {
            echo $processor['processor'];
        });
    }

    public function b(array $config)
    {
        (new Coll($config['processors']))->each(function (array $processor) {
            $processor = $processor['processor'] ?? $processor;
        });
    }

    public function c(array $config)
    {
        (new Coll([]))->each(function (array $processor) {
            $processor = $processor['processor'] ?? $processor;
        });
    }
}
