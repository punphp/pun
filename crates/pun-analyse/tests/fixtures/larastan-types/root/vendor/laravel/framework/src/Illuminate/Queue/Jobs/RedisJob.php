<?php

namespace Illuminate\Queue\Jobs;

use Illuminate\Container\Container;
use Illuminate\Contracts\Queue\Job as JobContract;
use Illuminate\Queue\RedisQueue;

class RedisJob extends Job implements JobContract
{
    /**
     * The Redis queue instance.
     *
     * @var \Illuminate\Queue\RedisQueue
     */
    protected $redis;

    /**
     * The Redis raw job payload.
     *
     * @var string
     */
    protected $job;

    /**
     * The decoded JSON version of "$job".
     *
     * @var array
     */
    protected $decoded;

    /**
     * The Redis job payload inside the reserved queue.
     *
     * @var string
     */
    protected $reserved;

    /**
     * Create a new job instance.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  \Illuminate\Queue\RedisQueue  $redis
     * @param  string  $job
     * @param  string  $reserved
     * @param  string  $connectionName
     * @param  string  $queue
     */
    public function __construct(Container $container, RedisQueue $redis, $job, $reserved, $connectionName, $queue)
    {}

    /**
     * Get the raw body string for the job.
     *
     * @return string
     */
    public function getRawBody()
    {}

    /**
     * Delete the job from the queue.
     *
     * @return void
     */
    public function delete()
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
     * Get the number of times the job has been attempted.
     *
     * @return int
     */
    public function attempts()
    {}

    /**
     * Get the job identifier.
     *
     * @return string|null
     */
    public function getJobId()
    {}

    /**
     * Get the underlying Redis factory implementation.
     *
     * @return \Illuminate\Queue\RedisQueue
     */
    public function getRedisQueue()
    {}

    /**
     * Get the underlying reserved Redis job.
     *
     * @return string
     */
    public function getReservedJob()
    {}
}
