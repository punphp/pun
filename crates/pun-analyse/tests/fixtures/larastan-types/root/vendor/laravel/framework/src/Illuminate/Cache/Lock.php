<?php

namespace Illuminate\Cache;

use Illuminate\Contracts\Cache\Lock as LockContract;
use Illuminate\Contracts\Cache\LockTimeoutException;
use Illuminate\Support\Carbon;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Sleep;
use Illuminate\Support\Str;
use RuntimeException;

abstract class Lock implements LockContract
{
    use InteractsWithTime;

    /**
     * The name of the lock.
     *
     * @var string
     */
    protected $name;

    /**
     * The number of seconds the lock should be maintained.
     *
     * @var int
     */
    protected $seconds;

    /**
     * The scope identifier of this lock.
     *
     * @var string
     */
    protected $owner;

    /**
     * The number of milliseconds to wait before re-attempting to acquire a lock while blocking.
     *
     * @var int
     */
    protected $sleepMilliseconds = 250;

    /**
     * Create a new lock instance.
     *
     * @param  string  $name
     * @param  int  $seconds
     * @param  string|null  $owner
     */
    public function __construct($name, $seconds, $owner = null)
    {}

    /**
     * Attempt to acquire the lock.
     *
     * @return bool
     */
    abstract public function acquire();

    /**
     * Release the lock.
     *
     * @return bool
     */
    abstract public function release();

    /**
     * Returns the owner value written into the driver for this lock.
     *
     * @return string|null
     */
    abstract protected function getCurrentOwner();

    /**
     * Attempt to acquire the lock.
     *
     * @param  callable|null  $callback
     * @return mixed
     */
    public function get($callback = null)
    {}

    /**
     * Attempt to acquire the lock for the given number of seconds.
     *
     * @param  int  $seconds
     * @param  callable|null  $callback
     * @return mixed
     *
     * @throws \Illuminate\Contracts\Cache\LockTimeoutException
     */
    public function block($seconds, $callback = null)
    {}

    /**
     * Attempt to refresh the lock for the given number of seconds.
     *
     * @param  int|null  $seconds
     * @return bool
     */
    public function refresh($seconds = null)
    {}

    /**
     * Returns the current owner of the lock.
     *
     * @return string
     */
    public function owner()
    {}

    /**
     * Determine if the lock is currently held by any process.
     *
     * @return bool
     */
    public function isLocked(): bool
    {}

    /**
     * Determines whether this lock is allowed to release the lock in the driver.
     *
     * @return bool
     */
    public function isOwnedByCurrentProcess()
    {}

    /**
     * Determine whether this lock is owned by the given identifier.
     *
     * @param  string|null  $owner
     * @return bool
     */
    public function isOwnedBy($owner)
    {}

    /**
     * Specify the number of milliseconds to sleep in between blocked lock acquisition attempts.
     *
     * @param  int  $milliseconds
     * @return $this
     */
    public function betweenBlockedAttemptsSleepFor($milliseconds)
    {}
}
