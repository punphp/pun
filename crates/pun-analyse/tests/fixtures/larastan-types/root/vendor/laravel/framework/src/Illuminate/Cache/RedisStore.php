<?php

namespace Illuminate\Cache;

use Illuminate\Contracts\Cache\CanFlushLocks;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Redis\Factory as Redis;
use Illuminate\Redis\Connections\PhpRedisClusterConnection;
use Illuminate\Redis\Connections\PhpRedisConnection;
use Illuminate\Redis\Connections\PredisClusterConnection;
use Illuminate\Redis\Connections\PredisConnection;
use Illuminate\Support\LazyCollection;
use Illuminate\Support\Str;
use RuntimeException;

class RedisStore extends TaggableStore implements CanFlushLocks, LockProvider
{
    use RetrievesMultipleKeys {
        many as private manyAlias;
        putMany as private putManyAlias;
    }

    /**
     * The Redis factory implementation.
     *
     * @var \Illuminate\Contracts\Redis\Factory
     */
    protected $redis;

    /**
     * A string that should be prepended to keys.
     *
     * @var string
     */
    protected $prefix;

    /**
     * The Redis connection instance that should be used to manage locks.
     *
     * @var string
     */
    protected $connection;

    /**
     * The name of the connection that should be used for locks.
     *
     * @var string|null
     */
    protected $lockConnection;

    /**
     * The classes that should be allowed during unserialization.
     *
     * @var array|bool|null
     */
    protected $serializableClasses;

    /**
     * Create a new Redis store.
     *
     * @param  \Illuminate\Contracts\Redis\Factory  $redis
     * @param  string  $prefix
     * @param  string  $connection
     * @param  array|bool|null  $serializableClasses
     */
    public function __construct(Redis $redis, $prefix = '', $connection = 'default', $serializableClasses = null)
    {}

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
     * @param  array  $keys
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
     * @param  mixed  $value
     * @return int
     */
    public function increment($key, $value = 1)
    {}

    /**
     * Decrement the value of an item in the cache.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return int
     */
    public function decrement($key, $value = 1)
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
     * Remove all expired tag set entries.
     *
     * @return void
     */
    public function flushStaleTags()
    {}

    /**
     * Begin executing a new tags operation.
     *
     * @param  mixed  $names
     * @return \Illuminate\Cache\RedisTaggedCache
     */
    public function tags($names)
    {}

    /**
     * Get a collection of all of the cache tags currently being used.
     *
     * @param  int  $chunkSize
     * @return \Illuminate\Support\LazyCollection
     */
    protected function currentTags($chunkSize = 1000)
    {}

    /**
     * Get the Redis connection instance.
     *
     * @return \Illuminate\Redis\Connections\Connection
     */
    public function connection()
    {}

    /**
     * Get the Redis connection instance that should be used to manage locks.
     *
     * @return \Illuminate\Redis\Connections\Connection
     */
    public function lockConnection()
    {}

    /**
     * Specify the name of the connection that should be used to store data.
     *
     * @param  string  $connection
     * @return void
     */
    public function setConnection($connection)
    {}

    /**
     * Specify the name of the connection that should be used to manage locks.
     *
     * @param  string  $connection
     * @return $this
     */
    public function setLockConnection($connection)
    {}

    /**
     * Get the Redis database instance.
     *
     * @return \Illuminate\Contracts\Redis\Factory
     */
    public function getRedis()
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
     * Prepare a value to be used with the Redis cache store when used by eval scripts.
     *
     * @param  mixed  $value
     * @param  \Illuminate\Redis\Connections\Connection  $connection
     * @return mixed
     */
    protected function pack($value, $connection)
    {}

    /**
     * Serialize the value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function serialize($value)
    {}

    /**
     * Determine if the given value should be stored as plain value.
     *
     * @param  mixed  $value
     * @return bool
     */
    protected function shouldBeStoredWithoutSerialization($value): bool
    {}

    /**
     * Unserialize the value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function unserialize($value)
    {}

    /**
     * Handle connection specific considerations when a value needs to be serialized.
     *
     * @param  mixed  $value
     * @param  \Illuminate\Redis\Connections\Connection  $connection
     * @return mixed
     */
    protected function connectionAwareSerialize($value, $connection)
    {}

    /**
     * Handle connection specific considerations when a value needs to be unserialized.
     *
     * @param  mixed  $value
     * @param  \Illuminate\Redis\Connections\Connection  $connection
     * @return mixed
     */
    protected function connectionAwareUnserialize($value, $connection)
    {}

    /**
     * Determine if the lock store is separate from the cache store.
     *
     * @return bool
     */
    public function hasSeparateLockStore(): bool
    {}
}
