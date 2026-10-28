<?php

namespace Probe;

final class Bag implements \IteratorAggregate
{
    public function getIterator(): \Iterator
    {
        return new \ArrayIterator([]);
    }

    /**
     * @param callable(TValue, TKey): mixed $callback
     */
    public function each(callable $callback, int $limit = 0)
    {
        return $this;
    }

    /**
     * @param int $limit
     * @param callable(TKey, TValue): bool $filter
     */
    public function filter($limit, callable $filter)
    {
        return $this;
    }
}

final class Plain
{
    /**
     * @param callable(TValue): mixed $callback
     */
    public function each(callable $callback)
    {
        return $this;
    }
}

final class Uses
{
    /**
     * @param Bag<int, \DateTime> $bag
     * @param Bag<string> $values
     * @param Plain<int, string> $plain
     */
    public function run(Bag $bag, Bag $values, Plain $plain)
    {
        $bag->each(function ($date, $key) {
        });
        $bag->each(function (string $date, int $key) {
        });
        $bag->each(fn ($date, $key) => $date);
        $bag->filter(filter: function ($key, $date) {
            return true;
        }, limit: 1);
        $values->each(function ($value, $key) {
        });
        $plain->each(function ($value) {
        });
    }
}
-----
<?php

namespace Probe;

final class Bag implements \IteratorAggregate
{
    public function getIterator(): \Iterator
    {
        return new \ArrayIterator([]);
    }

    /**
     * @param callable(TValue, TKey): mixed $callback
     */
    public function each(callable $callback, int $limit = 0)
    {
        return $this;
    }

    /**
     * @param int $limit
     * @param callable(TKey, TValue): bool $filter
     */
    public function filter($limit, callable $filter)
    {
        return $this;
    }
}

final class Plain
{
    /**
     * @param callable(TValue): mixed $callback
     */
    public function each(callable $callback)
    {
        return $this;
    }
}

final class Uses
{
    /**
     * @param Bag<int, \DateTime> $bag
     * @param Bag<string> $values
     * @param Plain<int, string> $plain
     */
    public function run(Bag $bag, Bag $values, Plain $plain)
    {
        $bag->each(function (\DateTime $date, int $key) {
        });
        $bag->each(function (\DateTime $date, int $key) {
        });
        $bag->each(fn ($date, $key) => $date);
        $bag->filter(filter: function ($key, $date) {
            return true;
        }, limit: 1);
        $values->each(function (string $value, mixed $key) {
        });
        $plain->each(function ($value) {
        });
    }
}
