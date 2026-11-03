<?php

namespace Illuminate\Cache;

use Aws\DynamoDb\DynamoDbClient;
use Closure;
use Illuminate\Contracts\Cache\Factory as FactoryContract;
use Illuminate\Contracts\Cache\Store;
use Illuminate\Contracts\Events\Dispatcher as DispatcherContract;
use Illuminate\Support\Arr;
use Illuminate\Support\RebindsCallbacksToSelf;
use InvalidArgumentException;
use Mockery;
use Mockery\LegacyMockInterface;
use ReflectionException;
use RuntimeException;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Cache\Repository
 * @mixin \Illuminate\Contracts\Cache\LockProvider
 */
class CacheManager implements FactoryContract
{
    use RebindsCallbacksToSelf;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The array of resolved cache stores.
     *
     * @var array
     */
    protected $stores = [];

    /**
     * The registered custom driver creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * Create a new Cache manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Get a cache store instance by name, wrapped in a repository.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Contracts\Cache\Repository
     */
    public function store($name = null)
    {}

    /**
     * Get a cache driver instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return \Illuminate\Contracts\Cache\Repository
     */
    public function driver($driver = null)
    {}

    /**
     * Get a memoized cache driver instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return \Illuminate\Contracts\Cache\Repository
     */
    public function memo($driver = null)
    {}

    /**
     * Resolve the given store.
     *
     * @param  string  $name
     * @return \Illuminate\Contracts\Cache\Repository
     *
     * @throws \InvalidArgumentException
     */
    public function resolve($name)
    {}

    /**
     * Build a cache repository with the given configuration.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     *
     * @throws \InvalidArgumentException
     */
    public function build(array $config)
    {}

    /**
     * Call a custom driver creator.
     *
     * @param  array  $config
     * @return mixed
     */
    protected function callCustomCreator(array $config)
    {}

    /**
     * Create an instance of the APC cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createApcDriver(array $config)
    {}

    /**
     * Create an instance of the array cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createArrayDriver(array $config)
    {}

    /**
     * Create an instance of the database cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createDatabaseDriver(array $config)
    {}

    /**
     * Create an instance of the DynamoDB cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createDynamodbDriver(array $config)
    {}

    /**
     * Create new DynamoDb Client instance.
     *
     * @return \Aws\DynamoDb\DynamoDbClient
     */
    protected function newDynamodbClient(array $config)
    {}

    /**
     * Create an instance of the failover cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createFailoverDriver(array $config)
    {}

    /**
     * Create an instance of the file cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createFileDriver(array $config)
    {}

    /**
     * Create an instance of the storage cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createStorageDriver(array $config)
    {}

    /**
     * Create an instance of the Memcached cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createMemcachedDriver(array $config)
    {}

    /**
     * Create an instance of the Null cache driver.
     *
     * @return \Illuminate\Cache\Repository
     */
    protected function createNullDriver()
    {}

    /**
     * Create an instance of the Redis cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createRedisDriver(array $config)
    {}

    /**
     * Create an instance of the session cache driver.
     *
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    protected function createSessionDriver(array $config)
    {}

    /**
     * Get the session store implementation.
     *
     * @return \Illuminate\Contracts\Session\Session
     *
     * @throws \InvalidArgumentException
     */
    protected function getSession()
    {}

    /**
     * Create a new cache repository with the given implementation.
     *
     * @param  \Illuminate\Contracts\Cache\Store  $store
     * @param  array  $config
     * @return \Illuminate\Cache\Repository
     */
    public function repository(Store $store, array $config = [])
    {}

    /**
     * Set the event dispatcher on the given repository instance.
     *
     * @param  \Illuminate\Cache\Repository  $repository
     * @return void
     */
    protected function setEventDispatcher(Repository $repository)
    {}

    /**
     * Re-set the event dispatcher on all resolved cache repositories.
     *
     * @return void
     */
    public function refreshEventDispatcher()
    {}

    /**
     * Get the cache prefix.
     *
     * @param  array  $config
     * @return string
     */
    protected function getPrefix(array $config)
    {}

    /**
     * Get the classes that should be allowed during unserialization.
     *
     * @param  array  $config
     * @return array|bool|null
     */
    protected function getSerializableClasses(array $config)
    {}

    /**
     * Get the cache connection configuration.
     *
     * @param  string  $name
     * @return array|null
     */
    protected function getConfig($name)
    {}

    /**
     * Get the default cache driver name.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the default cache driver name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}

    /**
     * Unset the given driver instances.
     *
     * @param  array|\UnitEnum|string|null  $name
     * @return $this
     */
    public function forgetDriver($name = null)
    {}

    /**
     * Disconnect the given driver and remove from local cache.
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
     * Set the application instance used by the manager.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return $this
     */
    public function setApplication($app)
    {}

    /**
     * Register a callback to be invoked when an unserializable class is encountered.
     *
     * @param  callable|null  $callback
     * @return void
     */
    public function handleUnserializableClassUsing(?callable $callback): void
    {}

    /**
     * Dynamically call the default driver instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
