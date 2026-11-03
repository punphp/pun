<?php

namespace Orchestra\Testbench\Foundation;

use Illuminate\Support\Arr;
use Illuminate\Support\LazyCollection;
use Orchestra\Sidekick\FluentDecorator;
use Orchestra\Testbench\Contracts\Config as ConfigContract;
use Symfony\Component\Yaml\Yaml;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Sidekick\transform_relative_path;
use function Orchestra\Testbench\parse_environment_variables;

/**
 * @api
 *
 * @phpstan-type TExtraConfig array{
 *   env: array,
 *   providers: array<int, class-string>,
 *   dont-discover: array<int, string>,
 *   bootstrappers: array<int, class-string>|class-string|null
 * }
 * @phpstan-type TOptionalExtraConfig array{
 *   env?: array,
 *   providers?: array<int, class-string>,
 *   dont-discover?: array<int, string>,
 *   bootstrappers?: array<int, class-string>|class-string|null
 * }
 * @phpstan-type TPurgeConfig array{
 *   directories: array<int, string>,
 *   files: array<int, string>
 * }
 * @phpstan-type TOptionalPurgeConfig array{
 *   directories?: array<int, string>,
 *   files?: array<int, string>
 * }
 * @phpstan-type TWorkbenchConfig array{
 *   start: string,
 *   user: string|int|null,
 *   guard: string|null,
 *   install: bool,
 *   auth: bool,
 *   welcome: bool|null,
 *   health: bool|null,
 *   sync: array<int, array{from: string, to: string, reverse?: bool}>,
 *   build: array<int|string, array<string, mixed>|string>,
 *   assets: array<int, string>,
 *   discovers: TWorkbenchDiscoversConfig
 * }
 * @phpstan-type TOptionalWorkbenchConfig array{
 *   start?: string,
 *   user?: string|int|null,
 *   guard?: string|null,
 *   install?: bool,
 *   auth?: bool,
 *   welcome?: bool|null,
 *   health?: bool|null,
 *   sync?: array<int, array{from: string, to: string, reverse?: bool}>,
 *   build?: array<int|string, array<string, mixed>|string>,
 *   assets?: array<int, string>,
 *   discovers?: TWorkbenchOptionalDiscoversConfig
 * }
 * @phpstan-type TWorkbenchDiscoversConfig array{
 *   config: bool,
 *   factories: bool,
 *   web: bool,
 *   api: bool,
 *   commands: bool,
 *   components: bool,
 *   views: bool
 * }
 * @phpstan-type TWorkbenchOptionalDiscoversConfig array{
 *   config?: bool,
 *   factories?: bool,
 *   web?: bool,
 *   api?: bool,
 *   commands?: bool,
 *   components?: bool,
 *   views?: bool
 * }
 * @phpstan-type TConfig array{
 *   laravel: string|null,
 *   env: array,
 *   providers: array<int, class-string>,
 *   dont-discover: array<int, string>,
 *   bootstrappers: array<int, class-string>|class-string|null,
 *   migrations: array<int, string>|bool|string,
 *   seeders: array<int, class-string>|bool|class-string,
 *   purge: TOptionalPurgeConfig,
 *   workbench: TOptionalWorkbenchConfig
 * }
 * @phpstan-type TOptionalConfig array{
 *   laravel?: string|null,
 *   env?: array,
 *   providers?: array<int, class-string>,
 *   dont-discover?: array<int, string>,
 *   bootstrappers?: array<int, class-string>|class-string|null,
 *   migrations?: array<int, string>|bool|string,
 *   seeders?: array<int, class-string>|bool|class-string,
 *   purge?: TOptionalPurgeConfig|null,
 *   workbench?: TOptionalWorkbenchConfig|null
 * }
 */
class Config extends FluentDecorator implements ConfigContract
{
    /**
     * All of the attributes set on the fluent instance.
     *
     * @var array<string, mixed>
     *
     * @phpstan-var TConfig
     */
    protected array $defaultAttributes = [
        'laravel' => null,
        'env' => [],
        'providers' => [],
        'dont-discover' => [],
        'migrations' => [],
        'seeders' => false,
        'bootstrappers' => [],
        'purge' => [],
        'workbench' => [],
    ];

    /**
     * The Workbench default configuration.
     *
     * @var array<string, array<int, string>>
     *
     * @phpstan-var TPurgeConfig
     */
    protected array $purgeConfig = [
        'directories' => [],
        'files' => [],
    ];

    /**
     * The Workbench default configuration.
     *
     * @var array<string, mixed>
     *
     * @phpstan-var TWorkbenchConfig
     */
    protected array $workbenchConfig = [
        'start' => '/',
        'user' => null,
        'guard' => null,
        'install' => true,
        'auth' => false,
        'welcome' => null,
        'health' => null,
        'sync' => [],
        'build' => [],
        'assets' => [],
        'discovers' => [
            'config' => false,
            'factories' => false,
            'web' => false,
            'api' => false,
            'commands' => false,
            'components' => false,
            'views' => false,
        ],
    ];

    /**
     * The Workbench discovers default configuration.
     *
     * @var array<string, mixed>
     *
     * @phpstan-var TWorkbenchDiscoversConfig
     */
    protected array $workbenchDiscoversConfig = [
        'config' => false,
        'factories' => false,
        'web' => false,
        'api' => false,
        'commands' => false,
        'components' => false,
        'views' => false,
    ];

    /**
     * The cached configuration used during tests.
     *
     * @var static|null
     */
    protected static $cachedConfiguration;

    /**
     * Construct a new Config instance.
     *
     * @param  iterable<string, mixed>  $attributes
     *
     * @phpstan-param TOptionalConfig $attributes
     */
    public function __construct($attributes = [])
    {}

    /**
     * Load configuration from Yaml file.
     *
     * @param  string  $workingPath
     * @param  string|null  $filename
     * @param  array<string, mixed>  $defaults
     * @return static
     */
    public static function loadFromYaml(string $workingPath, ?string $filename = 'testbench.yaml', array $defaults = [])
    {}

    /**
     * Load (and cache) configuration from Yaml file.
     *
     * @param  string  $workingPath
     * @param  string|null  $filename
     * @param  array<string, mixed>  $defaults
     * @return static
     *
     * @codeCoverageIgnore
     */
    public static function cacheFromYaml(string $workingPath, ?string $filename = 'testbench.yaml', array $defaults = [])
    {}

    /**
     * Add additional service providers.
     *
     * @param  array<int, class-string<\Illuminate\Support\ServiceProvider>>  $providers
     * @return $this
     */
    public function addProviders(array $providers)
    {}

    /**
     * Get extra attributes.
     *
     * @return array<string, mixed>
     *
     * @phpstan-return TExtraConfig
     */
    public function getExtraAttributes(): array
    {}

    /**
     * Get purge attributes.
     *
     * @return array<string, mixed>
     *
     * @phpstan-return TPurgeConfig
     */
    public function getPurgeAttributes(): array
    {}

    /**
     * Get workbench attributes.
     *
     * @return array<string, mixed>
     *
     * @phpstan-return TWorkbenchConfig
     */
    public function getWorkbenchAttributes(): array
    {}

    /**
     * Get workbench discovers attributes.
     *
     * @return array<string, mixed>
     *
     * @phpstan-return TWorkbenchDiscoversConfig
     */
    public function getWorkbenchDiscoversAttributes(): array
    {}
}
