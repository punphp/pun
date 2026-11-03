<?php

namespace Illuminate\Cache;

use Illuminate\Redis\Connections\PhpRedisConnection;
use Illuminate\Support\Carbon;
use Illuminate\Support\LazyCollection;

class RedisTagSet extends TagSet
{
    /**
     * Add a reference entry to the tag set's underlying sorted set.
     *
     * @param  string  $key
     * @param  int|null  $ttl
     * @param  string|null  $updateWhen
     * @return void
     */
    public function addEntry(string $key, ?int $ttl = null, $updateWhen = null)
    {}

    /**
     * Get all of the cache entry keys for the tag set.
     *
     * @return \Illuminate\Support\LazyCollection
     */
    public function entries()
    {}

    /**
     * Remove the stale entries from the tag set.
     *
     * @return void
     */
    public function flushStaleEntries()
    {}

    /**
     * Flush the tag from the cache.
     *
     * @param  string  $name
     * @return string
     */
    public function flushTag($name)
    {}

    /**
     * Reset the tag and return the new tag identifier.
     *
     * @param  string  $name
     * @return string
     */
    public function resetTag($name)
    {}

    /**
     * Get the unique tag identifier for a given tag.
     *
     * @param  string  $name
     * @return string
     */
    public function tagId($name)
    {}

    /**
     * Get the tag identifier key for a given tag.
     *
     * @param  string  $name
     * @return string
     */
    public function tagKey($name)
    {}
}
