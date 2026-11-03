<?php

namespace Illuminate\Session;

use Illuminate\Support\Manager;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Session\Store
 */
class SessionManager extends Manager
{
    /**
     * Call a custom driver creator.
     *
     * @param  string  $driver
     * @return \Illuminate\Session\Store
     */
    protected function callCustomCreator($driver)
    {}

    /**
     * Create an instance of the "null" session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createNullDriver()
    {}

    /**
     * Create an instance of the "array" session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createArrayDriver()
    {}

    /**
     * Create an instance of the "cookie" session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createCookieDriver()
    {}

    /**
     * Create an instance of the file session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createFileDriver()
    {}

    /**
     * Create an instance of the file session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createNativeDriver()
    {}

    /**
     * Create an instance of the database session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createDatabaseDriver()
    {}

    /**
     * Get the database connection for the database driver.
     *
     * @return \Illuminate\Database\Connection
     */
    protected function getDatabaseConnection()
    {}

    /**
     * Create an instance of the APC session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createApcDriver()
    {}

    /**
     * Create an instance of the Memcached session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createMemcachedDriver()
    {}

    /**
     * Create an instance of the Redis session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createRedisDriver()
    {}

    /**
     * Create an instance of the DynamoDB session driver.
     *
     * @return \Illuminate\Session\Store
     */
    protected function createDynamodbDriver()
    {}

    /**
     * Create an instance of a cache driven driver.
     *
     * @param  string  $driver
     * @return \Illuminate\Session\Store
     */
    protected function createCacheBased($driver)
    {}

    /**
     * Create the cache based session handler instance.
     *
     * @param  string  $driver
     * @return \Illuminate\Session\CacheBasedSessionHandler
     */
    protected function createCacheHandler($driver)
    {}

    /**
     * Build the session instance.
     *
     * @param  \SessionHandlerInterface  $handler
     * @return \Illuminate\Session\Store
     */
    protected function buildSession($handler)
    {}

    /**
     * Build the encrypted session instance.
     *
     * @param  \SessionHandlerInterface  $handler
     * @return \Illuminate\Session\EncryptedStore
     */
    protected function buildEncryptedSession($handler)
    {}

    /**
     * Determine if requests for the same session should wait for each to finish before executing.
     *
     * @return bool
     */
    public function shouldBlock()
    {}

    /**
     * Get the name of the cache store / driver that should be used to acquire session locks.
     *
     * @return string|null
     */
    public function blockDriver()
    {}

    /**
     * Get the maximum number of seconds the session lock should be held for.
     *
     * @return int
     */
    public function defaultRouteBlockLockSeconds()
    {}

    /**
     * Get the maximum number of seconds to wait while attempting to acquire a route block session lock.
     *
     * @return int
     */
    public function defaultRouteBlockWaitSeconds()
    {}

    /**
     * Get the session configuration.
     *
     * @return array
     */
    public function getSessionConfig()
    {}

    /**
     * Get the default session driver name.
     *
     * @return string|null
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the default session driver name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}
}
