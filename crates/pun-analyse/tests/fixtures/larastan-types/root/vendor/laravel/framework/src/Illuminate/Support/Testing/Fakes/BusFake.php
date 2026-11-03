<?php

namespace Illuminate\Support\Testing\Fakes;

use Closure;
use Illuminate\Bus\BatchRepository;
use Illuminate\Bus\ChainedBatch;
use Illuminate\Bus\PendingBatch;
use Illuminate\Contracts\Bus\QueueingDispatcher;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ReflectsClosures;
use PHPUnit\Framework\Assert as PHPUnit;
use RuntimeException;

class BusFake implements Fake, QueueingDispatcher
{
    use MatchesProperties, ReflectsClosures;

    /**
     * The original Bus dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Bus\QueueingDispatcher
     */
    public $dispatcher;

    /**
     * The job types that should be intercepted instead of dispatched.
     *
     * @var array
     */
    protected $jobsToFake = [];

    /**
     * The job types that should be dispatched instead of faked.
     *
     * @var array
     */
    protected $jobsToDispatch = [];

    /**
     * The fake repository to track batched jobs.
     *
     * @var \Illuminate\Bus\BatchRepository
     */
    protected $batchRepository;

    /**
     * The commands that have been dispatched.
     *
     * @var array
     */
    protected $commands = [];

    /**
     * The commands that have been dispatched synchronously.
     *
     * @var array
     */
    protected $commandsSync = [];

    /**
     * The commands that have been dispatched after the response has been sent.
     *
     * @var array
     */
    protected $commandsAfterResponse = [];

    /**
     * The batches that have been dispatched.
     *
     * @var array
     */
    protected $batches = [];

    /**
     * Indicates if commands should be serialized and restored when pushed to the Bus.
     *
     * @var bool
     */
    protected bool $serializeAndRestore = false;

    /**
     * Create a new bus fake instance.
     *
     * @param  \Illuminate\Contracts\Bus\QueueingDispatcher  $dispatcher
     * @param  array|string  $jobsToFake
     * @param  \Illuminate\Bus\BatchRepository|null  $batchRepository
     */
    public function __construct(QueueingDispatcher $dispatcher, $jobsToFake = [], ?BatchRepository $batchRepository = null)
    {}

    /**
     * Specify the jobs that should be dispatched instead of faked.
     *
     * @param  array|string  $jobsToDispatch
     * @return $this
     */
    public function except($jobsToDispatch)
    {}

    /**
     * Assert if a job was dispatched based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|array<string, mixed>|int|null  $callback
     * @return void
     */
    public function assertDispatched($command, $callback = null)
    {}

    /**
     * Assert if a job was pushed exactly once.
     *
     * @param  string|\Closure  $command
     * @return void
     */
    public function assertDispatchedOnce($command)
    {}

    /**
     * Assert if a job was pushed a number of times.
     *
     * @param  string|\Closure  $command
     * @param  int  $times
     * @return void
     */
    public function assertDispatchedTimes($command, $times = 1)
    {}

    /**
     * Determine if a job was dispatched based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|array<string, mixed>|null  $callback
     * @return void
     */
    public function assertNotDispatched($command, $callback = null)
    {}

    /**
     * Assert that no jobs were dispatched.
     *
     * @return void
     */
    public function assertNothingDispatched()
    {}

    /**
     * Assert if a job was explicitly dispatched synchronously based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|int|null  $callback
     * @return void
     */
    public function assertDispatchedSync($command, $callback = null)
    {}

    /**
     * Assert if a job was pushed synchronously a number of times.
     *
     * @param  string|\Closure  $command
     * @param  int  $times
     * @return void
     */
    public function assertDispatchedSyncTimes($command, $times = 1)
    {}

    /**
     * Determine if a job was dispatched based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|null  $callback
     * @return void
     */
    public function assertNotDispatchedSync($command, $callback = null)
    {}

    /**
     * Assert if a job was dispatched after the response was sent based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|int|null  $callback
     * @return void
     */
    public function assertDispatchedAfterResponse($command, $callback = null)
    {}

