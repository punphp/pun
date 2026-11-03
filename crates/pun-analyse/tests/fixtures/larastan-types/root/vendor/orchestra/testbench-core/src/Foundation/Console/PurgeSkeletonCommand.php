<?php

namespace Orchestra\Testbench\Foundation\Console;

use Illuminate\Console\Command;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use Illuminate\Support\LazyCollection;
use Orchestra\Sidekick\Env;
use Orchestra\Testbench\Contracts\Config as ConfigContract;
use Orchestra\Testbench\Foundation\Actions\DeleteVendorSymlink;
use Orchestra\Testbench\Workbench\Actions\RemoveAssetSymlinkFolders;
use Symfony\Component\Console\Attribute\AsCommand;

use function Orchestra\Sidekick\Filesystem\join_paths;

/**
 * @codeCoverageIgnore
 */
#[AsCommand(name: 'package:purge-skeleton', description: 'Purge skeleton folder to original state')]
class PurgeSkeletonCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'package:purge-skeleton
                                {--pretend : Outputs the operations but will not execute anything}';

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $filesystem
     * @param  \Orchestra\Testbench\Contracts\Config  $config
     * @return int
     */
    public function handle(Filesystem $filesystem, ConfigContract $config)
    {}
}
