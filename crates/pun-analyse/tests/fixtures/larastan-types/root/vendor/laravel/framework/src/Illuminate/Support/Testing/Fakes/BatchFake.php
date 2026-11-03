<?php

namespace Illuminate\Support\Testing\Fakes;

use Carbon\CarbonImmutable;
use Illuminate\Bus\Batch;
use Illuminate\Bus\UpdatedBatchJobCounts;
use Illuminate\Support\Collection;
use Throwable;

class BatchFake extends Batch
{
    /**
     * The jobs that have been added to the batch.
     *
     * @var array
     */
    public $added = [];

    /**
     * Indicates if the batch has been deleted.
     *
     * @var bool
     */
    public $deleted = false;

    /**
     * Create a new batch instance.
     *
     * @param  string  $id
     * @param  string  $name
     * @param  int  $totalJobs
     * @param  int  $pendingJobs
     * @param  int  $failedJobs
     * @param  array  $failedJobIds
     * @param  array  $options
     * @param  \Carbon\CarbonImmutable  $createdAt
     * @param  \Carbon\CarbonImmutable|null  $cancelledAt
     * @param  \Carbon\CarbonImmutable|null  $finishedAt
     */
    public function __construct(
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
     * @return self
     */
    #[\Override]
    public function fresh()
    {}

    /**
     * Add additional jobs to the batch.
     *
     * @param  \Illuminate\Support\Enumerable|object|array  $jobs
     * @return self
     */
    #[\Override]
    public function add($jobs)
    {}

    /**
     * Record that a job within the batch finished successfully, executing any callbacks if necessary.
     *
     * @param  string  $jobId
     * @return void
     */
    #[\Override]
    public function recordSuccessfulJob(string $jobId)
    {}

    /**
     * Decrement the pending jobs for the batch.
     *
     * @param  string  $jobId
     * @return void
     */
    #[\Override]
    public function decrementPendingJobs(string $jobId)
    {}

    /**
     * Record that a job within the batch failed to finish successfully, executing any callbacks if necessary.
     *
     * @param  string  $jobId
     * @param  \Throwable  $e
     * @return void
     */
    #[\Override]
    public function recordFailedJob(string $jobId, $e)
    {}

    /**
     * Increment the failed jobs for the batch.
     *
     * @param  string  $jobId
     * @return \Illuminate\Bus\UpdatedBatchJobCounts
     */
    #[\Override]
    public function incrementFailedJobs(string $jobId)
    {}

    /**
     * Cancel the batch.
     *
     * @return void
     */
    #[\Override]
    public function cancel(?Throwable $exception = null)
    {}

    /**
     * Delete the batch from storage.
     *
     * @return void
     */
    #[\Override]
    public function delete()
    {}

    /**
     * Determine if the batch has been deleted.
     *
     * @return bool
     */
    public function deleted()
    {}
}
