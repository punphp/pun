<?php

namespace Orchestra\Testbench\Foundation\Bootstrap;

use Illuminate\Contracts\Console\Kernel as ConsoleKernel;
use Illuminate\Contracts\Events\Dispatcher as EventDispatcher;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Database\Events\DatabaseRefreshed;
use Illuminate\Support\Collection;
use Orchestra\Sidekick\Env;

use function Orchestra\Sidekick\transform_relative_path;
use function Orchestra\Testbench\default_migration_path;
use function Orchestra\Testbench\load_migration_paths;
use function Orchestra\Testbench\workbench;

/**
 * @internal
 */
final class LoadMigrationsFromArray
{
    /**
     * Construct a new Create Vendor Symlink bootstrapper.
     *
     * @param  array<int, string>|bool|string  $migrations
     * @param  array<int, class-string>|bool|class-string  $seeders
     */
    public function __construct(
        public readonly array|bool|string $migrations = [],
        public readonly array|bool|string $seeders = false
    ) {}

    /**
     * Bootstrap the given application.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    public function bootstrap(Application $app): void
    {}

    /**
     * Bootstrap seeders.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    protected function bootstrapSeeders(Application $app): void
    {}

    /**
     * Bootstrap migrations.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    protected function bootstrapMigrations(Application $app): void
    {}

    /**
     * Determine whether default migrations should be included.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return bool
     */
    protected function includesDefaultMigrations($app): bool
    {}
}
