<?php

namespace Orchestra\Testbench\Foundation\Console\Concerns;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Facades\ParallelTesting;
use Illuminate\Support\LazyCollection;
use Orchestra\Sidekick\Env;
use Orchestra\Testbench\Foundation\Console\TerminatingConsole;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Testbench\in_parallel_testing;

/**
 * @codeCoverageIgnore
 */
trait CopyTestbenchFiles
{
    /**
     * Copy the "testbench.yaml" file.
     *
     * @internal
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Illuminate\Filesystem\Filesystem  $filesystem
     * @param  string  $workingPath
     * @param  bool  $resetOnTerminating
     * @return void
     */
    protected function copyTestbenchConfigurationFile(
        Application $app,
        Filesystem $filesystem,
        string $workingPath,
        bool $backupExistingFile = true,
        bool $resetOnTerminating = true
    ): void {}

    /**
     * Copy the ".env" file.
     *
     * @internal
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @param  \Illuminate\Filesystem\Filesystem  $filesystem
     * @param  string  $workingPath
     * @param  bool  $resetOnTerminating
     * @return void
     */
    protected function copyTestbenchDotEnvFile(
        Application $app,
        Filesystem $filesystem,
        string $workingPath,
        bool $backupExistingFile = true,
        bool $resetOnTerminating = true
    ): void {}

    /**
     * Determine the Testbench's environment file.
     *
     * @internal
     *
     * @return string
     */
    protected function testbenchEnvironmentFile(): string
    {}
}
