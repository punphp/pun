<?php

namespace Orchestra\Testbench\Workbench\Actions;

use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Orchestra\Testbench\Contracts\Config as ConfigContract;

use function Orchestra\Sidekick\Filesystem\is_symlink;
use function Orchestra\Testbench\package_path;

/**
 * @internal
 *
 * @codeCoverageIgnore
 */
final class AddAssetSymlinkFolders
{
    /**
     * Construct a new action.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     */
    public function __construct(
        protected Filesystem $files,
        protected ConfigContract $config
    ) {}

    /**
     * Execute the command.
     *
     * @return void
     */
    public function handle(): void
    {}
}
