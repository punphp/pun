<?php

namespace Orchestra\Testbench\Foundation\Actions;

use ErrorException;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Filesystem\Filesystem;

use function Orchestra\Testbench\laravel_vendor_exists;

/**
 * @internal
 */
final class CreateVendorSymlink
{
    /**
     * Construct a new action.
     *
     * @param  string  $workingPath
     */
    public function __construct(
        protected readonly string $workingPath
    ) {}

    /**
     * Execute the command.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    public function handle(Application $app): void
    {}
}
