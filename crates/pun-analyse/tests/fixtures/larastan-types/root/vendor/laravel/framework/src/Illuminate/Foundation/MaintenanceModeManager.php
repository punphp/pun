<?php

namespace Illuminate\Foundation;

use Illuminate\Support\Manager;

class MaintenanceModeManager extends Manager
{
    /**
     * Create an instance of the file based maintenance driver.
     *
     * @return \Illuminate\Foundation\FileBasedMaintenanceMode
     */
    protected function createFileDriver(): FileBasedMaintenanceMode
    {}

    /**
     * Create an instance of the array based maintenance driver.
     *
     * @return \Illuminate\Foundation\ArrayMaintenanceMode
     */
    protected function createArrayDriver(): ArrayMaintenanceMode
    {}

    /**
     * Create an instance of the cache based maintenance driver.
     *
     * @return \Illuminate\Foundation\CacheBasedMaintenanceMode
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected function createCacheDriver(): CacheBasedMaintenanceMode
    {}

    /**
     * Get the default driver name.
     *
     * @return string
     */
    public function getDefaultDriver(): string
    {}
}
