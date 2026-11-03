<?php

namespace Illuminate\Queue\Console;

use DateTimeInterface;
use Illuminate\Console\Command;
use Illuminate\Contracts\Encryption\Encrypter;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Queue\Events\JobRetryRequested;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;
use RuntimeException;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'queue:retry')]
class RetryCommand extends Command
{
    /**
     * The console command signature.
     *
     * @var string
     */
    protected $signature = 'queue:retry
                            {id?* : The ID of the failed job or "all" to retry all jobs}
                            {--queue= : Retry all of the failed jobs for the specified queue}
                            {--range=* : Range of job IDs (numeric) to be retried (e.g. 1-5)}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Retry a failed queue job';

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Get the job IDs to be retried.
     *
     * @return array
     */
    protected function getJobIds()
    {}

    /**
     * Get the job IDs by queue, if applicable.
     *
     * @param  string  $queue
     * @return array
     */
    protected function getJobIdsByQueue($queue)
    {}

    /**
     * Get the job IDs ranges, if applicable.
     *
     * @param  array  $ranges
     * @return array
     */
    protected function getJobIdsByRanges(array $ranges)
    {}

    /**
     * Retry the queue job.
     *
     * @param  \stdClass  $job
     * @return void
     */
    protected function retryJob($job)
    {}

    /**
     * Reset the payload attempts.
     *
     * Applicable to Redis and other jobs which store attempts in their payload.
     *
     * @param  string  $payload
     * @return string
     */
    protected function resetAttempts($payload)
    {}

    /**
     * Refresh the "retry until" timestamp for the job.
     *
     * @param  string  $payload
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function refreshRetryUntil($payload)
    {}

    /**
     * Get the queueable options from the job.
     *
     * @param  $queue
     * @param  \stdClass  $job
     * @return array
     */
    protected function getQueueableOptions($queue, $job)
    {}

    /**
     * Get the job instance from the given payload.
     *
     * @param  array  $payload
     * @return mixed
     *
     * @throws \RuntimeException
     */
    protected function getInstanceFromPayload($payload)
    {}
}
