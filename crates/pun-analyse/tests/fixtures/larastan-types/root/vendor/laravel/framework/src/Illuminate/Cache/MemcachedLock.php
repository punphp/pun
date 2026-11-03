<?php

namespace Illuminate\Cache;

class MemcachedLock extends Lock
{
    /**
     * The Memcached instance.
     *
     * @var \Memcached
     */
    protected $memcached;

    /**
     * Create a new lock instance.
     *
     * @param  \Memcached  $memcached
     * @param  string  $name
     * @param  int  $seconds
     * @param  string|null  $owner
     */
    public function __construct($memcached, $name, $seconds, $owner = null)
    {}

    /**
     * Attempt to acquire the lock.
     *
     * @return bool
     */
    public function acquire()
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
     * Release the lock.
     *
     * @return bool
     */
    public function release()
    {}

    /**
     * Releases this lock in disregard of ownership.
     *
     * @return void
     */
    public function forceRelease()
    {}

    /**
     * Returns the owner value written into the driver for this lock.
     *
     * @return mixed
     */
    protected function getCurrentOwner()
    {}

    /**
     * Get the Memcached expiration value for the given number of seconds.
     *
     * Memcached treats expirations over 30 days as UNIX timestamps, so long durations are converted to timestamps.
     *
     * @param  int  $seconds
     * @return int
     */
    protected function calculateExpiration($seconds)
    {}
}
