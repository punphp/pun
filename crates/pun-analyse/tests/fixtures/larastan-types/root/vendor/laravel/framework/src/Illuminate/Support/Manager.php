<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Contracts\Container\Container;
use InvalidArgumentException;
use ReflectionException;
use RuntimeException;

abstract class Manager
{
    use RebindsCallbacksToSelf;

    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * The configuration repository instance.
     *
     * @var \Illuminate\Contracts\Config\Repository
     */
    protected $config;

    /**
     * The registered custom driver creators.
     *
     * @var array<string, \Closure>
     */
    protected $customCreators = [];

    /**
     * The array of created "drivers".
     *
     * @var array<string, mixed>
     */
    protected $drivers = [];

    /**
     * Create a new manager instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     */
    public function __construct(Container $container)
    {}

    /**
     * Get the default driver name.
     *
     * @return string|null
     */
    abstract public function getDefaultDriver();

    /**
     * Get a driver instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    public function driver($driver = null)
    {}

    /**
     * Create a new driver instance.
     *
     * @param  string  $driver
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    protected function createDriver($driver)
    {}

    /**
     * Call a custom driver creator.
     *
     * @param  string  $driver
     * @return mixed
     */
    protected function callCustomCreator($driver)
    {}

    /**
     * Register a custom driver creator Closure.
     *
     * @param  string  $driver
     *
     * @param-closure-this  $this  $callback
     *
     * @return $this
     */
    public function extend($driver, Closure $callback)
    {}

    /**
     * Get all of the created "drivers".
     *
     * @return array<string, mixed>
     */
    public function getDrivers()
    {}

    /**
     * Get the container instance used by the manager.
     *
     * @return \Illuminate\Contracts\Container\Container
     */
    public function getContainer()
    {}

    /**
     * Set the container instance used by the manager.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}

    /**
     * Forget all of the resolved driver instances.
     *
     * @return $this
     */
    public function forgetDrivers()
    {}

    /**
     * Dynamically call the default driver instance.
     *
     * @param  string  $method
     * @param  array<string, mixed>  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
