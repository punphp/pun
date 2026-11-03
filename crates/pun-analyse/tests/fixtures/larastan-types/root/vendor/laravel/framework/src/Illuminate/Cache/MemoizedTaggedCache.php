<?php

namespace Illuminate\Cache;

use function Illuminate\Support\enum_value;

class MemoizedTaggedCache extends TaggedCache
{
    /**
     * The memoized cache values.
     *
     * @var array<string, mixed>
     */
    protected $cache = [];

    /**
     * The tagged cache instance.
     *
     * @var \Illuminate\Cache\TaggedCache
     */
    protected TaggedCache $taggedCache;

    /**
     * The memoized store instance.
     *
     * @var \Illuminate\Cache\MemoizedStore
     */
    protected MemoizedStore $memoizedStore;

    /**
     * Create a new memoized tagged cache.
     */
    public function __construct(TaggedCache $taggedCache, MemoizedStore $memoizedStore)
    {}

    /**
     * Retrieve an item from the cache by key.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function get($key, $default = null): mixed
    {}

    /**
     * Retrieve multiple items from the cache by key.
     *
     * @param  array  $defaults
     * @return array
     */
    public function many(array $defaults)
    {}

    /**
     * Store an item in the cache for a given number of seconds.
     *
     * @param  \UnitEnum|array|string  $key
     * @param  mixed  $value
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function put($key, $value, $ttl = null)
    {}

    /**
     * Store multiple items in the cache for a given number of seconds.
     *
     * @param  array  $values
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function putMany(array $values, $ttl = null)
    {}

    /**
     * Store an item in the cache if the key does not exist.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function add($key, $value, $ttl = null)
    {}

    /**
     * Increment the value of an item in the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return int|bool
     */
    public function increment($key, $value = 1)
    {}

    /**
     * Decrement the value of an item in the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return int|bool
     */
    public function decrement($key, $value = 1)
    {}

    /**
     * Store an item in the cache indefinitely.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return bool
     */
    public function forever($key, $value)
    {}

    /**
     * Adjust the expiration time of a cached item.
     *
     * @param  \UnitEnum|string  $key
     * @param  \DateTimeInterface|\DateInterval|int  $ttl
     * @return bool
     */
    public function touch($key, $ttl)
    {}

    /**
     * Remove an item from the cache.
     *
     * @param  \UnitEnum|string  $key
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
     * Remove all items from the cache.
     *
     * @return bool
     */
    public function clear(): bool
    {}

    /**
     * Remove all memoized items from the cache.
     *
     * @return void
     */
    public function flushMemoized()
    {}

    /**
     * {@inheritdoc}
     */
    protected function itemKey($key)
    {}

    /**
     * Handle dynamic calls into macros or pass missing methods to the tagged cache.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
