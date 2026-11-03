<?php

namespace Illuminate\Cache;

class CacheLock extends Lock
{
    /**
     * The cache store implementation.
     *
     * @var \Illuminate\Contracts\Cache\Store
     */
    protected $store;

    /**
     * Create a new lock instance.
     *
     * @param  \Illuminate\Contracts\Cache\Store  $store
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
     * Release the lock.
     *
     * @return bool
     */
    public function release()
    {}

    /**
     * Releases this lock regardless of ownership.
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
}
