<?php

namespace Illuminate\Redis\Connectors;

use Illuminate\Contracts\Redis\Connector;
use Illuminate\Redis\Connections\PhpRedisClusterConnection;
use Illuminate\Redis\Connections\PhpRedisConnection;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Redis as RedisFacade;
use Illuminate\Support\Str;
use InvalidArgumentException;
use LogicException;
use Redis;
use RedisCluster;

class PhpRedisConnector implements Connector
{
    /**
     * Create a new connection.
     *
     * @param  array  $config
     * @param  array  $options
     * @return \Illuminate\Redis\Connections\PhpRedisConnection
     */
    public function connect(array $config, array $options)
    {}

    /**
     * Create a new clustered PhpRedis connection.
     *
     * @param  array  $config
     * @param  array  $clusterOptions
     * @param  array  $options
     * @return \Illuminate\Redis\Connections\PhpRedisClusterConnection
     */
    public function connectToCluster(array $config, array $clusterOptions, array $options)
    {}

    /**
     * Build a single cluster seed string from an array.
     *
     * @param  array  $server
     * @return string
     */
    protected function buildClusterConnectionString(array $server)
    {}

    /**
     * Create the Redis client instance.
     *
     * @param  array  $config
     * @return \Redis
     *
     * @throws \LogicException
     */
    protected function createClient(array $config)
    {}

    /**
     * Establish a connection with the Redis host.
     *
     * @param  \Redis  $client
     * @param  array  $config
     * @return void
     */
    protected function establishConnection($client, array $config)
    {}

    /**
     * Create a new redis cluster instance.
     *
     * @param  array  $servers
     * @param  array  $options
     * @return \RedisCluster
     */
    protected function createRedisClusterInstance(array $servers, array $options)
    {}

    /**
     * Format the host using the scheme if available.
     *
     * @param  array  $options
     * @return string
     */
    protected function formatHost(array $options)
    {}

    /**
     * Normalize the SSL context for a single Redis connection.
     *
     * Redis::connect() expects the context as ['stream' => ['verify_peer' => false, ...]].
     *
     * @param  array  $context
     * @return array
     */
    protected function normalizeContext(array $context)
    {}

    /**
     * Normalize the SSL context for a RedisCluster connection.
     *
     * RedisCluster::__construct() expects a flat context ['verify_peer' => false, ...].
     *
     * @param  array  $context
     * @return array
     */
    protected function normalizeClusterContext(array $context)
    {}

    /**
     * Format the password for a Redis cluster connection.
     *
     * @param  array  $options
     * @return string|array|null
     */
    protected function formatClusterPassword(array $options)
    {}

    /**
     * Parse a "friendly" backoff algorithm name into an integer.
     *
     * @param  mixed  $algorithm
     * @return int
     *
     * @throws \InvalidArgumentException
     */
    protected function parseBackoffAlgorithm(mixed $algorithm)
    {}
}
