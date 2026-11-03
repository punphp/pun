<?php

namespace Illuminate\Database\Schema;

use Illuminate\Database\QueryException;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\File;

class SQLiteBuilder extends Builder
{
    /**
     * Create a database in the schema.
     *
     * @param  string  $name
     * @return bool
     */
    public function createDatabase($name)
    {}

    /**
     * Drop a database from the schema if the database exists.
     *
     * @param  string  $name
     * @return bool
     */
    public function dropDatabaseIfExists($name)
    {}

    /** @inheritDoc */
    public function getTables($schema = null)
    {}

    /** @inheritDoc */
    public function getViews($schema = null)
    {}

    /** @inheritDoc */
    public function getColumns($table)
    {}

    /**
     * Drop all tables from the database.
     *
     * @return void
     */
    public function dropAllTables()
    {}

    /**
     * Drop all views from the database.
     *
     * @return void
     */
    public function dropAllViews()
    {}

    /**
     * Get the value for the given pragma name or set the given value.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    public function pragma($key, $value = null)
    {}

    /**
     * Empty the database file.
     *
     * @param  string|null  $path
     * @return void
     */
    public function refreshDatabaseFile($path = null)
    {}

    /**
     * Get the names of current schemas for the connection.
     *
     * @return string[]
     */
    public function getCurrentSchemaListing()
    {}
}
