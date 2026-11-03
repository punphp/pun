<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Console\Command;
use RuntimeException;
use Symfony\Component\Process\Exception\ProcessSignaledException;
use Symfony\Component\Process\Process;

use function Laravel\Prompts\confirm;
use function Orchestra\Sidekick\is_testbench_cli;
use function Orchestra\Testbench\package_path;
use function Orchestra\Testbench\php_binary;

/**
 * @codeCoverageIgnore
 */
class TestFallbackCommand extends Command
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
        {--without-cache : Indicates if cache configuration should be performed}
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

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Install testing needed dependencies.
     *
     * @return void
     */
    protected function installCollisionDependencies(): void
    {}

    /**
     * Get the composer command for the environment.
     *
     * @return string
     */
    protected function findComposer(): string
    {}
}
