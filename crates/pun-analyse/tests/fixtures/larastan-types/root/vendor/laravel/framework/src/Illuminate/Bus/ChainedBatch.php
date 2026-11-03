<?php

namespace Illuminate\Bus;

use Illuminate\Container\Container;
use Illuminate\Contracts\Bus\Dispatcher;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Support\Collection;
use Throwable;

class ChainedBatch implements ShouldQueue
{
    use Batchable, Dispatchable, InteractsWithQueue, Queueable;

    /**
     * The collection of batched jobs.
     *
     * @var \Illuminate\Support\Collection
     */
    public Collection $jobs;

    /**
     * The name of the batch.
     *
     * @var string
     */
    public string $name;

    /**
     * The batch options.
     *
     * @var array
     */
    public array $options;

    /**
     * Create a new chained batch instance.
     *
     * @param  \Illuminate\Bus\PendingBatch  $batch
     */
    public function __construct(PendingBatch $batch)
    {}

    /**
     * Prepare any nested batches within the given collection of jobs.
     *
     * @param  \Illuminate\Support\Collection  $jobs
     * @return \Illuminate\Support\Collection
     */
    public static function prepareNestedBatches(Collection $jobs): Collection
    {}

    /**
     * Handle the job.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Convert the chained batch instance into a pending batch.
     *
     * @return \Illuminate\Bus\PendingBatch
     */
    public function toPendingBatch()
    {}

    /**
     * Move the remainder of the chain to a "finally" batch callback.
     *
     * @param  \Illuminate\Bus\PendingBatch  $batch
     * @return \Illuminate\Bus\PendingBatch
     */
    protected function attachRemainderOfChainToEndOfBatch(PendingBatch $batch)
    {}
}
