<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Support\Collection;
use NunoMaduro\Collision\Adapters\Laravel\Commands\TestCommand as Command;
use Orchestra\Sidekick\Env;
use Orchestra\Testbench\Features\ParallelRunner;
use Symfony\Component\Console\Input\InputOption;

use function Orchestra\Sidekick\is_testbench_cli;
use function Orchestra\Sidekick\package_version_compare;
use function Orchestra\Testbench\defined_environment_variables;
use function Orchestra\Testbench\package_path;

/**
 * @codeCoverageIgnore
 */
class TestCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'package:test
        {--without-tty : Disable output to TTY}
        {--compact : Indicates whether the compact printer should be used}
        {--configuration= : Read configuration from XML file}
        {--coverage : Indicates whether the coverage information should be collected}
        {--min= : Indicates the minimum threshold enforcement for coverage}
        {--p|parallel : Indicates if the tests should run in parallel}
        {--profile : Lists top 10 slowest tests}
        {--recreate-databases : Indicates if the test databases should be re-created}
        {--drop-databases : Indicates if the test databases should be dropped}
        {--without-databases : Indicates if database configuration should be performed}
        {--c|--custom-argument : Add custom env variables}
    ';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Run the package tests';

    /** {@inheritDoc} */
    #[\Override]
    public function configure(): void
    {}

    /** {@inheritDoc} */
    #[\Override]
    public function handle()
    {}

    /**
     * Get the PHPUnit configuration file path.
     *
     * @return string
     */
    public function phpUnitConfigurationFile()
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function phpunitArguments($options)
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function paratestArguments($options)
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function phpunitEnvironmentVariables()
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function paratestEnvironmentVariables()
    {}

    /**
     * Get the configuration file.
     *
     * @return string
     */
    #[\Override]
    protected function getConfigurationFile()
    {}
}
