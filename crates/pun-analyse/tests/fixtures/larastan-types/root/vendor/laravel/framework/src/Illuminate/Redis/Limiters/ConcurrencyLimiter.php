<?php

namespace Illuminate\Redis\Limiters;

use Illuminate\Contracts\Redis\LimiterTimeoutException;
use Illuminate\Redis\Connections\Connection;
use Illuminate\Support\Carbon;
use Illuminate\Support\Sleep;
use Illuminate\Support\Str;
use Throwable;

class ConcurrencyLimiter
{
    /**
     * The Redis factory implementation.
     *
     * @var \Illuminate\Redis\Connections\Connection
     */
    protected $redis;

    /**
     * The name of the limiter.
     *
     * @var string
     */
    protected $name;

    /**
     * The allowed number of concurrent tasks.
     *
     * @var int
     */
    protected $maxLocks;

    /**
     * The number of seconds a slot should be maintained.
     *
     * @var int
     */
    protected $releaseAfter;

    /**
     * The cluster-safe key prefix for lock slots.
     *
     * @var string|null
     */
    protected $prefix;

    /**
     * Create a new concurrency limiter instance.
     *
     * @param  \Illuminate\Redis\Connections\Connection  $redis
     * @param  string  $name
     * @param  int  $maxLocks
     * @param  int  $releaseAfter
     */
    public function __construct($redis, $name, $maxLocks, $releaseAfter)
    {}

    /**
     * Attempt to acquire the lock for the given number of seconds.
     *
     * @param  int  $timeout
     * @param  callable|null  $callback
     * @param  int  $sleep
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Redis\LimiterTimeoutException
     * @throws \Throwable
     */
    public function block($timeout, $callback = null, $sleep = 250)
    {}

    /**
     * Attempt to acquire the lock.
     *
     * @param  string  $id  A unique identifier for this lock
     * @return mixed
     */
    protected function acquire($id)
    {}

    /**
     * Get the Lua script for acquiring a lock.
     *
     * KEYS    - The keys that represent available slots
     * ARGV[1] - The limiter name
     * ARGV[2] - The number of seconds the slot should be reserved
     * ARGV[3] - The unique identifier for this lock
     *
     * @return string
     */
    protected function lockScript()
    {}

    /**
     * Release the lock.
     *
     * @param  string  $key
     * @param  string  $id
     * @return void
     */
    protected function release($key, $id)
    {}

    /**
     * Get the Lua script to atomically release a lock.
     *
     * KEYS[1] - The name of the lock
     * ARGV[1] - The unique identifier for this lock
     *
     * @return string
     */
    protected function releaseScript()
    {}

    /**
     * Get the cluster-safe key prefix for lock slots.
     *
     * The result is cached for the lifetime of this limiter instance.
     *
     * @return string
     */
    protected function getPrefix()
    {}
}
