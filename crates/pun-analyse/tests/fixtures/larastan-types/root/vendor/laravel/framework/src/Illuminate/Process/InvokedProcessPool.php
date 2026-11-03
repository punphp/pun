<?php

namespace Illuminate\Process;

use ArrayIterator;
use Countable;
use Illuminate\Support\Collection;
use IteratorAggregate;
use Traversable;

class InvokedProcessPool implements Countable, IteratorAggregate
{
    /**
     * The array of invoked processes.
     *
     * @var array
     */
    protected $invokedProcesses;

    /**
     * Create a new invoked process pool.
     *
     * @param  array  $invokedProcesses
     */
    public function __construct(array $invokedProcesses)
    {}

    /**
     * Send a signal to each running process in the pool, returning the processes that were signalled.
     *
     * @param  int  $signal
     * @return \Illuminate\Support\Collection
     */
    public function signal(int $signal)
    {}

    /**
     * Stop all processes that are still running.
     *
     * @param  float  $timeout
     * @param  int|null  $signal
     * @return \Illuminate\Support\Collection
     */
    public function stop(float $timeout = 10, ?int $signal = null)
    {}

    /**
     * Get the processes in the pool that are still currently running.
     *
     * @return \Illuminate\Support\Collection
     */
    public function running()
    {}

    /**
     * Wait for the processes to finish.
     *
     * @return \Illuminate\Process\ProcessPoolResults
     */
    public function wait()
    {}

    /**
     * Get the total number of processes.
     *
     * @return int
     */
    public function count(): int
    {}

    /**
     * Get an iterator for the invoked processes.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}
}
