<?php

namespace Illuminate\Bus;

use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Queue\Attributes\DebounceFor;
use Illuminate\Support\Carbon;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ReadsClassAttributes;

class DebounceLock
{
    use ReadsClassAttributes;

    /**
     * The cache repository implementation.
     *
     * @var \Illuminate\Contracts\Cache\Repository
     */
    protected $cache;

    /**
     * Create a new debounce lock manager instance.
     *
     * @param  \Illuminate\Contracts\Cache\Repository  $cache
     */
    public function __construct(Cache $cache)
    {}

    /**
     * Store a debounce owner token for the given job.
     *
     * Overwrites any existing token, implementing last-writer-wins semantics.
     *
     * @param  mixed  $job
     * @param  int|null  $debounceFor
     * @param  int|null  $maxWait
     * @return array{owner: string, maxWaitExceeded: bool}
     */
    public function acquire($job, $debounceFor = null, $maxWait = null)
    {}

    /**
     * Determine if the maximum debounce wait time has been exceeded.
     */
    protected function maxWaitExceeded(Cache $cache, string $key, int $ttl, ?int $maxWait): bool
    {}

    /**
     * Get the current owner for the given job.
     *
     * @param  mixed  $job
     * @return string|null
     */
    public function getCurrentOwner($job)
    {}

    /**
     * Remove the debounce token for the given job.
     *
     * @param  mixed  $job
     * @param  string  $owner
     * @return void
     */
    public function release($job, string $owner = '')
    {}

    /**
     * Remove the maximum wait timestamp for the given job.
     *
     * @param  mixed  $job
     * @return void
     */
    public function releaseMaxWait($job)
    {}

    /**
     * Get the debounce delay for the given job.
     *
     * @param  mixed  $job
     * @return int|null
     */
    public function getDebounceDelay($job)
    {}

    /**
     * Get the maximum debounce wait time for the given job.
     *
     * @param  mixed  $job
     * @return int|null
     */
    public function getMaxDebounceWait($job)
    {}

    /**
     * Generate the cache key for the given job.
     *
     * @param  mixed  $job
     * @return string
     */
    public static function getKey($job)
    {}

    /**
     * Resolve the cache store for the given job.
     *
     * @param  mixed  $job
     * @return \Illuminate\Contracts\Cache\Repository
     */
    protected function resolveCache($job)
    {}
}
