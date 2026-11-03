<?php

namespace Illuminate\Queue;

use Illuminate\Contracts\Queue\ClearableQueue;
use Illuminate\Contracts\Queue\Queue as QueueContract;
use Illuminate\Database\Connection;
use Illuminate\Database\DetectsConcurrencyErrors;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Jobs\DatabaseJob;
use Illuminate\Queue\Jobs\DatabaseJobRecord;
use Illuminate\Queue\Jobs\InspectedJob;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use PDO;
use Throwable;

use function Illuminate\Support\enum_value;

class DatabaseQueue extends Queue implements QueueContract, ClearableQueue
{
    use DetectsConcurrencyErrors;

    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\Connection
     */
    protected $database;

    /**
     * The database table that holds the jobs.
     *
     * @var string
     */
    protected $table;

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
     * The cached lock type for popping jobs.
     *
     * @var string|bool|null
     */
    protected $lockForPopping = null;

    /**
     * Create a new database queue instance.
     *
     * @param  \Illuminate\Database\Connection  $database
     * @param  string  $table
     * @param  string  $default
     * @param  int  $retryAfter
     * @param  bool  $dispatchAfterCommit
     */
    public function __construct(
        Connection $database,
        $table,
        $default = 'default',
        $retryAfter = 60,
        $dispatchAfterCommit = false,
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
     * Get the creation timestamp of the oldest pending job, excluding delayed jobs.
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
     * Push an array of jobs onto the queue.
     *
     * @param  array  $jobs
     * @param  mixed  $data
     * @param  \UnitEnum|string|null  $queue
     * @return mixed
     */
    public function bulk($jobs, $data = '', $queue = null)
    {}

    /**
     * Release a reserved job back onto the queue after (n) seconds.
     *
     * @param  string  $queue
     * @param  \Illuminate\Queue\Jobs\DatabaseJobRecord  $job
     * @param  int  $delay
     * @return mixed
     */
    public function release($queue, $job, $delay)
    {}

    /**
     * Push a raw payload to the database with a given delay of (n) seconds.
     *
     * @param  \UnitEnum|string|null  $queue
     * @param  string  $payload
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  int  $attempts
     * @return mixed
     */
    protected function pushToDatabase($queue, $payload, $delay = 0, $attempts = 0)
    {}

    /**
     * Create an array to insert for the given job.
     *
     * @param  string|null  $queue
     * @param  string  $payload
     * @param  int  $availableAt
     * @param  int  $attempts
     * @return array
     */
    protected function buildDatabaseRecord($queue, $payload, $availableAt, $attempts = 0)
    {}

    /**
     * Pop the next job off of the queue.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return \Illuminate\Contracts\Queue\Job|null
     *
     * @throws \Throwable
     */
    public function pop($queue = null)
    {}

    /**
     * Get the next available job for the queue.
     *
     * @param  string|null  $queue
     * @return \Illuminate\Queue\Jobs\DatabaseJobRecord|null
     */
    protected function getNextAvailableJob($queue)
    {}

    /**
     * Get the lock required for popping the next job.
     *
     * @return string|bool
     */
    protected function getLockForPopping()
    {}

    /**
     * Modify the query to check for available jobs.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @return void
     */
    protected function isAvailable($query)
    {}

    /**
     * Modify the query to check for jobs that are reserved but have expired.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @return void
     */
    protected function isReservedButExpired($query)
    {}

    /**
     * Marshal the reserved job into a DatabaseJob instance.
     *
     * @param  string  $queue
     * @param  \Illuminate\Queue\Jobs\DatabaseJobRecord  $job
     * @return \Illuminate\Queue\Jobs\DatabaseJob
     */
    protected function marshalJob($queue, $job)
    {}

    /**
     * Mark the given job ID as reserved.
     *
     * @param  \Illuminate\Queue\Jobs\DatabaseJobRecord  $job
     * @return \Illuminate\Queue\Jobs\DatabaseJobRecord
     */
    protected function markJobAsReserved($job)
    {}

    /**
     * Delete a reserved job from the queue.
     *
     * @param  string  $queue
     * @param  string  $id
     * @return void
     *
     * @throws \Throwable
     */
    public function deleteReserved($queue, $id)
    {}

    /**
     * Delete a reserved job from the reserved queue and release it.
     *
     * @param  string  $queue
     * @param  \Illuminate\Queue\Jobs\DatabaseJob  $job
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
     * Get the queue or return the default.
     *
     * @param  \UnitEnum|string|null  $queue
     * @return string
     */
    public function getQueue($queue)
    {}

    /**
     * Get the underlying database instance.
     *
     * @return \Illuminate\Database\Connection
     */
    public function getDatabase()
    {}
}
