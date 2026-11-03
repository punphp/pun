<?php

namespace Illuminate\Queue\Jobs;

use Illuminate\Bus\Batchable;
use Illuminate\Bus\BatchRepository;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Queue\Events\JobFailed;
use Illuminate\Queue\ManuallyFailedException;
use Illuminate\Queue\TimeoutExceededException;
use Illuminate\Support\InteractsWithTime;
use Throwable;

abstract class Job
{
    use InteractsWithTime;

    /**
     * The job handler instance.
     *
     * @var mixed
     */
    protected $instance;

    /**
     * The IoC container instance.
     *
     * @var \Illuminate\Container\Container
     */
    protected $container;

    /**
     * Indicates if the job has been deleted.
     *
     * @var bool
     */
    protected $deleted = false;

    /**
     * Indicates if the job has been released.
     *
     * @var bool
     */
    protected $released = false;

    /**
     * Indicates if the job has failed.
     *
     * @var bool
     */
    protected $failed = false;

    /**
     * The name of the connection the job belongs to.
     *
     * @var string
     */
    protected $connectionName;

    /**
     * The name of the queue the job belongs to.
     *
     * @var string
     */
    protected $queue;

    /**
     * Get the job identifier.
     *
     * @return string|int|null
     */
    abstract public function getJobId();

    /**
     * Get the raw body of the job.
     *
     * @return string
     */
    abstract public function getRawBody();

    /**
     * Get the UUID of the job.
     *
     * @return string|null
     */
    public function uuid()
    {}

    /**
     * Fire the job.
     *
     * @return void
     */
    public function fire()
    {}

    /**
     * Delete the job from the queue.
     *
     * @return void
     */
    public function delete()
    {}

    /**
     * Determine if the job has been deleted.
     *
     * @return bool
     */
    public function isDeleted()
    {}

    /**
     * Release the job back into the queue after (n) seconds.
     *
     * @param  int  $delay
     * @return void
     */
    public function release($delay = 0)
    {}

    /**
     * Determine if the job was released back into the queue.
     *
     * @return bool
     */
    public function isReleased()
    {}

    /**
     * Determine if the job has been deleted or released.
     *
     * @return bool
     */
    public function isDeletedOrReleased()
    {}

    /**
     * Determine if the job has been marked as a failure.
     *
     * @return bool
     */
    public function hasFailed()
    {}

    /**
     * Mark the job as "failed".
     *
     * @return void
     */
    public function markAsFailed()
    {}

    /**
     * Delete the job, call the "failed" method, and raise the failed job event.
     *
     * @param  \Throwable|null  $e
     * @return void
     */
    public function fail($e = null)
    {}

    /**
     * Determine if the current database transaction should be rolled back to level zero.
     *
     * @param  \Throwable|null  $e
     * @return bool
     */
    protected function shouldRollBackDatabaseTransaction($e)
    {}

    /**
     * Process an exception that caused the job to fail.
     *
     * @param  \Throwable|null  $e
     * @return void
     */
    protected function failed($e)
    {}

    /**
     * Resolve the given class.
     *
     * @param  string  $class
     * @return mixed
     */
    protected function resolve($class)
    {}

    /**
     * Get the resolved job handler instance.
     *
     * @return mixed
     */
    public function getResolvedJob()
    {}

    /**
     * Get the decoded body of the job.
     *
     * @return array
     */
    public function payload()
    {}

    /**
     * Get the number of times to attempt a job.
     *
     * @return int|null
     */
    public function maxTries()
    {}

    /**
     * Get the number of times to attempt a job after an exception.
     *
     * @return int|null
     */
    public function maxExceptions()
    {}

    /**
     * Determine if the job should fail when it timeouts.
     *
     * @return bool
     */
    public function shouldFailOnTimeout()
    {}

    /**
     * The number of seconds to wait before retrying a job that encountered an uncaught exception.
     *
     * @return int|int[]|null
     */
    public function backoff()
    {}

    /**
     * Get the number of seconds the job can run.
     *
     * @return int|null
     */
    public function timeout()
    {}

    /**
     * Get the timestamp indicating when the job should timeout.
     *
     * @return int|null
     */
    public function retryUntil()
    {}

    /**
     * Get the name of the queued job class.
     *
     * @return string
     */
    public function getName()
    {}

    /**
     * Get the resolved display name of the queued job class.
     *
     * Resolves the name of "wrapped" jobs such as class-based handlers.
     *
     * @return string
     */
    public function resolveName()
    {}

    /**
     * Get the class of the queued job.
     *
     * Resolves the class of "wrapped" jobs such as class-based handlers.
     *
     * @return string
     */
    public function resolveQueuedJobClass()
    {}

    /**
     * Get the name of the connection the job belongs to.
     *
     * @return string
     */
    public function getConnectionName()
    {}

    /**
     * Get the name of the queue the job belongs to.
     *
     * @return string
     */
    public function getQueue()
    {}

    /**
     * Get the service container instance.
     *
     * @return \Illuminate\Container\Container
     */
    public function getContainer()
    {}
}
