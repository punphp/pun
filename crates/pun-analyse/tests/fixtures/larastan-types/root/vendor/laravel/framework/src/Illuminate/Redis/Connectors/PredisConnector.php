<?php

namespace Illuminate\Redis\Connectors;

use Illuminate\Contracts\Redis\Connector;
use Illuminate\Redis\Connections\PredisClusterConnection;
use Illuminate\Redis\Connections\PredisConnection;
use Illuminate\Support\Arr;
use Illuminate\Support\Str;
use InvalidArgumentException;
use Predis\Client;
use Predis\Retry\Retry;
use Predis\Retry\Strategy\RetryStrategyInterface;

class PredisConnector implements Connector
{
    /**
     * Create a new connection.
     *
     * @param  array  $config
     * @param  array  $options
     * @return \Illuminate\Redis\Connections\PredisConnection
     */
    public function connect(array $config, array $options)
    {}

    /**
     * Create a new clustered Predis connection.
     *
     * @param  array  $config
     * @param  array  $clusterOptions
     * @param  array  $options
     * @return \Illuminate\Redis\Connections\PredisClusterConnection
     */
    public function connectToCluster(array $config, array $clusterOptions, array $options)
    {}

    /**
     * Format the host using the scheme if available.
     *
     * @param  array  $config
     * @return array
     */
    protected function formatHost(array $config)
    {}

    /**
     * Format a scalar retry configuration into a Predis retry instance if applicable.
     *
     * @param  array  $config
     * @return array
     */
    protected function formatRetry(array $config)
    {}
}
