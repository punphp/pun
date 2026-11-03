<?php

namespace Illuminate\Queue;

use Illuminate\Contracts\Events\Dispatcher as EventDispatcher;
use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Events\QueueFailedOver;
use Illuminate\Support\Collection;
use RuntimeException;
use Throwable;

class FailoverQueue extends Queue implements QueueContract
{
    /**
     * The queues which failed on the last action.
     *
     * @var list<string>
     */
    protected array $failingQueues = [];

    /**
     * Create a new failover queue instance.
     */
    public function __construct(
        public QueueManager $manager,
        public EventDispatcher $events,
        public array $connections
    ) {}

    /**
     * Get the size of the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function size($queue = null)
    {}

    /**
     * Get the number of pending jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function pendingSize($queue = null)
    {}

    /**
     * Get the number of delayed jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function delayedSize($queue = null)
    {}

    /**
     * Get the number of reserved jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function reservedSize($queue = null)
    {}

    /**
     * Get the number of jobs across every queue.
     *
     * @return int
     */
    public function totalSize()
    {}

    /**
     * Get the number of pending jobs across every queue.
     *
     * @return int
     */
    public function totalPendingSize()
    {}

    /**
     * Get the number of delayed jobs across every queue.
     *
     * @return int
     */
    public function totalDelayedSize()
    {}

    /**
     * Get the number of reserved jobs across every queue.
     *
     * @return int
     */
    public function totalReservedSize()
    {}

    /**
     * Get the pending jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function pendingJobs($queue = null): Collection
    {}

    /**
     * Get the delayed jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function delayedJobs($queue = null): Collection
    {}

    /**
     * Get the reserved jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function reservedJobs($queue = null): Collection
    {}

    /**
     * Get all pending jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allPendingJobs(): Collection
    {}

    /**
     * Get all delayed jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allDelayedJobs(): Collection
    {}

    /**
     * Get all reserved jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allReservedJobs(): Collection
    {}

    /**
     * Get the creation timestamp of the oldest pending job, excluding delayed jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int|null
     */
    public function creationTimeOfOldestPendingJob($queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  object|string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function push($job, $data = '', $queue = null)
    {}

    /**
     * Push a raw payload onto the queue.
     *
     * @param  string  $payload
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function pushRaw($payload, $queue = null, array $options = [])
    {}

    /**
     * Push a new job onto the queue after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return void
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Pop the next job off of the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Contracts\Queue\Job|null
     */
    public function pop($queue = null)
    {}

    /**
     * Attempt the given method on all connections.
     *
     * @param  mixed  $job
     * @return mixed
     *
     * @throws \Throwable
     */
    protected function attemptOnAllConnections(string $method, array $arguments, $job = null)
    {}
}
