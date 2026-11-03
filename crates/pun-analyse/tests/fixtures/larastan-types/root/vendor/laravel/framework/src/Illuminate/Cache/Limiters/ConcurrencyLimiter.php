<?php

namespace Illuminate\Cache\Limiters;

use Illuminate\Support\Carbon;
use Illuminate\Support\Sleep;
use Illuminate\Support\Str;
use Throwable;

class ConcurrencyLimiter
{
    /**
     * The cache store instance.
     *
     * @var \Illuminate\Contracts\Cache\LockProvider
     */
    protected $store;

    /**
     * The name of the limiter.
     *
     * @var string
     */
    protected $name;

    /**
     * The allowed number of concurrent locks.
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
     * Create a new concurrency limiter instance.
     *
     * @param  \Illuminate\Contracts\Cache\LockProvider  $store
     * @param  string  $name
     * @param  int  $maxLocks
     * @param  int  $releaseAfter
     */
    public function __construct($store, $name, $maxLocks, $releaseAfter)
    {}

    /**
     * Attempt to acquire the lock for the given number of seconds.
     *
     * @param  int  $timeout
     * @param  callable|null  $callback
     * @param  int  $sleep
     * @return mixed
     *
     * @throws \Illuminate\Cache\Limiters\LimiterTimeoutException
     * @throws \Throwable
     */
    public function block($timeout, $callback = null, $sleep = 250)
    {}

    /**
     * Attempt to acquire a slot lock.
     *
     * @param  string  $id
     * @return \Illuminate\Contracts\Cache\Lock|false
     */
    protected function acquire($id)
    {}

    /**
     * Release the lock.
     *
     * @param  \Illuminate\Contracts\Cache\Lock  $lock
     * @return void
     */
    protected function release($lock)
    {}
}
