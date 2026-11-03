<?php

namespace Illuminate\Cache;

use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Exception\DynamoDbException;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Cache\Store;
use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Str;
use RuntimeException;

class DynamoDbStore implements LockProvider, Store
{
    use InteractsWithTime;

    /**
     * A string that should be prepended to keys.
     *
     * @var string
     */
    protected $prefix;

    /**
     * The classes that should be allowed during unserialization.
     *
     * @var array|bool|null
     */
    protected $serializableClasses;

    /**
     * Create a new store instance.
     *
     * @param  \Aws\DynamoDb\DynamoDbClient  $dynamo  The DynamoDB client instance.
     * @param  string  $table  The table name.
     * @param  string  $keyAttribute  The name of the attribute that should hold the key.
     * @param  string  $valueAttribute  The name of the attribute that should hold the value.
     * @param  string  $expirationAttribute  The name of the attribute that should hold the expiration timestamp.
     * @param  string  $prefix
     * @param  array|bool|null  $serializableClasses
     */
    public function __construct(
        protected DynamoDbClient $dynamo,
        protected $table,
        protected $keyAttribute = 'key',
        protected $valueAttribute = 'value',
        protected $expirationAttribute = 'expires_at',
        $prefix = '',
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
     * @param  array  $keys
     * @return array
     */
    public function many(array $keys)
    {}

    /**
     * Determine if the given item is expired.
     *
     * @param  array  $item
     * @param  \DateTimeInterface|null  $expiration
     * @return bool
     */
    protected function isExpired(array $item, $expiration = null)
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
     *
     * @throws \Aws\DynamoDb\Exception\DynamoDbException
     */
    public function add($key, $value, $seconds)
    {}

    /**
     * Increment the value of an item in the cache.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return int|false
     *
     * @throws \Aws\DynamoDb\Exception\DynamoDbException
     */
    public function increment($key, $value = 1)
    {}

    /**
     * Decrement the value of an item in the cache.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return int|false
     *
     * @throws \Aws\DynamoDb\Exception\DynamoDbException
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
     * Atomically refresh the expiration of a key if it matches the expected owner.
     *
     * @param  string  $key
     * @param  mixed  $expectedOwner
     * @param  int  $seconds
     * @return bool
     */
    public function refreshIfOwned($key, $expectedOwner, $seconds)
    {}

    /**
     * Adjust the expiration time of a cached item.
     *
     * @param  string  $key
     * @param  int  $seconds
     * @return bool
     *
     * @throws DynamoDbException
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
     * @return never
     *
     * @throws \RuntimeException
     */
    public function flush()
    {}

    /**
     * Get the UNIX timestamp for the given number of seconds.
     *
     * @param  int  $seconds
     * @return int
     */
    protected function toTimestamp($seconds)
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
     * Unserialize the value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function unserialize($value)
    {}

    /**
     * Get the DynamoDB type for the given value.
     *
     * @param  mixed  $value
     * @return string
     */
    protected function type($value)
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
     * Get the DynamoDb Client instance.
     *
     * @return \Aws\DynamoDb\DynamoDbClient
     */
    public function getClient()
    {}
}
