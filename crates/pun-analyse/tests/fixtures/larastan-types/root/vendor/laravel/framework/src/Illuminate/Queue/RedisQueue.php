<?php

namespace Illuminate\Queue;

use Illuminate\Contracts\Queue\ClearableQueue;
use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Contracts\Redis\Factory as Redis;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Jobs\InspectedJob;
use Illuminate\Queue\Jobs\RedisJob;
use Illuminate\Redis\Connections\Connection;
use Illuminate\Redis\Connections\PhpRedisClusterConnection;
use Illuminate\Redis\Connections\PredisClusterConnection;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;

use function Illuminate\Support\enum_value;

class RedisQueue extends Queue implements QueueContract, ClearableQueue
{
    /**
     * The Redis factory implementation.
     *
     * @var \Illuminate\Contracts\Redis\Factory
     */
    protected $redis;

    /**
     * The connection name.
     *
     * @var string|null
     */
    protected $connection;

    /**
     * The name of the default queue.
     *
     * @var string
     */
    protected $default;

    /**
     * The expiration time of a job.
     *
     * @var int|null
     */
    protected $retryAfter = 60;

    /**
     * The maximum number of seconds to block for a job.
     *
     * @var int|null
     */
    protected $blockFor = null;

    /**
     * The batch size to use when migrating delayed / expired jobs onto the primary queue.
     *
     * Negative values are infinite.
     *
     * @var int
     */
    protected $migrationBatchSize = -1;

    /**
     * Indicates if a secondary queue had a job available between checks of the primary queue.
     *
     * Only applicable when monitoring multiple named queues with a single instance.
     *
     * @var bool
     */
    protected $secondaryQueueHadJob = false;

    /**
     * Indicates if the connection is a Redis Cluster connection.
     *
     * @var bool|null
     */
    protected $isCluster = null;

    /**
     * Create a new Redis queue instance.
     *
     * @param  \Illuminate\Contracts\Redis\Factory  $redis
     * @param  string  $default
     * @param  string|null  $connection
     * @param  int  $retryAfter
     * @param  int|null  $blockFor
     * @param  bool  $dispatchAfterCommit
     * @param  int  $migrationBatchSize
     */
    public function __construct(
        Redis $redis,
        $default = 'default',
        $connection = null,
        $retryAfter = 60,
        $blockFor = null,
        $dispatchAfterCommit = false,
        $migrationBatchSize = -1,
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
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function pendingJobs($queue = null): Collection
    {}

    /**
     * Get the delayed jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function delayedJobs($queue = null): Collection
    {}

    /**
     * Get the reserved jobs for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function reservedJobs($queue = null): Collection
    {}

    /**
     * Get all pending jobs across every queue.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function allPendingJobs(): Collection
    {}

    /**
     * Get all delayed jobs across every queue.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function allDelayedJobs(): Collection
    {}

    /**
     * Get all reserved jobs across every queue.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Queue\Jobs\InspectedJob>
     */
    public function allReservedJobs(): Collection
    {}

    /**
     * Get the unique queue names.
     *
     * @return \Illuminate\Support\Collection<int, string>
     */
    protected function allQueueNames(): Collection
    {}

    /**
     * Get all of the queue keys from the underlying connection.
     *
     * @return array<int, string>
     */
    protected function scanQueueKeys(): array
    {}

    /**
     * Get the creation timestamp of the oldest pending job, excluding delayed jobs.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int|null
     */
    public function creationTimeOfOldestPendingJob($queue = null)
    {}

    /**
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return void
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Push an array of jobs onto the queue on a cluster connection.
     *
     * Every job in the bulk targets the same queue - and therefore the same key
     * slot - so the payloads are collected and pushed with a single slot Lua
     * script. Delayed jobs and "after commit" jobs are pushed like normal.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return void
     */
    protected function bulkOnClusterConnection($jobs, $data = '', $queue = null)
    {}

    /**
     * Push a new job onto the queue.
     *
     * @param  object|string  $job
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
     * Push a new job onto the queue after a delay.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  object|string  $job
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function later($delay, $job, $data = '', $queue = null)
    {}

    /**
     * Push a raw job onto the queue after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  string  $payload
     * @param  string|null  $queue
     * @return mixed
     */
    protected function laterRaw($delay, $payload, $queue = null)
    {}

    /**
     * Create a payload string from the given job and data.
     *
     * @param  string  $job
     * @param  string  $queue
     * @param  mixed  $data
     * @return array
     */
    protected function createPayloadArray($job, $queue, $data = '')
    {}

    /**
     * Pop the next job off of the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @param  int  $index
     * @return \Illuminate\Contracts\Queue\Job|null
     */
    public function pop($queue = null, $index = 0)
    {}

    /**
     * Migrate any delayed or expired jobs onto the primary queue.
     *
     * @param  string  $queue
     * @return void
     */
    protected function migrate($queue)
    {}

    /**
     * Migrate the delayed jobs that are ready to the regular queue.
     *
     * @param  string  $from
     * @param  string  $to
     * @return array
     */
    public function migrateExpiredJobs($from, $to)
    {}

    /**
     * Retrieve the next job from the queue.
     *
     * @param  string  $queue
     * @param  bool  $block
     * @return array
     */
    protected function retrieveNextJob($queue, $block = true)
    {}

    /**
     * Delete a reserved job from the queue.
     *
     * @param  string  $queue
     * @param  \Illuminate\Queue\Jobs\RedisJob  $job
     * @return void
     */
    public function deleteReserved($queue, $job)
    {}

    /**
     * Delete a reserved job from the reserved queue and release it.
     *
     * @param  string  $queue
     * @param  \Illuminate\Queue\Jobs\RedisJob  $job
     * @param  int  $delay
     * @return void
     */
    public function deleteAndRelease($queue, $job, $delay)
    {}

    /**
     * Delete all of the jobs from the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return int
     */
    public function clear($queue = null)
    {}

    /**
     * Get a random ID string.
     *
     * @return string
     */
    protected function getRandomId()
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
     * Get the cluster-safe Redis key for the given queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return string
     */
    protected function getQueueRedisKey($queue = null)
    {}

    /**
     * Get the connection for the queue.
     *
     * @return \Illuminate\Redis\Connections\Connection
     */
    public function getConnection()
    {}

    /**
     * Determine if the connection is a Redis Cluster connection.
     *
     * @return bool
     */
    protected function isClusterConnection()
    {}

    /**
     * Get the underlying Redis instance.
     *
     * @return \Illuminate\Contracts\Redis\Factory
     */
    public function getRedis()
    {}
}
