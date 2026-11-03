<?php

namespace Orchestra\Testbench\Bootstrap;

use Illuminate\Contracts\Foundation\Application;
use Spatie\LaravelRay\RayServiceProvider;
use Spatie\Ray\Settings\Settings;

use function Orchestra\Testbench\after_resolving;

/**
 * @internal
 */
final class ConfigureRay
{
    /**
     * Bootstrap the given application.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     *
     * @codeCoverageIgnore
     */
    public function bootstrap(Application $app): void
    {}
}
