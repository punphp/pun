<?php

namespace Illuminate\Cache;

use Illuminate\Support\Carbon;

class ArrayLock extends Lock
{
    /**
     * The parent array cache store.
     *
     * @var \Illuminate\Cache\ArrayStore
     */
    protected $store;

    /**
     * Create a new lock instance.
     *
     * @param  \Illuminate\Cache\ArrayStore  $store
     * @param  string  $name
     * @param  int  $seconds
     * @param  string|null  $owner
     */
    public function __construct($store, $name, $seconds, $owner = null)
    {}

    /**
     * Attempt to acquire the lock.
     *
     * @return bool
     */
    public function acquire()
    {}

    /**
     * Determine if the current lock exists.
     *
     * @return bool
     */
    protected function exists()
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
     * Returns the owner value written into the driver for this lock.
     *
     * @return string|null
     */
    protected function getCurrentOwner()
    {}

    /**
     * Releases this lock regardless of ownership.
     *
     * @return void
     */
    public function forceRelease()
    {}
}
