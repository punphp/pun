<?php

namespace Illuminate\Cache\Console;

use BadMethodCallException;
use Illuminate\Cache\CacheManager;
use Illuminate\Console\Command;
use Illuminate\Console\Prohibitable;
use Illuminate\Filesystem\Filesystem;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'cache:clear')]
class ClearCommand extends Command
{
    use Prohibitable;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'cache:clear
                    {store? : The name of the store you would like to clear}
                    {--tags= : The cache tags you would like to clear}
                    {--locks : Only clear cache locks}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Flush the application cache';

    /**
     * The cache manager instance.
     *
     * @var \Illuminate\Cache\CacheManager
     */
    protected $cache;

    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * Create a new cache clear command instance.
     *
     * @param  \Illuminate\Cache\CacheManager  $cache
     * @param  \Illuminate\Filesystem\Filesystem  $files
     */
    public function __construct(CacheManager $cache, Filesystem $files)
    {}

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Clear all locks from the cache store.
     *
     * @return int
     */
    protected function clearLocks()
    {}

    /**
     * Flush the real-time facades stored in the cache directory.
     *
     * @return void
     */
    public function flushFacades()
    {}

    /**
     * Get the cache instance for the command.
     *
     * @return \Illuminate\Cache\Repository
     */
    protected function cache()
    {}

    /**
     * Get the tags passed to the command.
     *
     * @return array
     */
    protected function tags()
    {}
}
