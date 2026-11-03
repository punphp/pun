<?php

namespace Orchestra\Testbench\Workbench;

use Illuminate\Console\Application as Artisan;
use Illuminate\Console\Command;
use Illuminate\Contracts\Foundation\Application as ApplicationContract;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Foundation\Events\DiagnosingHealth;
use Illuminate\Routing\Router;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Event;
use Illuminate\Support\Facades\View;
use Illuminate\Support\Str;
use Orchestra\Sidekick\Env;
use Orchestra\Testbench\Contracts\Config as ConfigContract;
use Orchestra\Testbench\Foundation\Config;
use ReflectionClass;
use Symfony\Component\Finder\Finder;
use Throwable;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Testbench\after_resolving;
use function Orchestra\Testbench\package_path;
use function Orchestra\Testbench\workbench_path;

/**
 * @api
 *
 * @phpstan-import-type TWorkbenchDiscoversConfig from \Orchestra\Testbench\Foundation\Config
 */
class Workbench
{
    /**
     * The cached test case configuration.
     *
     * @var \Orchestra\Testbench\Contracts\Config|null
     */
    protected static ?ConfigContract $cachedConfiguration = null;

    /**
     * Cached namespace by path.
     *
     * @var array<string, string|null>
     */
    protected static array $cachedNamespaces = [];

    /**
     * The cached test case configuration.
     *
     * @var class-string<\Illuminate\Foundation\Auth\User>|false|null
     */
    protected static string|false|null $cachedUserModel = null;

    /**
     * The cached core workbench bindings.
     *
     * @var array{kernel: array{console?: string|null, http?: string|null}, handler: array{exception?: string|null}}
     */
    public static array $cachedCoreBindings = [
        'kernel' => [],
        'handler' => [],
    ];

    /**
     * Start Workbench.
     *
     * @internal
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @param  array<int, string|class-string<\Illuminate\Support\ServiceProvider>>  $providers
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function start(ApplicationContract $app, ConfigContract $config, array $providers = []): void
    {}

    /**
     * Start Workbench with providers.
     *
     * @internal
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function startWithProviders(ApplicationContract $app, ConfigContract $config): void
    {}

    /**
     * Discover Workbench routes.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @return void
     */
    public static function discoverRoutes(ApplicationContract $app, ConfigContract $config): void
    {}

    /**
     * Discover Workbench command routes.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    public static function discoverCommandsRoutes(ApplicationContract $app): void
    {}

    /**
     * Resolve the configuration.
     *
     * @return \Orchestra\Testbench\Contracts\Config
     *
     * @codeCoverageIgnore
     */
    public static function configuration(): ConfigContract
    {}

    /**
     * Get application Console Kernel implementation.
     *
     * @return string|null
     */
    public static function applicationConsoleKernel(): ?string
    {}

    /**
     * Get application HTTP Kernel implementation using Workbench.
     *
     * @return string|null
     */
    public static function applicationHttpKernel(): ?string
    {}

    /**
     * Get application HTTP exception handler using Workbench.
     *
     * @return string|null
     */
    public static function applicationExceptionHandler(): ?string
    {}

    /**
     * Get application User Model
     *
     * @return class-string<\Illuminate\Foundation\Auth\User>|null
     */
    public static function applicationUserModel(): ?string
    {}

    /**
     * Detect namespace by type.
     *
     * @param  string  $type
     * @param  bool  $force
     * @return string|null
     */
    public static function detectNamespace(string $type, bool $force = false): ?string
    {}

    /**
     * Flush the cached configuration.
     *
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function flush(): void
    {}

    /**
     * Flush the cached namespace configuration.
     *
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function flushCachedClassAndNamespaces(): void
    {}
}
