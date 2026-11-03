<?php

namespace Illuminate\Container;

use Countable;
use IteratorAggregate;
use Traversable;

class RewindableGenerator implements Countable, IteratorAggregate
{
    /**
     * The generator callback.
     *
     * @var callable
     */
    protected $generator;

    /**
     * The number of tagged services.
     *
     * @var callable|int
     */
    protected $count;

    /**
     * Create a new generator instance.
     *
     * @param  callable  $generator
     * @param  callable|int  $count
     */
    public function __construct(callable $generator, $count)
    {}

    /**
     * Get an iterator from the generator.
     *
     * @return \Traversable
     */
    public function getIterator(): Traversable
    {}

    /**
     * Get the total number of tagged services.
     *
     * @return int
     */
    public function count(): int
    {}
}
