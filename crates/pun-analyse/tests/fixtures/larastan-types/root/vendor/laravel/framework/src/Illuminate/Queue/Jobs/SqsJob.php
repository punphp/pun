<?php

namespace Illuminate\Queue\Jobs;

use Aws\Sqs\SqsClient;
use Illuminate\Container\Container;
use Illuminate\Contracts\Queue\Job as JobContract;
use Illuminate\Support\Arr;

class SqsJob extends Job implements JobContract
{
    /**
     * The Amazon SQS client instance.
     *
     * @var \Aws\Sqs\SqsClient
     */
    protected $sqs;

    /**
     * The Amazon SQS job instance.
     *
     * @var array
     */
    protected $job;

    /**
     * The overflow storage options for large payload offloading.
     *
     * @var array
     */
    protected $overflowStorage = [];

    /**
     * The cached raw body of the job.
     *
     * @var string|null
     */
    protected $cachedRawBody = null;

    /**
     * Create a new job instance.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  \Aws\Sqs\SqsClient  $sqs
     * @param  array  $job
     * @param  string  $connectionName
     * @param  string  $queue
     * @param  array  $overflowStorage
     */
    public function __construct(Container $container, SqsClient $sqs, array $job, $connectionName, $queue, array $overflowStorage = [])
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
     * Reset the SQS message's visibility timeout so it becomes available again.
     *
     * @param  int  $delay
     * @return void
     */
    protected function changeMessageVisibilityInSqs($delay)
    {}

    /**
     * Delete the job from the queue.
     *
     * @return void
     */
    public function delete()
    {}

    /**
     * Delete the message from the SQS queue.
     *
     * @return void
     */
    protected function deleteMessageFromSqs()
    {}

    /**
     * Delete the offloaded overflow payload from the cache, if applicable.
     *
     * @return void
     */
    protected function deleteOverflowPayload()
    {}

    /**
     * Get the number of times the job has been attempted.
     *
     * @return int
     */
    public function attempts()
    {}

    /**
     * Get the job identifier.
     *
     * @return string
     */
    public function getJobId()
    {}

    /**
     * Get the raw body string for the job.
     *
     * @return string
     */
    public function getRawBody()
    {}

    /**
     * Resolve the pointer path from the job body, if present.
     *
     * @return string|null
     */
    protected function overflowPointer()
    {}

    /**
     * Resolve the configured cache store for extended storage.
     *
     * @return \Illuminate\Contracts\Cache\Repository
     */
    protected function overflowStore()
    {}

    /**
     * Get the underlying SQS client instance.
     *
     * @return \Aws\Sqs\SqsClient
     */
    public function getSqs()
    {}

    /**
     * Get the underlying raw SQS job.
     *
     * @return array
     */
    public function getSqsJob()
    {}
}
