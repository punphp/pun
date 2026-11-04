<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Console\Application as Artisan;
use Illuminate\Contracts\Foundation\CachesConfiguration;
use Illuminate\Contracts\Foundation\CachesRoutes;
use Illuminate\Contracts\Support\DeferrableProvider;
use Illuminate\Database\Eloquent\Factory as ModelFactory;
use Illuminate\View\Compilers\BladeCompiler;

/**
 * @property array<string, string> $bindings All of the container bindings that should be registered.
 * @property array<array-key, string> $singletons All of the singletons that should be registered.
 */
abstract class ServiceProvider
{
    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * All of the registered booting callbacks.
     *
     * @var array
     */
    protected $bootingCallbacks = [];

    /**
     * All of the registered booted callbacks.
     *
     * @var array
     */
    protected $bootedCallbacks = [];

    /**
     * The paths that should be published.
     *
     * @var array
     */
    public static $publishes = [];

    /**
     * The paths that should be published by group.
     *
     * @var array
     */
    public static $publishGroups = [];

    /**
     * The migration paths available for publishing.
     *
     * @var array
     */
    protected static $publishableMigrationPaths = [];

    /**
     * Commands that should be run during the "optimize" command.
     *
     * @var array<string, string>
     */
    public static array $optimizeCommands = [];

    /**
     * Commands that should be run during the "optimize:clear" command.
     *
     * @var array<string, string>
     */
    public static array $optimizeClearCommands = [];

    /**
     * Commands that should be run during the "reload" command.
     *
     * @var array<string, string>
     */
    public static array $reloadCommands = [];

    /**
     * Create a new service provider instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Register any application services.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register a booting callback to be run before the "boot" method is called.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function booting(Closure $callback)
    {}

    /**
     * Register a booted callback to be run after the "boot" method is called.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function booted(Closure $callback)
    {}

    /**
     * Call the registered booting callbacks.
     *
     * @return void
     */
    public function callBootingCallbacks()
    {}

    /**
     * Call the registered booted callbacks.
     *
     * @return void
     */
    public function callBootedCallbacks()
    {}

    /**
     * Merge the given configuration with the existing configuration.
     *
     * @param  string  $path
     * @param  string  $key
     * @return void
     */
    protected function mergeConfigFrom($path, $key)
    {}

    /**
     * Replace the given configuration with the existing configuration recursively.
     *
     * @param  string  $path
     * @param  string  $key
     * @return void
     */
    protected function replaceConfigRecursivelyFrom($path, $key)
    {}

    /**
     * Load the given routes file if routes are not already cached.
     *
     * @param  string  $path
     * @return void
     */
    protected function loadRoutesFrom($path)
    {}

    /**
     * Register a view file namespace.
     *
     * @param  string|array  $path
     * @param  string  $namespace
     * @return void
     */
    protected function loadViewsFrom($path, $namespace)
    {}

    /**
     * Register the given view components with a custom prefix.
     *
     * @param  string  $prefix
     * @param  array  $components
     * @return void
     */
    protected function loadViewComponentsAs($prefix, array $components)
    {}

    /**
     * Register a translation file namespace or path.
     *
     * @param  string  $path
     * @param  string|null  $namespace
     * @return void
     */
    protected function loadTranslationsFrom($path, $namespace = null)
    {}

    /**
     * Register a JSON translation file path.
     *
     * @param  string  $path
     * @return void
     */
    protected function loadJsonTranslationsFrom($path)
    {}

    /**
     * Register database migration paths.
     *
     * @param  array|string  $paths
     * @return void
     */
    protected function loadMigrationsFrom($paths)
    {}

    /**
     * Register Eloquent model factory paths.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @param  array|string  $paths
     * @return void
     */
    protected function loadFactoriesFrom($paths)
    {}

    /**
     * Setup an after resolving listener, or fire immediately if already resolved.
     *
     * @param  string  $name
     * @param  callable  $callback
     * @return void
     */
    protected function callAfterResolving($name, $callback)
    {}

    /**
     * Register migration paths to be published by the publish command.
     *
     * @param  array  $paths
     * @param  mixed  $groups
     * @return void
     */
    protected function publishesMigrations(array $paths, $groups = null)
    {}

    /**
     * Register paths to be published by the publish command.
     *
     * @param  array  $paths
     * @param  mixed  $groups
     * @return void
     */
    protected function publishes(array $paths, $groups = null)
    {}

    /**
     * Ensure the publish array for the service provider is initialized.
     *
     * @param  string  $class
     * @return void
     */
    protected function ensurePublishArrayInitialized($class)
    {}

    /**
     * Add a publish group / tag to the service provider.
     *
     * @param  string  $group
     * @param  array  $paths
     * @return void
     */
    protected function addPublishGroup($group, $paths)
    {}

    /**
     * Get the paths to publish.
     *
     * @param  string|null  $provider
     * @param  string|null  $group
     * @return array
     */
    public static function pathsToPublish($provider = null, $group = null)
    {}

    /**
     * Get the paths for the provider or group (or both).
     *
     * @param  string|null  $provider
     * @param  string|null  $group
     * @return array
     */
    protected static function pathsForProviderOrGroup($provider, $group)
    {}

    /**
     * Get the paths for the provider and group.
     *
     * @param  string  $provider
     * @param  string  $group
     * @return array
     */
    protected static function pathsForProviderAndGroup($provider, $group)
    {}

    /**
     * Get the service providers available for publishing.
     *
     * @return array
     */
    public static function publishableProviders()
    {}

    /**
     * Get the migration paths available for publishing.
     *
     * @return array
     */
    public static function publishableMigrationPaths()
    {}

    /**
     * Get the groups available for publishing.
     *
     * @return array
     */
    public static function publishableGroups()
    {}

    /**
     * Register the package's custom Artisan commands.
     *
     * @param  mixed  $commands
     * @return void
     */
    public function commands($commands)
    { func_get_args(); }

    /**
     * Register commands that should run on "optimize" or "optimize:clear".
     *
     * @param  string|null  $optimize
     * @param  string|null  $clear
     * @param  string|null  $key
     * @return void
     */
    protected function optimizes(?string $optimize = null, ?string $clear = null, ?string $key = null)
    {}

    /**
     * Register commands that should run on "reload".
     *
     * @param  string  $reload
     * @param  string|null  $key
     * @return void
     */
    protected function reloads(string $reload, ?string $key = null)
    {}

    /**
     * Get a short descriptive key for the current service provider.
     *
     * @param  string|null  $key
     * @return string
     */
    protected function getProviderKey(?string $key = null): string
    {}

    /**
     * Get the services provided by the provider.
     *
     * @return array
     */
    public function provides()
    {}

    /**
     * Get the events that trigger this service provider to register.
     *
     * @return array
     */
    public function when()
    {}

    /**
     * Determine if the provider is deferred.
     *
     * @return bool
     */
    public function isDeferred()
    {}

    /**
     * Get the default providers for a Laravel application.
     *
     * @return \Illuminate\Support\DefaultProviders
     */
    public static function defaultProviders()
    {}

    /**
     * Add the given provider to the application's provider bootstrap file.
     *
     * @param  string  $provider
     * @param  string|null  $path
     * @return bool
     */
    public static function addProviderToBootstrapFile(string $provider, ?string $path = null)
    {}

    /**
     * Remove a provider from the application's provider bootstrap file.
     *
     * @param  string|array  $providersToRemove
     * @param  string|null  $path
     * @param  bool  $strict
     * @return bool
     */
    public static function removeProviderFromBootstrapFile(string|array $providersToRemove, ?string $path = null, bool $strict = false)
    {}
}
