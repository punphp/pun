<?php

namespace Illuminate\Queue;

use Exception;
use Illuminate\Bus\Batchable;
use Illuminate\Bus\BatchRepository;
use Illuminate\Bus\DebounceLock;
use Illuminate\Bus\Queueable;
use Illuminate\Bus\UniqueLock;
use Illuminate\Contracts\Bus\Dispatcher;
use Illuminate\Contracts\Cache\Factory as CacheFactory;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Encryption\Encrypter;
use Illuminate\Contracts\Queue\Job;
use Illuminate\Contracts\Queue\ShouldBeUnique;
use Illuminate\Contracts\Queue\ShouldBeUniqueUntilProcessing;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Events\CallQueuedListener;
use Illuminate\Log\Context\Repository as ContextRepository;
use Illuminate\Pipeline\Pipeline;
use Illuminate\Queue\Events\JobDebounced;
use RuntimeException;

class CallQueuedHandler
{
    /**
     * The bus dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Bus\Dispatcher
     */
    protected $dispatcher;

    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The command currently being processed.
     *
     * @var mixed
     */
    protected $runningCommand;

    /**
     * Create a new handler instance.
     *
     * @param  \Illuminate\Contracts\Bus\Dispatcher  $dispatcher
     * @param  \Illuminate\Contracts\Container\Container  $container
     */
    public function __construct(Dispatcher $dispatcher, Container $container)
    {}

    /**
     * Handle the queued job.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  array  $data
     * @return void
     */
    public function call(Job $job, array $data)
    {}

    /**
     * Get the command from the given payload.
     *
     * @param  array  $data
     * @return mixed
     *
     * @throws \RuntimeException
     */
    protected function getCommand(array $data)
    {}

    /**
     * Dispatch the given job / command through its specified middleware.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  mixed  $command
     * @return mixed
     */
    protected function dispatchThroughMiddleware(Job $job, $command)
    {}

    /**
     * Resolve the handler for the given command.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  mixed  $command
     * @return mixed
     */
    protected function resolveHandler($job, $command)
    {}

    /**
     * Set the job instance of the given class if necessary.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  mixed  $instance
     * @return mixed
     */
    protected function setJobInstanceIfNecessary(Job $job, $instance)
    {}

    /**
     * Ensure the next job in the chain is dispatched if applicable.
     *
     * @param  mixed  $command
     * @return void
     */
    protected function ensureNextJobInChainIsDispatched($command)
    {}

    /**
     * Ensure the batch is notified of the successful job completion.
     *
     * @param  mixed  $command
     * @return void
     */
    protected function ensureSuccessfulBatchJobIsRecorded($command)
    {}

    /**
     * Determine if the unique job lock can be safely released.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  mixed  $command
     * @return bool
     */
    protected function uniqueJobLockShouldBeReleased(Job $job, $command)
    {}

    /**
     * Ensure the lock for a unique job is released.
     *
     * @param  mixed  $command
     * @return void
     */
    protected function ensureUniqueJobLockIsReleased($command)
    {}

    /**
     * Determine if the debounced command was superseded by a newer dispatch.
     *
     * @param  mixed  $command
     * @return bool
     */
    protected function commandShouldBeDebounced($command)
    {}

    /**
     * Handle a debounced (superseded) job by firing an event and deleting it.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  mixed  $command
     * @return void
     */
    protected function deleteDebouncedJob($job, $command)
    {}

    /**
     * Determine if the given command should be unique.
     */
    protected function commandShouldBeUnique(mixed $command): bool
    {}

    /**
     * Determine if the given command should be unique until processing begins.
     */
    protected function commandShouldBeUniqueUntilProcessing(mixed $command): bool
    {}

    /**
     * Handle a model not found exception.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  \Throwable  $e
     * @return void
     */
    protected function handleModelNotFound(Job $job, $e)
    {}

    /**
     * Ensure the lock for a unique job is released via context.
     *
     * This is required when we can't unserialize the job due to missing models.
     *
     * @return void
     */
    protected function ensureUniqueJobLockIsReleasedViaContext()
    {}

    /**
     * Record a potentially batched job as successful when deleted because models were missing.
     *
     * @param  \Illuminate\Contracts\Queue\Job  $job
     * @param  string  $class
     * @return void
     */
    protected function ensureSuccessfulBatchJobIsRecordedForMissingModel(Job $job, string $class)
    {}

    /**
     * Call the failed method on the job instance.
     *
     * The exception that caused the failure will be passed.
     *
     * @param  array  $data
     * @param  \Throwable|null  $e
     * @param  string  $uuid
     * @param  \Illuminate\Contracts\Queue\Job|null  $job
     * @return void
     */
    public function failed(array $data, $e, string $uuid, ?Job $job = null)
    {}

    /**
     * Ensure the batch is notified of the failed job.
     *
     * @param  string  $uuid
     * @param  mixed  $command
     * @param  \Throwable|null  $e
     * @return void
     */
    protected function ensureFailedBatchJobIsRecorded(string $uuid, $command, $e)
    {}

    /**
     * Ensure the chained job catch callbacks are invoked.
     *
     * @param  string  $uuid
     * @param  mixed  $command
     * @param  \Throwable|null  $e
     * @return void
     */
    protected function ensureChainCatchCallbacksAreInvoked(string $uuid, $command, $e)
    {}

    /**
     * Get the command currently being processed.
     *
     * @return mixed
     */
    public function getRunningCommand()
    {}
}
