<?php

namespace Orchestra\Testbench\Foundation\Bootstrap;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Support\Collection;
use Orchestra\Sidekick\Env;

/**
 * @internal
 */
final class EnsuresDefaultConfiguration
{
    /**
     * Bootstrap the given application.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    public function bootstrap(Application $app): void
    {}

    /**
     * Determine whether default migrations should be included.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return bool
     */
    protected function includesDefaultConfigurations($app): bool
    {}
}
