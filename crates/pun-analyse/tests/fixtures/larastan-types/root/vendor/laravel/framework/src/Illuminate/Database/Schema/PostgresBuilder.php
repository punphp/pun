<?php

namespace Illuminate\Database\Schema;

use Illuminate\Database\Concerns\ParsesSearchPath;

class PostgresBuilder extends Builder
{
    use ParsesSearchPath;

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
     * Drop all types from the database.
     *
     * @return void
     */
    public function dropAllTypes()
    {}

    /**
     * Get the current schemas for the connection.
     *
     * @return string[]
     */
    public function getCurrentSchemaListing()
    {}
}
