<?php

namespace Illuminate\Process;

use Illuminate\Support\Collection;
use InvalidArgumentException;

/**
 * @mixin \Illuminate\Process\Factory
 * @mixin \Illuminate\Process\PendingProcess
 */
class Pipe
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
     * Create a new series of piped processes.
     *
     * @param  \Illuminate\Process\Factory  $factory
     * @param  callable  $callback
     */
    public function __construct(Factory $factory, callable $callback)
    {}

    /**
     * Add a process to the pipe with a key.
     *
     * @param  string  $key
     * @return \Illuminate\Process\PendingProcess
     */
    public function as(string $key)
    {}

    /**
     * Runs the processes in the pipe.
     *
     * @param  callable|null  $output
     * @return \Illuminate\Contracts\Process\ProcessResult
     *
     * @throws \InvalidArgumentException
     */
    public function run(?callable $output = null)
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
