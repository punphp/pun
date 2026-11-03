<?php

namespace Orchestra\Testbench\Concerns;

use Closure;
use Illuminate\Database\Events\DatabaseRefreshed;
use Orchestra\Testbench\Attributes\DefineDatabase;
use Orchestra\Testbench\Attributes\RequiresDatabase;
use Orchestra\Testbench\Attributes\WithMigration;
use Orchestra\Testbench\Features\TestingFeature;

use function Orchestra\Testbench\laravel_or_fail;

/**
 * @internal
 */
trait HandlesDatabases
{
    /**
     * Setup database requirements.
     *
     * @internal
     *
     * @param  \Closure():void  $callback
     */
    protected function setUpDatabaseRequirements(Closure $callback): void
    {}

    /**
     * Determine if using in-memory SQLite database connection
     *
     * @api
     *
     * @param  string|null  $connection
     * @return bool
     */
    protected function usesSqliteInMemoryDatabaseConnection(?string $connection = null): bool
    {}

    /**
     * Define database migrations.
     *
     * @api
     *
     * @return void
     */
    protected function defineDatabaseMigrations()
    {}

    /**
     * Define database migrations after database refreshed.
     *
     * @api
     *
     * @return void
     */
    protected function defineDatabaseMigrationsAfterDatabaseRefreshed()
    {}

    /**
     * Destroy database migrations.
     *
     * @api
     *
     * @return void
     */
    protected function destroyDatabaseMigrations()
    {}

    /**
     * Define database seeders.
     *
     * @api
     *
     * @return void
     */
    protected function defineDatabaseSeeders()
    {}
}
