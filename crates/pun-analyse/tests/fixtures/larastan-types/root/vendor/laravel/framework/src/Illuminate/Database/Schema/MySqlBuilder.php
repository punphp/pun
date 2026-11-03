<?php

namespace Illuminate\Database\Schema;

class MySqlBuilder extends Builder
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
     * Get the names of current schemas for the connection.
     *
     * @return string[]
     */
    public function getCurrentSchemaListing()
    {}
}
