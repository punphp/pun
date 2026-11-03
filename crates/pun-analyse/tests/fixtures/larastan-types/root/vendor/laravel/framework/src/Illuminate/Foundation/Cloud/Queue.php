<?php

namespace Illuminate\Foundation\Cloud;

use Carbon\CarbonImmutable;
use Illuminate\Contracts\Queue\ClearableQueue;
use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Foundation\Application;
use Illuminate\Http\Client\ConnectionException;
use Illuminate\Http\Client\RequestException;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\ForwardsCalls;
use Symfony\Component\Console\Input\ArgvInput;

class Queue implements QueueContract, ClearableQueue
{
    use ForwardsCalls;

    /**
     * The currently processing job.
     *
     * @var \Illuminate\Contracts\Queue\Job|null
     */
    protected $processingJob = null;

    /**
     * The queue for the currently processing job.
     *
     * @var string|null
     */
    protected $processingQueue = null;

    /**
     * The date the last job started processing.
     *
     * @var \Carbon\CarbonImmutable
     */
    protected $processingJobStartedAt = null;

    /**
     * Create a new Queue instance.
     */
    public function __construct(
        protected Application $app,
        protected QueueContract $queue,
        protected Events $events,
        protected array $config,
    ) {}

    /**
     * Get the size of the queue.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function size($queue = null)
    {}

    /**
     * Get the number of pending jobs.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function pendingSize($queue = null)
    {}

    /**
     * Get the number of delayed jobs.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function delayedSize($queue = null)
    {}

    /**
     * Get the number of reserved jobs.
     *
     * @param  string|null  $queue
     * @return int
     */
    public function reservedSize($queue = null)
    {}

    /**
     * Get the number of jobs across every managed queue.
     *
     * @return int
     */
    public function totalSize()
    {}

    /**
     * Get the number of pending jobs across every managed queue.
     *
     * @return int
     */
    public function totalPendingSize()
    {}

    /**
     * Get the number of delayed jobs across every managed queue.
     *
     * @return int
     */
    public function totalDelayedSize()
    {}

    /**
     * Get the number of reserved jobs across every managed queue.
     *
     * @return int
     */
    public function totalReservedSize()
    {}

    /**
     * Get the creation timestamp of the oldest pending job, excluding delayed jobs.
     *
     * @param  string|null  $queue
     * @return int|null
     */
    public function creationTimeOfOldestPendingJob($queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  string|object  $job
     * @param  mixed  $data
     * @param  string|null  $queue
     * @return mixed
     */
    public function push($job, $data = '', $queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  string  $queue
     * @param  string|object  $job
     * @param  mixed  $data
     * @return mixed
     */
    public function pushOn($queue, $job, $data = '')
    {}

    /**
     * Push a raw payload onto the queue.
     *
     * @param  string  $payload
     * @param  string|null  $queue
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
     * @param  string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Push a new job onto a specific queue after (n) seconds.
     *
     * @param  string  $queue
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string|object  $job
     * @param  mixed  $data
     * @return mixed
     */
    public function laterOn($queue, $delay, $job, $data = '')
    {}

    /**
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  string|null  $queue
     * @return mixed
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Pop the next job off of the queue.
     *
     * Jobs come straight from SQS unless the cloud-agent is enabled.
     *
     * When cloud-agent is enabled, we long-poll the agent's runtime socket instead.
     *
     * @param  string|null  $queue
     * @return \Illuminate\Contracts\Queue\Job|null
     */
    public function pop($queue = null)
    {}

    /**
     * Long-poll the cloud-agent's runtime socket and wrap the next message in a CloudJob.
     *
     * @return \Illuminate\Foundation\Cloud\CloudJob|null
     */
    protected function popFromAgent()
    {}

    /**
     * Long-poll the agent's runtime socket (GET /next) for the next job.
     *
     * @throws \Illuminate\Foundation\Cloud\AgentUnreachableException
     */
    protected function requestNextJobFromAgent(): ?array
    {}

    /**
     * Report a job's terminal outcome back to the agent (POST /result).
     *
     * @throws \Illuminate\Http\Client\RequestException
     * @throws \Illuminate\Foundation\Cloud\AgentUnreachableException
     */
    protected function reportJobStatusToAgent(string $messageId, ?string $receiptHandle, string $status, ?int $delay = null): void
    {}

    /**
     * Get a pending HTTP request bound to the agent's Unix runtime socket.
     *
     * @return \Illuminate\Http\Client\PendingRequest
     */
    protected function agentRequest()
    {}

    /**
     * Delete all of the jobs from the queue.
     *
     * @param  string  $queue
     * @return int
     */
    public function clear($queue)
    {}

    /**
     * Determine whether the next job should be received from the in-container cloud-agent.
     *
     * @param  string|null  $queue
     */
    protected function usesAgent($queue = null): bool
    {}

    /**
     * Get the queue the running queue:work worker is processing.
     *
     * @return string
     */
    protected function workerQueue()
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
     * Set the queue configuration array.
     *
     * @param  array  $config
     * @return $this
     */
    public function setConfig($config)
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
     * Finish processing the current job and emit a queue event.
     *
     * @param  string  $default
     * @param  \Carbon\CarbonImmutable|null  $timestamp
     * @return void
     */
    public function finishProcessingJob($default = 'processed', $timestamp = null)
    {}

    /**
     * Last job details resolver.
     *
     * @return array{queue: string, attempts: int, started_at: CarbonImmutable}
     */
    public function processingJobDetails()
    {}

    /**
     * Handle jobs finishing being queued.
     *
     * @param  string  $queue
     */
    public function finishQueueingJob($queue)
    {}

    /**
     * Handle a job being popped.
     *
     * @param  string|null  $queue
     * @param  \Illuminate\Contracts\Queue\Job|null  $job
     * @return void
     */
    protected function startProcessingJob($queue, $job)
    {}

    /**
     * Normalize the queue name.
     *
     * @param  string|null  $queue
     * @return string
     */
    public function normalizeQueue($queue)
    {}

    /**
     * Get the names of the managed queues configured for this connection.
     *
     * @return array<int, string>
     */
    public function managedQueues()
    {}

    /**
     * Dynamically pass method calls to the underlying queue.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
