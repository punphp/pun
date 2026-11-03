<?php

namespace Orchestra\Testbench\Foundation;

use Illuminate\Console\Application as Artisan;
use Illuminate\Console\Scheduling\ScheduleListCommand;
use Illuminate\Console\Signals;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Migrations\Migrator;
use Illuminate\Database\Schema\Builder as SchemaBuilder;
use Illuminate\Foundation\Bootstrap\HandleExceptions;
use Illuminate\Foundation\Bootstrap\LoadEnvironmentVariables;
use Illuminate\Foundation\Bootstrap\RegisterProviders;
use Illuminate\Foundation\Console\AboutCommand;
use Illuminate\Foundation\Console\ChannelListCommand;
use Illuminate\Foundation\Console\RouteListCommand;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Foundation\Http\Middleware\ConvertEmptyStringsToNull;
use Illuminate\Foundation\Http\Middleware\PreventRequestsDuringMaintenance;
use Illuminate\Foundation\Http\Middleware\TrimStrings;
use Illuminate\Foundation\Http\Middleware\ValidateCsrfToken;
use Illuminate\Http\Middleware\TrustHosts;
use Illuminate\Http\Middleware\TrustProxies;
use Illuminate\Http\Resources\Json\JsonResource;
use Illuminate\Http\Resources\JsonApi\JsonApiResource;
use Illuminate\Mail\Markdown;
use Illuminate\Queue\Console\WorkCommand;
use Illuminate\Queue\Queue;
use Illuminate\Routing\Middleware\ThrottleRequests;
use Illuminate\Support\Arr;
use Illuminate\Support\EncodedHtmlString;
use Illuminate\Support\Once;
use Illuminate\Support\Sleep;
use Illuminate\Support\Str;
use Illuminate\Validation\Validator;
use Illuminate\View\Component;
use Laravel\Prompts\Prompt;
use Orchestra\Testbench\Concerns\CreatesApplication;
use Orchestra\Testbench\Console\Commander;
use Orchestra\Testbench\Contracts\Config as ConfigContract;
use Orchestra\Testbench\Workbench\Workbench;

use function Orchestra\Sidekick\Filesystem\join_paths;

/**
 * @api
 *
 * @phpstan-import-type TExtraConfig from \Orchestra\Testbench\Foundation\Config
 * @phpstan-import-type TOptionalExtraConfig from \Orchestra\Testbench\Foundation\Config
 *
 * @phpstan-type TConfig array{
 *   extra?: TOptionalExtraConfig,
 *   load_environment_variables?: bool,
 *   enabled_package_discoveries?: bool
 * }
 */
class Application
{
    use CreatesApplication {
        resolveApplicationResolvingCallback as protected resolveApplicationResolvingCallbackFromTrait;
        resolveApplicationConfiguration as protected resolveApplicationConfigurationFromTrait;
    }

    /**
     * The Illuminate application instance.
     *
     * @var \Illuminate\Foundation\Application|null
     */
    protected $app;

    /**
     * List of configurations.
     *
     * @var array<string, mixed>
     *
     * @phpstan-var TExtraConfig
     */
    protected array $config = [
        'env' => [],
        'providers' => [],
        'dont-discover' => [],
        'bootstrappers' => [],
    ];

    /**
     * The application resolving callback.
     *
     * @var (callable(\Illuminate\Foundation\Application):(void))|null
     */
    protected $resolvingCallback;

    /**
     * Load Environment variables.
     *
     * @var bool
     */
    protected bool $loadEnvironmentVariables = false;

    /**
     * Create new application resolver.
     *
     * @param  string|null  $basePath
     * @param  (callable(\Illuminate\Foundation\Application):(void))|null  $resolvingCallback
     */
    public function __construct(
        protected readonly ?string $basePath = null,
        ?callable $resolvingCallback = null
    ) {}

    /**
     * Create new application resolver.
     *
     * @param  string|null  $basePath
     * @param  (callable(\Illuminate\Foundation\Application):(void))|null  $resolvingCallback
     * @param  array<string, mixed>  $options
     *
     * @phpstan-param TConfig  $options
     *
     * @return static
     */
    public static function make(?string $basePath = null, ?callable $resolvingCallback = null, array $options = [])
    {}

    /**
     * Create new application resolver from configuration file.
     *
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @param  (callable(\Illuminate\Foundation\Application):(void))|null  $resolvingCallback
     * @param  array<string, mixed>  $options
     *
     * @phpstan-param TConfig  $options
     *
     * @return static
     */
    public static function makeFromConfig(ConfigContract $config, ?callable $resolvingCallback = null, array $options = [])
    {}

    /**
     * Create symlink to vendor path via new application instance.
     *
     * @param  string|null  $basePath
     * @param  string  $workingVendorPath
     * @return \Illuminate\Foundation\Application
     *
     * @codeCoverageIgnore
     */
    public static function createVendorSymlink(?string $basePath, string $workingVendorPath)
    {}

    /**
     * Delete symlink to vendor path via new application instance.
     *
     * @param  string|null  $basePath
     * @return \Illuminate\Foundation\Application
     *
     * @codeCoverageIgnore
     */
    public static function deleteVendorSymlink(?string $basePath)
    {}

    /**
     * Create new application instance.
     *
     * @param  string|null  $basePath
     * @param  (callable(\Illuminate\Foundation\Application):(void))|null  $resolvingCallback
     * @param  array<string, mixed>  $options
     *
     * @phpstan-param TConfig  $options
     *
     * @return \Illuminate\Foundation\Application
     */
    public static function create(?string $basePath = null, ?callable $resolvingCallback = null, array $options = [])
    {}

    /**
     * Create new application instance from configuration file.
     *
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @param  (callable(\Illuminate\Foundation\Application):(void))|null  $resolvingCallback
     * @param  array<string, mixed>  $options
     *
     * @phpstan-param TConfig  $options
     *
     * @return \Illuminate\Foundation\Application
     */
    public static function createFromConfig(ConfigContract $config, ?callable $resolvingCallback = null, array $options = [])
    {}

    /**
     * Flush the application states.
     *
     * @param  \Orchestra\Testbench\Console\Commander|\Orchestra\Testbench\PHPUnit\TestCase  $instance
     * @return void
     */
    public static function flushState(object $instance): void
    {}

    /**
     * Configure the application options.
     *
     * @param  array<string, mixed>  $options
     *
     * @phpstan-param TConfig  $options
     *
     * @return $this
     */
    public function configure(array $options)
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
     * Resolve application resolving callback.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    protected function resolveApplicationResolvingCallback($app): void
    {}

    /**
     * Resolve the application's base path.
     *
     * @api
     *
     * @return string
     */
    protected function getApplicationBasePath()
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
}
