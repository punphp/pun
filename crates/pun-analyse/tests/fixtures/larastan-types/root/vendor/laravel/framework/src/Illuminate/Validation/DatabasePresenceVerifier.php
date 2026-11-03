<?php

namespace Illuminate\Validation;

use Closure;
use Illuminate\Database\ConnectionResolverInterface;

class DatabasePresenceVerifier implements DatabasePresenceVerifierInterface
{
    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\ConnectionResolverInterface
     */
    protected $db;

    /**
     * The database connection to use.
     *
     * @var string|null
     */
    protected $connection;

    /**
     * Create a new database presence verifier.
     *
     * @param  \Illuminate\Database\ConnectionResolverInterface  $db
     */
    public function __construct(ConnectionResolverInterface $db)
    {}

    /**
     * Count the number of objects in a collection having the given value.
     *
     * @param  string  $collection
     * @param  string  $column
     * @param  string  $value
     * @param  int|string|null  $excludeId
     * @param  string|null  $idColumn
     * @param  array  $extra
     * @return int
     */
    public function getCount($collection, $column, $value, $excludeId = null, $idColumn = null, array $extra = [])
    {}

    /**
     * Count the number of objects in a collection with the given values.
     *
     * @param  string  $collection
     * @param  string  $column
     * @param  array  $values
     * @param  array  $extra
     * @return int
     */
    public function getMultiCount($collection, $column, array $values, array $extra = [])
    {}

    /**
     * Add the given conditions to the query.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  array  $conditions
     * @return \Illuminate\Database\Query\Builder
     */
    protected function addConditions($query, $conditions)
    {}

    /**
     * Add a "where" clause to the given query.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  string  $key
     * @param  string  $extraValue
     * @return void
     */
    protected function addWhere($query, $key, $extraValue)
    {}

    /**
     * Get a query builder for the given table.
     *
     * @param  string  $table
     * @return \Illuminate\Database\Query\Builder
     */
    protected function table($table)
    {}

    /**
     * Set the connection to be used.
     *
     * @param  string|null  $connection
     * @return void
     */
    public function setConnection($connection)
    {}
}
