<?php

namespace Orchestra\Testbench\Bootstrap;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Foundation\Auth\User as Authenticatable;
use Illuminate\Support\Collection;
use Illuminate\Support\LazyCollection;
use Orchestra\Testbench\Workbench\Workbench;
use Symfony\Component\Finder\Finder;

use function Orchestra\Testbench\workbench_path;

/**
 * @internal
 */
class LoadConfigurationWithWorkbench extends LoadConfiguration
{
    /**
     * Determine if workbench config file should be loaded.
     *
     * @var bool
     */
    protected readonly bool $usesWorkbenchConfigFile;

    /**
     * Construct a new bootstrap class.
     */
    public function __construct()
    {}

    /** {@inheritDoc} */
    #[\Override]
    public function bootstrap(Application $app): void
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function resolveConfigurationFile(string $path, string $key): string
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function extendsLoadedConfiguration(Collection $configurations): Collection
    {}
}
