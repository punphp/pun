<?php

namespace Illuminate\Cache;

use BadMethodCallException;
use Illuminate\Contracts\Cache\CanFlushLocks;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Cache\Store;

class MemoizedStore implements CanFlushLocks, LockProvider, Store
{
    /**
     * The memoized cache values.
     *
     * @var array<string, mixed>
     */
    protected $cache = [];

    /**
     * The memoized tagged cache instances.
     *
     * @var array<string, \Illuminate\Cache\MemoizedTaggedCache>
     */
    protected $taggedCaches = [];

    /**
     * Create a new memoized cache instance.
     *
     * @param  string  $name
     * @param  \Illuminate\Cache\Repository  $repository
     */
    public function __construct(
        protected $name,
        protected $repository,
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
     * @param  int  $seconds
     * @return bool
     */
    public function putMany(array $values, $seconds)
    {}

    /**
     * Increment the value of an item in the cache.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return int|bool
     */
    public function increment($key, $value = 1)
    {}

    /**
     * Decrement the value of an item in the cache.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return int|bool
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
     *
     * @throws \BadMethodCallException
     */
    public function lock($name, $seconds = 0, $owner = null)
    { func_get_args(); }

    /**
     * Restore a lock instance using the owner identifier.
     *
     * @param  string  $name
     * @param  string  $owner
     * @return \Illuminate\Contracts\Cache\Lock
     *
     * @throws \BadMethodCallException
     */
    public function restoreLock($name, $owner)
    { func_get_args(); }

    /**
     * Flush all locks managed by the store.
     *
     * @throws \BadMethodCallException
     */
    public function flushLocks(): bool
    {}

    /**
     * Determine if the lock store is separate from the cache store.
     */
    public function hasSeparateLockStore(): bool
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
     * Begin executing a new tags operation.
     *
     * @param  array|mixed  $names
     * @return \Illuminate\Cache\TaggedCache
     *
     * @throws \BadMethodCallException
     */
    public function tags($names)
    { func_get_args(); }

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
     * Remove all memoized items from the tagged caches.
     *
     * @return void
     */
    public function flushTagged()
    {}

    /**
     * Get the cache key prefix.
     *
     * @return string
     */
    public function getPrefix()
    {}

    /**
     * Prefix the given key.
     *
     * @param  string  $key
     * @return string
     */
    protected function prefix($key)
    {}
}