    /**
     * Assert if a job was pushed after the response was sent a number of times.
     *
     * @param  string|\Closure  $command
     * @param  int  $times
     * @return void
     */
    public function assertDispatchedAfterResponseTimes($command, $times = 1)
    {}

    /**
     * Determine if a job was dispatched based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|null  $callback
     * @return void
     */
    public function assertNotDispatchedAfterResponse($command, $callback = null)
    {}

    /**
     * Assert if a chain of jobs was dispatched.
     *
     * @param  array  $expectedChain
     * @return void
     */
    public function assertChained(array $expectedChain)
    {}

    /**
     * Assert no chained jobs was dispatched.
     *
     * @return void
     */
    public function assertNothingChained()
    {}

    /**
     * Reset the chain properties to their default values on the job.
     *
     * @param  mixed  $job
     * @return mixed
     */
    protected function resetChainPropertiesToDefaults($job)
    {}

    /**
     * Assert if a job was dispatched with an empty chain based on a truth-test callback.
     *
     * @param  string|\Closure  $command
     * @param  callable|null  $callback
     * @return void
     */
    public function assertDispatchedWithoutChain($command, $callback = null)
    {}

    /**
     * Assert if a job was dispatched with chained jobs based on a truth-test callback.
     *
     * @param  string  $command
     * @param  array  $expectedChain
     * @param  callable|null  $callback
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function assertDispatchedWithChainOfObjects($command, $expectedChain, $callback)
    {}

    /**
     * Create a new assertion about a chained batch.
     *
     * @param  \Closure(\Illuminate\Bus\PendingBatch): bool  $callback
     * @return \Illuminate\Support\Testing\Fakes\ChainedBatchTruthTest
     */
    public function chainedBatch(Closure $callback)
    {}

    /**
     * Assert if a batch was dispatched based on a truth-test callback.
     *
     * @param  array|callable(\Illuminate\Bus\PendingBatch): bool  $callback
     * @return void
     */
    public function assertBatched(callable|array $callback)
    {}

    /**
     * Assert the number of batches that have been dispatched.
     *
     * @param  int  $count
     * @return void
     */
    public function assertBatchCount($count)
    {}

    /**
     * Assert that no batched jobs were dispatched.
     *
     * @return void
     */
    public function assertNothingBatched()
    {}

    /**
     * Assert that no jobs were dispatched, chained, or batched.
     *
     * @return void
     */
    public function assertNothingPlaced()
    {}

    /**
     * Get all of the jobs matching a truth-test callback.
     *
     * @param  string  $command
     * @param  callable|array<string, mixed>|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function dispatched($command, $callback = null)
    {}

    /**
     * Get all of the jobs dispatched synchronously matching a truth-test callback.
     *
     * @param  string  $command
     * @param  callable|array<string, mixed>|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function dispatchedSync(string $command, $callback = null)
    {}

    /**
     * Get all of the jobs dispatched after the response was sent matching a truth-test callback.
     *
     * @param  string  $command
     * @param  callable|array<string, mixed>|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function dispatchedAfterResponse(string $command, $callback = null)
    {}

    /**
     * Get all of the pending batches matching a truth-test callback.
     *
     * @param  callable(\Illuminate\Bus\PendingBatch): bool  $callback
     * @return \Illuminate\Support\Collection<int, \Illuminate\Bus\PendingBatch>
     */
    public function batched(callable $callback)
    {}

    /**
     * Determine if there are any stored commands for a given class.
     *
     * @param  string  $command
     * @return bool
     */
    public function hasDispatched($command)
    {}

    /**
     * Determine if there are any stored commands for a given class.
     *
     * @param  string  $command
     * @return bool
     */
    public function hasDispatchedSync($command)
    {}

    /**
     * Determine if there are any stored commands for a given class.
     *
     * @param  string  $command
     * @return bool
     */
    public function hasDispatchedAfterResponse($command)
    {}

    /**
     * Dispatch a command to its appropriate handler.
     *
     * @param  mixed  $command
     * @return mixed
     */
    public function dispatch($command)
    {}

