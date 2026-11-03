<?php

namespace Orchestra\Testbench\Console;

use Illuminate\Console\Concerns\InteractsWithSignals;
use Illuminate\Contracts\Console\Kernel as ConsoleKernel;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Foundation\Application as LaravelApplication;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Orchestra\Testbench\Foundation\Application as Testbench;
use Orchestra\Testbench\Foundation\Bootstrap\LoadMigrationsFromArray;
use Orchestra\Testbench\Foundation\Config;
use Orchestra\Testbench\Foundation\Console\Concerns\CopyTestbenchFiles;
use Orchestra\Testbench\Foundation\Console\Signals;
use Orchestra\Testbench\Foundation\Console\TerminatingConsole;
use Orchestra\Testbench\Foundation\TestbenchServiceProvider;
use Orchestra\Testbench\Workbench\Workbench;
use Symfony\Component\Console\Application as ConsoleApplication;
use Symfony\Component\Console\Input\ArgvInput;
use Symfony\Component\Console\Output\ConsoleOutput;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\SignalRegistry\SignalRegistry;
use Throwable;

use function Orchestra\Sidekick\Filesystem\is_symlink;
use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Sidekick\transform_relative_path;

/**
 * @phpstan-import-type TConfig from \Orchestra\Testbench\Foundation\Config
 *
 * @codeCoverageIgnore
 */
class Commander
{
    use CopyTestbenchFiles;
    use InteractsWithSignals;

    /**
     * Application instance.
     *
     * @var \Illuminate\Foundation\Application|null
     */
    protected $app;

    /**
     * List of configurations.
     *
     * @var \Orchestra\Testbench\Foundation\Config
     */
    protected readonly Config $config;

    /**
     * The environment file name.
     *
     * @var string
     */
    protected string $environmentFile = '.env';

    /**
     * The testbench implementation class.
     *
     * @var class-string<\Orchestra\Testbench\Foundation\Application>
     */
    protected static string $testbench = Testbench::class;

    /**
     * List of providers.
     *
     * @var array<int, class-string<\Illuminate\Support\ServiceProvider>>
     */
    protected array $providers = [
        TestbenchServiceProvider::class,
    ];

    /**
     * Construct a new Commander.
     *
     * @param  \Orchestra\Testbench\Foundation\Config|array  $config
     * @param  string  $workingPath
     *
     * @phpstan-param \Orchestra\Testbench\Foundation\Config|TConfig  $config
     */
    public function __construct(
        Config|array $config,
        protected readonly string $workingPath
    ) {}

    /**
     * Handle the command.
     *
     * @return void
     */
    public function handle(): void
    {}

    /**
     * Create a Laravel application.
     *
     * @return \Illuminate\Foundation\Application
     */
    public function laravel()
    {}

    /**
     * Resolve application implementation callback.
     *
     * @return \Closure(\Illuminate\Foundation\Application): void
     */
    protected function resolveApplicationCallback()
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
     * Get the application's base path.
     *
     * @api
     *
     * @return string
     */
    public static function applicationBasePath()
    {}

    /**
     * Render an exception to the console.
     *
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @param  \Throwable  $error
     * @return int
     */
    protected function handleException(OutputInterface $output, Throwable $error): int
    {}

    /**
     * Prepare command signals.
     *
     * @return void
     */
    protected function prepareCommandSignals(): void
    {}
}
