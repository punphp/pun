<?php

namespace Orchestra\Testbench\Concerns;

use Illuminate\Foundation\Testing\RefreshDatabaseState;

use function Orchestra\Testbench\after_resolving;
use function Orchestra\Testbench\default_migration_path;

trait WithLaravelMigrations
{
    use InteractsWithWorkbench;

    /**
     * Bootstrap with laravel migrations.
     *
     * @internal
     *
     * @return void
     */
    protected function setUpWithLaravelMigrations(): void
    {}
}
