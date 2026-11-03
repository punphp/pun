<?php

namespace Illuminate\Cache;

use Exception;
use Illuminate\Contracts\Cache\CanFlushLocks;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Contracts\Cache\Store;
use Illuminate\Contracts\Filesystem\LockTimeoutException;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Filesystem\LockableFile;
use Illuminate\Support\InteractsWithTime;
use RuntimeException;

class FileStore implements CanFlushLocks, LockProvider, Store
{
    use InteractsWithTime, RetrievesMultipleKeys;

    /**
     * The Illuminate Filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The file cache directory.
     *
     * @var string
     */
    protected $directory;

    /**
     * The file cache lock directory.
     *
     * @var string|null
     */
    protected $lockDirectory;

    /**
     * Octal representation of the cache file permissions.
     *
     * @var int|null
     */
    protected $filePermission;

    /**
     * The classes that should be allowed during unserialization.
     *
     * @var array|bool|null
     */
    protected $serializableClasses;

    /**
     * Create a new file cache store instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  string  $directory
     * @param  int|null  $filePermission
     * @param  array|bool|null  $serializableClasses
     */
    public function __construct(Filesystem $files, $directory, $filePermission = null, $serializableClasses = null)
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
     * Create the file cache directory if necessary.
     *
     * @param  string  $path
     * @return void
     */
    protected function ensureCacheDirectoryExists($path)
    {}

    /**
     * Ensure the created node has the correct permissions.
     *
     * @param  string  $path
     * @return void
     */
    protected function ensurePermissionsAreCorrect($path)
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
     * Atomically refresh the expiration of a cache key if it matches the expected owner.
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
     * Get the Filesystem instance.
     *
     * @return \Illuminate\Filesystem\Filesystem
     */
    public function getFilesystem()
    {}

    /**
     * Get the working directory of the cache.
     *
     * @return string
     */
    public function getDirectory()
    {}

    /**
     * Set the working directory of the cache.
     *
     * @param  string  $directory
     * @return $this
     */
    public function setDirectory($directory)
    {}

    /**
     * Set the cache directory where locks should be stored.
     *
     * @param  string|null  $lockDirectory
     * @return $this
     */
    public function setLockDirectory($lockDirectory)
    {}

    /**
     * Get the cache key prefix.
     *
     * @return string
     */
    public function getPrefix()
    {}

    /**
     * Determine if the lock store is separate from the cache store.
     *
     * @return bool
     */
    public function hasSeparateLockStore(): bool
    {}
}
