<?php

namespace Illuminate\Cache;

class DynamoDbLock extends Lock
{
    /**
     * The DynamoDB client instance.
     *
     * @var \Illuminate\Cache\DynamoDbStore
     */
    protected $dynamo;

    /**
     * Create a new lock instance.
     *
     * @param  \Illuminate\Cache\DynamoDbStore  $dynamo
     * @param  string  $name
     * @param  int  $seconds
     * @param  string|null  $owner
     */
    public function __construct(DynamoDbStore $dynamo, $name, $seconds, $owner = null)
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
     * Release this lock in disregard of ownership.
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
