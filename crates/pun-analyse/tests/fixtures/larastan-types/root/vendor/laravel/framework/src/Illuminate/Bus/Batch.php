<?php

namespace Illuminate\Bus;

use Carbon\CarbonImmutable;
use Closure;
use Illuminate\Bus\Events\BatchCanceled;
use Illuminate\Bus\Events\BatchFinished;
use Illuminate\Bus\Events\BatchStarted;
use Illuminate\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Queue\Factory as QueueFactory;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Queue\CallQueuedClosure;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use JsonSerializable;
use Throwable;

class Batch implements Arrayable, JsonSerializable
{
    /**
     * The queue factory implementation.
     *
     * @var \Illuminate\Contracts\Queue\Factory
     */
    protected $queue;

    /**
     * The repository implementation.
     *
     * @var \Illuminate\Bus\BatchRepository
     */
    protected $repository;

    /**
     * The batch ID.
     *
     * @var string
     */
    public $id;

    /**
     * The batch name.
     *
     * @var string
     */
    public $name;

    /**
     * The total number of jobs that belong to the batch.
     *
     * @var int
     */
    public $totalJobs;

    /**
     * The total number of jobs that are still pending.
     *
     * @var int
     */
    public $pendingJobs;

    /**
     * The total number of jobs that have failed.
     *
     * @var int
     */
    public $failedJobs;

    /**
     * The IDs of the jobs that have failed.
     *
     * @var array
     */
    public $failedJobIds;

    /**
     * The batch options.
     *
     * @var array
     */
    public $options;

    /**
     * The date indicating when the batch was created.
     *
     * @var \Carbon\CarbonImmutable
     */
    public $createdAt;

    /**
     * The date indicating when the batch was cancelled.
     *
     * @var \Carbon\CarbonImmutable|null
     */
    public $cancelledAt;

    /**
     * The date indicating when the batch was finished.
     *
     * @var \Carbon\CarbonImmutable|null
     */
    public $finishedAt;

    /**
     * Create a new batch instance.
     */
    public function __construct(
        QueueFactory $queue,
        BatchRepository $repository,
        string $id,
        string $name,
        int $totalJobs,
        int $pendingJobs,
        int $failedJobs,
        array $failedJobIds,
        array $options,
        CarbonImmutable $createdAt,
        ?CarbonImmutable $cancelledAt = null,
        ?CarbonImmutable $finishedAt = null,
    ) {}

    /**
     * Get a fresh instance of the batch represented by this ID.
     *
     * @return self|null
     */
    public function fresh()
    {}

    /**
     * Add additional jobs to the batch.
     *
     * @param  \Illuminate\Support\Enumerable|object|array  $jobs
     * @return self|null
     */
    public function add($jobs)
    {}

    /**
     * Prepare a chain that exists within the jobs being added.
     *
     * @return \Illuminate\Support\Collection
     */
    protected function prepareBatchedChain(array $chain)
    {}

    /**
     * Get the total number of jobs that have been processed by the batch thus far.
     *
     * @return int
     */
    public function processedJobs()
    {}

    /**
     * Get the percentage of jobs that have been processed (between 0-100).
     *
     * @return int<0, 100>
     */
    public function progress()
    {}

    /**
     * Record that a job within the batch finished successfully, executing any callbacks if necessary.
     *
     * @return void
     */
    public function recordSuccessfulJob(string $jobId)
    {}

    /**
     * Decrement the pending jobs for the batch.
     *
     * @return \Illuminate\Bus\UpdatedBatchJobCounts
     */
    public function decrementPendingJobs(string $jobId)
    {}

    /**
     * Invoke the callbacks of the given type.
     */
    protected function invokeCallbacks(string $type, ?Throwable $e = null): void
    {}

    /**
     * Determine if the batch has finished executing.
     *
     * @return bool
     */
    public function finished()
    {}

    /**
     * Determine if the batch has "progress" callbacks.
     *
     * @return bool
     */
    public function hasProgressCallbacks()
    {}

    /**
     * Determine if the batch has "success" callbacks.
     *
     * @return bool
     */
    public function hasThenCallbacks()
    {}

    /**
     * Determine if the batch allows jobs to fail without cancelling the batch.
     *
     * @return bool
     */
    public function allowsFailures()
    {}

    /**
     * Determine if the batch has job failures.
     *
     * @return bool
     */
    public function hasFailures()
    {}

    /**
     * Record that a job within the batch failed to finish successfully, executing any callbacks if necessary.
     *
     * @param  \Throwable  $e
     * @return void
     */
    public function recordFailedJob(string $jobId, $e)
    {}

    /**
     * Increment the failed jobs for the batch.
     *
     * @return \Illuminate\Bus\UpdatedBatchJobCounts
     */
    public function incrementFailedJobs(string $jobId)
    {}

    /**
     * Determine if this is the first job processed in the batch.
     *
     * @return bool
     */
    protected function isFirstJobProcessed(UpdatedBatchJobCounts $counts): bool
    {}

    /**
     * Determine if the batch has "catch" callbacks.
     *
     * @return bool
     */
    public function hasCatchCallbacks()
    {}

    /**
     * Determine if the batch has "failure" callbacks.
     */
    public function hasFailureCallbacks(): bool
    {}

    /**
     * Determine if the batch has "finally" callbacks.
     *
     * @return bool
     */
    public function hasFinallyCallbacks()
    {}

    /**
     * Cancel the batch.
     *
     * @param  \Throwable|null  $exception
     * @return void
     */
    public function cancel(?Throwable $exception = null)
    {}

    /**
     * Determine if the batch has been cancelled.
     *
     * @return bool
     */
    public function canceled()
    {}

    /**
     * Determine if the batch has been cancelled.
     *
     * @return bool
     */
    public function cancelled()
    {}

    /**
     * Delete the batch from storage.
     *
     * @return void
     */
    public function delete()
    {}

    /**
     * Invoke a batch callback handler.
     *
     * @param  callable  $handler
     * @return void
     */
    protected function invokeHandlerCallback($handler, Batch $batch, ?Throwable $e = null)
    {}

    /**
     * Convert the batch to an array.
     *
     * @return array
     */
    public function toArray()
    {}

    /**
     * Get the JSON serializable representation of the object.
     */
    public function jsonSerialize(): array
    {}

    /**
     * Dynamically access the batch's "options" via properties.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}
}
