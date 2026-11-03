<?php

namespace Illuminate\Queue;

use Aws\Command;
use Aws\Sqs\Exception\SqsException;
use Aws\Sqs\SqsClient;
use Illuminate\Contracts\Queue\ClearableQueue;
use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Jobs\SqsJob;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;

use function Illuminate\Support\enum_value;

class SqsQueue extends Queue implements QueueContract, ClearableQueue
{
    /**
     * The maximum SQS payload size in bytes (1 MB).
     *
     * @var int
     */
    const MAX_SQS_PAYLOAD_SIZE = 1048576;

    /**
     * The maximum number of messages allowed per SendMessageBatch request.
     *
     * @var int
     */
    const MAX_MESSAGES_PER_BATCH = 10;

    /**
     * The cache key prefix for extended SQS payloads.
     *
     * @var string
     */
    const EXTENDED_PAYLOAD_CACHE_PREFIX = 'laravel:sqs-payloads:';

    /**
     * The Amazon SQS instance.
     *
     * @var \Aws\Sqs\SqsClient
     */
    protected $sqs;

    /**
     * The name of the default queue.
     *
     * @var string
     */
    protected $default;

    /**
     * The queue URL prefix.
     *
     * @var string
     */
    protected $prefix;

    /**
     * The queue name suffix.
     *
     * @var string
     */
    protected $suffix;

    /**
     * The overflow storage options for large payload offloading.
     *
     * @var array
     */
    protected $overflowStorage = [];

    /**
     * Create a new Amazon SQS queue instance.
     *
     * @param  \Aws\Sqs\SqsClient  $sqs
     * @param  string  $default
     * @param  string  $prefix
     * @param  string  $suffix
     * @param  bool  $dispatchAfterCommit
     * @param  array  $overflowStorage
     */
    public function __construct(
        SqsClient $sqs,
        $default,
        $prefix = '',
        $suffix = '',
        $dispatchAfterCommit = false,
        array $overflowStorage = [],
    ) {}

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
     * @return \Illuminate\Support\Collection
     */
    public function pendingJobs($queue = null): Collection
    {}

    /**
     * Get the delayed jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function delayedJobs($queue = null): Collection
    {}

    /**
     * Get the reserved jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection
     */
    public function reservedJobs($queue = null): Collection
    {}

    /**
     * Get all pending jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allPendingJobs(): Collection
    {}

    /**
     * Get all delayed jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allDelayedJobs(): Collection
    {}

    /**
     * Get all reserved jobs across every queue.
     *
     * @return \Illuminate\Support\Collection
     */
    public function allReservedJobs(): Collection
    {}

    /**
     * Get the creation timestamp of the oldest pending job, excluding delayed jobs.
     *
     * Not supported by SQS, returns null.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int|null
     */
    public function creationTimeOfOldestPendingJob($queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function push($job, $data = '', $queue = null)
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
     * @param  string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Push an array of jobs onto the queue using the SendMessageBatch API.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return void
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Create the payload for each of the given jobs.
     *
     * Payloads are created at dispatch time, even for jobs deferred until after the transaction commits.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  string|null  $queue
     * @return array<int, array{job: mixed, delay: mixed, payload: string}>
     */
    protected function prepareBatchMessages(array $jobs, $data, $queue)
    {}

    /**
     * Build entries, raise queueing events, dispatch chunks, and raise queued events with SQS message IDs.
     *
     * @param  array  $messages
     * @param  string|null  $queue
     * @return void
     *
     * @throws \Aws\Sqs\Exception\SqsException
     * @throws \Throwable
     */
    protected function sendBatchedMessages(array $messages, $queue)
    {}

    /**
     * Build the SendMessageBatch entry for a single prepared message.
     *
     * The entry Id maps each Successful or Failed result returned by SQS back to its job.
     *
     * @param  int  $id
     * @param  array{job: mixed, delay: mixed, payload: string}  $message
     * @param  string|null  $queue
     * @return array
     */
    protected function prepareSendMessageBatchEntry($id, array $message, $queue)
    {}

    /**
     * Chunk batch entries respecting both the 10-message and cumulative payload-size limits enforced by SendMessageBatch.
     *
     * @param  array  $entries
     * @return array
     */
    protected function chunkBatchEntries(array $entries)
    {}

    /**
     * Determine if the payload should be stored in cache.
     *
     * @param  string  $payload
     * @return bool
     */
    protected function willOverflow($payload)
    {}

    /**
     * Store the payload in cache and return a pointer payload.
     *
     * @param  string  $payload
     * @return string
     */
    protected function overflow($payload)
    {}

    /**
     * Get the queueable options from the job.
     *
     * @param  mixed  $job
     * @param  string|null  $queue
     * @param  string  $payload
     * @param  \DateTimeInterface|\DateInterval|int|null  $delay
     * @return array{DelaySeconds?: int, MessageGroupId?: string, MessageDeduplicationId?: string}
     */
    public function getQueueableOptions($job, $queue, $payload, $delay = null): array
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
     * Delete all of the jobs from the queue.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function clear($queue = null)
    {}

    /**
     * Get the queue or return the default.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return string
     */
    public function getQueue($queue)
    {}

    /**
     * Add the given suffix to the given queue name.
     *
     * @param  string  $queue
     * @param  string  $suffix
     * @return string
     */
    protected function suffixQueue($queue, $suffix = '')
    {}

    /**
     * Get the underlying SQS instance.
     *
     * @return \Aws\Sqs\SqsClient
     */
    public function getSqs()
    {}
}
