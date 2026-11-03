<?php

namespace Illuminate\Process;

use Illuminate\Support\Collection;
use InvalidArgumentException;

/**
 * @mixin \Illuminate\Process\Factory
 * @mixin \Illuminate\Process\PendingProcess
 */
class Pool
{
    /**
     * The process factory instance.
     *
     * @var \Illuminate\Process\Factory
     */
    protected $factory;

    /**
     * The callback that resolves the pending processes.
     *
     * @var callable
     */
    protected $callback;

    /**
     * The array of pending processes.
     *
     * @var array
     */
    protected $pendingProcesses = [];

    /**
     * Create a new process pool.
     *
     * @param  \Illuminate\Process\Factory  $factory
     * @param  callable  $callback
     */
    public function __construct(Factory $factory, callable $callback)
    {}

    /**
     * Add a process to the pool with a key.
     *
     * @param  string  $key
     * @return \Illuminate\Process\PendingProcess
     */
    public function as(string $key)
    {}

    /**
     * Start all of the processes in the pool.
     *
     * @param  callable|null  $output
     * @return \Illuminate\Process\InvokedProcessPool
     *
     * @throws \InvalidArgumentException
     */
    public function start(?callable $output = null)
    {}

    /**
     * Start and wait for the processes to finish.
     *
     * @return \Illuminate\Process\ProcessPoolResults
     */
    public function run()
    {}

    /**
     * Start and wait for the processes to finish.
     *
     * @return \Illuminate\Process\ProcessPoolResults
     */
    public function wait()
    {}

    /**
     * Dynamically proxy methods calls to a new pending process.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return \Illuminate\Process\PendingProcess
     */
    public function __call($method, $parameters)
    {}
}
