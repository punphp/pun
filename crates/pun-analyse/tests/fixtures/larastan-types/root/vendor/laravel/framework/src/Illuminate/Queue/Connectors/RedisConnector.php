<?php

namespace Illuminate\Queue\Connectors;

use Illuminate\Contracts\Redis\Factory as Redis;
use Illuminate\Queue\RedisQueue;

class RedisConnector implements ConnectorInterface
{
    /**
     * The Redis database instance.
     *
     * @var \Illuminate\Contracts\Redis\Factory
     */
    protected $redis;

    /**
     * The connection name.
     *
     * @var string|null
     */
    protected $connection;

    /**
     * Create a new Redis queue connector instance.
     *
     * @param  \Illuminate\Contracts\Redis\Factory  $redis
     * @param  string|null  $connection
     */
    public function __construct(Redis $redis, $connection = null)
    {}

    /**
     * Establish a queue connection.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Queue\Queue
     */
    public function connect(array $config)
    {}
}
