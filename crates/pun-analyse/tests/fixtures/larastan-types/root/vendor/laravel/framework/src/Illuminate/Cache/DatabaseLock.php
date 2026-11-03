<?php

namespace Illuminate\Cache;

use Illuminate\Database\Connection;
use Illuminate\Database\DetectsConcurrencyErrors;
use Illuminate\Database\QueryException;
use Illuminate\Database\SqlServerConnection;
use Throwable;

class DatabaseLock extends Lock
{
    use DetectsConcurrencyErrors;

    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\Connection
     */
    protected $connection;

    /**
     * The database table name.
     *
     * @var string
     */
    protected $table;

    /**
     * The prune probability odds.
     *
     * @var array{int, int}|null
     */
    protected $lottery;

    /**
     * The default number of seconds that a lock should be held.
     *
     * @var int
     */
    protected $defaultTimeoutInSeconds;

    /**
     * Create a new lock instance.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @param  string  $table
     * @param  string  $name
     * @param  int  $seconds
     * @param  string|null  $owner
     * @param  array{int, int}|null  $lottery
     * @param  int  $defaultTimeoutInSeconds
     */
    public function __construct(Connection $connection, $table, $name, $seconds, $owner = null, $lottery = [2, 100], $defaultTimeoutInSeconds = 86400)
    {}

    /**
     * Attempt to acquire the lock.
     *
     * @return bool
     *
     * @throws \Throwable
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
     * Get the UNIX timestamp indicating when the lock should expire.
     *
     * @return int
     */
    protected function expiresAt($seconds = null)
    {}

    /**
     * Release the lock.
     *
     * @return bool
     *
     * @throws \Throwable
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
     * Deletes locks that are past expiration.
     *
     * @return void
     *
     * @throws \Throwable
     */
    public function pruneExpiredLocks()
    {}

    /**
     * Returns the owner value written into the driver for this lock.
     *
     * @return string|null
     */
    protected function getCurrentOwner()
    {}

    /**
     * Get the name of the database connection being used to manage the lock.
     *
     * @return string
     */
    public function getConnectionName()
    {}
}
