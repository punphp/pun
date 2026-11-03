<?php

namespace Orchestra\Testbench\Concerns;

use Attribute;
use Closure;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Foundation\Application as LaravelApplication;
use Laravel\SerializableClosure\SerializableClosure;
use Orchestra\Testbench\Attributes\DefineRoute;
use Orchestra\Testbench\Attributes\UsesVendor;
use Orchestra\Testbench\Features\TestingFeature;
use Orchestra\Testbench\Foundation\Bootstrap\SyncTestbenchCachedRoutes;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Testbench\in_parallel_testing;
use function Orchestra\Testbench\refresh_router_lookups;
use function Orchestra\Testbench\remote;

trait HandlesRoutes
{
    use HandlesAssertions;
    use InteractsWithPHPUnit;
    use InteractsWithTestCase;

    /**
     * Indicates if we have made it through the requireApplicationCachedRoutes function.
     *
     * @var bool
     */
    protected bool $requireApplicationCachedRoutesHasRun = false;

    /**
     * Setup routes requirements.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     */
    protected function setUpApplicationRoutes($app): void
    {}

    /**
     * Define routes setup.
     *
     * @api
     *
     * @param  \Illuminate\Routing\Router  $router
     * @return void
     */
    protected function defineRoutes($router)
    {}

    /**
     * Define web routes setup.
     *
     * @api
     *
     * @param  \Illuminate\Routing\Router  $router
     * @return void
     */
    protected function defineWebRoutes($router)
    {}

    /**
     * Define stash routes setup.
     *
     * @api
     *
     * @param  \Closure|string  $route
     * @return void
     */
    protected function defineStashRoutes(Closure|string $route): void
    {}

    /**
     * Define cache routes setup.
     *
     * @api
     *
     * @param  \Closure|string  $route
     * @param  bool  $cached
     * @return void
     */
    protected function defineCacheRoutes(Closure|string $route, bool $cached = true): void
    {}

    /**
     * Require application cached routes.
     *
     * @internal
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @return void
     */
    protected function requireApplicationCachedRoutes(Filesystem $files, bool $cached): void
    {}
}
