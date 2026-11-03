<?php

namespace Orchestra\Testbench\Foundation\Bootstrap;

use Illuminate\Contracts\Config\Repository;
use Illuminate\Contracts\Foundation\Application;
use Orchestra\Testbench\Foundation\Concerns\HandlesDatabaseConnections;

/**
 * @internal
 */
final class SyncDatabaseEnvironmentVariables
{
    use HandlesDatabaseConnections;

    /**
     * Bootstrap the given application.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    public function bootstrap(Application $app): void
    {}
}
