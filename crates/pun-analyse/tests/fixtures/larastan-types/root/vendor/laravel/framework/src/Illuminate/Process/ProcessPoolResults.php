<?php

namespace Illuminate\Process;

use ArrayAccess;
use ArrayIterator;
use Illuminate\Support\Collection;
use IteratorAggregate;
use Traversable;

class ProcessPoolResults implements ArrayAccess, IteratorAggregate
{
    /**
     * The results of the processes.
     *
     * @var array
     */
    protected $results = [];

    /**
     * Create a new process pool result set.
     *
     * @param  array  $results
     */
    public function __construct(array $results)
    {}

    /**
     * Determine if all of the processes in the pool were successful.
     *
     * @return bool
     */
    public function successful()
    {}

    /**
     * Determine if any of the processes in the pool failed.
     *
     * @return bool
     */
    public function failed()
    {}

    /**
     * Get the results as a collection.
     *
     * @return \Illuminate\Support\Collection
     */
    public function collect()
    {}

    /**
     * Determine if the given array offset exists.
     *
     * @param  int  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the result at the given offset.
     *
     * @param  int  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the result at the given offset.
     *
     * @param  int  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the result at the given offset.
     *
     * @param  int  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Get an iterator for the results.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}
}
