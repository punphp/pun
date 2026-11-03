<?php

namespace Illuminate\Support;

use Closure;
use InvalidArgumentException;
use ReflectionException;
use RuntimeException;

abstract class MultipleInstanceManager
{
    use RebindsCallbacksToSelf;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The configuration repository instance.
     *
     * @var \Illuminate\Contracts\Config\Repository
     */
    protected $config;

    /**
     * The array of resolved instances.
     *
     * @var array
     */
    protected $instances = [];

    /**
     * The registered custom instance creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * The key name of the "driver" equivalent configuration option.
     *
     * @var string
     */
    protected $driverKey = 'driver';

    /**
     * Create a new manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Get the default instance name.
     *
     * @return string
     */
    abstract public function getDefaultInstance();

    /**
     * Set the default instance name.
     *
     * @param  string  $name
     * @return void
     */
    abstract public function setDefaultInstance($name);

    /**
     * Get the instance specific configuration.
     *
     * @param  string  $name
     * @return array
     */
    abstract public function getInstanceConfig($name);

    /**
     * Get an instance by name.
     *
     * @param  string|null  $name
     * @return mixed
     */
    public function instance($name = null)
    {}

    /**
     * Attempt to get an instance from the local cache.
     *
     * @param  string  $name
     * @return mixed
     */
    protected function get($name)
    {}

    /**
     * Resolve the given instance.
     *
     * @param  string  $name
     * @return mixed
     *
     * @throws \InvalidArgumentException
     * @throws \RuntimeException
     */
    protected function resolve($name)
    {}

    /**
     * Call a custom instance creator.
     *
     * @param  array  $config
     * @return mixed
     */
    protected function callCustomCreator(array $config)
    {}

    /**
     * Unset the given instances.
     *
     * @param  array|string|null  $name
     * @return $this
     */
    public function forgetInstance($name = null)
    {}

    /**
     * Disconnect the given instance and remove from local cache.
     *
     * @param  string|null  $name
     * @return void
     */
    public function purge($name = null)
    {}

    /**
     * Register a custom instance creator Closure.
     *
     * @param  string  $name
     * @param  \Closure  $callback
     *
     * @param-closure-this  $this  $callback
     *
     * @return $this
     */
    public function extend($name, Closure $callback)
    {}

    /**
     * Set the application instance used by the manager.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return $this
     */
    public function setApplication($app)
    {}

    /**
     * Dynamically call the default instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
