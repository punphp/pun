<?php

namespace Illuminate\Cache;

use Exception;
use Illuminate\Contracts\Cache\Store;
use Illuminate\Contracts\Filesystem\Filesystem;
use Illuminate\Support\InteractsWithTime;

class StorageStore implements Store
{
    use InteractsWithTime, RetrievesMultipleKeys;

    /**
     * The filesystem disk instance.
     *
     * @var \Illuminate\Contracts\Filesystem\Filesystem
     */
    protected $disk;

    /**
     * The storage path where cache files should be written.
     *
     * @var string
     */
    protected $directory;

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
     * Create a new storage cache store instance.
     *
     * @param  \Illuminate\Contracts\Filesystem\Filesystem  $disk
     * @param  string  $directory
     * @param  string  $prefix
     * @param  array|bool|null  $serializableClasses
     */
    public function __construct(Filesystem $disk, $directory = '', $prefix = '', $serializableClasses = null)
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
     * Retrieve an item and expiry time from the cache by key.
     *
     * @param  string  $key
     * @return array
     */
    protected function getPayload($key)
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
     * Get a default empty payload for the cache.
     *
     * @return array
     */
    protected function emptyPayload()
    {}

    /**
     * Get the full path for the given cache key.
     *
     * @param  string  $key
     * @return string
     */
    public function path($key)
    {}

    /**
     * Get the expiration time based on the given seconds.
     *
     * @param  int  $seconds
     * @return int
     */
    protected function expiration($seconds)
    {}

    /**
     * Get the filesystem disk instance.
     *
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function getDisk()
    {}

    /**
     * Get the working directory of the cache.
     *
     * @return string
     */
    public function getDirectory()
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
}
