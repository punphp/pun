<?php

namespace Illuminate\Cache;

use Illuminate\Contracts\Cache\CanFlushLocks;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Support\Carbon;
use Illuminate\Support\InteractsWithTime;
use RuntimeException;

class ArrayStore extends TaggableStore implements CanFlushLocks, LockProvider
{
    use InteractsWithTime, RetrievesMultipleKeys;

    /**
     * The array of stored values.
     *
     * @var array<string, array{value: mixed, expiresAt: float}>
     */
    protected $storage = [];

    /**
     * The array of locks.
     *
     * @var array<string, array{owner: ?string, expiresAt: ?\Illuminate\Support\Carbon}>
     */
    public $locks = [];

    /**
     * Indicates if values are serialized within the store.
     *
     * @var bool
     */
    protected $serializesValues;

    /**
     * The classes that should be allowed during unserialization.
     *
     * @var array|bool|null
     */
    protected $serializableClasses;

    /**
     * Create a new Array store.
     *
     * @param  bool  $serializesValues
     * @param  array|bool|null  $serializableClasses
     */
    public function __construct($serializesValues = false, $serializableClasses = null)
    {}

    /**
     * Get all of the cached values and their expiration times.
     *
     * @param  bool  $unserialize
     * @return array<string, array{value: mixed, expiresAt: float}>
     */
    public function all($unserialize = true)
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
     * Get the cache key prefix.
     *
     * @return string
     */
    public function getPrefix()
    {}

    /**
     * Get the expiration time of the key.
     *
     * @param  int  $seconds
     * @return float
     */
    protected function calculateExpiration($seconds)
    {}

    /**
     * Get the UNIX timestamp, with milliseconds, for the given number of seconds in the future.
     *
     * @param  int  $seconds
     * @return float
     */
    protected function toTimestamp($seconds)
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
     * Determine if the lock store is separate from the cache store.
     *
     * @return bool
     */
    public function hasSeparateLockStore(): bool
    {}

    /**
     * Unserialize the given value.
     *
     * @param  string  $value
     * @return mixed
     */
    protected function unserialize($value)
    {}
}
