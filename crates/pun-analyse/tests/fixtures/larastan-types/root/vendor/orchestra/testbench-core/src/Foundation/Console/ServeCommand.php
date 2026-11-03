<?php

namespace Orchestra\Testbench\Foundation\Console;

use Composer\Config as ComposerConfig;
use Illuminate\Foundation\Auth\User;
use Illuminate\Foundation\Console\ServeCommand as Command;
use Orchestra\Testbench\Foundation\Events\ServeCommandEnded;
use Orchestra\Testbench\Foundation\Events\ServeCommandStarted;
use Orchestra\Testbench\Workbench\Workbench;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Process\Process;

use function Orchestra\Testbench\package_path;

/**
 * @codeCoverageIgnore
 */
class ServeCommand extends Command
{
    /** {@inheritDoc} */
    #[\Override]
    protected function execute(InputInterface $input, OutputInterface $output): int
    {}

    /**
     * Add an environment variable that should be passed through to the server process.
     *
     * @param  string  $name
     * @param  mixed  $value
     * @return void
     */
    protected function addPassThroughEnvironmentVariable(string $name, mixed $value): void
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function startProcess($hasEnvironment)
    {}

    /** {@inhertiDoc} */
    #[\Override]
    public function trap($signals, $callback)
    {}

    /** {@inheritDoc} */
    #[\Override]
    public function option($key = null)
    {}
}
