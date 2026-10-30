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
    public function each(callable $callback)
    {
        return $this;
    }

    /**
     * @param callable(TValue, TValue|int, \Other\TKey, TOther, int): mixed $callback
     */
    public function odd(callable $callback)
    {
        return $this;
    }

    /**
     * @param int $first
     * @param callable(TValue): mixed $callback
     */
    public function second($first, callable $callback)
    {
        return $this;
    }

    public function plain(int $value)
    {
        return $this;
    }
}

final class Uses
{
    /**
     * @param Bag<int, \DateTime> $bag
     * @param Bag<int, string, bool> $three
     * @param Bag<string> $values
     */
    public function run(Bag $bag, Bag $three, Bag $values, string $method)
    {
        $bag->{$method}(function ($date) {
        });
        $bag->missing(function ($date) {
        });
        $bag->plain(1);
        $bag->each('trim');
        $bag->second(function ($first) {
        }, function ($value) {
        });
        $bag->each(function ($only) {
        });
        $bag->odd(function ($a, $b, $c, $d, $e) {
        });
        $three->each(function ($date, $key) {
        });
        $values->each(function ($value, $key) {
        });
        $maybe = rand(0, 1) ? $bag : null;
        $maybe->each(function ($date) {
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
    public function each(callable $callback)
    {
        return $this;
    }

    /**
     * @param callable(TValue, TValue|int, \Other\TKey, TOther, int): mixed $callback
     */
    public function odd(callable $callback)
    {
        return $this;
    }

    /**
     * @param int $first
     * @param callable(TValue): mixed $callback
     */
    public function second($first, callable $callback)
    {
        return $this;
    }

    public function plain(int $value)
    {
        return $this;
    }
}

final class Uses
{
    /**
     * @param Bag<int, \DateTime> $bag
     * @param Bag<int, string, bool> $three
     * @param Bag<string> $values
     */
    public function run(Bag $bag, Bag $three, Bag $values, string $method)
    {
        $bag->{$method}(function ($date) {
        });
        $bag->missing(function ($date) {
        });
        $bag->plain(1);
        $bag->each('trim');
        $bag->second(function ($first) {
        }, function (\DateTime $value) {
        });
        $bag->each(function (\DateTime $only) {
        });
        $bag->odd(function (\DateTime $a, $b, $c, $d, $e) {
        });
        $three->each(function ($date, $key) {
        });
        $values->each(function (string $value, mixed $key) {
        });
        $maybe = rand(0, 1) ? $bag : null;
        $maybe->each(function ($date) {
        });
    }
}
