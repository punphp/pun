<?php

namespace Illuminate\Bus;

use Closure;
use Illuminate\Queue\CallQueuedClosure;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Laravel\SerializableClosure\SerializableClosure;
use PHPUnit\Framework\Assert as PHPUnit;
use RuntimeException;

use function Illuminate\Support\enum_value;

trait Queueable
{
    /**
     * The name of the connection the job should be sent to.
     *
     * @var string|null
     */
    public $connection;

    /**
     * The name of the queue the job should be sent to.
     *
     * @var string|null
     */
    public $queue;

    /**
     * The job "group" the job should be sent to.
     *
     * @var string|null
     */
    public $messageGroup;

    /**
     * The job deduplicator callback the job should use to generate the deduplication ID.
     *
     * @var \Laravel\SerializableClosure\SerializableClosure|callable|null
     */
    public $deduplicator;

    /**
     * The lock owner token for debounce supersession checks.
     *
     * @var string
     */
    public $debounceOwner = '';

    /**
     * The owner of the unique job lock.
     *
     * @var string
     */
    public $uniqueLockOwner = '';

    /**
     * The number of seconds before the job should be made available.
     *
     * @var \DateTimeInterface|\DateInterval|array|int|null
     */
    public $delay;

    /**
     * Indicates whether the job should be dispatched after all database transactions have committed.
     *
     * @var bool|null
     */
    public $afterCommit;

    /**
     * The middleware the job should be dispatched through.
     *
     * @var array
     */
    public $middleware = [];

    /**
     * The jobs that should run if this job is successful.
     *
     * @var array
     */
    public $chained = [];

    /**
     * The name of the connection the chain should be sent to.
     *
     * @var string|null
     */
    public $chainConnection;

    /**
     * The name of the queue the chain should be sent to.
     *
     * @var string|null
     */
    public $chainQueue;

    /**
     * The callbacks to be executed on chain failure.
     *
     * @var array|null
     */
    public $chainCatchCallbacks;

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
     * Set the desired job "group".
     *
     * This feature is only supported by some queues, such as Amazon SQS.
     *
     * @param  \UnitEnum|string  $group
     * @return $this
     */
    public function onGroup($group)
    {}

    /**
     * Set the desired job deduplicator callback.
     *
     * This feature is only supported by some queues, such as Amazon SQS FIFO.
     *
     * @param  callable|null  $deduplicator
     * @return $this
     */
    public function withDeduplicator($deduplicator)
    {}

    /**
     * Set the desired connection for the chain.
     *
     * @param  \UnitEnum|string|null  $connection
     * @return $this
     */
    public function allOnConnection($connection)
    {}

    /**
     * Set the desired queue for the chain.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return $this
     */
    public function allOnQueue($queue)
    {}

    /**
     * Set the desired delay in seconds for the job.
     *
     * @param  \DateTimeInterface|\DateInterval|array|int|null  $delay
     * @return $this
     */
    public function delay($delay)
    {}

    /**
     * Set the delay for the job to zero seconds.
     *
     * @return $this
     */
    public function withoutDelay()
    {}

    /**
     * Indicate that the job should be dispatched after all database transactions have committed.
     *
     * @return $this
     */
    public function afterCommit()
    {}

    /**
     * Indicate that the job should not wait until database transactions have been committed before dispatching.
     *
     * @return $this
     */
    public function beforeCommit()
    {}

    /**
     * Specify the middleware the job should be dispatched through.
     *
     * @param  array|object  $middleware
     * @return $this
     */
    public function through($middleware)
    {}

    /**
     * Set the jobs that should run if this job is successful.
     *
     * @param  array  $chain
     * @return $this
     */
    public function chain($chain)
    {}

    /**
     * Prepend a job to the current chain so that it is run after the currently running job.
     *
     * @param  mixed  $job
     * @return $this
     */
    public function prependToChain($job)
    {}

    /**
     * Append a job to the end of the current chain.
     *
     * @param  mixed  $job
     * @return $this
     */
    public function appendToChain($job)
    {}

    /**
     * Serialize a job for queuing.
     *
     * @param  mixed  $job
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function serializeJob($job)
    {}

    /**
     * Dispatch the next job on the chain.
     *
     * @return void
     */
    public function dispatchNextJobInChain()
    {}

    /**
     * Invoke all of the chain's failed job callbacks.
     *
     * @param  \Throwable  $e
     * @return void
     */
    public function invokeChainCatchCallbacks($e)
    {}

    /**
     * Assert that the job has the given chain of jobs attached to it.
     *
     * @param  array  $expectedChain
     * @return void
     */
    public function assertHasChain($expectedChain)
    {}

    /**
     * Assert that the job has no remaining chained jobs.
     *
     * @return void
     */
    public function assertDoesntHaveChain()
    {}
}
