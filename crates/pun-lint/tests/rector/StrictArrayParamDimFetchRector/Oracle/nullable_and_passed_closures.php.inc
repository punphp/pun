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

/**
 * @template T
 * @param  T  $value
 * @param  callable(T): mixed  $callback
 * @return T
 */
function tapIt($value, $callback)
{
    return $value;
}

final class Manager
{
    public function processors(array $config)
    {
        (new Coll($config['processors'] ?? []))->each(function ($processor) {
            $processor = $processor['processor'] ?? $processor;
        });
    }

    /**
     * @param  array|null  $callableBinding
     * @param  array|null  $controllerBinding
     */
    protected function restore($callableBinding, $controllerBinding)
    {
        if ($callableBinding) {
            echo $callableBinding['concrete'];
        }

        if ($controllerBinding) {
            echo $controllerBinding['concrete'];
        }
    }

    public function toArray(): array
    {
        return [];
    }

    public function tapped()
    {
        tapIt($this->toArray(), function ($postsAsArray) {
            echo $postsAsArray[0];
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

/**
 * @template T
 * @param  T  $value
 * @param  callable(T): mixed  $callback
 * @return T
 */
function tapIt($value, $callback)
{
    return $value;
}

final class Manager
{
    public function processors(array $config)
    {
        (new Coll($config['processors'] ?? []))->each(function (array $processor) {
            $processor = $processor['processor'] ?? $processor;
        });
    }

    /**
     * @param  array|null  $callableBinding
     * @param  array|null  $controllerBinding
     */
    protected function restore(array $callableBinding, array $controllerBinding)
    {
        if ($callableBinding) {
            echo $callableBinding['concrete'];
        }

        if ($controllerBinding) {
            echo $controllerBinding['concrete'];
        }
    }

    public function toArray(): array
    {
        return [];
    }

    public function tapped()
    {
        tapIt($this->toArray(), function (array $postsAsArray) {
            echo $postsAsArray[0];
        });
    }
}
