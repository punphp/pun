<?php

namespace Orchestra\Testbench\Concerns;

use Illuminate\Foundation\Testing\RefreshDatabaseState;
use Illuminate\Support\Arr;
use InvalidArgumentException;
use Orchestra\Testbench\Attributes\ResetRefreshDatabaseState;
use Orchestra\Testbench\Database\MigrateProcessor;

use function Orchestra\Testbench\default_migration_path;
use function Orchestra\Testbench\laravel_or_fail;
use function Orchestra\Testbench\load_migration_paths;

/**
 * @internal
 */
trait InteractsWithMigrations
{
    /**
     * List of cached migrators instances.
     *
     * @var array<int, \Orchestra\Testbench\Database\MigrateProcessor>
     */
    protected array $cachedTestMigratorProcessors = [];

    /**
     * Setup the test environment.
     *
     * @return void
     */
    protected function setUpInteractsWithMigrations(): void
    {}

    /**
     * Teardown the test environment.
     *
     * @return void
     */
    protected function tearDownInteractsWithMigrations(): void
    {}

    /**
     * Define hooks to migrate the database before and after each test.
     *
     * @api
     *
     * @param  array<int|string, mixed>|string  $paths
     * @return void
     */
    protected function loadMigrationsFrom(array|string $paths): void
    {}

    /**
     * Resolve Package Migrations Artisan command options.
     *
     * @internal
     *
     * @param  array<string, mixed>|string  $paths
     * @return array<string, mixed>
     *
     * @throws \InvalidArgumentException
     */
    protected function resolvePackageMigrationsOptions(array|string $paths = []): array
    {}

    /**
     * Migrate Laravel's default migrations.
     *
     * @api
     *
     * @param  array<string, mixed>|string  $database
     * @return void
     */
    protected function loadLaravelMigrations(array|string $database = []): void
    {}

    /**
     * Migrate all Laravel's migrations.
     *
     * @api
     *
     * @param  array<string, mixed>|string  $database
     * @return void
     */
    protected function runLaravelMigrations(array|string $database = []): void
    {}

    /**
     * Resolve Laravel Migrations Artisan command options.
     *
     * @internal
     *
     * @param  array<string, mixed>|string  $database
     * @return array<string, mixed>
     */
    protected function resolveLaravelMigrationsOptions(array|string $database = []): array
    {}
}
