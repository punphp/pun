<?php

namespace Illuminate\Concurrency;

use Illuminate\Process\Factory as ProcessFactory;
use Illuminate\Support\MultipleInstanceManager;
use RuntimeException;
use Spatie\Fork\Fork;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Contracts\Concurrency\Driver
 */
class ConcurrencyManager extends MultipleInstanceManager
{
    /**
     * Get a driver instance by name.
     *
     * @param  \UnitEnum|string|null  $name
     * @return mixed
     */
    public function driver($name = null)
    {}

    /**
     * Create an instance of the process concurrency driver.
     *
     * @return \Illuminate\Concurrency\ProcessDriver
     */
    public function createProcessDriver()
    {}

    /**
     * Create an instance of the fork concurrency driver.
     *
     * @return \Illuminate\Concurrency\ForkDriver
     *
     * @throws \RuntimeException
     */
    public function createForkDriver()
    {}

    /**
     * Create an instance of the sync concurrency driver.
     *
     * @return \Illuminate\Concurrency\SyncDriver
     */
    public function createSyncDriver()
    {}

    /**
     * Get the default instance name.
     *
     * @return string
     */
    public function getDefaultInstance()
    {}

    /**
     * Set the default instance name.
     *
     * @param  string  $name
     * @return void
     */
    public function setDefaultInstance($name)
    {}

    /**
     * Get the instance specific configuration.
     *
     * @param  string  $name
     * @return array
     */
    public function getInstanceConfig($name)
    {}
}
