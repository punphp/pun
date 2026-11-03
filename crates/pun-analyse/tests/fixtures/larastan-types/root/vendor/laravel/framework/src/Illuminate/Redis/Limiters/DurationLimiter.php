<?php

namespace Illuminate\Redis\Limiters;

use Illuminate\Contracts\Redis\LimiterTimeoutException;
use Illuminate\Support\Carbon;
use Illuminate\Support\Sleep;

class DurationLimiter
{
    /**
     * The Redis factory implementation.
     *
     * @var \Illuminate\Redis\Connections\Connection
     */
    private $redis;

    /**
     * The unique name of the lock.
     *
     * @var string
     */
    private $name;

    /**
     * The allowed number of concurrent tasks.
     *
     * @var int
     */
    private $maxLocks;

    /**
     * The number of seconds a slot should be maintained.
     *
     * @var int
     */
    private $decay;

    /**
     * The timestamp of the end of the current duration.
     *
     * @var int
     */
    public $decaysAt;

    /**
     * The number of remaining slots.
     *
     * @var int
     */
    public $remaining;

    /**
     * Create a new duration limiter instance.
     *
     * @param  \Illuminate\Redis\Connections\Connection  $redis
     * @param  string  $name
     * @param  int  $maxLocks
     * @param  int  $decay
     */
    public function __construct($redis, $name, $maxLocks, $decay)
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
     */
    public function block($timeout, $callback = null, $sleep = 750)
    {}

    /**
     * Attempt to acquire the lock.
     *
     * @return bool
     */
    public function acquire()
    {}

    /**
     * Determine if the key has been "accessed" too many times.
     *
     * @return bool
     */
    public function tooManyAttempts()
    {}

    /**
     * Clear the limiter.
     *
     * @return void
     */
    public function clear()
    {}

    /**
     * Get the Lua script for acquiring a lock.
     *
     * KEYS[1] - The limiter name
     * ARGV[1] - Current time in microseconds
     * ARGV[2] - Current time in seconds
     * ARGV[3] - Duration of the bucket
     * ARGV[4] - Allowed number of tasks
     *
     * @return string
     */
    protected function luaScript()
    {}

    /**
     * Get the Lua script to determine if the key has been "accessed" too many times.
     *
     * KEYS[1] - The limiter name
     * ARGV[1] - Current time in microseconds
     * ARGV[2] - Current time in seconds
     * ARGV[3] - Duration of the bucket
     * ARGV[4] - Allowed number of tasks
     *
     * @return string
     */
    protected function tooManyAttemptsLuaScript()
    {}
}
