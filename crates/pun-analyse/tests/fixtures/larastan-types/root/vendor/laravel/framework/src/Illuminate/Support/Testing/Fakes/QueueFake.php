<?php

namespace Illuminate\Support\Testing\Fakes;

use BadMethodCallException;
use Closure;
use Illuminate\Bus\UniqueLock;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Queue\Queue;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Events\CallQueuedListener;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Attributes\ReadsQueueAttributes;
use Illuminate\Queue\CallQueuedClosure;
use Illuminate\Queue\Jobs\InspectedJob;
use Illuminate\Queue\QueueManager;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ReflectsClosures;
use PHPUnit\Framework\Assert as PHPUnit;

use function Illuminate\Support\enum_value;

/**
 * @phpstan-type RawPushType array{"payload": string, "queue": string|null, "options": array<array-key, mixed>}
 */
class QueueFake extends QueueManager implements Fake, Queue
{
    use MatchesProperties, ReadsQueueAttributes, ReflectsClosures;

    /**
     * The original queue manager.
     *
     * @var \Illuminate\Contracts\Queue\Queue
     */
    public $queue;

    /**
     * The job types that should be intercepted instead of pushed to the queue.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $jobsToFake;

    /**
     * The job types that should be pushed to the queue and not intercepted.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $jobsToBeQueued;

    /**
     * All of the jobs that have been pushed.
     *
     * @var array
     */
    protected $jobs = [];

    /**
     * All of the jobs that have been pushed with a delay.
     *
     * @var array
     */
    protected $delayed = [];

    /**
     * All of the payloads that have been raw pushed.
     *
     * @var list<RawPushType>
     */
    protected $rawPushes = [];

    /**
     * All of the unique jobs that were pushed.
     *
     * @var array
     */
    protected $uniqueJobs = [];

    /**
     * All of the jobs that have been marked as reserved.
     *
     * @var array
     */
    protected $reserved = [];

    /**
     * Indicates if items should be serialized and restored when pushed to the queue.
     *
     * @var bool
     */
    protected bool $serializeAndRestore = false;

    /**
     * The callbacks that should be invoked before pushing a job.
     *
     * @var array<int, callable>
     */
    protected $beforePushingCallbacks = [];

    /**
     * The callbacks that should be invoked after pushing a job.
     *
     * @var array<int, callable>
     */
    protected $afterPushingCallbacks = [];

    /**
     * Create a new fake queue instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  array  $jobsToFake
     * @param  \Illuminate\Queue\QueueManager|null  $queue
     */
    public function __construct($app, $jobsToFake = [], $queue = null)
    {}

    /**
     * Specify the jobs that should be queued instead of faked.
     *
     * @param  array|string  $jobsToBeQueued
     * @return $this
     */
    public function except($jobsToBeQueued)
    {}

    /**
     * Assert if a job was pushed based on a truth-test callback.
     *
     * @param  string|\Closure  $job
     * @param  callable|array<string, mixed>|int|null  $callback
     * @return void
     */
    public function assertPushed($job, $callback = null)
    {}

    /**
     * Assert if a job was pushed a number of times.
     *
     * @param  string  $job
     * @param  int  $times
     * @return void
     */
    public function assertPushedTimes($job, $times = 1)
    {}

    /**
     * Assert if a job was pushed exactly once.
     *
     * @param  string  $job
     * @return void
     */
    public function assertPushedOnce($job)
    {}

    /**
     * Assert if a job was pushed based on a truth-test callback.
     *
     * @param  \UnitEnum|string  $queue
     * @param  string|\Closure  $job
     * @param  callable|array<string, mixed>|null  $callback
     * @return void
     */
    public function assertPushedOn($queue, $job, $callback = null)
    {}

    /**
     * Assert if a job was pushed with chained jobs based on a truth-test callback.
     *
     * @param  string  $job
     * @param  array  $expectedChain
     * @param  callable|null  $callback
     * @return void
     */
    public function assertPushedWithChain($job, $expectedChain = [], $callback = null)
    {}

    /**
     * Assert if a job was pushed with an empty chain based on a truth-test callback.
     *
     * @param  string  $job
     * @param  callable|null  $callback
     * @return void
     */
    public function assertPushedWithoutChain($job, $callback = null)
    {}

    /**
     * Assert if a job was pushed with chained jobs based on a truth-test callback.
     *
     * @param  string  $job
     * @param  array  $expectedChain
     * @param  callable|null  $callback
     * @return void
     */
    protected function assertPushedWithChainOfObjects($job, $expectedChain, $callback)
    {}

    /**
     * Assert if a job was pushed with chained jobs based on a truth-test callback.
     *
     * @param  string  $job
     * @param  array  $expectedChain
     * @param  callable|null  $callback
     * @return void
     */
    protected function assertPushedWithChainOfClasses($job, $expectedChain, $callback)
    {}

    /**
     * Assert if a closure was pushed based on a truth-test callback.
     *
     * @param  callable|int|null  $callback
     * @return void
     */
    public function assertClosurePushed($callback = null)
    {}

    /**
     * Assert that a closure was not pushed based on a truth-test callback.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public function assertClosureNotPushed($callback = null)
    {}

    /**
     * Determine if the given chain is entirely composed of objects.
     *
     * @param  array  $chain
     * @return bool
     */
    protected function isChainOfObjects($chain)
    {}

