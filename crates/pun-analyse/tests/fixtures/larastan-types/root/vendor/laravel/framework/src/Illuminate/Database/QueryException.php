<?php

namespace Illuminate\Database;

use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use PDOException;
use Throwable;

class QueryException extends PDOException
{
    /**
     * The database connection name.
     *
     * @var string
     */
    public $connectionName;

    /**
     * The SQL for the query.
     *
     * @var string
     */
    protected $sql;

    /**
     * The bindings for the query.
     *
     * @var array
     */
    protected $bindings;

    /**
     * The PDO read / write type for the executed query.
     *
     * @var null|'read'|'write'|'direct'
     */
    public $readWriteType;

    /**
     * The connection details for the query (host, port, database, etc.).
     *
     * @var array
     */
    protected $connectionDetails = [];

    /**
     * Create a new query exception instance.
     *
     * @param  string  $connectionName
     * @param  string  $sql
     * @param  array  $bindings
     * @param  \Throwable  $previous
     * @param  array  $connectionDetails
     * @param  null|'read'|'write'|'direct'  $readWriteType
     * @param  bool  $maskBindings
     */
    public function __construct($connectionName, $sql, array $bindings, Throwable $previous, array $connectionDetails = [], $readWriteType = null, $maskBindings = false)
    {}

    /**
     * Format the SQL error message.
     *
     * @param  string  $connectionName
     * @param  string  $sql
     * @param  array  $bindings
     * @param  \Throwable  $previous
     * @param  bool  $maskBindings
     * @return string
     */
    protected function formatMessage($connectionName, $sql, $bindings, Throwable $previous, $maskBindings = false)
    {}

    /**
     * Format the connection details for the error message.
     *
     * @return string
     */
    protected function formatConnectionDetails()
    {}

    /**
     * Get the connection name for the query.
     *
     * @return string
     */
    public function getConnectionName()
    {}

    /**
     * Get the SQL for the query.
     *
     * @return string
     */
    public function getSql()
    {}

    /**
     * Get the raw SQL representation of the query with embedded bindings.
     */
    public function getRawSql(): string
    {}

    /**
     * Get the bindings for the query.
     *
     * @return array
     */
    public function getBindings()
    {}

    /**
     * Get information about the connection such as host, port, database, etc.
     *
     * @return array
     */
    public function getConnectionDetails()
    {}
}
