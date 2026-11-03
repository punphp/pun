<?php

namespace Illuminate\Database;

use Illuminate\Database\Connectors\ConnectionFactory;
use Illuminate\Database\Events\ConnectionEstablished;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\ConfigurationUrlParser;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use PDO;
use RuntimeException;
use UnitEnum;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Database\Connection
 */
class DatabaseManager implements ConnectionResolverInterface
{
    use Macroable {
        __call as macroCall;
    }

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The database connection factory instance.
     *
     * @var \Illuminate\Database\Connectors\ConnectionFactory
     */
    protected $factory;

    /**
     * The active connection instances.
     *
     * @var array<string, \Illuminate\Database\Connection>
     */
    protected $connections = [];

    /**
     * The dynamically configured (DB::build) connection configurations.
     *
     * @var array<string, array>
     */
    protected $dynamicConnectionConfigurations = [];

    /**
     * The custom connection resolvers.
     *
     * @var array<string, callable>
     */
    protected $extensions = [];

    /**
     * The callback to be executed to reconnect to a database.
     *
     * @var callable
     */
    protected $reconnector;

    /**
     * Create a new database manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Illuminate\Database\Connectors\ConnectionFactory  $factory
     */
    public function __construct($app, ConnectionFactory $factory)
    {}

    /**
     * Get a database connection instance.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Database\Connection
     */
    public function connection($name = null)
    {}

    /**
     * Build a database connection instance from the given configuration.
     *
     * @param  array  $config
     * @return \Illuminate\Database\ConnectionInterface
     */
    public function build(array $config)
    {}

    /**
     * Calculate the dynamic connection name for an on-demand connection based on its configuration.
     *
     * @param  array  $config
     * @return string
     */
    public static function calculateDynamicConnectionName(array $config)
    {}

    /**
     * Get a database connection instance from the given configuration.
     *
     * @param  \UnitEnum|string  $name
     * @param  array  $config
     * @param  bool  $force
     * @return \Illuminate\Database\ConnectionInterface
     *
     * @throws \RuntimeException
     */
    public function connectUsing(UnitEnum|string $name, array $config, bool $force = false)
    {}

    /**
     * Parse the connection into an array of the name and read / write type.
     *
     * @param  string  $name
     * @return array
     */
    protected function parseConnectionName($name)
    {}

    /**
     * Make the database connection instance.
     *
     * @param  string  $name
     * @return \Illuminate\Database\Connection
     */
    protected function makeConnection($name)
    {}

    /**
     * Get the configuration for a connection.
     *
     * @param  string  $name
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected function configuration($name)
    {}

    /**
     * Prepare the database connection instance.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @param  string  $type
     * @return \Illuminate\Database\Connection
     */
    protected function configure(Connection $connection, $type)
    {}

    /**
     * Dispatch the ConnectionEstablished event if the event dispatcher is available.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @return void
     */
    protected function dispatchConnectionEstablishedEvent(Connection $connection)
    {}

    /**
     * Prepare the read / write mode for database connection instance.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @param  string|null  $type
     * @return \Illuminate\Database\Connection
     */
    protected function setPdoForType(Connection $connection, $type = null)
    {}

    /**
     * Disconnect from the given database and remove from local cache.
     *
     * @param  \UnitEnum|string|null  $name
     * @return void
     */
    public function purge($name = null)
    {}

    /**
     * Disconnect from the given database.
     *
     * @param  \UnitEnum|string|null  $name
     * @return void
     */
    public function disconnect($name = null)
    {}

    /**
     * Reconnect to the given database.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Database\Connection
     */
    public function reconnect($name = null)
    {}

    /**
     * Set the default database connection for the callback execution.
     *
     * @template TReturn
     *
     * @param  \UnitEnum|string  $name
     * @param  (callable(): TReturn)  $callback
     * @return TReturn
     */
    public function usingConnection($name, callable $callback)
    {}

    /**
     * Refresh the PDO connections on a given connection.
     *
     * @param  string  $name
     * @return \Illuminate\Database\Connection
     */
    protected function refreshPdoConnections($name)
    {}

    /**
     * Get the default connection name.
     *
     * @return string
     */
    public function getDefaultConnection()
    {}

    /**
     * Set the default connection name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultConnection($name)
    {}

    /**
     * Get all of the supported drivers.
     *
     * @return string[]
     */
    public function supportedDrivers()
    {}

    /**
     * Get all of the drivers that are actually available.
     *
     * @return string[]
     */
    public function availableDrivers()
    {}

    /**
     * Register an extension connection resolver.
     *
     * @param  string  $name
     * @param  callable  $resolver
     * @return void
     */
    public function extend($name, callable $resolver)
    {}

    /**
     * Remove an extension connection resolver.
     *
     * @param  string  $name
     * @return void
     */
    public function forgetExtension($name)
    {}

    /**
     * Return all of the created connections.
     *
     * @return array<string, \Illuminate\Database\Connection>
     */
    public function getConnections()
    {}

    /**
     * Set the database reconnector callback.
     *
     * @param  callable  $reconnector
     * @return void
     */
    public function setReconnector(callable $reconnector)
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
     * Dynamically pass methods to the default connection.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