    /**
     * Dispatch a command to its appropriate handler in the current process.
     *
     * Queueable jobs will be dispatched to the "sync" queue.
     *
     * @param  mixed  $command
     * @param  mixed  $handler
     * @return mixed
     */
    public function dispatchSync($command, $handler = null)
    {}

    /**
     * Dispatch a command to its appropriate handler in the current process.
     *
     * @param  mixed  $command
     * @param  mixed  $handler
     * @return mixed
     */
    public function dispatchNow($command, $handler = null)
    {}

    /**
     * Dispatch a command to its appropriate handler behind a queue.
     *
     * @param  mixed  $command
     * @return mixed
     */
    public function dispatchToQueue($command)
    {}

    /**
     * Dispatch a command to its appropriate handler after the current process.
     *
     * @param  mixed  $command
     * @param  mixed  $handler
     * @return void
     */
    public function dispatchAfterResponse($command, $handler = null)
    {}

    /**
     * Dispatch multiple commands in bulk to their appropriate handlers on the queue.
     *
     * @param  iterable  $jobs
     * @return void
     */
    public function bulk($jobs)
    {}

    /**
     * Create a new chain of queueable jobs.
     *
     * @param  \Illuminate\Support\Collection|array|null  $jobs
     * @return \Illuminate\Foundation\Bus\PendingChain
     */
    public function chain($jobs = null)
    {}

    /**
     * Attempt to find the batch with the given ID.
     *
     * @param  string  $batchId
     * @return \Illuminate\Bus\Batch|null
     */
    public function findBatch(string $batchId)
    {}

    /**
     * Create a new batch of queueable jobs.
     *
     * @param  \Illuminate\Support\Collection|array  $jobs
     * @return \Illuminate\Bus\PendingBatch
     */
    public function batch($jobs)
    {}

    /**
     * Dispatch an empty job batch for testing.
     *
     * @param  string  $name
     * @return \Illuminate\Bus\Batch
     */
    public function dispatchFakeBatch($name = '')
    {}

    /**
     * Record the fake pending batch dispatch.
     *
     * @param  \Illuminate\Bus\PendingBatch  $pendingBatch
     * @return \Illuminate\Bus\Batch
     */
    public function recordPendingBatch(PendingBatch $pendingBatch)
    {}

    /**
     * Determine if a command should be faked or actually dispatched.
     *
     * @param  mixed  $command
     * @return bool
     */
    protected function shouldFakeJob($command)
    {}

    /**
     * Determine if a command should be dispatched or not.
     *
     * @param  mixed  $command
     * @return bool
     */
    protected function shouldDispatchCommand($command)
    {}

    /**
     * Specify if commands should be serialized and restored when being batched.
     *
     * @param  bool  $serializeAndRestore
     * @return $this
     */
    public function serializeAndRestore(bool $serializeAndRestore = true)
    {}

    /**
     * Serialize and unserialize the command to simulate the queueing process.
     *
     * @param  mixed  $command
     * @return mixed
     */
    protected function serializeAndRestoreCommand($command)
    {}

    /**
     * Return the command representation that should be stored.
     *
     * @param  mixed  $command
     * @return mixed
     */
    protected function getCommandRepresentation($command)
    {}

    /**
     * Set the pipes commands should be piped through before dispatching.
     *
     * @param  array  $pipes
     * @return $this
     */
    public function pipeThrough(array $pipes)
    {}

    /**
     * Determine if the given command has a handler.
     *
     * @param  mixed  $command
     * @return bool
     */
    public function hasCommandHandler($command)
    {}

    /**
     * Retrieve the handler for a command.
     *
     * @param  mixed  $command
     * @return mixed
     */
    public function getCommandHandler($command)
    {}

    /**
     * Map a command to a handler.
     *
     * @param  array  $map
     * @return $this
     */
    public function map(array $map)
    {}

    /**
     * Allow dispatching after responses.
     *
     * @return $this
     */
    public function withDispatchingAfterResponses()
    {}

    /**
     * Disable dispatching after responses.
     *
     * @return $this
     */
    public function withoutDispatchingAfterResponses()
    {}

    /**
     * Get the batches that have been dispatched.
     *
     * @return array
     */
    public function dispatchedBatches()
    {}
}
