<?php

namespace Orchestra\Testbench\Concerns;

use Illuminate\Foundation\Testing\Traits\CanConfigureMigrationCommands;
use Illuminate\Support\Collection;
use Orchestra\Testbench\Contracts\Config as ConfigContract;
use Orchestra\Testbench\Foundation\Bootstrap\LoadMigrationsFromArray;
use Orchestra\Testbench\Workbench\Workbench;

trait WithWorkbench
{
    use InteractsWithPHPUnit;
    use InteractsWithWorkbench;

    /**
     * Bootstrap with Workbench.
     *
     * @internal
     *
     * @return void
     */
    protected function setUpWithWorkbench(): void
    {}

    /**
     * Bootstrap discover routes.
     *
     * @internal
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    protected function bootDiscoverRoutesForWorkbench($app): void
    {}

    /**
     * Merge seeders for Workbench.
     *
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @return array<int, class-string>|false
     */
    protected function mergeSeedersForWorkbench(ConfigContract $config): array|false
    {}
}
