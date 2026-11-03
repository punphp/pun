<?php

namespace Illuminate\Queue;

use Illuminate\Contracts\Queue\Job;
use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Queue\Events\JobAttempted;
use Illuminate\Queue\Events\JobExceptionOccurred;
use Illuminate\Queue\Events\JobProcessed;
use Illuminate\Queue\Events\JobProcessing;
use Illuminate\Queue\Jobs\SyncJob;
use Illuminate\Support\Collection;
use Throwable;

class SyncQueue extends Queue implements QueueContract
{
    /**
     * Create a new sync queue instance.
     *
     * @param  bool  $dispatchAfterCommit
     */
    public function __construct($dispatchAfterCommit = false)
    {}

    /**
     * Get the size of the queue.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function size($queue = null)
    {}

    /**
     * Get the number of pending jobs.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function pendingSize($queue = null)
    {}

    /**
     * Get the number of delayed jobs.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function delayedSize($queue = null)
    {}

    /**
     * Get the number of reserved jobs.
     *
     * @param  string|null  $queue
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
     * @param  string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function pendingJobs($queue = null): Collection
    {}

    /**
     * Get the delayed jobs for the given queue.
     *
     * @param  string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function delayedJobs($queue = null): Collection
    {}

    /**
     * Get the reserved jobs for the given queue.
     *
     * @param  string|null  $queue
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
     * @param  string|null  $queue
     * @return int|null
     */
    public function creationTimeOfOldestPendingJob($queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  string  $job
     * @param  mixed  $data
     * @param  string|null  $queue
     * @return mixed
     *
     * @throws \Throwable
     */
    public function push($job, $data = '', $queue = null)
    {}

    /**
     * Execute a given job synchronously.
     *
     * @param  string  $job
     * @param  mixed  $data
     * @param  string|null  $queue
     * @return int
     *
     * @throws \Throwable
     */
    protected function executeJob($job, $data = '', $queue = null)
    {}

    /**
     * Resolve a Sync job instance.
     *
     * @param  string  $payload
     * @param  string  $queue
     * @return \Illuminate\Queue\Jobs\SyncJob
     */
    protected function resolveJob($payload, $queue)
    {}

    /**
     * Raise the before queue job event.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @return void
     */
    protected function raiseBeforeJobEvent(Job $job)
    {}

    /**
     * Raise the after queue job event.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  float|null  $duration
     * @return void
     */
    protected function raiseAfterJobEvent(Job $job, $duration = null)
    {}

    /**
     * Raise the job attempted event.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable|null  $exceptionOccurred
     * @param  float|null  $duration
     * @return void
     */
    protected function raiseJobAttemptedEvent(Job $job, ?Throwable $exceptionOccurred = null, $duration = null)
    {}

    /**
     * Raise the exception occurred queue job event.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function raiseExceptionOccurredJobEvent(Job $job, Throwable $e)
    {}

    /**
     * Handle an exception that occurred while processing a job.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $queueJob
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleException(Job $queueJob, Throwable $e)
    {}

    /**
     * Push a raw payload onto the queue.
     *
     * @param  string  $payload
     * @param  string|null  $queue
     * @param  array  $options
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
     * @param  string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Pop the next job off of the queue.
     *
     * @param  string|null  $queue
     * @return \Illuminate\Contracts\Queue\Job|null
     */
    public function pop($queue = null)
    {}
}
