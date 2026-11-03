<?php

namespace Orchestra\Testbench\Foundation\Actions;

use Illuminate\Contracts\Foundation\Application;

use function Orchestra\Sidekick\Filesystem\is_symlink;

/**
 * @internal
 */
final class DeleteVendorSymlink
{
    /**
     * Execute the command.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return void
     */
    public function handle(Application $app): void
    {}
}
