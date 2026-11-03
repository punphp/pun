<?php

namespace Illuminate\Testing\Concerns;

use Illuminate\Support\Facades\ParallelTesting;

trait TestCaches
{
    /**
     * The original cache prefix prior to appending the token.
     *
     * @var string|null
     */
    protected static $originalCachePrefix = null;

    /**
     * Boot test cache for parallel testing.
     *
     * @return void
     */
    protected function bootTestCache()
    {}

    /**
     * Get the test cache prefix.
     *
     * @return string
     */
    protected function parallelSafeCachePrefix()
    {}

    /**
     * Switch to the given cache prefix.
     *
     * @param  string  $prefix
     * @return void
     */
    protected function switchToCachePrefix($prefix)
    {}
}
