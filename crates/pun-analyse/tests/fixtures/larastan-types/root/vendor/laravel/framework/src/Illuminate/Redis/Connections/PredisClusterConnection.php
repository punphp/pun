<?php

namespace Illuminate\Redis\Connections;

use Predis\Command\Redis\FLUSHDB;
use Predis\Command\ServerFlushDatabase;

class PredisClusterConnection extends PredisConnection
{
    /**
     * Get the keys that match the given pattern.
     *
     * @param  string  $pattern
     * @return array
     */
    public function keys(string $pattern)
    {}

    /**
     * Flush the selected Redis database on all cluster nodes.
     *
     * @return void
     */
    public function flushdb()
    {}

    /**
     * Determine if the connection is a cluster connection.
     *
     * @return bool
     */
    #[\Override]
    public function isCluster()
    {}
}
