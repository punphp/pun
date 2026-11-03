<?php

namespace Illuminate\Foundation\Bus;

use Closure;
use Illuminate\Bus\ChainedBatch;
use Illuminate\Contracts\Bus\Dispatcher;
use Illuminate\Queue\CallQueuedClosure;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Conditionable;
use Laravel\SerializableClosure\SerializableClosure;

use function Illuminate\Support\enum_value;

class PendingChain
{
    use Conditionable;

    /**
     * The class name of the job being dispatched.
     *
     * @var mixed
     */
    public $job;

    /**
     * The jobs to be chained.
     *
     * @var array
     */
    public $chain;

    /**
     * The name of the connection the chain should be sent to.
     *
     * @var string|null
     */
    public $connection;

    /**
     * The name of the queue the chain should be sent to.
     *
     * @var string|null
     */
    public $queue;

    /**
     * The number of seconds before the chain should be made available.
     *
     * @var \DateTimeInterface|\DateInterval|int|null
     */
    public $delay;

    /**
     * The callbacks to be executed on failure.
     *
     * @var array
     */
    public $catchCallbacks = [];

    /**
     * Create a new PendingChain instance.
     *
     * @param  mixed  $job
     * @param  array  $chain
     */
    public function __construct($job, $chain)
    {}

    /**
     * Set the desired connection for the job.
     *
     * @param  \UnitEnum|string|null  $connection
     * @return $this
     */
    public function onConnection($connection)
    {}

    /**
     * Set the desired queue for the job.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return $this
     */
    public function onQueue($queue)
    {}

    /**
     * Prepend a job to the chain.
     *
     * @param  mixed  $job
     * @return $this
     */
    public function prepend($job)
    {}

    /**
     * Append a job to the chain.
     *
     * @param  mixed  $job
     * @return $this
     */
    public function append($job)
    {}

    /**
     * Set the desired delay in seconds for the chain.
     *
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @return $this
     */
    public function delay($delay)
    {}

    /**
     * Add a callback to be executed on job failure.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function catch($callback)
    {}

    /**
     * Get the "catch" callbacks that have been registered.
     *
     * @return array
     */
    public function catchCallbacks()
    {}

    /**
     * Dispatch the job chain.
     *
     * @return \Illuminate\Foundation\Bus\PendingDispatch
     */
    public function dispatch()
    {}

    /**
     * Dispatch the job chain if the given truth test passes.
     *
     * @param  bool|\Closure  $boolean
     * @return \Illuminate\Foundation\Bus\PendingDispatch|null
     */
    public function dispatchIf($boolean)
    {}

    /**
     * Dispatch the job chain unless the given truth test passes.
     *
     * @param  bool|\Closure  $boolean
     * @return \Illuminate\Foundation\Bus\PendingDispatch|null
     */
    public function dispatchUnless($boolean)
    {}
}
