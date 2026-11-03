<?php

namespace Illuminate\Database\Schema;

use Illuminate\Support\Arr;

class SqlServerBuilder extends Builder
{
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
     * Get the default schema name for the connection.
     *
     * @return string|null
     */
    public function getCurrentSchemaName()
    {}
}
