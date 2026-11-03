<?php

namespace Illuminate\Queue;

use Closure;
use DateTimeInterface;
use Illuminate\Bus\DebounceLock;
use Illuminate\Bus\Queueable;
use Illuminate\Bus\UniqueLock;
use Illuminate\Container\Container;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Encryption\Encrypter;
use Illuminate\Contracts\Queue\ShouldBeEncrypted;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Contracts\Queue\ShouldQueueAfterCommit;
use Illuminate\Events\CallQueuedListener;
use Illuminate\Queue\Attributes\Backoff;
use Illuminate\Queue\Attributes\CountCrashesAsExceptions;
use Illuminate\Queue\Attributes\DeleteWhenMissingModels;
use Illuminate\Queue\Attributes\FailOnTimeout;
use Illuminate\Queue\Attributes\MaxExceptions;
use Illuminate\Queue\Attributes\ReadsQueueAttributes;
use Illuminate\Queue\Attributes\Timeout;
use Illuminate\Queue\Attributes\Tries;
use Illuminate\Queue\Events\JobQueued;
use Illuminate\Queue\Events\JobQueueing;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Queue\Concerns\ResolvesQueueRoutes;
use Illuminate\Support\Str;
use RuntimeException;
use Throwable;

abstract class Queue
{
    use InteractsWithTime, ReadsQueueAttributes, ResolvesQueueRoutes;

    /**
     * The IoC container instance.
     *
     * @var \Illuminate\Container\Container
     */
    protected $container;

    /**
     * The connection name for the queue.
     *
     * @var string
     */
    protected $connectionName;

    /**
     * The original configuration for the queue.
     *
     * @var array
     */
    protected $config;

    /**
     * Indicates that jobs should be dispatched after all database transactions have committed.
     *
     * @var bool|null
     */
    protected $dispatchAfterCommit;

    /**
     * The create payload callbacks.
     *
     * @var callable[]
     */
    protected static $createPayloadCallbacks = [];

    /**
     * Push a new job onto the queue.
     *
     * @param  string  $queue
     * @param  string  $job
     * @param  mixed  $data
     * @return mixed
     */
    public function pushOn($queue, $job, $data = '')
    {}

    /**
     * Push a new job onto a specific queue after (n) seconds.
     *
     * @param  string  $queue
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string  $job
     * @param  mixed  $data
     * @return mixed
     */
    public function laterOn($queue, $delay, $job, $data = '')
    {}

    /**
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  string|null  $queue
     * @return void
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Create a payload string from the given job and data.
     *
     * @param  \Closure|string|object  $job
     * @param  string  $queue
     * @param  mixed  $data
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @return string
     *
     * @throws \Illuminate\Queue\InvalidPayloadException
     */
    protected function createPayload($job, $queue, $data = '', $delay = null)
    {}

    /**
     * Create a payload array from the given job and data.
     *
     * @param  string|object  $job
     * @param  string  $queue
     * @param  mixed  $data
     * @return array
     */
    protected function createPayloadArray($job, $queue, $data = '')
    {}

    /**
     * Create a payload for an object-based queue handler.
     *
     * @param  object  $job
     * @param  string  $queue
     * @return array
     *
     * @throws \RuntimeException
     */
    protected function createObjectPayload($job, $queue)
    {}

    /**
     * Get the display name for the given job.
     *
     * @param  object  $job
     * @return string
     */
    protected function getDisplayName($job)
    {}

    /**
     * Get the maximum number of attempts for an object-based queue handler.
     *
     * @param  mixed  $job
     * @return mixed
     */
    public function getJobTries($job)
    {}

    /**
     * Get the backoff for an object-based queue handler.
     *
     * @param  mixed  $job
     * @return mixed
     */
    public function getJobBackoff($job)
    {}

    /**
     * Get the expiration timestamp for an object-based queue handler.
     *
     * @param  mixed  $job
     * @return mixed
     */
    public function getJobExpiration($job)
    {}

    /**
     * Determine if the job should be encrypted.
     *
     * @param  object  $job
     * @return bool
     */
    protected function jobShouldBeEncrypted($job)
    {}

    /**
     * Create a typical, string based queue payload array.
     *
     * @param  string  $job
     * @param  string  $queue
     * @param  mixed  $data
     * @return array
     */
    protected function createStringPayload($job, $queue, $data)
    {}

    /**
     * Register a callback to be executed when creating job payloads.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public static function createPayloadUsing($callback)
    {}

    /**
     * Create the given payload using any registered payload hooks.
     *
     * @param  string  $queue
     * @return array
     */
    protected function withCreatePayloadHooks($queue, array $payload)
    {}

    /**
     * Enqueue a job using the given callback.
     *
     * @param  \Closure|string|object  $job
     * @param  string  $payload
     * @param  string|null  $queue
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @param  callable  $callback
     * @return mixed
     */
    protected function enqueueUsing($job, $payload, $queue, $delay, $callback)
    {}

    /**
     * Determine if the job should be dispatched after all database transactions have committed.
     *
     * @param  \Closure|string|object  $job
     * @return bool
     */
    protected function shouldDispatchAfterCommit($job)
    {}

    /**
     * Partition the given jobs by whether they should be deferred until the active database transaction commits.
     *
     * @param  array  $jobs
     * @return array{0: array, 1: array}
     */
    protected function partitionJobsByAfterCommit(array $jobs)
    {}

    /**
     * Register callbacks to release locks if the current database transaction is rolled back.
     *
     * @param  \Closure|string|object  $job
     * @return void
     */
    protected function registerRollbackCallbacksForJobsThatDispatchAfterCommit($job)
    {}

    /**
     * Raise the job queueing event.
     *
     * @param  string|null  $queue
     * @param  \Closure|string|object  $job
     * @param  string  $payload
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @return void
     */
    protected function raiseJobQueueingEvent($queue, $job, $payload, $delay)
    {}

    /**
     * Raise the job queued event.
     *
     * @param  string|null  $queue
     * @param  string|int|null  $jobId
     * @param  \Closure|string|object  $job
     * @param  string  $payload
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @return void
     */
    protected function raiseJobQueuedEvent($queue, $jobId, $job, $payload, $delay)
    {}

    /**
     * Get the routed queue name for the given queue.
     *
     * @param  string  $queue
     * @return string
     */
    protected function resolveQueue($queue)
    {}

    /**
     * Get the connection name for the queue.
     *
     * @return string
     */
    public function getConnectionName()
    {}

    /**
     * Set the connection name for the queue.
     *
     * @param  string  $name
     * @return $this
     */
    public function setConnectionName($name)
    {}

    /**
     * Get the queue configuration array.
     *
     * @return array
     */
    public function getConfig()
    {}

    /**
     * Set the queue configuration array.
     *
     * @return $this
     */
    public function setConfig(array $config)
    {}

    /**
     * Get the container instance being used by the connection.
     *
     * @return \Illuminate\Container\Container
     */
    public function getContainer()
    {}

    /**
     * Set the IoC container instance.
     *
     * @return void
     */
    public function setContainer(Container $container)
    {}
}
