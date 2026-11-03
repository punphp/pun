<?php

namespace Orchestra\Testbench\Bootstrap;

use Generator;
use Illuminate\Config\Repository;
use Illuminate\Contracts\Config\Repository as RepositoryContract;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Support\Collection;
use Illuminate\Support\LazyCollection;
use Orchestra\Sidekick\Env;
use RuntimeException;
use SplFileInfo;
use Symfony\Component\Finder\Finder;

use function Orchestra\Testbench\default_skeleton_path;
use function Orchestra\Testbench\package_path;
use function Orchestra\Testbench\uses_default_skeleton;

/**
 * @internal
 *
 * @phpstan-type TLaravel \Illuminate\Contracts\Foundation\Application
 */
class LoadConfiguration
{
    /**
     * Cached Laravel Framework default configuration.
     *
     * @var \Illuminate\Contracts\Config\Repository|null
     */
    protected static ?RepositoryContract $cachedFrameworkConfigurations = null;

    /**
     * Bootstrap the given application.
     *
     * @param  TLaravel  $app
     * @return void
     */
    public function bootstrap(Application $app): void
    {}

    /**
     * Load the configuration items from all of the files.
     *
     * @param  TLaravel  $app
     * @param  \Illuminate\Contracts\Config\Repository  $config
     * @return void
     */
    private function loadConfigurationFiles(Application $app, RepositoryContract $config): void
    {}

    /**
     * Get the configuration file nesting path.
     *
     * @param  \SplFileInfo  $file
     * @param  string  $configPath
     * @return string
     */
    protected function getNestedDirectory(SplFileInfo $file, string $configPath): string
    {}

    /**
     * Resolve the configuration file.
     *
     * @param  string  $path
     * @param  string  $key
     * @return string
     */
    protected function resolveConfigurationFile(string $path, string $key): string
    {}

    /**
     * Extend the loaded configuration.
     *
     * @param  \Illuminate\Support\Collection  $configurations
     * @return \Illuminate\Support\Collection
     */
    protected function extendsLoadedConfiguration(Collection $configurations): Collection
    {}

    /**
     * Configure the default database connection.
     *
     * @param  \Illuminate\Contracts\Config\Repository  $config
     * @return void
     */
    protected function configureDefaultDatabaseConnection(RepositoryContract $config): void
    {}

    /**
     * Get the application configuration path.
     *
     * @param  TLaravel  $app
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function getConfigurationPath(Application $app): string
    {}

    /**
     * Get the framework default configurations.
     *
     * @return array<string, string>
     *
     * @codeCoverageIgnore
     */
    protected function getFrameworkDefaultConfigurations(): array
    {}

    /**
     * Get the configurations from path.
     *
     * @param  string  $path
     * @return \Generator
     */
    protected function getConfigurationsFromPath(string $path): Generator
    { yield; }

    /**
     * Get the options within the configuration file that should be merged again.
     *
     * @param  string  $name
     * @return array<int, string>
     */
    public static function mergeableOptions(string $name): array
    {}
}
