<?php

namespace Illuminate\Redis;

use Closure;
use Illuminate\Contracts\Redis\Factory;
use Illuminate\Redis\Connections\Connection;
use Illuminate\Redis\Connectors\PhpRedisConnector;
use Illuminate\Redis\Connectors\PredisConnector;
use Illuminate\Support\Arr;
use Illuminate\Support\ConfigurationUrlParser;
use Illuminate\Support\RebindsCallbacksToSelf;
use InvalidArgumentException;
use ReflectionException;
use RuntimeException;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Redis\Connections\Connection
 */
class RedisManager implements Factory
{
    use RebindsCallbacksToSelf;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The name of the default driver.
     *
     * @var string
     */
    protected $driver;

    /**
     * The registered custom driver creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * The Redis server configurations.
     *
     * @var array
     */
    protected $config;

    /**
     * The Redis connections.
     *
     * @var mixed
     */
    protected $connections;

    /**
     * Indicates whether event dispatcher is set on connections.
     *
     * @var bool
     */
    protected $events = false;

    /**
     * Create a new Redis manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  string  $driver
     * @param  array  $config
     */
    public function __construct($app, $driver, array $config)
    {}

    /**
     * Get a Redis connection by name.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Redis\Connections\Connection
     */
    public function connection($name = null)
    {}

    /**
     * Resolve the given connection by name.
     *
     * @param  string|null  $name
     * @return \Illuminate\Redis\Connections\Connection
     *
     * @throws \InvalidArgumentException
     */
    public function resolve($name = null)
    {}

    /**
     * Resolve the given cluster connection by name.
     *
     * @param  string  $name
     * @return \Illuminate\Redis\Connections\Connection
     */
    protected function resolveCluster($name)
    {}

    /**
     * Configure the given connection to prepare it for commands.
     *
     * @param  \Illuminate\Redis\Connections\Connection  $connection
     * @param  string  $name
     * @return \Illuminate\Redis\Connections\Connection
     */
    protected function configure(Connection $connection, $name)
    {}

    /**
     * Get the connector instance for the current driver.
     *
     * @return \Illuminate\Contracts\Redis\Connector|null
     */
    protected function connector()
    {}

    /**
     * Parse the Redis connection configuration.
     *
     * @param  mixed  $config
     * @return array
     */
    protected function parseConnectionConfiguration($config)
    {}

    /**
     * Return all of the created connections.
     *
     * @return array
     */
    public function connections()
    {}

    /**
     * Enable the firing of Redis command events.
     *
     * @return void
     */
    public function enableEvents()
    {}

    /**
     * Disable the firing of Redis command events.
     *
     * @return void
     */
    public function disableEvents()
    {}

    /**
     * Set the default driver.
     *
     * @param  string  $driver
     * @return void
     */
    public function setDriver($driver)
    {}

    /**
     * Disconnect the given connection and remove from local cache.
     *
     * @param  \UnitEnum|string|null  $name
     * @return void
     */
    public function purge($name = null)
    {}

    /**
     * Register a custom driver creator Closure.
     *
     * @param  string  $driver
     * @param  \Closure  $callback
     *
     * @param-closure-this  $this  $callback
     *
     * @return $this
     */
    public function extend($driver, Closure $callback)
    {}

    /**
     * Pass methods onto the default Redis connection.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
