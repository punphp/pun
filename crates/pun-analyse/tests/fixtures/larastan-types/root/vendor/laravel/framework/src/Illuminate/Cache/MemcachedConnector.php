<?php

namespace Illuminate\Cache;

use Memcached;

class MemcachedConnector
{
    /**
     * Create a new Memcached connection.
     *
     * @param  array  $servers
     * @param  string|null  $connectionId
     * @param  array  $options
     * @param  array  $credentials
     * @return \Memcached
     */
    public function connect(array $servers, $connectionId = null, array $options = [], #[\SensitiveParameter] array $credentials = [])
    {}

    /**
     * Get a new Memcached instance.
     *
     * @param  string|null  $connectionId
     * @param  array  $credentials
     * @param  array  $options
     * @return \Memcached
     */
    protected function getMemcached($connectionId, #[\SensitiveParameter] array $credentials, array $options)
    {}

    /**
     * Create the Memcached instance.
     *
     * @param  string|null  $connectionId
     * @return \Memcached
     */
    protected function createMemcachedInstance($connectionId)
    {}

    /**
     * Set the SASL credentials on the Memcached connection.
     *
     * @param  \Memcached  $memcached
     * @param  array  $credentials
     * @return void
     */
    protected function setCredentials($memcached, #[\SensitiveParameter] $credentials)
    {}
}
