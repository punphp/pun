<?php

namespace Illuminate\Testing\Concerns;

use Illuminate\Database\QueryException;
use Illuminate\Foundation\Testing;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Artisan;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\ParallelTesting;
use Illuminate\Support\Facades\Schema;

trait TestDatabases
{
    /**
     * Indicates if the test database schema is up to date.
     *
     * @var bool
     */
    protected static $schemaIsUpToDate = false;

    /**
     * The root database name prior to concatenating the token.
     *
     * @var null|string
     */
    protected static $originalDatabaseName = null;

    /**
     * Boot a test database.
     *
     * @return void
     */
    protected function bootTestDatabase()
    {}

    /**
     * Ensure a test database exists and returns its name.
     *
     * @param  string  $database
     * @return array
     */
    protected function ensureTestDatabaseExists($database)
    {}

    /**
     * Ensure the current database test schema is up to date.
     *
     * @return void
     */
    protected function ensureSchemaIsUpToDate()
    {}

    /**
     * Runs the given callable using the given database.
     *
     * @param  string  $database
     * @param  callable  $callable
     * @return void
     */
    protected function usingDatabase($database, $callable)
    {}

    /**
     * Apply the given callback when tests are not using in memory database.
     *
     * @param  callable  $callback
     * @return void
     */
    protected function whenNotUsingInMemoryDatabase($callback)
    {}

    /**
     * Switch to the given database.
     *
     * @param  string  $database
     * @return void
     */
    protected function switchToDatabase($database)
    {}

    /**
     * Returns the test database name.
     *
     * @return string
     */
    protected function testDatabase($database)
    {}
}
