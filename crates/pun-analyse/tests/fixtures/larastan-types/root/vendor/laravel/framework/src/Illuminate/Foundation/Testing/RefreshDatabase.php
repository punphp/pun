<?php

namespace Illuminate\Foundation\Testing;

use Illuminate\Contracts\Console\Kernel;
use Illuminate\Foundation\Testing\Traits\CanConfigureMigrationCommands;

trait RefreshDatabase
{
    use CanConfigureMigrationCommands;

    /**
     * Define hooks to migrate the database before and after each test.
     *
     * @return void
     */
    public function refreshDatabase()
    {}

    /**
     * Determine if any of the connections transacting is using in-memory databases.
     *
     * @return bool
     */
    protected function usingInMemoryDatabases()
    {}

    /**
     * Determine if a given database connection is an in-memory database.
     *
     * @return bool
     */
    protected function usingInMemoryDatabase(?string $name = null)
    {}

    /**
     * Restore the in-memory database between tests.
     *
     * @return void
     */
    protected function restoreInMemoryDatabase()
    {}

    /**
     * Refresh a conventional test database.
     *
     * @return void
     */
    protected function refreshTestDatabase()
    {}

    /**
     * Update locally cached in-memory PDO connections after migration.
     *
     * @return void
     */
    protected function updateLocalCacheOfInMemoryDatabases()
    {}

    /**
     * Migrate the database.
     *
     * @return void
     */
    protected function migrateDatabases()
    {}

    /**
     * Migrate the database without dispatching database events.
     *
     * @return void
     */
    protected function migrateDatabasesWithoutEvents()
    {}

    /**
     * Begin a database transaction on the testing database.
     *
     * @return void
     */
    public function beginDatabaseTransaction()
    {}

    /**
     * The database connections that should have transactions.
     *
     * @return array
     */
    protected function connectionsToTransact()
    {}

    /**
     * Perform any work that should take place before the database has started refreshing.
     *
     * @return void
     */
    protected function beforeRefreshingDatabase()
    {}

    /**
     * Perform any work that should take place once the database has finished refreshing.
     *
     * @return void
     */
    protected function afterRefreshingDatabase()
    {}
}
