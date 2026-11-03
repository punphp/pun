<?php

namespace Illuminate\Filesystem;

use Illuminate\Contracts\Foundation\CachesRoutes;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use Illuminate\Support\ServiceProvider;
use InvalidArgumentException;

class FilesystemServiceProvider extends ServiceProvider
{
    /**
     * Bootstrap the filesystem.
     *
     * @return void
     */
    public function boot()
    {}

    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register the native filesystem implementation.
     *
     * @return void
     */
    protected function registerNativeFilesystem()
    {}

    /**
     * Register the driver based filesystem.
     *
     * @return void
     */
    protected function registerFlysystem()
    {}

    /**
     * Register the filesystem manager.
     *
     * @return void
     */
    protected function registerManager()
    {}

    /**
     * Register protected file serving.
     *
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    protected function serveFiles()
    {}

    /**
     * Determine if the disk is serveable.
     *
     * @param  array  $config
     * @return bool
     */
    protected function shouldServeFiles(array $config)
    {}

    /**
     * Get the default file driver.
     *
     * @return string
     */
    protected function getDefaultDriver()
    {}

    /**
     * Get the default cloud based file driver.
     *
     * @return string
     */
    protected function getCloudDriver()
    {}
}
