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
}

final class Uses
{
    /**
     * @param Bag<string> $values
     */
    public function run(Bag $values)
    {
        $values->each(function ($value, $key) {
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
}

final class Uses
{
    /**
     * @param Bag<string> $values
     */
    public function run(Bag $values)
    {
        $values->each(function (string $value, $key) {
        });
    }
}
