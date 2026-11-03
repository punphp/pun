<?php

namespace Orchestra\Testbench\Features;

use Orchestra\Sidekick\Env;

use function Orchestra\Testbench\container;

/**
 * @internal
 *
 * @codeCoverageIgnore
 */
class ParallelRunner extends \Illuminate\Testing\ParallelRunner
{
    /**
     * Creates the application.
     *
     * @return \Illuminate\Contracts\Foundation\Application
     */
    #[\Override]
    protected function createApplication()
    {}
}
