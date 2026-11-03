<?php

namespace Orchestra\Testbench\Bootstrap;

use Dotenv\Dotenv;
use Illuminate\Support\Facades\ParallelTesting;
use Orchestra\Sidekick\Env;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Testbench\in_parallel_testing;

/**
 * @internal
 */
final class LoadEnvironmentVariables extends \Illuminate\Foundation\Bootstrap\LoadEnvironmentVariables
{
    /** {@inheritDoc} */
    #[\Override]
    protected function createDotenv($app)
    {}
}
