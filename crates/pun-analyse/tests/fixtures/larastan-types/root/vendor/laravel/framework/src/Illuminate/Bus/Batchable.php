<?php

namespace Illuminate\Bus;

use Carbon\CarbonImmutable;
use Illuminate\Container\Container;
use Illuminate\Support\Str;
use Illuminate\Support\Testing\Fakes\BatchFake;

trait Batchable
{
    /**
     * The batch ID (if applicable).
     *
     * @var string|null
     */
    public $batchId;

    /**
     * The fake batch, if applicable.
     *
     * @var \Illuminate\Support\Testing\Fakes\BatchFake|null
     */
    private $fakeBatch;

    /**
     * Get the batch instance for the job, if applicable.
     *
     * @return \Illuminate\Bus\Batch|null
     */
    public function batch()
    {}

    /**
     * Determine if the batch is still active and processing.
     *
     * @return bool
     */
    public function batching()
    {}

    /**
     * Set the batch ID on the job.
     *
     * @param  string  $batchId
     * @return $this
     */
    public function withBatchId(string $batchId)
    {}

    /**
     * Indicate that the job should use a fake batch.
     *
     * @param  string  $id
     * @param  string  $name
     * @param  int  $totalJobs
     * @param  int  $pendingJobs
     * @param  int  $failedJobs
     * @param  array  $failedJobIds
     * @param  array  $options
     * @param  \Carbon\CarbonImmutable|null  $createdAt
     * @param  \Carbon\CarbonImmutable|null  $cancelledAt
     * @param  \Carbon\CarbonImmutable|null  $finishedAt
     * @return array{0: $this, 1: \Illuminate\Support\Testing\Fakes\BatchFake}
     */
    public function withFakeBatch(string $id = '',
                                  string $name = '',
                                  int $totalJobs = 0,
                                  int $pendingJobs = 0,
                                  int $failedJobs = 0,
                                  array $failedJobIds = [],
                                  array $options = [],
                                  ?CarbonImmutable $createdAt = null,
                                  ?CarbonImmutable $cancelledAt = null,
                                  ?CarbonImmutable $finishedAt = null)
    {}
}
