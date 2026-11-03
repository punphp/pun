<?php

namespace Illuminate\Foundation\Configuration;

use Closure;
use Illuminate\Console\Application as Artisan;
use Illuminate\Console\Scheduling\Schedule;
use Illuminate\Contracts\Console\Kernel as ConsoleKernel;
use Illuminate\Contracts\Http\Kernel as HttpKernel;
use Illuminate\Foundation\Application;
use Illuminate\Foundation\Bootstrap\RegisterProviders;
use Illuminate\Foundation\Http\HealthCheck;
use Illuminate\Foundation\Http\Middleware\PreventRequestsDuringMaintenance;
use Illuminate\Foundation\Support\Providers\EventServiceProvider as AppEventServiceProvider;
use Illuminate\Foundation\Support\Providers\RouteServiceProvider as AppRouteServiceProvider;
use Illuminate\Http\Middleware\PrefersJsonResponses;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Broadcast;
use Illuminate\Support\Facades\Route;
use Laravel\Folio\Folio;

class ApplicationBuilder
{
    /**
     * The service provider that are marked for registration.
     *
     * @var array
     */
    protected array $pendingProviders = [];

    /**
     * Any additional routing callbacks that should be invoked while registering routes.
     *
     * @var array
     */
    protected array $additionalRoutingCallbacks = [];

    /**
     * The Folio / page middleware that have been defined by the user.
     *
     * @var array
     */
    protected array $pageMiddleware = [];

    /**
     * Create a new application builder instance.
     */
    public function __construct(protected Application $app)
    {}

    /**
     * Register the standard kernel classes for the application.
     *
     * @return $this
     */
    public function withKernels()
    {}

    /**
     * Register additional service providers.
     *
     * @param  array  $providers
     * @param  bool  $withBootstrapProviders
     * @return $this
     */
    public function withProviders(array $providers = [], bool $withBootstrapProviders = true)
    {}

    /**
     * Register the core event service provider for the application.
     *
     * @param  iterable<int, string>|bool  $discover
     * @return $this
     */
    public function withEvents(iterable|bool $discover = true)
    {}

    /**
     * Register the broadcasting services for the application.
     *
     * @param  string  $channels
     * @param  array  $attributes
     * @return $this
     */
    public function withBroadcasting(string $channels, array $attributes = [])
    {}

    /**
     * Register the routing services for the application.
     *
     * @param  \Closure|null  $using
     * @param  array|string|null  $web
     * @param  array|string|null  $api
     * @param  string|null  $commands
     * @param  string|null  $channels
     * @param  string|null  $pages
     * @param  string|null  $health
     * @param  string  $apiPrefix
     * @param  callable|null  $then
     * @return $this
     */
    public function withRouting(?Closure $using = null,
        array|string|null $web = null,
        array|string|null $api = null,
        ?string $commands = null,
        ?string $channels = null,
        ?string $pages = null,
        ?string $health = null,
        string $apiPrefix = 'api',
        ?callable $then = null)
    {}

    /**
     * Create the routing callback for the application.
     *
     * @param  array|string|null  $web
     * @param  array|string|null  $api
     * @param  string|null  $pages
     * @param  string|null  $health
     * @param  string  $apiPrefix
     * @param  callable|null  $then
     * @return \Closure
     *
     * @throws \Throwable
     */
    protected function buildRoutingCallback(array|string|null $web,
        array|string|null $api,
        ?string $pages,
        ?string $health,
        string $apiPrefix,
        ?callable $then)
    {}

    /**
     * Register the global middleware, middleware groups, and middleware aliases for the application.
     *
     * @param  callable|null  $callback
     * @return $this
     */
    public function withMiddleware(?callable $callback = null)
    {}

    /**
     * Register additional Artisan commands with the application.
     *
     * @param  array  $commands
     * @return $this
     */
    public function withCommands(array $commands = [])
    {}

    /**
     * Register additional Artisan route paths.
     *
     * @param  array  $paths
     * @return $this
     */
    protected function withCommandRouting(array $paths)
    {}

    /**
     * Register the scheduled tasks for the application.
     *
     * @param  callable(\Illuminate\Console\Scheduling\Schedule $schedule): void  $callback
     * @return $this
     */
    public function withSchedule(callable $callback)
    {}

    /**
     * Register and configure the application's exception handler.
     *
     * @param  (callable(\Illuminate\Foundation\Configuration\Exceptions): mixed)|null  $using
     * @return $this
     */
    public function withExceptions(?callable $using = null)
    {}

    /**
     * Register an array of container bindings to be bound when the application is booting.
     *
     * @param  array  $bindings
     * @return $this
     */
    public function withBindings(array $bindings)
    {}

    /**
     * Register an array of singleton container bindings to be bound when the application is booting.
     *
     * @param  array  $singletons
     * @return $this
     */
    public function withSingletons(array $singletons)
    {}

    /**
     * Register an array of scoped singleton container bindings to be bound when the application is booting.
     *
     * @param  array  $scopedSingletons
     * @return $this
     */
    public function withScopedSingletons(array $scopedSingletons)
    {}

    /**
     * Globally prefer JSON responses when the incoming "Accept" header is broad.
     *
     * @param  bool  $prefer
     * @return $this
     */
    public function prefersJsonResponses(bool $prefer = true)
    {}

    /**
     * Register a callback to be invoked when the application's service providers are registered.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function registered(callable $callback)
    {}

    /**
     * Register a callback to be invoked when the application is "booting".
     *
     * @param  callable  $callback
     * @return $this
     */
    public function booting(callable $callback)
    {}

    /**
     * Register a callback to be invoked when the application is "booted".
     *
     * @param  callable  $callback
     * @return $this
     */
    public function booted(callable $callback)
    {}

    /**
     * Get the application instance.
     *
     * @return \Illuminate\Foundation\Application
     */
    public function create()
    {}
}
