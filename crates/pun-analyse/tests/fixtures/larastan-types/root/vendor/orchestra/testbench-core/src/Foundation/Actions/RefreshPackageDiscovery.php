<?php

namespace Orchestra\Testbench\Foundation\Actions;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Foundation\PackageManifest;

use function Orchestra\Sidekick\Filesystem\join_paths;

/**
 * @internal
 */
final class RefreshPackageDiscovery
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
