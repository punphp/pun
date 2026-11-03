<?php

namespace Illuminate\Cache;

use Illuminate\Cache\Events\CacheFlushed;
use Illuminate\Cache\Events\CacheFlushing;
use Illuminate\Redis\Connections\PhpRedisClusterConnection;
use Illuminate\Redis\Connections\PhpRedisConnection;
use Illuminate\Redis\Connections\PredisClusterConnection;
use Illuminate\Redis\Connections\PredisConnection;

use function Illuminate\Support\enum_value;

class RedisTaggedCache extends TaggedCache
{
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
     * Store an item in the cache.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @param  \DateTimeInterface|\DateInterval|int|null  $ttl
     * @return bool
     */
    public function put($key, $value, $ttl = null)
    {}

    /**
     * Set the expiration of a cached item.
     *
     * @param  \UnitEnum|string  $key
     * @param  \DateTimeInterface|\DateInterval|int  $ttl
     * @return bool
     */
    public function touch($key, $ttl)
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
    protected function flushClusteredConnection()
    {}

    /**
     * Flush the individual cache entries for the tags.
     *
     * @return void
     */
    protected function flushValues()
    {}

    /**
     * Remove all stale reference entries from the tag set.
     *
     * @return bool
     */
    public function flushStale()
    {}
}
