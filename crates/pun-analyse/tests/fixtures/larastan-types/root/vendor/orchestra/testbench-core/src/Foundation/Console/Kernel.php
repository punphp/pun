<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Foundation\Console\Kernel as ConsoleKernel;

abstract class Kernel extends ConsoleKernel
{
    /**
     * Get the bootstrap classes for the application.
     *
     * @return array
     */
    #[\Override]
    protected function bootstrappers()
    {}

    /**
     * Determine if the kernel should discover commands.
     *
     * @return bool
     */
    #[\Override]
    protected function shouldDiscoverCommands()
    {}
}
