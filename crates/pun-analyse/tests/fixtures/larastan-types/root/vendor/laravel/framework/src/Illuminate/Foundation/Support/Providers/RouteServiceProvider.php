<?php

namespace Illuminate\Foundation\Support\Providers;

use Closure;
use Illuminate\Contracts\Routing\UrlGenerator;
use Illuminate\Routing\Router;
use Illuminate\Support\ServiceProvider;
use Illuminate\Support\Traits\ForwardsCalls;

/**
 * @mixin \Illuminate\Routing\Router
 */
class RouteServiceProvider extends ServiceProvider
{
    use ForwardsCalls;

    /**
     * The controller namespace for the application.
     *
     * @var string|null
     */
    protected $namespace;

    /**
     * The callback that should be used to load the application's routes.
     *
     * @var \Closure|null
     */
    protected $loadRoutesUsing;

    /**
     * The global callback that should be used to load the application's routes.
     *
     * @var \Closure|null
     */
    protected static $alwaysLoadRoutesUsing;

    /**
     * The callback that should be used to load the application's cached routes.
     *
     * @var \Closure|null
     */
    protected static $alwaysLoadCachedRoutesUsing;

    /**
     * Register any application services.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Bootstrap any application services.
     *
     * @return void
     */
    public function boot()
    {}

    /**
     * Register the callback that will be used to load the application's routes.
     *
     * @param  \Closure  $routesCallback
     * @return $this
     */
    protected function routes(Closure $routesCallback)
    {}

    /**
     * Register the callback that will be used to load the application's routes.
     *
     * @param  \Closure|null  $routesCallback
     * @return void
     */
    public static function loadRoutesUsing(?Closure $routesCallback)
    {}

    /**
     * Register the callback that will be used to load the application's cached routes.
     *
     * @param  \Closure|null  $routesCallback
     * @return void
     */
    public static function loadCachedRoutesUsing(?Closure $routesCallback)
    {}

    /**
     * Set the root controller namespace for the application.
     *
     * @return void
     */
    protected function setRootControllerNamespace()
    {}

    /**
     * Determine if the application routes are cached.
     *
     * @return bool
     */
    protected function routesAreCached()
    {}

    /**
     * Load the cached routes for the application.
     *
     * @return void
     */
    protected function loadCachedRoutes()
    {}

    /**
     * Load the application routes.
     *
     * @return void
     */
    protected function loadRoutes()
    {}

    /**
     * Pass dynamic methods onto the router instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
