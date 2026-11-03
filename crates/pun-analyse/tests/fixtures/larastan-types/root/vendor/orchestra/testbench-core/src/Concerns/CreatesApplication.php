<?php

namespace Orchestra\Testbench\Concerns;

use Illuminate\Cache\RateLimiting\Limit;
use Illuminate\Contracts\Console\Kernel as ConsoleKernelContract;
use Illuminate\Contracts\Http\Kernel as HttpKernelContract;
use Illuminate\Foundation\Application;
use Illuminate\Foundation\Configuration\ApplicationBuilder;
use Illuminate\Foundation\Configuration\Middleware;
use Illuminate\Http\Request;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Facade;
use Illuminate\Support\Facades\RateLimiter;
use Illuminate\Support\ServiceProvider;
use Orchestra\Testbench\Attributes\DefineEnvironment;
use Orchestra\Testbench\Attributes\RequiresEnv;
use Orchestra\Testbench\Attributes\RequiresLaravel;
use Orchestra\Testbench\Attributes\ResolvesLaravel;
use Orchestra\Testbench\Attributes\UsesFrameworkConfiguration;
use Orchestra\Testbench\Attributes\WithConfig;
use Orchestra\Testbench\Attributes\WithEnv;
use Orchestra\Testbench\Attributes\WithImmutableDates;
use Orchestra\Testbench\Bootstrap\LoadEnvironmentVariables;
use Orchestra\Testbench\Bootstrap\RegisterProviders;
use Orchestra\Testbench\Features\TestingFeature;
use Orchestra\Testbench\Foundation\PackageManifest;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;

use function Orchestra\Sidekick\after_resolving;
use function Orchestra\Testbench\default_skeleton_path;
use function Orchestra\Testbench\refresh_router_lookups;

/**
 * @property bool|null $enablesPackageDiscoveries
 * @property bool|null $loadEnvironmentVariables
 */
trait CreatesApplication
{
    use InteractsWithWorkbench;
    use WithLaravelBootstrapFile;

    /**
     * Get the application's base path.
     *
     * @api
     *
     * @return string
     */
    public static function applicationBasePath()
    {}

    /**
     * Ignore package discovery from.
     *
     * @api
     *
     * @return array<int, string>
     */
    public function ignorePackageDiscoveriesFrom()
    {}

    /**
     * Get the application timezone.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return string|null
     */
    protected function getApplicationTimezone($app)
    {}

    /**
     * Override application bindings.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<string|class-string, string|class-string>
     */
    protected function overrideApplicationBindings($app)
    {}

    /**
     * Resolve application bindings.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    final protected function resolveApplicationBindings($app): void
    {}

    /**
     * Get application aliases.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<string, class-string>
     */
    protected function getApplicationAliases($app)
    {}

    /**
     * Override application aliases.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<string, class-string|false>
     */
    protected function overrideApplicationAliases($app)
    {}

    /**
     * Resolve application aliases.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<string, class-string>
     */
    final protected function resolveApplicationAliases($app): array
    {}

    /**
     * Get package aliases.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<string, class-string>
     */
    protected function getPackageAliases($app)
    {}

    /**
     * Get package bootstrapper.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<int, class-string>
     */
    protected function getPackageBootstrappers($app)
    {}

    /**
     * Get application providers.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<int, class-string>
     */
    protected function getApplicationProviders($app)
    {}

    /**
     * Override application aliases.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<class-string, class-string|false>
     */
    protected function overrideApplicationProviders($app)
    {}

    /**
     * Resolve application aliases.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<int, class-string>
     */
    final protected function resolveApplicationProviders($app): array
    {}

    /**
     * Get package providers.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<int, class-string>
     */
    protected function getPackageProviders($app)
    {}

    /**
     * Resolve the application's base path.
     *
     * @internal
     *
     * @return string
     */
    protected function getApplicationBasePath()
    {}

    /**
     * Creates the application.
     *
     * @internal
     *
     * @return \Illuminate\Foundation\Application
     */
    public function createApplication()
    {}

    /**
     * Create the default application implementation.
     *
     * @internal
     *
     * @return \Illuminate\Foundation\Application
     */
    final protected function resolveDefaultApplication()
    {}

    /**
     * Resolve application implementation.
     *
     * @api
     *
     * @return \Illuminate\Foundation\Application
     */
    protected function resolveApplication()
    {}

    /**
     * Resolve application resolving callback.
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationResolvingCallback($app): void
    {}

    /**
     * Resolve application facades implementation.
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationFacades($app)
    {}

    /**
     * Resolve application core environment variables implementation.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationEnvironmentVariables($app)
    {}

    /**
     * Resolve application core configuration implementation.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationConfiguration($app)
    {}

    /**
     * Resolve application core implementation.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationCore($app)
    {}

    /**
     * Resolve application Console Kernel implementation.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationConsoleKernel($app)
    {}

    /**
     * Resolve application HTTP Kernel implementation.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationHttpKernel($app)
    {}

    /**
     * Resolve application HTTP default middlewares.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationHttpMiddlewares($app)
    {}

    /**
     * Resolve application HTTP exception handler.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationExceptionHandler($app)
    {}

    /**
     * Resolve application bootstrapper.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationBootstrappers($app)
    {}

    /**
     * Refresh route name lookup for the application.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    final protected function refreshApplicationRouteNameLookups($app): void
    {}

    /**
     * Resolve application rate limiting configuration.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationRateLimiting($app)
    {}

    /**
     * Reset artisan commands for the application.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    final protected function resetApplicationArtisanCommands($app): void
    {}

    /**
     * Define environment setup.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function defineEnvironment($app)
    {}

    /**
     * Define environment setup.
     *
     * @api
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     *
     * @deprecated 10.0 Use "defineEnvironment()" instead.
     */
    protected function getEnvironmentSetUp($app)
    {}
}