    /**
     * Determine if a job was pushed based on a truth-test callback.
     *
     * @param  string|\Closure  $job
     * @param  callable|array<string, mixed>|null  $callback
     * @return void
     */
    public function assertNotPushed($job, $callback = null)
    {}

    /**
     * Assert the total count of jobs that were pushed.
     *
     * @param  int  $expectedCount
     * @return void
     */
    public function assertCount($expectedCount)
    {}

    /**
     * Assert that no jobs were pushed.
     *
     * @return void
     */
    public function assertNothingPushed()
    {}

    /**
     * Get all of the jobs matching a truth-test callback.
     *
     * @param  string  $job
     * @param  callable|array<string, mixed>|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function pushed($job, $callback = null)
    {}

    /**
     * Get all of the raw pushes matching a truth-test callback.
     *
     * @param  null|\Closure(string, ?string, array): bool  $callback
     * @return \Illuminate\Support\Collection<int, RawPushType>
     */
    public function pushedRaw($callback = null)
    {}

    /**
     * Get all of the jobs by listener class, passing an optional truth-test callback.
     *
     * @param  class-string  $listenerClass
     * @param  (\Closure(mixed, \Illuminate\Events\CallQueuedListener, string|null, mixed): bool)|null  $callback
     * @return \Illuminate\Support\Collection<int, \Illuminate\Events\CallQueuedListener>
     */
    public function listenersPushed($listenerClass, $callback = null)
    {}

    /**
     * Determine if there are any stored jobs for a given class.
     *
     * @param  string  $job
     * @return bool
     */
    public function hasPushed($job)
    {}

    /**
     * Resolve a queue connection instance.
     *
     * @param  mixed  $value
     * @return \Illuminate\Contracts\Queue\Queue
     */
    public function connection($value = null)
    {}

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
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function pendingJobs($queue = null): Collection
    {}

    /**
     * Get the delayed jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function delayedJobs($queue = null): Collection
    {}

    /**
     * Get the reserved jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function reservedJobs($queue = null): Collection
    {}

    /**
     * Get all pending jobs across every queue.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function allPendingJobs(): Collection
    {}

    /**
     * Get all delayed jobs across every queue.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function allDelayedJobs(): Collection
    {}

    /**
     * Map an array of jobs to a collection of inspected jobs.
     *
     * @param  array  $jobs
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    protected function inspectJobs(array $jobs): Collection
    {}

    /**
     * Get all reserved jobs across every queue.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
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
     * @param  string|object  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function push($job, $data = '', $queue = null)
    {}

    /**
     * Determine if a job should be faked or actually dispatched.
     *
     * @param  object  $job
     * @return bool
     */
    public function shouldFakeJob($job)
    {}

    /**
     * Determine if a job should be pushed to the queue instead of faked.
     *
     * @param  object  $job
     * @return bool
     */
    protected function shouldDispatchJob($job)
    {}

    /**
     * Push a raw payload onto the queue.
     *
     * @param  string  $payload
     * @param  \UnitEnum|string|null  $queue
     * @param  array  $options
     * @return mixed
     */
    public function pushRaw($payload, $queue = null, array $options = [])
    {}

    /**
     * Push a new job onto the queue after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string|object  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  \UnitEnum|string  $queue
     * @param  string|object  $job
     * @param  mixed  $data
     * @return mixed
     */
    public function pushOn($queue, $job, $data = '')
    {}

    /**
     * Push a new job onto a specific queue after (n) seconds.
     *
     * @param  \UnitEnum|string  $queue
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string|object  $job
     * @param  mixed  $data
     * @return mixed
     */
    public function laterOn($queue, $delay, $job, $data = '')
    {}

    /**
     * Mark the given job as reserved.
     *
     * @param  \Closure|string|object  $job
     * @param  \UnitEnum|string|null  $queue
     * @return void
     */
    public function reserve($job, $queue = null)
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
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Get the jobs that have been pushed.
     *
     * @return array
     */
    public function pushedJobs()
    {}

    /**
     * Get the payloads that were pushed raw.
     *
     * @return list<RawPushType>
     */
    public function rawPushes()
    {}

    /**
     * Specify if jobs should be serialized and restored when being "pushed" to the queue.
     *
     * @param  bool  $serializeAndRestore
     * @return $this
     */
    public function serializeAndRestore(bool $serializeAndRestore = true)
    {}

    /**
     * Serialize and unserialize the job to simulate the queueing process.
     *
     * @param  mixed  $job
     * @return mixed
     */
    protected function serializeAndRestoreJob($job)
    {}

    /**
     * Release the locks for all unique jobs that were pushed.
     *
     * @return void
     */
    public function releaseUniqueJobLocks()
    {}

    /**
     * Clear all of the reserved jobs.
     *
     * @return void
     */
    public function clearReserved()
    {}

    /**
     * Register a callback to be invoked before pushing a job.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function beforePushing(callable $callback)
    {}

    /**
     * Register a callback to be invoked after pushing a job.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function afterPushing(callable $callback)
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
     * Override the QueueManager to prevent circular dependency.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
