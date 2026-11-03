<?php

namespace Illuminate\Database\Connectors;

use Illuminate\Database\Concerns\ParsesSearchPath;
use PDO;

class PostgresConnector extends Connector implements ConnectorInterface
{
    use ParsesSearchPath;

    /**
     * The default PDO connection options.
     *
     * @var array
     */
    protected $options = [
        PDO::ATTR_CASE => PDO::CASE_NATURAL,
        PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
        PDO::ATTR_ORACLE_NULLS => PDO::NULL_NATURAL,
        PDO::ATTR_STRINGIFY_FETCHES => false,
    ];

    /**
     * Establish a database connection.
     *
     * @param  array  $config
     * @return \PDO
     */
    public function connect(array $config)
    {}

    /**
     * Create a DSN string from a configuration.
     *
     * @param  array  $config
     * @return string
     */
    protected function getDsn(array $config)
    {}

    /**
     * Add the SSL options to the DSN.
     *
     * @param  string  $dsn
     * @param  array  $config
     * @return string
     */
    protected function addSslOptions($dsn, array $config)
    {}

    /**
     * Add the keepalive options to the DSN.
     *
     * @param  string  $dsn
     * @param  array  $config
     * @return string
     */
    protected function addKeepaliveOptions($dsn, array $config)
    {}

    /**
     * Add the server options to the DSN.
     *
     * @param  string  $dsn
     * @param  array  $config
     * @return string
     */
    protected function addServerOptions($dsn, array $config)
    {}

    /**
     * Set the connection transaction isolation level.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureIsolationLevel($connection, array $config)
    {}

    /**
     * Set the timezone on the connection.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureTimezone($connection, array $config)
    {}

    /**
     * Set the "search_path" on the database connection.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureSearchPath($connection, $config)
    {}

    /**
     * Format the search path for the DSN.
     *
     * @param  array  $searchPath
     * @return string
     */
    protected function quoteSearchPath($searchPath)
    {}

    /**
     * Configure the synchronous_commit setting.
     *
     * @param  \PDO  $connection
     * @param  array  $config
     * @return void
     */
    protected function configureSynchronousCommit($connection, array $config)
    {}
}
