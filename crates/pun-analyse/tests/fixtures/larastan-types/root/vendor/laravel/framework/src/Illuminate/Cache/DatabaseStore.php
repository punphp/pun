<?php

namespace Illuminate\Cache;

use Closure;
use Illuminate\Contracts\Cache\CanFlushLocks;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Cache\Store;
use Illuminate\Database\ConnectionInterface;
use Illuminate\Database\PostgresConnection;
use Illuminate\Database\QueryException;
use Illuminate\Database\SQLiteConnection;
use Illuminate\Database\SqlServerConnection;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Str;
use RuntimeException;

class DatabaseStore implements CanFlushLocks, LockProvider, Store
{
    use InteractsWithTime;

    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\ConnectionInterface
     */
    protected $connection;

    /**
     * The database connection instance that should be used to manage locks.
     *
     * @var \Illuminate\Database\ConnectionInterface|null
     */
    protected $lockConnection;

    /**
     * The name of the cache table.
     *
     * @var string
     */
    protected $table;

    /**
     * A string that should be prepended to keys.
     *
     * @var string
     */
    protected $prefix;

    /**
     * The name of the cache locks table.
     *
     * @var string
     */
    protected $lockTable;

    /**
     * An array representation of the lock lottery odds.
     *
     * @var array
     */
    protected $lockLottery;

    /**
     * The default number of seconds that a lock should be held.
     *
     * @var int
     */
    protected $defaultLockTimeoutInSeconds;

    /**
     * The classes that should be allowed during unserialization.
     *
     * @var array|bool|null
     */
    protected $serializableClasses;

    /**
     * Create a new database store.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @param  string  $table
     * @param  string  $prefix
     * @param  string  $lockTable
     * @param  array  $lockLottery
     * @param  int  $defaultLockTimeoutInSeconds
     * @param  array|bool|null  $serializableClasses
     */
    public function __construct(
        ConnectionInterface $connection,
        $table,
        $prefix = '',
        $lockTable = 'cache_locks',
        $lockLottery = [2, 100],
        $defaultLockTimeoutInSeconds = 86400,
        $serializableClasses = null,
    ) {}

    /**
     * Retrieve an item from the cache by key.
     *
     * @param  string  $key
     * @return mixed
     */
    public function get($key)
    {}

    /**
     * Retrieve multiple items from the cache by key.
     *
     * Items not found in the cache will have a null value.
     *
     * @return array
     */
    public function many(array $keys)
    {}

    /**
     * Store an item in the cache for a given number of seconds.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  int  $seconds
     * @return bool
     */
    public function put($key, $value, $seconds)
    {}

    /**
     * Store multiple items in the cache for a given number of seconds.
     *
     * @param  array  $values
     * @param  int  $seconds
     * @return bool
     */
    public function putMany(array $values, $seconds)
    {}

    /**
     * Store an item in the cache if the key doesn't exist.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  int  $seconds
     * @return bool
     */
    public function add($key, $value, $seconds)
    {}

    /**
     * Increment the value of an item in the cache.
     *
     * @param  string  $key
     * @param  int  $value
     * @return int|false
     */
    public function increment($key, $value = 1)
    {}

    /**
     * Decrement the value of an item in the cache.
     *
     * @param  string  $key
     * @param  int  $value
     * @return int|false
     */
    public function decrement($key, $value = 1)
    {}

    /**
     * Increment or decrement an item in the cache.
     *
     * @param  string  $key
     * @param  int|float  $value
     * @param  \Closure  $callback
     * @return int|false
     */
    protected function incrementOrDecrement($key, $value, Closure $callback)
    {}

    /**
     * Get the current system time.
     *
     * @return int
     */
    protected function getTime()
    {}

    /**
     * Store an item in the cache indefinitely.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return bool
     */
    public function forever($key, $value)
    {}

    /**
     * Get a lock instance.
     *
     * @param  string  $name
     * @param  int  $seconds
     * @param  string|null  $owner
     * @return \Illuminate\Contracts\Cache\Lock
     */
    public function lock($name, $seconds = 0, $owner = null)
    {}

    /**
     * Restore a lock instance using the owner identifier.
     *
     * @param  string  $name
     * @param  string  $owner
     * @return \Illuminate\Contracts\Cache\Lock
     */
    public function restoreLock($name, $owner)
    {}

    /**
     * Adjust the expiration time of a cached item.
     *
     * @param  string  $key
     * @param  int  $seconds
     * @return bool
     */
    public function touch($key, $seconds)
    {}

    /**
     * Remove an item from the cache.
     *
     * @param  string  $key
     * @return bool
     */
    public function forget($key)
    {}

    /**
     * Remove an item from the cache if it is expired.
     *
     * @param  string  $key
     * @return bool
     */
    public function forgetIfExpired($key)
    {}

    /**
     * Remove all items from the cache.
     *
     * @param  array  $keys
     * @return bool
     */
    protected function forgetMany(array $keys)
    {}

    /**
     * Remove all expired items from the given set from the cache.
     *
     * @param  array  $keys
     * @param  bool  $prefixed
     * @return bool
     */
    protected function forgetManyIfExpired(array $keys, bool $prefixed = false)
    {}

    /**
     * Remove all items from the cache.
     *
     * @return bool
     */
    public function flush()
    {}

    /**
     * Remove all locks from the store.
     *
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function flushLocks(): bool
    {}

    /**
     * Get a query builder for the cache table.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    protected function table()
    {}

    /**
     * Get a query builder for the cache locks table.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    protected function lockTable()
    {}

    /**
     * Get the underlying database connection.
     *
     * @return \Illuminate\Database\ConnectionInterface
     */
    public function getConnection()
    {}

    /**
     * Set the underlying database connection.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @return $this
     */
    public function setConnection($connection)
    {}

    /**
     * Get the connection used to manage locks.
     *
     * @return \Illuminate\Database\ConnectionInterface|null
     */
    public function getLockConnection()
    {}

    /**
     * Specify the connection that should be used to manage locks.
     *
     * @param  \Illuminate\Database\ConnectionInterface  $connection
     * @return $this
     */
    public function setLockConnection($connection)
    {}

    /**
     * Get the cache key prefix.
     *
     * @return string
     */
    public function getPrefix()
    {}

    /**
     * Set the cache key prefix.
     *
     * @param  string  $prefix
     * @return void
     */
    public function setPrefix($prefix)
    {}

    /**
     * Serialize the given value.
     *
     * @param  mixed  $value
     * @return string
     */
    protected function serialize($value)
    {}

    /**
     * Unserialize the given value.
     *
     * @param  string  $value
     * @return mixed
     */
    protected function unserialize($value)
    {}

    /**
     * Determine if the lock store is separate from the cache store.
     *
     * @return bool
     */
    public function hasSeparateLockStore(): bool
    {}
}
