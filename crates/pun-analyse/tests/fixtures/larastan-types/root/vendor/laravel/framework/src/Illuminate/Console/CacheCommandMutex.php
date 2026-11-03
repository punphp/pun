<?php

namespace Illuminate\Console;

use Carbon\CarbonInterval;
use Illuminate\Cache\DynamoDbStore;
use Illuminate\Contracts\Cache\Factory as Cache;
use Illuminate\Contracts\Cache\LockProvider;
use Illuminate\Support\InteractsWithTime;

class CacheCommandMutex implements CommandMutex
{
    use InteractsWithTime;

    /**
     * The cache factory implementation.
     *
     * @var \Illuminate\Contracts\Cache\Factory
     */
    public $cache;

    /**
     * The cache store that should be used.
     *
     * @var string|null
     */
    public $store = null;

    /**
     * Create a new command mutex.
     *
     * @param  \Illuminate\Contracts\Cache\Factory  $cache
     */
    public function __construct(Cache $cache)
    {}

    /**
     * Attempt to obtain a command mutex for the given command.
     *
     * @param  \Illuminate\Console\Command  $command
     * @return bool
     */
    public function create($command)
    {}

    /**
     * Determine if a command mutex exists for the given command.
     *
     * @param  \Illuminate\Console\Command  $command
     * @return bool
     */
    public function exists($command)
    {}

    /**
     * Release the mutex for the given command.
     *
     * @param  \Illuminate\Console\Command  $command
     * @return bool
     */
    public function forget($command)
    {}

    /**
     * Get the isolatable command mutex name.
     *
     * @param  \Illuminate\Console\Command  $command
     * @return string
     */
    protected function commandMutexName($command)
    {}

    /**
     * Specify the cache store that should be used.
     *
     * @param  string|null  $store
     * @return $this
     */
    public function useStore($store)
    {}

    /**
     * Determine if the given store should use locks for command mutexes.
     *
     * @param  \Illuminate\Contracts\Cache\Store  $store
     * @return bool
     */
    protected function shouldUseLocks($store)
    {}
}
