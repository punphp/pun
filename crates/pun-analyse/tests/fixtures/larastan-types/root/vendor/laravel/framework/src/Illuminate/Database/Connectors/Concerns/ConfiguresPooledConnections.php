<?php

namespace Illuminate\Database\Connectors\Concerns;

use Illuminate\Support\Arr;
use PDO;

trait ConfiguresPooledConnections
{
    /**
     * Get the direct configuration for a connection.
     *
     * @param  array  $config
     * @return array
     */
    protected function getDirectConfig(array $config)
    {}

    /**
     * Merge a configuration for a direct connection.
     *
     * @param  array  $config
     * @param  array  $merge
     * @return array
     */
    protected function mergeDirectConfig(array $config, array $merge)
    {}

    /**
     * Ensure pooled PostgreSQL connections are configured correctly.
     *
     * @param  array  $config
     * @return array
     */
    protected function ensurePooledPostgresIsProperlyConfigured(array $config)
    {}

    /**
     * Stamp emulated prepares onto a connection configuration when not explicit.
     *
     * @param  array  $config
     * @return array
     */
    protected function withEmulatedPrepares(array $config)
    {}

    /**
     * Determine if the configuration has a direct PostgreSQL connection.
     *
     * @param  array  $config
     * @return bool
     */
    protected function hasDirectConnection(array $config)
    {}
}
