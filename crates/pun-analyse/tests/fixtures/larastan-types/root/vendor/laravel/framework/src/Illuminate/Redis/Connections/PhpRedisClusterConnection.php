<?php

namespace Illuminate\Redis\Connections;

use InvalidArgumentException;

class PhpRedisClusterConnection extends PhpRedisConnection
{
    /**
     * The RedisCluster client.
     *
     * @var \RedisCluster
     */
    protected $client;

    /**
     * Scan all keys based on the given options.
     *
     * @param  mixed  $cursor
     * @param  array  $options
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    #[\Override]
    public function scan($cursor, $options = [])
    {}

    /**
     * Flush the selected Redis database on all master nodes.
     *
     * @return void
     */
    public function flushdb()
    { func_get_args(); }

    /**
     * Determine if the connection is a cluster connection.
     *
     * @return bool
     */
    #[\Override]
    public function isCluster()
    {}
}
